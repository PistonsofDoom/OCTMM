/*
 *
 *  OKAY
 *  Make a custom string parser for synths,
 *  when reading the string, go something like this:
 *  remove all whitespace
 *  match words between operators
 *  if word doesn't exist, panic!
 *  else, bind it to an enum
 *  end up putting it all in a sorted vector, ordered by the order of
 *  operations.
 *
 *  HAVE FUN! (no though seriously, this should be interesting lol)
 *
 *
 */
/*
 * Valid Operations:
 * | Expr   | Name |
 * | A + B  | Sum  |
 * | A - B  | Diff |
 * | A * B  | Mix  |
 * | A >> B | Pipe |
 *
 * Order of Operations:
 * Left to right, but with parenthesis to define specific order
 *
 * Oscillators:
 * [ ] hammond
 * [ ] organ
 * [ ] pulse
 * [ ] saw
 * [ ] sine
 * [ ] soft_saw
 * [ ] square
 * [ ] triangle
 *
 * Special:
 * [ ] shared -> Defined by two square brackets, with the inside being
 *              the key to a dictionary of values. If no entry exists, it
 *              is 1.0
 *
 * [ ] constants -> Any defined numbers
 *
 * Example:
 *
 * ([freq] * 2.0) >> sine
 *
 * This is equivalent to taking a shared variable "frequency", multiplying
 * it by 2, and piping it into a "sine" oscillator.
 *
*/
use crate::runner::Module;
use mlua::Lua;

#[derive(Debug)]
enum Instruction {
    // Oscillators
    Hammond,
    Organ,
    Pulse,
    Saw,
    Sine,
    SoftSaw,
    Square,
    Triangle,

    // Special
    Shared(String),
    Constant(f64),
    Group(std::string::String),
}

impl Instruction {
    pub fn from_string(s: &str) -> Option<Instruction> {
        let oscillator: Option<Instruction> = match s {
            "hammond" => Some(Instruction::Hammond),
            "organ" => Some(Instruction::Organ),
            "pulse" => Some(Instruction::Pulse),
            "saw" => Some(Instruction::Saw),
            "sine" => Some(Instruction::Sine),
            "softsaw" => Some(Instruction::SoftSaw),
            "soft_saw" => Some(Instruction::SoftSaw),
            "square" => Some(Instruction::Square),
            "triangle" => Some(Instruction::Triangle),
            _ => None,
        };

        if oscillator.is_some() {
            return oscillator;
        }

        // If no oscillator is found, check for specials
        // Shared Enum
        if s.starts_with(":") {
            // This quick and dirty s.replace might cause issues if the shared has multiple colons
            // (for some reason?)
            return Some(Instruction::Shared(s.replace(":", "")));
        }

        // Constants Enum
        let number = s.parse::<f64>();

        if number.is_ok() {
            return Some(Instruction::Constant(number.unwrap()));
        }

        // No instruction was found, return none
        None
    }
}

#[derive(Debug)]
enum Operation {
    Sum,
    Diff,
    Mix,
    Pipe,
}

