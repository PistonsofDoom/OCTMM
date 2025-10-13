use crate::runner::CommandModule;
use fundsp::hacker32::*;
use mlua::Lua;
use std::collections::HashMap;

const LUA_MODULE: &str = include_str!("dsp.luau");

#[derive(Debug)]
/// Used to describe the applicable "base components" that we want to use
/// Contains oscillators, noise, and filters
pub enum NodeType {
    // Single-input
    // Generators
    Hammond,
    Organ,
    Saw,
    Sine,
    SoftSaw,
    Square,
    Triangle,
    Lorenz,
    Rossler,
    Ramp,

    // Dual-input
    // Generators
    Pulse,

    // Zero-input
    // Generators
    MLS,
    White,
    Pink,
    Brown,

    // Effects
    Resonator,
    Peak,
    Pinkpass,
    Panner,
    Notch,
    Moog,
    Morph,
    Lowpole,
    Lowpass,
    Lowshelf,
    Highpole,
    Highpass,
    Highshelf,
    Butterpass,
    Bell,
    Bandpass,
    Bandrez,
    Allpass,
}

impl NodeType {
    pub fn as_unit(&self) -> Box<dyn AudioUnit> {
        match self {
            NodeType::Hammond => Box::new(hammond()),
            NodeType::Organ => Box::new(organ()),
            NodeType::Saw => Box::new(saw()),
            NodeType::Sine => Box::new(sine()),
            NodeType::SoftSaw => Box::new(soft_saw()),
            NodeType::Square => Box::new(square()),
            NodeType::Triangle => Box::new(triangle()),
            NodeType::Lorenz => Box::new(lorenz()),
            NodeType::Rossler => Box::new(rossler()),
            NodeType::Ramp => Box::new(ramp()),
            NodeType::Pulse => Box::new(pulse()),
            NodeType::MLS => Box::new(mls()),
            NodeType::White => Box::new(white()),
            NodeType::Pink => Box::new(pink()),
            NodeType::Brown => Box::new(brown()),
            NodeType::Resonator => Box::new(resonator()),
            NodeType::Peak => Box::new(peak()),
            NodeType::Pinkpass => Box::new(pinkpass()),
            NodeType::Panner => Box::new(panner()),
            NodeType::Notch => Box::new(notch()),
            NodeType::Moog => Box::new(moog()),
            NodeType::Morph => Box::new(morph()),
            NodeType::Lowpole => Box::new(lowpole()),
            NodeType::Lowpass => Box::new(lowpass()),
            NodeType::Lowshelf => Box::new(lowshelf()),
            NodeType::Highpole => Box::new(highpole()),
            NodeType::Highpass => Box::new(highpass()),
            NodeType::Highshelf => Box::new(highshelf()),
            NodeType::Butterpass => Box::new(butterpass()),
            NodeType::Bell => Box::new(bell()),
            NodeType::Bandpass => Box::new(bandpass()),
            NodeType::Bandrez => Box::new(bandrez()),
            NodeType::Allpass => Box::new(allpass()),
        }
    }

    /// Returns the network id for all constant defaults,
    /// or none if the NodeType is Shared or Constant
    pub fn as_net_id(&self) -> Option<usize> {
        match self {
            NodeType::Hammond => Some(0),
            NodeType::Organ => Some(1),
            NodeType::Saw => Some(2),
            NodeType::Sine => Some(3),
            NodeType::SoftSaw => Some(4),
            NodeType::Square => Some(5),
            NodeType::Triangle => Some(6),
            NodeType::Lorenz => Some(7),
            NodeType::Rossler => Some(8),
            NodeType::Ramp => Some(9),
            NodeType::Pulse => Some(10),
            NodeType::MLS => Some(11),
            NodeType::White => Some(12),
            NodeType::Pink => Some(13),
            NodeType::Brown => Some(14),
            NodeType::Resonator => Some(15),
            NodeType::Peak => Some(16),
            NodeType::Pinkpass => Some(17),
            NodeType::Panner => Some(18),
            NodeType::Notch => Some(19),
            NodeType::Moog => Some(20),
            NodeType::Morph => Some(21),
            NodeType::Lowpole => Some(22),
            NodeType::Lowpass => Some(23),
            NodeType::Lowshelf => Some(24),
            NodeType::Highpole => Some(25),
            NodeType::Highpass => Some(26),
            NodeType::Highshelf => Some(27),
            NodeType::Butterpass => Some(28),
            NodeType::Bell => Some(29),
            NodeType::Bandpass => Some(30),
            NodeType::Bandrez => Some(31),
            NodeType::Allpass => Some(32),
        }
    }

    /// Returns a vector containing every constant nodetype.
    /// Constant as in "cannot be changed by user".
    /// NodeType::Constant
    pub fn get_defaults() -> Vec<Net> {
        Vec::from([
            Net::wrap(NodeType::Hammond.as_unit()),
            Net::wrap(NodeType::Organ.as_unit()),
            Net::wrap(NodeType::Saw.as_unit()),
            Net::wrap(NodeType::Sine.as_unit()),
            Net::wrap(NodeType::SoftSaw.as_unit()),
            Net::wrap(NodeType::Square.as_unit()),
            Net::wrap(NodeType::Triangle.as_unit()),
            Net::wrap(NodeType::Lorenz.as_unit()),
            Net::wrap(NodeType::Rossler.as_unit()),
            Net::wrap(NodeType::Ramp.as_unit()),
            Net::wrap(NodeType::Pulse.as_unit()),
            Net::wrap(NodeType::MLS.as_unit()),
            Net::wrap(NodeType::White.as_unit()),
            Net::wrap(NodeType::Pink.as_unit()),
            Net::wrap(NodeType::Brown.as_unit()),
            Net::wrap(NodeType::Resonator.as_unit()),
            Net::wrap(NodeType::Peak.as_unit()),
            Net::wrap(NodeType::Pinkpass.as_unit()),
            Net::wrap(NodeType::Panner.as_unit()),
            Net::wrap(NodeType::Notch.as_unit()),
            Net::wrap(NodeType::Moog.as_unit()),
            Net::wrap(NodeType::Morph.as_unit()),
            Net::wrap(NodeType::Lowpole.as_unit()),
            Net::wrap(NodeType::Lowpass.as_unit()),
            Net::wrap(NodeType::Lowshelf.as_unit()),
            Net::wrap(NodeType::Highpole.as_unit()),
            Net::wrap(NodeType::Highpass.as_unit()),
            Net::wrap(NodeType::Highshelf.as_unit()),
            Net::wrap(NodeType::Butterpass.as_unit()),
            Net::wrap(NodeType::Bandpass.as_unit()),
            Net::wrap(NodeType::Bandrez.as_unit()),
            Net::wrap(NodeType::Allpass.as_unit()),
        ])
    }
}

