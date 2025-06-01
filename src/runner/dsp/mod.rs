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
    Pulse,
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
            NodeType::Pulse => Box::new(pulse()),
            NodeType::Saw => Box::new(saw()),
            NodeType::Sine => Box::new(sine()),
            NodeType::SoftSaw => Box::new(soft_saw()),
            NodeType::Square => Box::new(square()),
            NodeType::Triangle => Box::new(triangle()),
            NodeType::Shared(key) => panic!("Not implemented"),
            NodeType::Constant(num) => Box::new(constant(num.clone())),
        }
    }
}

// TODO: Add nearly all NodeTypes as predefined nets
pub struct DspModule {
    nets: Vec<Net>,
}

impl DspModule {
    pub fn new() -> DspModule {
        DspModule { nets: Vec::new() }
    }

    /* Network Management Functions */
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

    /* Network Functions */
    pub fn net_product(&mut self, target_a: usize, target_b: usize) -> Option<usize> {
        if !self.net_exists(target_a) || !self.net_exists(target_b) {
            return None;
        }

        let net_a = self.nets[target_a].clone();
        let net_b = self.nets[target_b].clone();

        if !Net::can_product(&net_a, &net_b) {
            return None;
        }

        let new_network = Net::product(net_a, net_b);

        Some(self.net_from(&new_network))
    }

    pub fn net_sum(&mut self, target_a: usize, target_b: usize) -> Option<usize> {
        if !self.net_exists(target_a) || !self.net_exists(target_b) {
            return None;
        }

        let net_a = self.nets[target_a].clone();
        let net_b = self.nets[target_b].clone();

        if !Net::can_sum(&net_a, &net_b) {
            return None;
        }

        let new_network = Net::sum(net_a, net_b);

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

        // If target_b is a predefined network (such as a sine), create a new network.
        // Otherwise replace target_b with the pipe result
        if target_b <= 0 {
            return Some(self.net_from(&new_network));
        }
        else {
            return self.net_replace(target_b, &new_network);
        }
    }

    pub fn net_push(&mut self, target_net: usize, node_type: NodeType) -> Option<NodeId> {
        if !self.net_exists(target_net) {
            return None;
        }

        Some(self.nets[target_net].push(node_type.as_unit()))
    }

    pub fn net_chain(&mut self, target_net: usize, node_type: NodeType) -> Option<NodeId> {
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
    use crate::runner::DspModule;
    use fundsp::hacker32::*;

    /* Network Testing */
    // Tests all network management functions
    #[test]
    pub fn test_net_management() {
        let mut dsp = DspModule::new();

        // Test that start of user networks are empty
        assert!(!dsp.net_exists(0));

        // Test create network entries from net_from
        let id1 = dsp.net_from(&Net::new(0,3));
        assert_eq!(id1, 0);
        assert!(dsp.net_exists(0));
        assert!(!dsp.net_exists(1));
        let id2 = dsp.net_from(&Net::new(0,4));
        assert_eq!(id2, 1);
        assert!(dsp.net_exists(1));

        // Test net_replace
        // TODO: make this actually test whether or not 
        // the network was replaced
        assert!(dsp.net_replace(2, &Net::new(5,5)).is_none());
        assert_eq!(dsp.net_replace(1, &Net::new(5,5)), Some(1));
    }

    #[test]
    pub fn test_net_functions() {
        // TODO: Test all network combination functions
    }
}