impl Operation {
    pub fn from_char(c: &char) -> Option<Operation> {
        match c {
            '+' => Some(Operation::Sum),
            '-' => Some(Operation::Diff),
            '*' => Some(Operation::Mix),
            '>' => Some(Operation::Pipe),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    BadOperator(char),
    BadInstruction(String),
    MissingInstruction,
}

pub struct DspModule {}

impl DspModule {
    pub fn new() -> DspModule {
        DspModule {}
    }

    pub fn parse_string(&self, input: &String) -> Result<(), ParseError> {
        // Modify string to be easier to parse
        let mut filtered = input.clone();

        // Remove whitespace
        filtered.retain(|c| c != ' ');
        // Modify 'pipe' symbol for easier parsing
        filtered = filtered.replace(">>", ">");

        // Convert into vectors of instructions/operations
        let mut instructions: Vec<Instruction> = Vec::new();
        let mut operations: Vec<Operation> = Vec::new();

        let mut current_string: String = String::new();
        // If 'c' is (, and the length of current_string is
        // 0, we are in a group
        // This is an integer, as any sub-groups shouldn't terminate the
        // main group
        let mut group_depth = 0;
        // Once we are done parsing a group, the next character
        // should be an operator (if there is a character)
        let mut force_next_as_operator = false;

        for c in filtered.chars() {
            // If we are parsing a group
            if group_depth > 0 {
                // If our group_depth is 1, and the parenthesis is a close parenthesis, stop
                // parsing the group
                //
                // TODO: test a case such as (sine * 2.0) sine
                // as this case should not succeed
                if c == ')' && group_depth == 1 {
                    instructions.push(Instruction::Group(current_string.clone()));
                    current_string = String::new();

                    group_depth = 0;
                    force_next_as_operator = true;
                    continue;
                } else if c == '(' {
                    group_depth += 1;
                } else if c == ')' {
                    group_depth -= 1;
                }

                current_string += &c.to_lowercase().to_string();
                continue;
            }

            // If the character is a "group open", and its the first
            // character, start a group selection
            if c == '(' && !force_next_as_operator {
                // If this isn't the first character, this is a malformed
                // instruction
                if current_string.chars().count() != 0 {
                    return Err(ParseError::BadInstruction(current_string));
                }

                group_depth += 1;
                continue;
            }

            // Get as operator
            let op = Operation::from_char(&c);

            if force_next_as_operator {
                if op.is_none() {
                    return Err(ParseError::BadOperator(c));
                }

                operations.push(op.unwrap());

                force_next_as_operator = false;
                continue;
            }

            if op.is_some() {
                // Check if we somehow started with an operation
                if current_string.chars().count() == 0 {
                    return Err(ParseError::MissingInstruction);
                }

                let inst = Instruction::from_string(&current_string);

                // If instruction doesn't exist, throw an error
                if inst.is_none() {
                    return Err(ParseError::BadInstruction(current_string));
                }

                instructions.push(inst.unwrap());
                current_string = String::new();
                operations.push(op.unwrap());
            } else {
                // Make lowercase, as we don't care if its
                // SINE, sine, or SiNe
                current_string += &c.to_lowercase().to_string();
            }
        }

        // If current_string can be converted, add it to
        // the total instructions
        let inst = Instruction::from_string(&current_string);

        if inst.is_some() {
            instructions.push(inst.unwrap());
            current_string = String::new();
        }

        println!("In: {:?}", instructions);
        println!("Op: {:?}", operations);

        // String has been parsed, instructions & operations
        // vectors contain the steps needed to create
        // the AudioUnit
        let inst_index: u16 = 0;
        let source; // = //audiounit

        // For this step, source is the input to the next thing
        // AKA, this is the combined audio unit.
        // So tl;dr, source is persistent, and is added
        // to by doing
        // source = source (operation) thing
        // After all operations are done, this is done.
        // If there is a missing instruction for the operation,
        // then throw an error

        for op in operations {
            // Bounds check
            if inst_index >= instructions.len() {
                return Err(ParseError::MissingInstruction);
            }
        }

        Ok(())
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

    #[test]
    fn test_string_parser() {
        println!("    =-=-=-=-=-=-=-=-=-=-=-=-");
        println!("           test begin       ");
        println!("    -=-=-=-=-=-=-=-=-=-=-=-=");

        let dspmod = DspModule::new();

        let result = dspmod.parse_string(&":freq >> (sine+(saw * 0.5)) * :amp".to_string());
        println!("{:?}", result);
        let result = dspmod.parse_string(&":freq >> ((sine * 0.5)+saw) * (:amp + 1.0)".to_string());
        println!("{:?}", result);

        println!("    =-=-=-=-=-=-=-=-=-=-=-=-");
        println!("            test end      ");
        println!("    -=-=-=-=-=-=-=-=-=-=-=-=");
        panic!("YOU FAIL");
    }
}
