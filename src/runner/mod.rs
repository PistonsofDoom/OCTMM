use crate::{project::Project, runner::timer::TimerModule};
use mlua::*;

mod timer;

pub trait Module {
    fn init(&self, lua: &Lua);
    fn update(&self, time: &f64, lua: &Lua);
    fn end(&self, lua: &Lua);

    fn get_program(&self) -> &str;
    fn get_name(&self) -> &str;
}

pub struct Runner {
    project: Project,
    now: std::time::Instant,
    lua: Lua,
    modules: [Box<dyn Module>; 1],
}

impl Runner {
    /// Creates a new runner based off a pre-existing project.
    pub fn new(project: Project) -> Runner {
        let lua = Lua::new();
        Runner {
            project: project,
            now: std::time::Instant::now(),
            lua: lua,
            modules: [Box::new(TimerModule::new())],
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
        // Initialize all internal modules
        for module in &self.modules {
            module.init(&self.lua);
            self.load_program(module.get_program(), module.get_name());
        }

        // Load user program
        self.load_program(self.project.get_program(), "user program");

        // Initiate program loop
        let globals = self.lua.globals();
        // Compensate for long initilizations
        let start_millis = self.now.elapsed().as_millis();

        loop {
            let time_passed: f64 = (self.now.elapsed().as_millis() - start_millis) as f64 / 1000.0;

            // Update all internal modules
            for module in &self.modules {
                module.update(&time_passed, &self.lua);
            }

            // Check if we should end the song
            let end_song: bool = globals.get("EndSong").unwrap_or(false);
            if end_song {
                break;
            }

            // Give the CPU a lil snooze
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        // Call 'end' on all internal modules
        for module in &self.modules {
            module.end(&self.lua);
        }
    }
}
