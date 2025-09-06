use crate::runner::{CommandModule, audio::dsp::DspModule};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, SizedSample, StreamConfig};
use fundsp::hacker32::*;
use mlua::Lua;
use std::collections::HashMap;
use std::path::PathBuf;

mod dsp;

pub const EXPORT_SAMPLE_RATE: f64 = 44100.0;
pub const SAMPLES_PER_UPDATE: u64 = 44;
const LUA_MODULE: &str = include_str!("audio.luau");

pub struct ExportManager {
    export_path: Option<PathBuf>,
    export_wave: Wave,
    audio_graph: Box<dyn AudioUnit>,
}

impl ExportManager {
    pub fn new(export_pathbuf: Option<PathBuf>) -> ExportManager {
        ExportManager {
            export_path: export_pathbuf,
            export_wave: Wave::new(2, EXPORT_SAMPLE_RATE),
            // Initialize with empty sequencer backend
            // If exportmanager is used, this will be replaced.
            audio_graph: Box::new(Sequencer::new(false, 2).backend()),
        }
    }

    // If export_path exists, we can export, therefore
    // we are exporting. If export_path is None, we are
    // playing live.
    pub fn is_live(&self) -> bool {
        self.export_path.is_none()
    }

    // Setup the export environment
    pub fn init(&mut self, new_audio_graph: Box<dyn AudioUnit>) {
        // Setup audio graph
        self.audio_graph = new_audio_graph;
    }

    // Update the wave file with new samples
    pub fn update(&mut self) {
        // Push new samples to the wave
        for _ in 0..SAMPLES_PER_UPDATE {
            self.export_wave.push(self.audio_graph.get_stereo())
        }
    }

    pub fn export(&mut self) {
        let mut export_path = self
            .export_path
            .clone()
            .expect("Tried to export without a path somehow...");
        export_path.push("export.wav");
        let _ = self
            .export_wave
            .save_wav32(export_path)
            .expect("Ran into an issue exporting: ");

        println!("Successfully exported wave file.");
    }
}

pub struct AudioModule {
    sequencer: Sequencer,
    // NOTE: Because fundsp doesn't expose any manners in which EventId can be
    // created from a non-eventid class, this event_map serves as a hashmap of
    // the debug output -> the event id. Its ugly, it uses ~800 mb of ram per
    // 1 million notes played. Which feels "good enough" for now.
    event_map: HashMap<String, EventId>,
    // Modules
    dsp: DspModule,
    // Utility struct for exporting to a .wav file
    export_manager: ExportManager,
}

impl AudioModule {
    pub fn new(samples: &HashMap<String, Wave>, export: Option<PathBuf>) -> AudioModule {
        AudioModule {
            sequencer: Sequencer::new(false, 2),
            event_map: HashMap::new(),
            dsp: DspModule::new(samples.clone()),
            export_manager: ExportManager::new(export),
        }
    }
}

impl AudioModule {
    fn handle_command(&mut self, arg: &String) -> String {
        let arg_vec: Vec<&str> = arg.split(';').collect();
        let arg_cmd = arg_vec.get(0).expect("No command found\n");

        match *arg_cmd {
            "play" => {
                let arg_id = arg_vec
                    .get(1)
                    .expect("play, id not found")
                    .parse::<usize>()
                    .expect("play id, string conversion");
                let arg_duration = arg_vec
                    .get(2)
                    .expect("play, duration not found")
                    .parse::<f64>()
                    .expect("play duration, string conversion");

                let net = self.dsp.get_net(arg_id);
                if net.is_none() {
                    return "nil".to_string();
                }

                // Check if the network only has one output,
                // if so, convert it to stereo
                let mut net = net.unwrap();
                if net.outputs() == 1 {
                    net = Net::pipe(net, Net::wrap(Box::new(pan(0.0))));
                }

                let event_id = self.sequencer.push_relative(
                    0.0,
                    arg_duration,
                    Fade::Smooth,
                    0.01,
                    0.01,
                    Box::new(net),
                );
                let event_name = format!("{:?}", event_id);

                self.event_map.insert(event_name.to_string(), event_id);
                return event_name;
            }
            "stop" => {
                let arg_event_id = arg_vec.get(1).expect("stop, id not found");

                let event_id = self.event_map.get(&arg_event_id.to_string());

                if event_id.is_none() {
                    return false.to_string();
                }

                self.sequencer
                    .edit_relative(event_id.unwrap().clone(), 0.01, 0.01);
                return true.to_string();
            }
            _ => {
                panic!("Invalid audio command {}", arg_cmd);
            }
        }
    }