pub struct DspModule {
    // Contains all the DSP networks used within the module
    nets: Vec<Net>,
    // Contains all fundsp Shared variables, mapped to a unique name
    shared: HashMap<String, Shared>,
    // Contains a map of unique shared names to network ids
    shared_to_net: HashMap<String, usize>,
    // String -> Wave HashMap for Samples
    samples: HashMap<String, Wave>,
}

impl DspModule {
    pub fn new(samples: HashMap<String, Wave>) -> DspModule {
        DspModule {
            nets: NodeType::get_defaults(),
            shared: HashMap::new(),
            shared_to_net: HashMap::new(),
            samples: samples,
        }
    }

    /// Resample all samples to match the output sample rate
    pub fn resample_to_output(&mut self, output_sample_rate: f64) {
        let mut resampled_samples: HashMap<String, Wave> = HashMap::new();

        for (name, sample) in &self.samples {
            let ratio: f32 = (sample.sample_rate() / output_sample_rate) as f32;
            let mut dsp_net: Net = Net::new(0, 0);

            for channel in 0..sample.channels() {
                let dsp_sampled = constant(ratio)
                    >> resample(wavech(&std::sync::Arc::new(sample.clone()), channel, None));
                dsp_net = Net::stack(dsp_net, Net::wrap(Box::new(dsp_sampled)));
            }

            let new_sample = Wave::render(output_sample_rate, sample.duration(), &mut dsp_net);

            resampled_samples.insert(name.clone(), new_sample);
        }

        self.samples = resampled_samples;
    }

    /* Shared Management */

    /// Returns whether a "shared" entry exists or not.
    pub fn shared_exists(&mut self, name: &String) -> bool {
        return self.shared.contains_key(name);
    }

    /// Set or create a new shared value
    pub fn shared_set(&mut self, name: &String, value: &f32) -> usize {
        let entry = self.shared_get(name);

        if entry.is_none() {
            self.shared.insert(name.clone(), shared(value.clone()));

            let entry = Box::new(var(&self
                .shared_get(name)
                .expect("Failed to create shared")));
            let net_id = self.net_from(&Net::wrap(entry));

            self.shared_to_net.insert(name.clone(), net_id.clone());
            return net_id;
        } else {
            entry.unwrap().set(value.clone());
            return self.shared_get_net(name).expect("No net id").clone();
        }
    }

    pub fn shared_get(&self, name: &String) -> Option<&Shared> {
        self.shared.get(name)
    }

    pub fn shared_get_net(&self, name: &String) -> Option<&usize> {
        self.shared_to_net.get(name)
    }

    /* Network Management */
    /*
     * Utility functions to help manage the
     * storage of networks
     */
    // NOTE: as it is right now, to create complex networks, multiple "temporary networks" need to
    // be created, which are then combined together in various ways (e.g., summing, mixing, piping)
    // This shouldn't create problems if the user program is written correctly, however if "voices"
    // are generated on the fly, rather than pre-generated, this could become a problem.

    /// Check whether a network entry exists at the target index
    pub fn net_exists(&self, target: usize) -> bool {
        return target < self.nets.len();
    }

    /// Create a new network entry from a Net reference
    pub fn net_from(&mut self, new_network: &Net) -> usize {
        self.nets.push(new_network.clone());
        return self.nets.len() - 1;
    }

    pub fn get_net(&self, target: usize) -> Option<Net> {
        if !self.net_exists(target) {
            return None;
        }

        return Some(self.nets[target].clone());
    }

    /// Create a new network that contains a constant of the given value
    pub fn net_constant(&mut self, value: f32) -> usize {
        self.net_from(&Net::wrap(Box::new(constant(value))))
    }

    // This isn't tested in 'cargo test', but is tested
    // within the audio_module.md test
    pub fn net_from_sample(
        &mut self,
        sample_name: &String,
        loop_point: Option<usize>,
        channel: Option<usize>,
    ) -> Option<usize> {
        let sample = self.samples.get(sample_name);

        if sample.is_none() {
            return None;
        }

        let sample = sample.unwrap();
        let mut net = Net::new(0, 0);

        // If no channel is specified, just use
        // all the channels provided within the
        // sample
        if channel.is_none() {
            for channel in 0..sample.channels() {
                net = Net::stack(
                    net,
                    Net::wrap(Box::new(resample(wavech(
                        &std::sync::Arc::new(sample.clone()),
                        channel,
                        loop_point,
                    )))),
                );
            }
        } else {
            net = Net::stack(
                net,
                Net::wrap(Box::new(resample(wavech(
                    &std::sync::Arc::new(sample.clone()),
                    channel.unwrap(),
                    loop_point,
                )))),
            );
        }

        return Some(self.net_from(&net));
    }

    pub fn net_vector_length(&self) -> usize {
        return self.nets.len();
    }

    /* Network Proxies */
    /*
     * Equivalent of the functions provided by the
     * fundsp "Net" struct, but with more
     * checks
     */

    /// Only works with constants / shared
    pub fn net_product(&mut self, target_a: usize, target_b: usize) -> Option<usize> {
        if !self.net_exists(target_a) || !self.net_exists(target_b) {
            return None;
        }

        let net_a = self.nets[target_a].clone();
        let net_b = self.nets[target_b].clone();

        if !Net::can_product(&net_a, &net_b) || net_b.inputs() != 0 {
            return None;
        }

        let new_network = Net::product(net_a, net_b);

        Some(self.net_from(&new_network))
    }

