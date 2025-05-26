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
    pub fn net_reserve(&mut self) -> usize {
        self.nets.push(Net::new(0, 2));
        return self.nets.len() - 1;
    }

    pub fn net_push(&mut self, target_net: usize, node_type: NodeType) -> Option<NodeId> {
        if target_net >= self.nets.len() {
            return None;
        }

        Some(self.nets[target_net].push(node_type.as_unit()))
    }

    pub fn net_chain(&mut self, target_net: usize, node_type: NodeType) -> Option<NodeId> {
        if target_net >= self.nets.len() {
            return None;
        }

        Some(self.nets[target_net].chain(node_type.as_unit()))
    }

    pub fn net_
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
}