    fn run_output(&mut self, audio_graph: Box<dyn AudioUnit>) {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("Failed to find a device");
        let config = device.default_output_config().unwrap();

        self.dsp.resample_to_output(config.sample_rate().0 as f64);

        match config.sample_format() {
            cpal::SampleFormat::F32 => {
                AudioModule::run_fundsp::<f32>(audio_graph, device, config.into())
            }
            cpal::SampleFormat::I16 => {
                AudioModule::run_fundsp::<i16>(audio_graph, device, config.into())
            }
            cpal::SampleFormat::U16 => {
                AudioModule::run_fundsp::<u16>(audio_graph, device, config.into())
            }
            _ => panic!("Unsupported audio format"),
        }
    }

    fn run_fundsp<T: SizedSample + FromSample<f64>>(
        mut sound: Box<dyn AudioUnit>,
        device: Device,
        config: StreamConfig,
    ) {
        std::thread::spawn(move || {
            let sample_rate = config.sample_rate.0 as f64;
            sound.set_sample_rate(sample_rate);

            let mut next_value = move || sound.get_stereo();
            let channels = config.channels as usize;
            let err_fun = |err| eprintln!("An error occured ons tream: {err}");
            let stream = device
                .build_output_stream(
                    &config,
                    move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                        for frame in data.chunks_mut(channels) {
                            let sample = next_value();
                            let left: T = T::from_sample(sample.0 as f64);
                            let right: T = T::from_sample(sample.1 as f64);

                            for (channel, sample) in frame.iter_mut().enumerate() {
                                *sample = if channel & 1 == 0 { left } else { right };
                            }
                        }
                    },
                    err_fun,
                    None,
                )
                .unwrap();

            stream.play().unwrap();
            loop {
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        });
    }
}

impl CommandModule for AudioModule {
    fn init(&mut self, lua: &Lua) {
        // Initialize modules
        self.dsp.init(lua);

        // Initialize lua
        lua.load(LUA_MODULE)
            .exec()
            .expect("Failed to load audio module, got\n");

        // Start playback
        let backend = self.sequencer.backend();

        // Playing live
        if self.export_manager.is_live() {
            self.run_output(Box::new(backend));
        }
        // Exporting audio
        else {
            self.sequencer.set_sample_rate(EXPORT_SAMPLE_RATE);
            self.export_manager.init(Box::new(backend));
            self.dsp.resample_to_output(EXPORT_SAMPLE_RATE);
        }
    }
    fn update(&mut self, time: &f64, lua: &Lua) {
        self.dsp.update(time, lua);

        if !self.export_manager.is_live() {
            self.export_manager.update();
        }
    }
    fn end(&mut self, lua: &Lua) {
        self.dsp.end(lua);

        if !self.export_manager.is_live() {
            self.export_manager.export();
        }
    }