    /// Buses two networks together. If one or more of the networks have 0 inputs, they are
    /// summed instead
    pub fn net_bus(&mut self, target_a: usize, target_b: usize) -> Option<usize> {
        if !self.net_exists(target_a) || !self.net_exists(target_b) {
            return None;
        }

        let net_a = self.nets[target_a].clone();
        let net_b = self.nets[target_b].clone();

        // When using constants / shared, the "input" count is 0.
        // So just do a sum instead, this gives intended behavior.
        if self.nets[target_a].inputs() == 0 || self.nets[target_b].inputs() == 0 {
            if !Net::can_sum(&net_a, &net_b) {
                return None;
            }

            let new_network = Net::sum(net_a, net_b);
            return Some(self.net_from(&new_network));
        }

        if !Net::can_bus(&net_a, &net_b) {
            return None;
        }

        let new_network = Net::bus(net_a, net_b);
        Some(self.net_from(&new_network))
    }

    pub fn net_pipe(&mut self, target_a: usize, target_b: usize) -> Option<usize> {
        if !self.net_exists(target_a) || !self.net_exists(target_b) {
            return None;
        }

        let net_a = self.nets[target_a].clone();
        let net_b = self.nets[target_b].clone();

        if !Net::can_pipe(&net_a, &net_b) {
            println!(
                "Can't pipe! Net A has {} outputs, Net B has {}",
                net_a.outputs(),
                net_b.inputs()
            );
            return None;
        }

        let new_network = Net::pipe(net_a, net_b);
        return Some(self.net_from(&new_network));
    }

    pub fn net_stack(&mut self, target_a: usize, target_b: usize) -> Option<usize> {
        if !self.net_exists(target_a) || !self.net_exists(target_b) {
            return None;
        }

        let net_a = self.nets[target_a].clone();
        let net_b = self.nets[target_b].clone();

        // We can always stack, no need to check
        return Some(self.net_from(&Net::stack(net_a, net_b)));
    }
}

impl CommandModule for DspModule {
    fn init(&mut self, _lua: &Lua) {}
    fn update(&mut self, _time: &f64, _lua: &Lua) {}
    fn end(&mut self, _lua: &Lua) {}

