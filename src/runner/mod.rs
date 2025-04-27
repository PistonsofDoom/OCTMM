use crate::project::Project;
use mlua::*;

mod timer;

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

    /// Load a string containing Luau code, then execute it
    /// Also takes in a 'name', which is only used for the
    /// error message
    fn load_program(&self, program: &str, name: &str) {
        self.lua
            .load(program)
            .exec()
            .expect(format!("Failed to load \"{}\", got\n", name).as_str());
    }

    /// Load the program and run it
    pub fn run(&self) {
        let globals = self.lua.globals();

        // Internal Modules
        self.load_program(timer::LUA_MODULE, "internal module timer.luau");

        // User program
        self.load_program(self.project.get_program(), "user program");
    }
}
