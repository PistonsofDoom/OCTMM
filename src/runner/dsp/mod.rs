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
    // This is fine, but the previous steps stay in memory. This could potentially be optimized by
    // implementing a "temporary network" index
    pub fn net_exists(&mut self, target_net: usize) -> bool {
        return target_net < self.nets.len();
    }

    pub fn net_new(&mut self) -> usize {
        self.nets.push(Net::new(0, 2));
        return self.nets.len() - 1;
    }

    pub fn net_from(&mut self, new_network: &Net) -> usize {
        self.nets.push(new_network.clone());
        return self.nets.len() - 1;
    }

    /* Network Combination Functions */
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

        Some(self.net_from(&new_network))
    }

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

    /* Network Usage Functions */
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
    #[test]
    pub fn test_net_vector() {
        // TODO: Test all vector facing functions (e.g., net_exists, net_reserve, net_from
    }

    #[test]
    pub fn test_net_combination() {
        // TODO: Test all network combination functions
    }

    #[test]
    pub fn test_net_usage()
    {
        // TODO: Test all but net_commit under "Usage Functions"
    }
}