    fn get_post_init_program(&self) -> Option<String> {
        Some(LUA_MODULE.to_string())
    }
    fn get_command_name(&self) -> String {
        "dsp".to_string()
    }
    fn command(&mut self, _lua: &Lua, arg: &String) -> String {
        let arg_vec: Vec<&str> = arg.split(';').collect();
        let arg_cmd = arg_vec.get(0).expect("No command found\n");

        match *arg_cmd {
            // Shared Commands
            "shared_exists" => {
                let arg_name = arg_vec.get(1).expect("shared_exists, name not found");
                return self.shared_exists(&arg_name.to_string()).to_string();
            }
            "shared_set" => {
                let arg_name = arg_vec.get(1).expect("shared_set, name not found");
                let arg_value = arg_vec
                    .get(2)
                    .expect("shared_set, value not found")
                    .parse::<f32>()
                    .expect("shared_set, parsing error");

                return self
                    .shared_set(&arg_name.to_string(), &arg_value)
                    .to_string();
            }
            "shared_get" => {
                let arg_name = arg_vec.get(1).expect("shared_get, name not found");

                let ret = self.shared_get(&arg_name.to_string());

                if ret.is_none() {
                    return "nil".to_string();
                } else {
                    return ret.unwrap().value().to_string();
                }
            }
            "shared_get_net" => {
                let arg_name = arg_vec.get(1).expect("shared_get_net, name not found");

                let ret = self.shared_get_net(&arg_name.to_string());

                if ret.is_none() {
                    return "nil".to_string();
                } else {
                    return ret.unwrap().to_string();
                }
            }
            // Network Management Commands
            "net_exists" => {
                let arg_id = arg_vec
                    .get(1)
                    .expect("net_exists, id not found")
                    .parse::<usize>()
                    .expect("net_exists, string conversion");

                return self.net_exists(arg_id).to_string();
            }
            "net_clone" => {
                let arg_id = arg_vec
                    .get(1)
                    .expect("net_clone, id not found")
                    .parse::<usize>()
                    .expect("net_clone, string conversion");

                if !self.net_exists(arg_id) {
                    return "nil".to_string();
                }

                let net = self.nets[arg_id].clone();

                return self.net_from(&net).to_string();
            }
            "net_constant" => {
                let arg_value = arg_vec
                    .get(1)
                    .expect("net_constant, value not found")
                    .parse::<f32>()
                    .expect("net_constant, string conversion");

                return self.net_constant(arg_value).to_string();
            }
            "net_from_sample" => {
                let arg_name = arg_vec
                    .get(1)
                    .expect("net_from_sample, name not found")
                    .to_string();
                let mut arg_loop: Option<usize> = None;
                let mut arg_channel: Option<usize> = None;

                // Get sample, so we can get some information
                // on it.
                let sample = self.samples.get(&arg_name);

                if sample.is_none() {
                    return "nil".to_string();
                }

                let sample = sample.unwrap();
                let duration = sample.duration();

                // Lua code is expected to always have this filled out. If no loop is wanted,
                // target_loop_time should be "nil"
                let target_loop_time = arg_vec.get(2).expect("net_from_sample, loop not found");

                // We only need a check here, as a user may not want to specify a loop
                // while specifying a specific channel.
                if target_loop_time != &"nil" {
                    let target_loop_time = target_loop_time
                        .parse::<f64>()
                        .expect("net_from_sample, string conversion");

                    if target_loop_time > duration {
                        println!("Tried to set loop point past duration of the sample.");
                        return "nil".to_string();
                    }

                    // percentage of sample duration * total sample count
                    let target_sample =
                        ((target_loop_time / duration) * (sample.len() as f64)) as usize;
                    arg_loop = Some(target_sample);
                }

                if arg_vec.get(3).is_some() {
                    let target_channel = arg_vec
                        .get(3)
                        .expect("net_from_sample, channel not found")
                        .parse::<usize>()
                        .expect("net_from_sample, channel string conversion");

                    arg_channel = Some(target_channel);
                }

                // Get network from the sample
                let ret = self.net_from_sample(&arg_name, arg_loop, arg_channel);

                if ret.is_none() {
                    return "nil".to_string();
                }

                return ret.unwrap().to_string() + ";" + &duration.to_string();
            }
            "net_vector_length" => {
                return self.net_vector_length().to_string();
            }
            // Network Proxy Commands
            "net_default" => {
                let arg_type = arg_vec.get(1).expect("net_default, type not found");

                return match *arg_type {
                    // Single-input
                    "hammond" => NodeType::Hammond.as_net_id().unwrap().to_string(),
                    "organ" => NodeType::Organ.as_net_id().unwrap().to_string(),
                    "saw" => NodeType::Saw.as_net_id().unwrap().to_string(),
                    "sine" => NodeType::Sine.as_net_id().unwrap().to_string(),
                    "softsaw" => NodeType::SoftSaw.as_net_id().unwrap().to_string(),
                    "square" => NodeType::Square.as_net_id().unwrap().to_string(),
                    "triangle" => NodeType::Triangle.as_net_id().unwrap().to_string(),
                    "lorenz" => NodeType::Lorenz.as_net_id().unwrap().to_string(),
                    "rossler" => NodeType::Rossler.as_net_id().unwrap().to_string(),
                    "ramp" => NodeType::Ramp.as_net_id().unwrap().to_string(),
                    // Dual-input
                    "pulse" => NodeType::Pulse.as_net_id().unwrap().to_string(),
                    // Zero-input
                    "mls" => NodeType::MLS.as_net_id().unwrap().to_string(),
                    "white" => NodeType::White.as_net_id().unwrap().to_string(),
                    "pink" => NodeType::Pink.as_net_id().unwrap().to_string(),
                    "brown" => NodeType::Brown.as_net_id().unwrap().to_string(),
                    "resonator" => NodeType::Resonator.as_net_id().unwrap().to_string(),
                    "peak" => NodeType::Peak.as_net_id().unwrap().to_string(),
                    "pinkpass" => NodeType::Pinkpass.as_net_id().unwrap().to_string(),
                    "panner" => NodeType::Panner.as_net_id().unwrap().to_string(),
                    "notch" => NodeType::Notch.as_net_id().unwrap().to_string(),
                    "moog" => NodeType::Moog.as_net_id().unwrap().to_string(),
                    "morph" => NodeType::Morph.as_net_id().unwrap().to_string(),
                    "lowpole" => NodeType::Lowpole.as_net_id().unwrap().to_string(),
                    "lowpass" => NodeType::Lowpass.as_net_id().unwrap().to_string(),
                    "lowshelf" => NodeType::Lowshelf.as_net_id().unwrap().to_string(),
                    "highpole" => NodeType::Highpole.as_net_id().unwrap().to_string(),
                    "highpass" => NodeType::Highpass.as_net_id().unwrap().to_string(),
                    "highshelf" => NodeType::Highshelf.as_net_id().unwrap().to_string(),
                    "butterpass" => NodeType::Butterpass.as_net_id().unwrap().to_string(),
                    "bell" => NodeType::Bell.as_net_id().unwrap().to_string(),
                    "bandpass" => NodeType::Bandpass.as_net_id().unwrap().to_string(),
                    "bandrez" => NodeType::Bandrez.as_net_id().unwrap().to_string(),
                    "allpass" => NodeType::Allpass.as_net_id().unwrap().to_string(),
                    _ => "nil".to_string(),
                };
            }
            "net_effect" => {
                let arg_type = arg_vec.get(1).expect("net_effect, type not found");

                // Initialize all arguments as 0
                let mut args: [f32; 6] = [0.0; 6];

                // Check for up to 6 provided arguments
                for index in 0..6 {
                    let arg = arg_vec.get(2 + index);

                    if arg.is_none() {
                        break;
                    }
                    let arg = arg
                        .expect("Failed to find net_effect argument")
                        .parse::<f32>()
                        .expect("net_effect, failed to convert arugment to f32");

                    args[index] = arg;
                }

                return match *arg_type {
                    "reverb" => self
                        .net_from(&Net::wrap(Box::new(reverb_stereo(
                            args[0], args[1], args[2],
                        ))))
                        .to_string(), // 3 arg
                    "delay" => self
                        .net_from(&Net::wrap(Box::new(delay(args[0]))))
                        .to_string(), // 1 arg
                    "flanger" => self
                        .net_from(&Net::wrap(Box::new(flanger(
                            args[0],
                            args[1],
                            args[2],
                            move |t| lerp11(args[3], args[4], sin_hz(args[5], t)),
                        ))))
                        .to_string(), // 6 arg, 4, 5, and 6 are used to tune the flanger closure
                    "phaser" => self
                        .net_from(&Net::wrap(Box::new(phaser(args[0], move |t| {
                            sin_hz(args[1], t) * args[2] + (1.0 - args[2])
                        }))))
                        .to_string(), // 3 arg, 2 and 3 are used to tune the phaser closure
                    "pluck" => self
                        .net_from(&Net::wrap(Box::new(pluck(args[0], args[1], args[2]))))
                        .to_string(), // 3 arg
                    "chorus" => self
                        .net_from(&Net::wrap(Box::new(chorus(
                            args[3] as u64,
                            args[0],
                            args[1],
                            args[2],
                        ))))
                        .to_string(), // 4 arg, args[3] is used for the u64 seed, optionally
                    "adsr_live" => self
                        .net_from(&Net::wrap(Box::new(adsr_live(
                            args[0], args[1], args[2], args[3],
                        ))))
                        .to_string(), // 4 arg
                    "clip_to" => self
                        .net_from(&Net::wrap(Box::new(clip_to(args[0], args[1]))))
                        .to_string(), // 2 arg
                    _ => "nil".to_string(),
                };
            }
            "net_product" => {
                let arg_id1 = arg_vec
                    .get(1)
                    .expect("net_product, id not found")
                    .parse::<usize>()
                    .expect("net_product, string conversion");
                let arg_id2 = arg_vec
                    .get(2)
                    .expect("net_product, id not found")
                    .parse::<usize>()
                    .expect("net_product, string conversion");

                let ret = self.net_product(arg_id1, arg_id2);

                if ret.is_none() {
                    return "nil".to_string();
                }

                return ret.unwrap().to_string();
            }
            "net_bus" => {
                let arg_id1 = arg_vec
                    .get(1)
                    .expect("net_bus, id not found")
                    .parse::<usize>()
                    .expect("net_bus, string conversion");
                let arg_id2 = arg_vec
                    .get(2)
                    .expect("net_bus, id not found")
                    .parse::<usize>()
                    .expect("net_bus, string conversion");

                let ret = self.net_bus(arg_id1, arg_id2);

                if ret.is_none() {
                    return "nil".to_string();
                }

                return ret.unwrap().to_string();
            }
            "net_pipe" => {
                let arg_id1 = arg_vec
                    .get(1)
                    .expect("net_pipe, id not found")
                    .parse::<usize>()
                    .expect("net_pipe, string conversion");
                let arg_id2 = arg_vec
                    .get(2)
                    .expect("net_pipe, id not found")
                    .parse::<usize>()
                    .expect("net_pipe, string conversion");

                let ret = self.net_pipe(arg_id1, arg_id2);

                if ret.is_none() {
                    return "nil".to_string();
                }

                return ret.unwrap().to_string();
            }
            "net_stack" => {
                let arg_id1 = arg_vec
                    .get(1)
                    .expect("net_stack, id not found")
                    .parse::<usize>()
                    .expect("net_stack, string conversion");
                let arg_id2 = arg_vec
                    .get(2)
                    .expect("net_stack, id not found")
                    .parse::<usize>()
                    .expect("net_stack, string conversion");

                let ret = self.net_stack(arg_id1, arg_id2);

                if ret.is_none() {
                    return "nil".to_string();
                }

                return ret.unwrap().to_string();
            }
            // Handle bad commands
            _ => {
                panic!(
                    "Tried to call command {} which doesn't exist for DSP module",
                    arg_cmd
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DspModule, NodeType};
    use crate::runner::{CommandModule, audio::AudioModule};
    use fundsp::hacker32::*;
    use mlua::Lua;
    use std::collections::HashMap;

    #[test]
    pub fn test_resample_to_output() {
        // Create initial hashmap
        let mut test_hashmap = HashMap::<String, Wave>::new();
        let mut test_wave = Wave::new(2, 22000.0);

        for _ in 0..10 {
            test_wave.push((0.0, 0.0));
        }

        test_hashmap.insert("test".to_string(), test_wave);

        // Create dsp module
        let mut dsp = DspModule::new(test_hashmap);
        dsp.resample_to_output(44000.0);

        // Confirm proper resample
        assert_eq!(dsp.samples.get("test").unwrap().len(), 20);
    }

    /* Shared Testing */
    #[test]
    pub fn test_shared_management() {
        let mut dsp = DspModule::new(HashMap::<String, Wave>::new());
        let test_name: String = "test shared".to_string();

        // Creation / Exists
        assert_eq!(dsp.shared_exists(&test_name), false);
        dsp.shared_set(&test_name, &2.5);
        assert_eq!(dsp.shared_exists(&test_name), true);

        // Values
        assert_eq!(dsp.shared_get(&test_name).unwrap().value(), 2.5);
        dsp.shared_set(&test_name, &0.0);
        assert_eq!(dsp.shared_get(&test_name).unwrap().value(), 0.0);
    }

    /* Network Testing */
    #[test]
    pub fn test_net_management() {
        let mut dsp = DspModule::new(HashMap::<String, Wave>::new());

        let default_length: usize = NodeType::get_defaults().len();

        assert_eq!(dsp.net_vector_length(), default_length);

        // Test if net entry doesn't exist
        // Create it
        // Test if the net id is where we expect
        // Check if network exists

        assert!(!dsp.net_exists(default_length));
        let id1 = dsp.net_from(&Net::new(0, 3));
        assert_eq!(id1, default_length);
        assert!(dsp.net_exists(default_length));

        assert!(!dsp.net_exists(default_length + 1));
        let id2 = dsp.net_from(&Net::new(0, 4));
        assert_eq!(id2, default_length + 1);
        assert!(dsp.net_exists(default_length + 1));

        assert!(dsp.get_net(default_length + 200).is_none());
        assert!(dsp.get_net(default_length + 1).is_some());

        // Test net_constant
        assert_eq!(dsp.net_constant(12.3), default_length + 2);
    }

    #[test]
    pub fn test_net_functions() {
        let mut dsp = DspModule::new(HashMap::<String, Wave>::new());

        let hammond = NodeType::Sine.as_net_id().expect("No ID exists");
        let organ = NodeType::Organ.as_net_id().expect("No ID exists");
        let saw = NodeType::Saw.as_net_id().expect("No ID exists");
        let sine = NodeType::Sine.as_net_id().expect("No ID exists");
        let softsaw = NodeType::SoftSaw.as_net_id().expect("No ID exists");
        let square = NodeType::Square.as_net_id().expect("No ID exists");
        let triangle = NodeType::Triangle.as_net_id().expect("No ID exists");

        let constant = dsp.net_constant(2.2);
        let my_shared = dsp.shared_set(&"my_shared".to_string(), &0.5);

        // Test net_product
        let my_network = dsp.net_product(hammond, organ);
        assert!(my_network.is_none());

        let my_network = dsp.net_product(hammond, constant);
        assert!(my_network.is_some());

        let my_network = dsp.net_product(my_network.unwrap(), my_shared);
        assert!(my_network.is_some());

        // Test net_bus
        let my_network = dsp.net_bus(hammond, square);
        assert!(my_network.is_some());

        let my_network = dsp.net_bus(my_network.unwrap(), softsaw);
        assert!(my_network.is_some());
        let my_network = dsp.net_bus(my_network.unwrap(), triangle);
        assert!(my_network.is_some());
        let my_network = dsp.net_bus(my_network.unwrap(), saw);
        assert!(my_network.is_some());

        let my_network = dsp.net_bus(constant, my_shared);
        assert!(my_network.is_some());
        let my_network = dsp.net_bus(constant, constant);
        assert!(my_network.is_some());
        let my_network = dsp.net_bus(sine, my_shared);
        assert!(my_network.is_some());
        let my_network = dsp.net_bus(my_shared, sine);
        assert!(my_network.is_some());

        // Test net_pipe
        let my_network = dsp.net_pipe(my_network.unwrap(), sine);
        assert!(my_network.is_some());

        let my_network = dsp.net_pipe(sine, my_network.unwrap());
        assert!(my_network.is_some());

        // Test net_stack
        let my_network = dsp.net_stack(saw, sine);
        assert!(my_network.is_some());

        let net = &dsp.nets[my_network.unwrap()];
        assert_eq!(net.inputs(), 2);
        assert_eq!(net.outputs(), 2);
    }

    #[test]
    fn test_rust_module() {
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

            // NOTE: Improvement could be to check the result to make sure the digital signal
            // network was created correctly
            let test_program = r#"
                local f = Shared.new(420)
                local m = Constant.new(0.5)

                local fm_synth = ((f..Sine) * f * m) + f..Sine
                local cloned = Net.clone(fm_synth)
                local operation_test = (Sine + f)..(Sine + Saw)..(f + Sine) + (f * m)

                local success = typeof(fm_synth._net_id) == "number" and 
                                typeof(operation_test._net_id) == "number" and 
                                cloned ~= nil and 
                                cloned._net_id ~= fm_synth._net_id

                _G.SUCCESS = success
            "#;

            lua.load(test_program)
                .exec()
                .expect("Failed to execute lua");
            assert!(globals.get::<bool>("SUCCESS").is_ok());
            assert!(globals.get::<bool>("SUCCESS").unwrap());

            Ok(())
        });

        module.end(&lua);
    }

    // LUA CODE TESTS
    #[test]
    fn test_shared_commands() {
        let lua = Lua::new();
        let globals = lua.globals();
        let module: &mut dyn CommandModule =
            &mut AudioModule::new(&HashMap::<String, Wave>::new(), None);

        let _ = lua.scope(|scope| {
            module.init(&lua);

            lua.globals()
                .set(
                    module.get_command_name(),
                    scope.create_function_mut(|_, arg: String| Ok(module.command(&lua, &arg)))?,
                )
                .expect("Error using command function");

            let test_program = r#"
                _G.r1 = _audio_command_handler("dsp;shared_exists;test")
                _G.r2 = _audio_command_handler("dsp;shared_set;test;1.2")
                _G.r3 = _audio_command_handler("dsp;shared_exists;test")
                _G.r4 = _audio_command_handler("dsp;shared_get;test")
                _G.r5 = _audio_command_handler("dsp;shared_get_net;test")
            "#;

            assert!(lua.load(test_program).exec().is_ok());

            let r1 = globals.get::<String>("r1").unwrap();
            let r2 = globals.get::<String>("r2").unwrap();
            let r3 = globals.get::<String>("r3").unwrap();
            let r4 = globals.get::<String>("r4").unwrap();
            let r5 = globals.get::<String>("r5").unwrap();

            assert_eq!(r1, "false");
            assert_eq!(r2, NodeType::get_defaults().len().to_string());
            assert_eq!(r3, "true");
            assert_eq!(r4, "1.2");
            assert_eq!(r5, NodeType::get_defaults().len().to_string());

            Ok(())
        });
    }

    #[test]
    fn test_net_management_commands() {
        let lua = Lua::new();
        let globals = lua.globals();
        let module: &mut dyn CommandModule =
            &mut AudioModule::new(&HashMap::<String, Wave>::new(), None);

        let _ = lua.scope(|scope| {
            module.init(&lua);

            lua.globals()
                .set(
                    module.get_command_name(),
                    scope.create_function_mut(|_, arg: String| Ok(module.command(&lua, &arg)))?,
                )
                .expect("Error using command function");

            let test_program = r#"
                _G.r1 = _audio_command_handler("dsp;net_vector_length")
                _G.r2 = _audio_command_handler("dsp;net_exists;" .. tostring(_G.r1))
                _G.r3 = _audio_command_handler("dsp;net_constant;3.3")
                _G.r4 = _audio_command_handler("dsp;net_exists;" .. tostring(_G.r1))
                _G.r5 = _audio_command_handler("dsp;net_clone;0")
            "#;

            assert!(lua.load(test_program).exec().is_ok());

            let r1 = globals.get::<String>("r1").unwrap();
            let r2 = globals.get::<String>("r2").unwrap();
            let r3 = globals.get::<String>("r3").unwrap();
            let r4 = globals.get::<String>("r4").unwrap();
            let r5 = globals.get::<String>("r5").unwrap();

            assert_eq!(r1, NodeType::get_defaults().len().to_string());
            assert_eq!(r2, "false");
            assert_eq!(r3, NodeType::get_defaults().len().to_string());
            assert_eq!(r4, "true");
            assert_eq!(r5, (NodeType::get_defaults().len() + 1).to_string());

            Ok(())
        });
    }

    #[test]
    fn test_net_effect_commands() {
        let lua = Lua::new();
        let globals = lua.globals();
        let module: &mut dyn CommandModule =
            &mut AudioModule::new(&HashMap::<String, Wave>::new(), None);
        let post_init_program = module.get_post_init_program();

        let _ = lua.scope(|scope| {
            module.init(&lua);

            lua.globals()
                .set(
                    module.get_command_name(),
                    scope.create_function_mut(|_, arg: String| Ok(module.command(&lua, &arg)))?,
                )
                .expect("Error using command function");

            lua.load(post_init_program.unwrap())
                .exec()
                .expect("Failed to load post init on module, got\n");

            // Test whether calling all global net_effect commands result
            // in a "nil" or a valid effect
            let test_program = r#"
                function GetBPMModifier()
                    return 1.0
                end

                _G.r1 = Reverb(10, 0.5, 0.5) ~= nil
                _G.r2 = Delay(2) ~= nil
                _G.r3 = Flanger(0.05, 0.01, 0.1, 30.0, 0.01, 0.1) ~= nil
                _G.r3 = _G.r3 and (Flanger(0.05, 0.01, 0.1, 30.0) ~= nil)
                _G.r4 = Pluck(440.0, 0.5, 0.5) ~= nil
                _G.r5 = Chorus(0.0, 0.1, 40.0) ~= nil
                _G.r6 = (ADSR() ~= nil) and (ADSR(1,2,3,4) ~= nil)
                _G.r7 = (Clip ~= nil) and (Clip(-0.5, 0.5) ~= nil)
                _G.r8 = Phaser(0.8, 5.0, 0.2) ~= nil
            "#;

            assert!(lua.load(test_program).exec().is_ok());

            let reverb = globals.get::<bool>("r1").unwrap();
            let delay = globals.get::<bool>("r2").unwrap();
            let flanger = globals.get::<bool>("r3").unwrap();
            let pluck = globals.get::<bool>("r4").unwrap();
            let chorus = globals.get::<bool>("r5").unwrap();
            let adsr = globals.get::<bool>("r6").unwrap();
            let clip = globals.get::<bool>("r7").unwrap();
            let phaser = globals.get::<bool>("r8").unwrap();

            assert!(reverb);
            assert!(delay);
            assert!(flanger);
            assert!(pluck);
            assert!(chorus);
            assert!(adsr);
            assert!(clip);
            assert!(phaser);

            Ok(())
        });
    }

    // This test only confirms whether the internal audio command returns the correct index.
    // It does not confirm if it is returning the correct audio unit
    #[test]
    fn test_net_proxy_commands() {
        let lua = Lua::new();
        let globals = lua.globals();
        let module: &mut dyn CommandModule =
            &mut AudioModule::new(&HashMap::<String, Wave>::new(), None);

        let _ = lua.scope(|scope| {
            module.init(&lua);

            lua.globals()
                .set(
                    module.get_command_name(),
                    scope.create_function_mut(|_, arg: String| Ok(module.command(&lua, &arg)))?,
                )
                .expect("Error using command function");

            // Test defaults
            let test_program = r#"
                _G.r1 = _audio_command_handler("dsp;net_default;hammond")
                _G.r2 = _audio_command_handler("dsp;net_default;organ")
                _G.r3 = _audio_command_handler("dsp;net_default;saw")
                _G.r4 = _audio_command_handler("dsp;net_default;sine")
                _G.r5 = _audio_command_handler("dsp;net_default;softsaw")
                _G.r6 = _audio_command_handler("dsp;net_default;square")
                _G.r7 = _audio_command_handler("dsp;net_default;triangle")
                _G.r8 = _audio_command_handler("dsp;net_default;lorenz")
                _G.r9 = _audio_command_handler("dsp;net_default;rossler")
                _G.r10 = _audio_command_handler("dsp;net_default;ramp")
                _G.r11 = _audio_command_handler("dsp;net_default;pulse")
                _G.r12 = _audio_command_handler("dsp;net_default;mls")
                _G.r13 = _audio_command_handler("dsp;net_default;white")
                _G.r14 = _audio_command_handler("dsp;net_default;pink")
                _G.r15 = _audio_command_handler("dsp;net_default;brown")
                _G.r16 = _audio_command_handler("dsp;net_default;resonator")
                _G.r17 = _audio_command_handler("dsp;net_default;peak")
                _G.r18 = _audio_command_handler("dsp;net_default;pinkpass")
                _G.r19 = _audio_command_handler("dsp;net_default;panner")
                _G.r20 = _audio_command_handler("dsp;net_default;notch")
                _G.r21 = _audio_command_handler("dsp;net_default;moog")
                _G.r22 = _audio_command_handler("dsp;net_default;morph")
                _G.r23 = _audio_command_handler("dsp;net_default;lowpole")
                _G.r24 = _audio_command_handler("dsp;net_default;lowpass")
                _G.r25 = _audio_command_handler("dsp;net_default;lowshelf")
                _G.r26 = _audio_command_handler("dsp;net_default;highpole")
                _G.r27 = _audio_command_handler("dsp;net_default;highpass")
                _G.r28 = _audio_command_handler("dsp;net_default;highshelf")
                _G.r29 = _audio_command_handler("dsp;net_default;butterpass")
                _G.r30 = _audio_command_handler("dsp;net_default;bell")
                _G.r31 = _audio_command_handler("dsp;net_default;bandpass")
                _G.r32 = _audio_command_handler("dsp;net_default;bandrez")
                _G.r33 = _audio_command_handler("dsp;net_default;allpass")
                _G.r34 = _audio_command_handler("dsp;net_default;bad_input")
            "#;

            assert!(lua.load(test_program).exec().is_ok());

            let r1 = globals.get::<String>("r1").unwrap();
            let r2 = globals.get::<String>("r2").unwrap();
            let r3 = globals.get::<String>("r3").unwrap();
            let r4 = globals.get::<String>("r4").unwrap();
            let r5 = globals.get::<String>("r5").unwrap();
            let r6 = globals.get::<String>("r6").unwrap();
            let r7 = globals.get::<String>("r7").unwrap();
            let r8 = globals.get::<String>("r8").unwrap();
            let r9 = globals.get::<String>("r9").unwrap();
            let r10 = globals.get::<String>("r10").unwrap();
            let r11 = globals.get::<String>("r11").unwrap();
            let r12 = globals.get::<String>("r12").unwrap();
            let r13 = globals.get::<String>("r13").unwrap();
            let r14 = globals.get::<String>("r14").unwrap();
            let r15 = globals.get::<String>("r15").unwrap();
            let r16 = globals.get::<String>("r16").unwrap();
            let r17 = globals.get::<String>("r17").unwrap();
            let r18 = globals.get::<String>("r18").unwrap();
            let r19 = globals.get::<String>("r19").unwrap();
            let r20 = globals.get::<String>("r20").unwrap();
            let r21 = globals.get::<String>("r21").unwrap();
            let r22 = globals.get::<String>("r22").unwrap();
            let r23 = globals.get::<String>("r23").unwrap();
            let r24 = globals.get::<String>("r24").unwrap();
            let r25 = globals.get::<String>("r25").unwrap();
            let r26 = globals.get::<String>("r26").unwrap();
            let r27 = globals.get::<String>("r27").unwrap();
            let r28 = globals.get::<String>("r28").unwrap();
            let r29 = globals.get::<String>("r29").unwrap();
            let r30 = globals.get::<String>("r30").unwrap();
            let r31 = globals.get::<String>("r31").unwrap();
            let r32 = globals.get::<String>("r32").unwrap();
            let r33 = globals.get::<String>("r33").unwrap();
            let r34 = globals.get::<String>("r34").unwrap();

            assert_eq!(r1, NodeType::Hammond.as_net_id().unwrap().to_string());
            assert_eq!(r2, NodeType::Organ.as_net_id().unwrap().to_string());
            assert_eq!(r3, NodeType::Saw.as_net_id().unwrap().to_string());
            assert_eq!(r4, NodeType::Sine.as_net_id().unwrap().to_string());
            assert_eq!(r5, NodeType::SoftSaw.as_net_id().unwrap().to_string());
            assert_eq!(r6, NodeType::Square.as_net_id().unwrap().to_string());
            assert_eq!(r7, NodeType::Triangle.as_net_id().unwrap().to_string());
            assert_eq!(r8, NodeType::Lorenz.as_net_id().unwrap().to_string());
            assert_eq!(r9, NodeType::Rossler.as_net_id().unwrap().to_string());
            assert_eq!(r10, NodeType::Ramp.as_net_id().unwrap().to_string());
            assert_eq!(r11, NodeType::Pulse.as_net_id().unwrap().to_string());
            assert_eq!(r12, NodeType::MLS.as_net_id().unwrap().to_string());
            assert_eq!(r13, NodeType::White.as_net_id().unwrap().to_string());
            assert_eq!(r14, NodeType::Pink.as_net_id().unwrap().to_string());
            assert_eq!(r15, NodeType::Brown.as_net_id().unwrap().to_string());
            assert_eq!(r16, NodeType::Resonator.as_net_id().unwrap().to_string());
            assert_eq!(r17, NodeType::Peak.as_net_id().unwrap().to_string());
            assert_eq!(r18, NodeType::Pinkpass.as_net_id().unwrap().to_string());
            assert_eq!(r19, NodeType::Panner.as_net_id().unwrap().to_string());
            assert_eq!(r20, NodeType::Notch.as_net_id().unwrap().to_string());
            assert_eq!(r21, NodeType::Moog.as_net_id().unwrap().to_string());
            assert_eq!(r22, NodeType::Morph.as_net_id().unwrap().to_string());
            assert_eq!(r23, NodeType::Lowpole.as_net_id().unwrap().to_string());
            assert_eq!(r24, NodeType::Lowpass.as_net_id().unwrap().to_string());
            assert_eq!(r25, NodeType::Lowshelf.as_net_id().unwrap().to_string());
            assert_eq!(r26, NodeType::Highpole.as_net_id().unwrap().to_string());
            assert_eq!(r27, NodeType::Highpass.as_net_id().unwrap().to_string());
            assert_eq!(r28, NodeType::Highshelf.as_net_id().unwrap().to_string());
            assert_eq!(r29, NodeType::Butterpass.as_net_id().unwrap().to_string());
            assert_eq!(r30, NodeType::Bell.as_net_id().unwrap().to_string());
            assert_eq!(r31, NodeType::Bandpass.as_net_id().unwrap().to_string());
            assert_eq!(r32, NodeType::Bandrez.as_net_id().unwrap().to_string());
            assert_eq!(r33, NodeType::Allpass.as_net_id().unwrap().to_string());
            assert_eq!(r34, "nil".to_string());

            // Test all other proxys
            let test_program = r#"
                local constant = _audio_command_handler("dsp;net_constant;2.0")
                -- Successes
                _G.s1 = _audio_command_handler("dsp;net_product;0;"..tostring(constant))
                _G.s2 = _audio_command_handler("dsp;net_bus;1;2")
                _G.s3 = _audio_command_handler("dsp;net_pipe;1;2")
                _G.s4 = _audio_command_handler("dsp;net_stack;1;2")
                -- Failures
                _G.f1 = _audio_command_handler("dsp;net_product;1;2")
                _G.f2 = _audio_command_handler("dsp;net_bus;1;100")
                _G.f3 = _audio_command_handler("dsp;net_pipe;1;100")
                _G.f4 = _audio_command_handler("dsp;net_stack;1;100")
            "#;

            assert!(lua.load(test_program).exec().is_ok());

            let s1 = globals.get::<String>("s1").unwrap();
            let s2 = globals.get::<String>("s2").unwrap();
            let s3 = globals.get::<String>("s3").unwrap();
            let s4 = globals.get::<String>("s4").unwrap();
            let f1 = globals.get::<String>("f1").unwrap();
            let f2 = globals.get::<String>("f2").unwrap();
            let f3 = globals.get::<String>("f3").unwrap();
            let f4 = globals.get::<String>("f4").unwrap();

            // Successes
            assert_eq!(s1, (NodeType::get_defaults().len() + 1).to_string());
            assert_eq!(s2, (NodeType::get_defaults().len() + 2).to_string());
            assert_eq!(s3, (NodeType::get_defaults().len() + 3).to_string());
            assert_eq!(s4, (NodeType::get_defaults().len() + 4).to_string());
            // Failures
            assert_eq!(f1, "nil".to_string());
            assert_eq!(f2, "nil".to_string());
            assert_eq!(f3, "nil".to_string());
            assert_eq!(f4, "nil".to_string());

            Ok(())
        });
    }
}
