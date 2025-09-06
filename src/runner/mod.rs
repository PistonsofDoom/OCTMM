use crate::{project::Project, runner::audio::AudioModule, runner::timer::TimerModule};
use mlua::Lua;
use std::path::PathBuf;

mod audio;
mod timer;

pub trait CommandModule {
    /// Ran before runtime, and before commands are binded
    fn init(&mut self, lua: &Lua);
    /// Ran every 'tick' during runtime
    /// This is ran before Rust <-> Lua callbacks are defined.
    fn update(&mut self, time: &f64, lua: &Lua);
    /// Ran after runtime
    fn end(&mut self, lua: &Lua);

    /// Optionally, return a string referring to a lua program to run after commands are setup
    fn get_post_init_program(&self) -> Option<String>;
    /// Return a String that refers to the lua global the command() rust function should be binded to
    fn get_command_name(&self) -> String;
    /// Function that gets binded to a lua global
    fn command(&mut self, lua: &Lua, arg: &String) -> String;
}

pub trait PollingModule {
    /// Ran before runtime
    fn init(&mut self, lua: &Lua);
    /// Ran every 'tick' during runtime
    fn update(&mut self, time: &f64, lua: &Lua);
    /// Ran after runtime
    fn end(&mut self, lua: &Lua);
}

pub struct Runner {
    command_modules: [Box<dyn CommandModule>; 1],
    polling_modules: [Box<dyn PollingModule>; 1],

    project: Project,
    is_live: bool, // If this is true, audio is being played live.
    // Otherwise, we are exporting it.
    now: std::time::Instant,
    lua: Lua,
}

impl Runner {
    /// Creates a new runner based off a pre-existing project.
    pub fn new(project: Project, export: Option<PathBuf>) -> Runner {
        Runner {
            is_live: export.is_none(),

            command_modules: [Box::new(AudioModule::new(project.get_samples(), export))],
            polling_modules: [Box::new(TimerModule::new())],

            project: project,
            now: std::time::Instant::now(),
            lua: Lua::new(),
        }
    }

    fn initialize_time(&mut self) {
        self.now = std::time::Instant::now();
    }

    fn get_time(&mut self) -> f64 {
        self.now.elapsed().as_millis() as f64 / 1000.0
    }

    /// Load the program and run it
    pub fn run(&mut self) {
        // Scope for initilization
        let _ = self.lua.scope(|scope| {
            // Initialize all internal modules
            for module in &mut self.polling_modules {
                module.init(&self.lua);
            }

            for module in &mut self.command_modules {
                let post_init_program = module.get_post_init_program();

                module.init(&self.lua);

                self.lua
                    .globals()
                    .set(
                        module.get_command_name(),
                        scope.create_function_mut(|_, arg: String| {
                            Ok(module.command(&self.lua, &arg))
                        })?,
                    )
                    .expect("Error using command function");

                if post_init_program.is_some() {
                    self.lua
                        .load(post_init_program.unwrap())
                        .exec()
                        .expect("Failed to load post init on module, got\n")
                }
            }

            // Load project modules
            for program in self.project.get_modules() {
                self.lua
                    .load(program)
                    .exec()
                    .expect("Failed to load project module, got\n");
            }

            // Load user program
            self.lua
                .load(self.project.get_program())
                .exec()
                .expect("Failed to load user program, got\n");

            // End scope
            Ok(())
        });

        // Initiate program loop
        let globals = self.lua.globals();

        println!("Took {} seconds to load project", self.get_time());
        self.initialize_time();
        loop {
            let time_passed: f64 = self.get_time();

            // Command update functions
            for module in &mut self.command_modules {
                module.update(&time_passed, &self.lua);
            }

            // Scope so user can call callback functions
            // NOTE: I don't like doing this, but it allows us to call an update() function on all
            // command modules, while still allowing proper callbacks.
            let _ = self.lua.scope(|scope| {
                for module in &mut self.command_modules {
                    self.lua
                        .globals()
                        .set(
                            module.get_command_name(),
                            scope.create_function_mut(|_, arg: String| {
                                Ok(module.command(&self.lua, &arg))
                            })?,
                        )
                        .expect("Error using command function");
                }

                // Update all polling modules
                for module in &mut self.polling_modules {
                    module.update(&time_passed, &self.lua);
                }

                // End scope
                Ok(())
            });

            // Check if we should end the song
            let end_song: bool = globals.get("EndSong").unwrap_or(false);
            if end_song {
                break;
            }

            // Give the CPU a lil snooze if playing live
            if self.is_live {
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        }

        // Call 'end' on all internal modules
        for module in &mut self.polling_modules {
            module.end(&self.lua);
        }
        for module in &mut self.command_modules {
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
        assert!(Project::create(&tmp, &"runner_test_prj".to_string()).is_ok());

        let mut proj_dir = tmp.clone();
        proj_dir.push("runner_test_prj");

        // Modify program file
        let mut program_dir = proj_dir.clone();
        program_dir.push(crate::project::FILE_PROGRAM);

        let mut program = File::create(program_dir).expect("Couldn't create file");
        program
            .write_all(b"_G.TestFunc()")
            .expect("Couldn't write to program file");

        // Add module
        let mut module_dir = proj_dir.clone();
        module_dir.push(crate::project::DIR_MODULES);
        module_dir.push("test-module.luau");
        let mut module = File::create(module_dir).expect("Couldn't create file");
        module
            .write_all(b"_G.TestFunc = function() _G.EndSong = true end")
            .expect("Couldn't write to module file");

        // Load project
        let project = Project::load(&proj_dir).expect("Failed to load project");

        // Test Runner
        let mut runner = Runner::new(project, None);

        runner.run();
    }
}
