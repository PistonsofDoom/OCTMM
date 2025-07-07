use crate::runner::{CommandModule, audio::dsp::DspModule};
use fundsp::hacker32::*;
use mlua::Lua;
use std::collections::HashMap;

mod dsp;

pub struct AudioModule {
    dsp: DspModule,
}

impl AudioModule {
    pub fn new() -> AudioModule {
        AudioModule {
            dsp: DspModule::new(),
        }
    }
}

impl CommandModule for AudioModule {
    fn init(&mut self, lua: &Lua) {
        self.dsp.init(lua);
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

        if arg_cmd == dsp_cmd_name {
            return self.dsp.command(
                lua,
                &arg.strip_prefix((dsp_cmd_name.to_owned() + ";").as_str())
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
