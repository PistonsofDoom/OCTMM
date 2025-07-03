use crate::runner::CommandModule;
use fundsp::hacker32::*;
use mlua::Lua;
use std::collections::HashMap;

const LUA_MODULE: &str = include_str!("dsp.luau");

#[derive(Debug)]
pub enum NodeType {
    // Oscillators
    Hammond,
    Organ,
    Saw,
    Sine,
    SoftSaw,
    Square,
    Triangle,
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
        ])
    }

    /// Hard-coded value of the "get_defaults()" vector size
    pub fn get_defaults_size() -> usize {
        7
    }
}

pub struct DspModule {
    nets: Vec<Net>,
    shared: HashMap<String, Shared>,
    shared_to_net: HashMap<String, usize>,
}

impl DspModule {
    pub fn new() -> DspModule {
        DspModule {
            nets: NodeType::get_defaults(),
            shared: HashMap::new(),
            shared_to_net: HashMap::new(),
        }
    }

    /* Shared Management */

    /// Returns whether a "shared" entry exists or not.
    pub fn shared_exists(&mut self, name: &String) -> bool {
        return self.shared.contains_key(name);
    }

    /// Set a shared value
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

    /// Check whether or an a network entry exists at the target index
    pub fn net_exists(&mut self, target: usize) -> bool {
        return target < self.nets.len();
    }

    /// Create a new network entry from a new network
    pub fn net_from(&mut self, new_network: &Net) -> usize {
        self.nets.push(new_network.clone());
        return self.nets.len() - 1;
    }

    /// Replace a pre-existing network entry with a new network
    pub fn net_replace(&mut self, target: usize, new_network: &Net) -> Option<usize> {
        if !self.net_exists(target) {
            return None;
        }

        self.nets[target] = new_network.clone();
        return Some(target);
    }

    /// Create a new network that is a constant of the value
    // NOTE: possible "optimization" by caching constants
    pub fn net_constant(&mut self, value: f32) -> usize {
        self.net_from(&Net::wrap(Box::new(constant(value))))
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

    pub fn net_bus(&mut self, target_a: usize, target_b: usize) -> Option<usize> {
        if !self.net_exists(target_a) || !self.net_exists(target_b) {
            return None;
        }

        let net_a = self.nets[target_a].clone();
        let net_b = self.nets[target_b].clone();

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
            return None;
        }

        let new_network = Net::pipe(net_a, net_b);
        return Some(self.net_from(&new_network));
    }

    pub fn net_chain(&mut self, target_net: usize, node_type: &NodeType) -> Option<NodeId> {
        if !self.net_exists(target_net) {
            return None;
        }

        Some(self.nets[target_net].chain(node_type.as_unit()))
    }

    pub fn net_commit(&mut self, target_net: usize) {
        if self.net_exists(target_net) {
            self.nets[target_net].commit();
        }
    }
}

impl CommandModule for DspModule {
    fn init(&mut self, lua: &Lua) {
        lua.load(LUA_MODULE)
            .exec()
            .expect("Failed to load DSP module, got\n");
    }
    fn end(&mut self, _lua: &Lua) {}

