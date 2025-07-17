use crate::runner::{CommandModule, audio::dsp::DspModule};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, SizedSample, StreamConfig};
use fundsp::hacker32::*;
use mlua::Lua;
use std::collections::HashMap;

mod dsp;

const LUA_MODULE: &str = include_str!("audio.luau");

pub struct AudioModule {
    sequencer: Sequencer,
    // NOTE: Because fundsp doesn't expose any manners in which EventId can be
    // created from a non-eventid class, this event_map serves as a hashmap of
    // the debug output -> the event id. Its ugly, it uses ~800 mb of ram per
    // 1 million notes played. Which feels "good enough" for now.
    event_map: HashMap<String, EventId>,
    // Modules
    dsp: DspModule,
}

impl AudioModule {
    // TODO: When audio export is implemented, add inputs
    // for mode & bitrate.
    pub fn new() -> AudioModule {
        AudioModule {
            sequencer: Sequencer::new(false, 1),
            event_map: HashMap::new(),
            dsp: DspModule::new(),
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

                let event_id = self.sequencer.push_relative(
                    0.0,
                    arg_duration,
                    Fade::Smooth,
                    0.01,
                    0.01,
                    Box::new(net.unwrap()),
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

    fn run_output(audio_graph: Box<dyn AudioUnit>) {
        let host = cpal::default_host();

        let device = host
            .default_output_device()
            .expect("Failed to find a device");
        let config = device.default_output_config().unwrap();

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

        AudioModule::run_output(Box::new(backend));
    }
    fn update(&mut self, time: &f64, lua: &Lua) {
        self.dsp.update(time, lua);
    }
    fn end(&mut self, lua: &Lua) {
        self.dsp.end(lua);
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
    use mlua::Lua;

    #[test]
    pub fn test_rust_module() {
        let lua = Lua::new();
        let globals = lua.globals();
        let module: &mut dyn CommandModule = &mut AudioModule::new();
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
}
