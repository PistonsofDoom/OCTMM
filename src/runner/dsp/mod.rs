/*
 * Valid Operations:
 * | Expr   | Name |
 * | A + B  | Sum  |
 * | A - B  | Diff |
 * | A * B  | Mix  |
 * | A >> B | Pipe |
 *
 * Order of Operations:
 *  Left to right, but with parenthesis to define specific order
 *
 * Oscillators:
 *  hammond
 *  organ
 *  pulse
 *  saw
 *  sine
 *  soft_saw
 *  square
 *  triangle
 *
 * Special:
 *  shared -> Defined by the character ':' before the start of a string key
 *            If no entry exists, it will default to 1.0
 *
 *  constants -> Any defined numbers
 *
 * Example:
 *
 * (:freq * 2.0) >> sine
 *
 * This is equivalent to taking a shared variable "freq", multiplying
 * it by 2, and piping it into a "sine" oscillator.
 *
*/
use crate::runner::Module;
use fundsp::hacker32::*;
use mlua::Lua;

#[derive(Debug)]
enum NodeType {
    // Oscillators
    Hammond,
    Organ,
    Saw,
    Sine,
    SoftSaw,
    Square,
    Triangle,

    // Special Number
    Shared(String),
    Constant(f32),
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
            NodeType::Shared(key) => panic!("Not implemented"),
            NodeType::Constant(num) => Box::new(constant(num.clone())),
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
            _ => None,
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

// TODO: Add nearly all NodeTypes as predefined nets
pub struct DspModule {
    nets: Vec<Net>,
}

impl DspModule {
    pub fn new() -> DspModule {
        DspModule {
            nets: NodeType::get_defaults(),
        }
    }

    /* Network Management Functions */
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

    pub fn net_vector_length(&self) -> usize {
        return self.nets.len();
    }

    /* Network Functions */
    /*
     * Equivalent of the functions provided by the
     * fundsp "Net" struct, but with more
     * checks
     */
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

    /*pub fn net_push(&mut self, target_net: usize, node_type: NodeType) -> Option<NodeId> {
        if !self.net_exists(target_net) {
            return None;
        }

        Some(self.nets[target_net].push(node_type.as_unit()))
    }*/

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

impl Module for DspModule {
    fn init(&self, _lua: &Lua) {}
    fn update(&self, _time: &f64, _lua: &Lua) {}
    fn end(&self, _lua: &Lua) {}
}

#[cfg(test)]
mod tests {
    use super::{DspModule, NodeType};
    use fundsp::hacker32::*;

    /* Network Testing */
    // Tests all network management functions
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

        // TODO: Test net_product when Constant / Shared is implemented

        /*let my_network = dsp.net_product(hammond, organ);
        assert!(my_network.is_some());
        println!("{}",dsp.nets[my_network.unwrap()].inputs());*/

        let my_network = dsp.net_bus(hammond, square);
        assert!(my_network.is_some());
        println!("{}",dsp.nets[my_network.unwrap()].inputs());

        let my_network = dsp.net_pipe(my_network.unwrap(), sine);
        assert!(my_network.is_some());
        println!("{}",dsp.nets[my_network.unwrap()].inputs());

        let my_network = dsp.net_pipe(sine, my_network.unwrap());
        assert!(my_network.is_some());

        let my_node_id = dsp.net_chain(my_network.unwrap(), &NodeType::Sine);
        assert!(my_node_id.is_some());
    }
}
