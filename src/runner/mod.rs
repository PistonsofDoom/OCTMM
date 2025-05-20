use crate::{project::Project, runner::dsp::DspModule, runner::timer::TimerModule};
use mlua::*;

mod dsp;
mod timer;

pub trait Module {
    fn init(&self, lua: &Lua);
    fn update(&self, time: &f64, lua: &Lua);
    fn end(&self, lua: &Lua);
}

pub struct Runner {
    project: Project,
    now: std::time::Instant,
    lua: Lua,
    modules: [Box<dyn Module>; 2],
}

impl Runner {
    /// Creates a new runner based off a pre-existing project.
    pub fn new(project: Project) -> Runner {
        Runner {
            project: project,
            now: std::time::Instant::now(),
            lua: Lua::new(),
            modules: [Box::new(TimerModule::new()), Box::new(DspModule::new())],
        }
    }

    /// Load the program and run it
    pub fn run(&self) {
        // Initialize all internal modules
        for module in &self.modules {
            module.init(&self.lua);
        }

        // Load user program
        self.lua
            .load(self.project.get_program())
            .exec()
            .expect("Failed to load user program, got\n");

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

#[cfg(test)]
mod tests {
    use crate::{project::Project, runner::Runner, test_utils::make_test_dir};
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_runner() {
        let tmp = make_test_dir("runner_test");
        assert!(tmp.is_some());
        let tmp = tmp.unwrap();

        // Make test project
        assert!(Project::new(&tmp, &"runner_test_prj".to_string()).is_ok());

        let mut proj_dir = tmp.clone();
        proj_dir.push("runner_test_prj");

        // Modify program file
        let mut program_dir = proj_dir.clone();
        program_dir.push(crate::project::FILE_PROGRAM);

        let mut program = File::create(program_dir).expect("Couldn't create file");
        program
            .write_all(b"_G.EndSong = true")
            .expect("Couldn't write to file");

        // Load project
        let project = Project::load(&proj_dir).expect("Failed to load project");

        // Test Runner
        let runner = Runner::new(project);

        runner.run();
    }
}