    fn get_command_name(&self) -> String {
        "_dsp_command_handler".to_string()
    }
    fn command(&mut self, lua: &Lua, arg: &String) -> String {
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
            "net_vector_length" => {
                return self.net_vector_length().to_string();
            }
            // Network Proxy Commands
            "net_default" => {
                let arg_type = arg_vec.get(1).expect("net_default, type not found");

                return match *arg_type {
                    "hammond" => NodeType::Hammond.as_net_id().unwrap().to_string(),
                    "organ" => NodeType::Organ.as_net_id().unwrap().to_string(),
                    "saw" => NodeType::Saw.as_net_id().unwrap().to_string(),
                    "sine" => NodeType::Sine.as_net_id().unwrap().to_string(),
                    "softsaw" => NodeType::SoftSaw.as_net_id().unwrap().to_string(),
                    "square" => NodeType::Square.as_net_id().unwrap().to_string(),
                    "triangle" => NodeType::Triangle.as_net_id().unwrap().to_string(),
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
            "net_commit" => {
                let arg_id = arg_vec
                    .get(1)
                    .expect("net_commit, id not found")
                    .parse::<usize>()
                    .expect("net_commit, string conversion");

                self.net_commit(arg_id);
            }
            // Handle bad commands
            _ => {
                panic!(
                    "Tried to call command {} which doesn't exist for DSP module",
                    arg_cmd
                );
            }
        }

        return "nil".to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::{DspModule, NodeType};
    use crate::runner::{CommandModule, dsp};
    use fundsp::hacker32::*;
    use mlua::Lua;

    /* Shared Testing */
    #[test]
    pub fn test_shared_management() {
        let mut dsp = DspModule::new();
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
        let mut dsp = DspModule::new();

        let default_length: usize = NodeType::get_defaults_size();

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

        // Test net_replace
        // TODO: make this actually test whether or not
        // the network was replaced

        // Should fail, as network doesn't exist here
        assert!(
            dsp.net_replace(default_length + 2, &Net::new(5, 5))
                .is_none()
        );
        // Should succeed, as network does exist
        assert_eq!(
            dsp.net_replace(default_length, &Net::new(5, 5)),
            Some(default_length)
        );

        // Test net_constant
        // TODO: test value of constant?
        assert_eq!(dsp.net_constant(12.3), default_length + 2);
    }

    #[test]
    pub fn test_net_functions() {
        let mut dsp = DspModule::new();

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

        // Test net_pipe
        let my_network = dsp.net_pipe(my_network.unwrap(), sine);
        assert!(my_network.is_some());

        let my_network = dsp.net_pipe(sine, my_network.unwrap());
        assert!(my_network.is_some());

        // Test net_chain
        let my_node_id = dsp.net_chain(my_network.unwrap(), &NodeType::Sine);
        assert!(my_node_id.is_some());
    }

    #[test]
    fn test_rust_module() {
        let lua = Lua::new();
        let globals = lua.globals();
        let module: &mut dyn CommandModule = &mut DspModule::new();

        let _ = lua.scope(|scope| {
            module.init(&lua);

            lua.globals()
                .set(
                    module.get_command_name(),
                    scope.create_function_mut(|_, arg: String| Ok(module.command(&lua, &arg)))?,
                )
                .expect("Error using command function");

            /*let test_program = r#"
                _dsp_command_handler("command type;arg 2; arg3")
            "#;

            assert!(lua.load(test_program).exec().is_ok());
            assert!(globals.get::<bool>("SUCCESS").is_ok());
            assert!(globals.get::<bool>("SUCCESS").unwrap());*/

            Ok(())
        });
    }

    // LUA CODE TESTS
    #[test]
    fn test_shared_commands() {
        let lua = Lua::new();
        let globals = lua.globals();
        let module: &mut dyn CommandModule = &mut DspModule::new();

        let _ = lua.scope(|scope| {
            module.init(&lua);

            lua.globals()
                .set(
                    module.get_command_name(),
                    scope.create_function_mut(|_, arg: String| Ok(module.command(&lua, &arg)))?,
                )
                .expect("Error using command function");

            let test_program = r#"
                _G.r1 = _dsp_command_handler("shared_exists;test")
                _G.r2 = _dsp_command_handler("shared_set;test;1.2")
                _G.r3 = _dsp_command_handler("shared_exists;test")
                _G.r4 = _dsp_command_handler("shared_get;test")
                _G.r5 = _dsp_command_handler("shared_get_net;test")
            "#;

            assert!(lua.load(test_program).exec().is_ok());

            let r1 = globals.get::<String>("r1").unwrap();
            let r2 = globals.get::<String>("r2").unwrap();
            let r3 = globals.get::<String>("r3").unwrap();
            let r4 = globals.get::<String>("r4").unwrap();
            let r5 = globals.get::<String>("r5").unwrap();

            assert_eq!(r1, "false");
            assert_eq!(r2, NodeType::get_defaults_size().to_string());
            assert_eq!(r3, "true");
            assert_eq!(r4, "1.2");
            assert_eq!(r5, NodeType::get_defaults_size().to_string());

            Ok(())
        });
    }

    #[test]
    fn test_net_management_commands() {
        let lua = Lua::new();
        let globals = lua.globals();
        let module: &mut dyn CommandModule = &mut DspModule::new();

        let _ = lua.scope(|scope| {
            module.init(&lua);

            lua.globals()
                .set(
                    module.get_command_name(),
                    scope.create_function_mut(|_, arg: String| Ok(module.command(&lua, &arg)))?,
                )
                .expect("Error using command function");

            let test_program = r#"
                _G.r1 = _dsp_command_handler("net_vector_length")
                _G.r2 = _dsp_command_handler("net_exists;" .. tostring(_G.r1))
                _G.r3 = _dsp_command_handler("net_constant;3.3")
                _G.r4 = _dsp_command_handler("net_exists;" .. tostring(_G.r1))
                _G.r5 = _dsp_command_handler("net_clone;0")
            "#;

            assert!(lua.load(test_program).exec().is_ok());

            let r1 = globals.get::<String>("r1").unwrap();
            let r2 = globals.get::<String>("r2").unwrap();
            let r3 = globals.get::<String>("r3").unwrap();
            let r4 = globals.get::<String>("r4").unwrap();
            let r5 = globals.get::<String>("r5").unwrap();

            assert_eq!(r1, NodeType::get_defaults_size().to_string());
            assert_eq!(r2, "false");
            assert_eq!(r3, NodeType::get_defaults_size().to_string());
            assert_eq!(r4, "true");
            assert_eq!(r5, (NodeType::get_defaults_size() + 1).to_string());

            Ok(())
        });
    }

    #[test]
    fn test_net_proxy_commands() {
        let lua = Lua::new();
        let globals = lua.globals();
        let module: &mut dyn CommandModule = &mut DspModule::new();

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
                _G.r1 = _dsp_command_handler("net_default;hammond")
                _G.r2 = _dsp_command_handler("net_default;organ")
                _G.r3 = _dsp_command_handler("net_default;saw")
                _G.r4 = _dsp_command_handler("net_default;sine")
                _G.r5 = _dsp_command_handler("net_default;softsaw")
                _G.r6 = _dsp_command_handler("net_default;square")
                _G.r7 = _dsp_command_handler("net_default;triangle")
                _G.r8 = _dsp_command_handler("net_default;badinput")
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

            assert_eq!(r1, NodeType::Hammond.as_net_id().unwrap().to_string());
            assert_eq!(r2, NodeType::Organ.as_net_id().unwrap().to_string());
            assert_eq!(r3, NodeType::Saw.as_net_id().unwrap().to_string());
            assert_eq!(r4, NodeType::Sine.as_net_id().unwrap().to_string());
            assert_eq!(r5, NodeType::SoftSaw.as_net_id().unwrap().to_string());
            assert_eq!(r6, NodeType::Square.as_net_id().unwrap().to_string());
            assert_eq!(r7, NodeType::Triangle.as_net_id().unwrap().to_string());
            assert_eq!(r8, "nil".to_string());

            // Test all other proxys
            let test_program = r#"
                _G.r1 = _dsp_command_handler("net_default;hammond")
            "#;

            assert!(lua.load(test_program).exec().is_ok());

            let r1 = globals.get::<String>("r1").unwrap();

            assert_eq!(r1, NodeType::Hammond.as_net_id().unwrap().to_string());
            
            Ok(())
        });
    }
}
