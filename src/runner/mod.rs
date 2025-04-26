use crate::{project::Project};
use mlua::*;

pub struct Runner {
    lua: Lua,
    project: Project,
}

impl Runner {
    /// Creates a new runner based off a pre-existing project.
    pub fn new(project: Project) -> Runner {
        Runner {
            lua: Lua::new(),
            project: project,
        }
    }
    
    /// Load the program and run it
    pub fn run(&self) {
        let globals = self.lua.globals();
        let user_chunk = self.lua.load(self.project.get_program());

        user_chunk.exec();

        /*let test: Function = globals.get("test_function").expect("No function");
        test.call::<()>("Hello from rust");*/
    }
}