    fn get_post_init_program(&self) -> Option<String> {
        self.dsp.get_post_init_program()
    }
    fn get_command_name(&self) -> String {
        "_audio_command_handler".to_string()
    }
    fn command(&mut self, lua: &Lua, arg: &String) -> String {
        let arg_vec: Vec<&str> = arg.split(';').collect();
        let arg_cmd = arg_vec.get(0).expect("No command found\n");

        let dsp_cmd_name = &self.dsp.get_command_name();

        // DSP Commands
        if arg_cmd == dsp_cmd_name {
            return self.dsp.command(
                lua,
                &arg.strip_prefix((dsp_cmd_name.to_owned() + ";").as_str())
                    .expect("No arguments after command")
                    .to_string(),
            );
        } else if arg_cmd == &"audio" {
            return self.handle_command(
                &arg.strip_prefix("audio;")
                    .expect("No arguments after command")
                    .to_string(),
            );
        } else {
            panic!(
                "Tried to call command {} which doesn't exist for Audio module",
                arg_cmd
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::runner::{CommandModule, audio::AudioModule};
    use fundsp::wave::Wave;
    use mlua::Lua;
    use std::collections::HashMap;

    #[test]
    pub fn test_rust_module() {
        let lua = Lua::new();
        let globals = lua.globals();
        let module: &mut dyn CommandModule =
            &mut AudioModule::new(&HashMap::<String, Wave>::new(), None);
        let post_init_program = module.get_post_init_program();

        module.init(&lua);
        module.update(&0.0, &lua);

        let _ = lua.scope(|scope| {
            lua.globals()
                .set(
                    module.get_command_name(),
                    scope.create_function_mut(|_, arg: String| Ok(module.command(&lua, &arg)))?,
                )
                .expect("Error using command function");

            lua.load(post_init_program.unwrap())
                .exec()
                .expect("Failed to load post init on module, got\n");

            let test_program = r#"
                local test_net = Constant.new(0.0) * (Constant.new(440)..Sine)

                _G.SUCCESS = (Stop(Play(test_net)) == true)
            "#;

            assert!(lua.load(test_program).exec().is_ok());
            assert!(globals.get::<bool>("SUCCESS").is_ok());
            assert!(globals.get::<bool>("SUCCESS").unwrap());
            Ok(())
        });

        module.end(&lua);
    }

    #[test]
    pub fn test_lua_note_utility() {
        let lua = Lua::new();
        let globals = lua.globals();
        let module: &mut dyn CommandModule =
            &mut AudioModule::new(&HashMap::<String, Wave>::new(), None);
        let post_init_program = module.get_post_init_program();

        module.init(&lua);
        module.update(&0.0, &lua);

        let _ = lua.scope(|scope| {
            lua.globals()
                .set(
                    module.get_command_name(),
                    scope.create_function_mut(|_, arg: String| Ok(module.command(&lua, &arg)))?,
                )
                .expect("Error using command function");

            lua.load(post_init_program.unwrap())
                .exec()
                .expect("Failed to load post init on module, got\n");

            let test_program = r#"
                _G.T1 = A4:GetFreq()
                _G.T2 = A4:Offset(1):GetFreq()
                _G.T3 = A4:Scale(MAJOR, 2):GetFreq()
                _G.T4 = A4:Chord(MAJOR_TRIAD)[1]:GetFreq()
            "#;

            lua.load(test_program)
                .exec()
                .expect("Test code failed to run");
            assert_eq!(globals.get::<f64>("T1").unwrap(), 440.0);
            assert_eq!(globals.get::<f64>("T2").unwrap(), 466.24);
            assert_eq!(globals.get::<f64>("T3").unwrap(), 554.24);
            assert_eq!(globals.get::<f64>("T4").unwrap(), 440.0);
            Ok(())
        });

        module.end(&lua);
    }

    #[test]
    pub fn test_lua_instrument_utility() {
        let lua = Lua::new();
        let globals = lua.globals();
        let module: &mut dyn CommandModule =
            &mut AudioModule::new(&HashMap::<String, Wave>::new(), None);
        let post_init_program = module.get_post_init_program();

        module.init(&lua);
        module.update(&0.0, &lua);

        let _ = lua.scope(|scope| {
            lua.globals()
                .set(
                    module.get_command_name(),
                    scope.create_function_mut(|_, arg: String| Ok(module.command(&lua, &arg)))?,
                )
                .expect("Error using command function");

            lua.load(post_init_program.unwrap())
                .exec()
                .expect("Failed to load post init on module, got\n");

            let test_program = r#"
                -- The test program doesn't have GetTime() as its
                -- a part of the timer module. We aren't checking
                -- time changes, so returning only 0.0 is OK
                function _G.GetTime()
                    return 0.0
                end

                function _G.GetBPMModifier()
                    return 1.0
                end

                local test_inst = Instrument.new(function(freq, amp)
                    return (freq .. Sine) * amp * 0.0
                end, 3)

                --[[ TEST PLAY/STOP ]]--
                _G.T1 = test_inst:Play(A4, 0.1, 0.3)
                _G.TF1 = test_inst:GetFreq()
                _G.T2 = test_inst:Play(C4, 0.2, 0.4)
                _G.TA2 = test_inst:GetAmp(2)
                _G.T3 = test_inst:Play(C4, 0.3, 0.5)
                _G.TA3 = test_inst:GetAmp(3)
                _G.T4 = test_inst:Play(C4, 0.1, 0.3)

                test_inst:Stop(T1)
                test_inst:Stop(T2)
                test_inst:Stop(T3)

                -- Only really checks to see if the Lua code errors
                test_inst:Play(C4:Chord(MAJOR_TRIAD),0.1)

                --[[ TEST SETFREQ / SETAMP ]]--
                test_inst:SetFreq(0.4)
                _G.T5 = test_inst:GetFreq()
                test_inst:SetFreq(1, 0.6)
                _G.T5 = _G.T5 + test_inst:GetFreq()

                test_inst:SetAmp(0.3)
                _G.T6 = test_inst:GetAmp()
                test_inst:SetAmp(1, 0.8)
                _G.T6 = _G.T6 + test_inst:GetAmp()
            "#;

            lua.load(test_program)
                .exec()
                .expect("Test code failed to run");

            assert_eq!(globals.get::<f64>("T1").unwrap(), 1.0);
            assert_eq!(globals.get::<f64>("TF1").unwrap(), 440.0);
            assert_eq!(globals.get::<f64>("T2").unwrap(), 2.0);
            assert_eq!(globals.get::<f64>("TA2").unwrap(), 0.4);
            assert_eq!(globals.get::<f64>("T3").unwrap(), 3.0);
            assert_eq!(globals.get::<f64>("TA3").unwrap(), 0.5);
            assert_eq!(globals.get::<f64>("T4").unwrap(), 1.0);
            assert_eq!(globals.get::<f64>("T5").unwrap(), 1.0);
            assert_eq!(globals.get::<f64>("T6").unwrap(), 1.1);
            Ok(())
        });

        module.end(&lua);
    }

    #[test]
    pub fn test_lua_sequence_utility() {
        let lua = Lua::new();
        let globals = lua.globals();
        let module: &mut dyn CommandModule =
            &mut AudioModule::new(&HashMap::<String, Wave>::new(), None);
        let post_init_program = module.get_post_init_program();

        module.init(&lua);
        module.update(&0.0, &lua);

        let _ = lua.scope(|scope| {
            lua.globals()
                .set(
                    module.get_command_name(),
                    scope.create_function_mut(|_, arg: String| Ok(module.command(&lua, &arg)))?,
                )
                .expect("Error using command function");

            lua.load(post_init_program.unwrap())
                .exec()
                .expect("Failed to load post init on module, got\n");

            let test_program = r#"
                local test_seq = Sequence.new({1,2,nil,3}, true)
                local test_seq2 = Sequence.new({1,2,nil,3})

                SUCCESS = test_seq:Step() == 1 and test_seq2:Step() == 1
                SUCCESS = SUCCESS and test_seq:Step() == 2 and test_seq2:Step() == 2
                SUCCESS = SUCCESS and test_seq:Step() == nil and test_seq2:Step() == nil 
                SUCCESS = SUCCESS and test_seq:Step() == 3 and test_seq2:Step() == 3 
                SUCCESS = SUCCESS and test_seq:Step() == 1 and test_seq2:Step() == nil 
                test_seq2:SetLoop(1)
                SUCCESS = SUCCESS and test_seq:Step() == 2 and test_seq2:Step() == nil 
                SUCCESS = SUCCESS and test_seq:Step() == nil and test_seq2:Step() == 1 
            "#;

            lua.load(test_program)
                .exec()
                .expect("Test code failed to run");
            assert_eq!(globals.get::<bool>("SUCCESS").unwrap(), true);
            Ok(())
        });

        module.end(&lua);
    }
}
