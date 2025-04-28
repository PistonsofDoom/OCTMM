use crate::{project::Project, runner::timer::TimerModule};
use mlua::*;

mod timer;

pub trait Module {
    fn init(&self, lua: &Lua);
    fn update(&self, time: f64, lua: &Lua);
    fn end(&self, lua: &Lua);

    fn get_program(&self) -> &str;
    fn get_name(&self) -> &str;
}

pub struct Runner {
    project: Project,
    lua: Lua,
    modules: [Box<dyn Module>; 1],
}

impl Runner {
    /// Creates a new runner based off a pre-existing project.
    pub fn new(project: Project) -> Runner {
        let lua = Lua::new();
        Runner {
            project: project,
            lua: lua,
            modules: [
                Box::new(TimerModule::new()),
            ],
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
        // Initialize internal all modules
        for module in &self.modules {
            module.init(&self.lua);
            self.load_program(module.get_program(), module.get_name());
        }

        // Load user program
        self.load_program(self.project.get_program(), "user program");

        // Initiate runtime
        loop {
            break;
        }

        // Call 'end' on all internal modules
        for module in &self.modules {
            module.end(&self.lua);
        }
    }
}
