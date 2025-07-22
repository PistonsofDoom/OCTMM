use crate::runner::PollingModule;
use mlua::{Function, Lua, Table};

const LUA_MODULE: &str = include_str!("timer.luau");

enum CallbackType {
    Tick,
    Beat,
}

impl CallbackType {
    pub fn to_string(&self) -> String {
        match self {
            CallbackType::Tick => "tick".to_string(),
            CallbackType::Beat => "beat".to_string(),
        }
    }
}

pub struct TimerModule {}

impl TimerModule {
    pub fn new() -> TimerModule {
        TimerModule {}
    }

    pub fn type_from_string(from_str: String) -> Option<CallbackType> {
        let tick_string: String = CallbackType::Tick.to_string();
        let beat_string: String = CallbackType::Beat.to_string();

        match from_str {
            tick_string => Some(CallbackType::Tick),
            beat_string => Some(CallbackType::Beat),
            _ => None,
        }
    }
}

impl PollingModule for TimerModule {
    fn init(&mut self, lua: &Lua) {
        let globals = lua.globals();

        globals
            .set("BEAT", CallbackType::Beat.to_string())
            .expect("Error initializing BEAT lua constant");
        globals
            .set("TICK", CallbackType::Tick.to_string())
            .expect("Error initializing TICK lua constant");

        lua.load(LUA_MODULE)
            .exec()
            .expect("Failed to load timer module, got\n");
    }
    fn update(&mut self, time: &f64, lua: &Lua) {
        let timer: Table = lua
            .globals()
            .get("Timer")
            .expect("Didn't find 'Timer' table");

        let callbacks: Table = timer
            .get("_Callbacks")
            .expect("Didn't find `Timer._Callbacks`");
        let bpm: f64 = timer.get("_BPM").expect("Invalid BPM");

        timer.set("Time", time.clone()).expect("Unable to set Time");

        // optimization: use Table::for_each
        for pair in callbacks.pairs::<String, Table>() {
            let (key, value) = pair.expect("Invalid callback");
            let name: &str = &key.to_string();

            let call_type = TimerModule::type_from_string(
                value
                    .get("type")
                    .expect(format!("Invalid callback type on callback {}:", name).as_str()),
            )
            .expect(format!("Invalid callback type on callback {}:", name).as_str());
            let call_func: Function = value
                .get("func")
                .expect(format!("Invalid callback function on callback {}:", name).as_str());

            match call_type {
                CallbackType::Beat => {
                    let call_freq: f64 = value.get("freq").expect(
                        format!("Invalid callback frequency on callback {}:", name).as_str(),
                    );
                    let call_time: f64 = value.get("time").unwrap_or(0.0);

                    if time - call_time >= (60.0 / bpm) * call_freq {
                        let time = time.clone();

                        value.set("time", time).expect(
                            format!("Failed to set callback time on callback {}:", name).as_str(),
                        );
                        call_func.call::<()>(time).expect(
                            format!(
                                "Error occured while running beat update on callback {}:",
                                name
                            )
                            .as_str(),
                        );
                    }
                },
                CallbackType::Tick => {
                    let time = time.clone();
                    call_func.call::<()>(time).expect(
                        format!(
                            "Error occured while running tick update on callback {}:",
                            name
                        )
                        .as_str(),
                    );
                }
            }
        }
    }
    fn end(&mut self, _lua: &Lua) {}
}

#[cfg(test)]
mod tests {
    use crate::runner::{PollingModule, TimerModule, timer};
    use mlua::*;

    #[test]
    fn test_rust_module() {
        let lua = Lua::new();
        let globals = lua.globals();
        let timer: &mut dyn PollingModule = &mut TimerModule::new();

        assert!(lua.load(timer::LUA_MODULE).exec().is_ok());

        // Init environment
        let test_program = r#"
            local timer = _G.Timer

            _G.TestValue_Tick = 0
            _G.TestValue_Beat = 0
            _G.TestValue_Beat2 = 0


            local function tick_callback()
                _G.TestValue_Tick += 1
            end

            local function beat_callback()
                _G.TestValue_Beat += 1
            end

            local function beat2_callback()
                _G.TestValue_Beat2 += 1
            end

            SetBPM(60)

            timer.AddTickCallback("TickCall", tick_callback)
            timer.AddBeatCallback("BeatCall", 1, beat_callback)
            timer.AddBeatCallback("Beat2Call", 1/2, beat2_callback)
        "#;

        assert!(lua.load(test_program).exec().is_ok());

        // Update timer twice, this should call
        // the Tick Callback twice, and
        // the Beat Callback once
        timer.update(&0.5, &lua);
        timer.update(&1.0, &lua);
        timer.update(&1.25, &lua);

        // Test Values
        assert_eq!(
            globals
                .get::<f64>("TestValue_Tick")
                .expect("Didn't find freq"),
            3.0
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue_Beat2")
                .expect("Didn't find freq"),
            2.0
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue_Beat")
                .expect("Didn't find freq"),
            1.0
        );
    }

    // LUA CODE TESTS
    #[test]
    fn test_bpm_utilities() {
        let lua = Lua::new();
        let globals = lua.globals();

        lua.load(timer::LUA_MODULE)
            .exec()
            .expect("Failed to load lua module");

        // Test failure case
        let fail_case1 = r#"
            local timer = _G.Timer

            SetBPM(0)
        "#;
        let fail_case2 = r#"
            local timer = _G.Timer

            SetBPM("1")
        "#;

        assert!(lua.load(fail_case1).exec().is_err());
        assert!(lua.load(fail_case2).exec().is_err());

        // Test success case
        let success_case = r#"
            local timer = _G.Timer

            SetBPM(321.50)

            _G.TestValue_BPM = GetBPM()
        "#;

        assert!(lua.load(success_case).exec().is_ok());

        // Test global now
        assert_eq!(
            globals
                .get::<f64>("TestValue_BPM")
                .expect("Didn't find BPM value"),
            321.50
        );
    }

    #[test]
    fn test_add_tick_callback() {
        let lua = Lua::new();
        let globals = lua.globals();

        lua.load(timer::LUA_MODULE)
            .exec()
            .expect("Failed to load lua module");

        // Test fail cases
        let fail_case1 = r#"
            local timer = _G.Timer

            timer.AddTickCallback()
        "#;
        let fail_case2 = r#"
            local timer = _G.Timer

            timer.AddTickCallback("My Callback")
        "#;
        let fail_case3 = r#"
            local timer = _G.Timer

            timer.AddTickCallback("My Callback", 2)
        "#;

        assert!(lua.load(fail_case1).exec().is_err());
        assert!(lua.load(fail_case2).exec().is_err());
        assert!(lua.load(fail_case3).exec().is_err());

        // Test success case
        let success_case = r#"
            local timer = _G.Timer

            local function userCallFunction()

            end

            timer.AddTickCallback("UserCall", userCallFunction)
        "#;

        assert!(lua.load(success_case).exec().is_ok());

        // Test global now
        let timer: Table = globals.get("Timer").expect("Timer table not found");
        let callbacks: Table = timer.get("_Callbacks").expect("Callback table not found");
        let user_call: Table = callbacks
            .get("UserCall")
            .expect("Didn't find user callback");

        assert_eq!(
            user_call.get::<String>("type").expect("Didn't find type"),
            "tick"
        );
        assert!(user_call.get::<Function>("func").is_ok());
    }

    #[test]
    fn test_add_beat_callback() {
        let lua = Lua::new();
        let globals = lua.globals();

        lua.load(timer::LUA_MODULE)
            .exec()
            .expect("Failed to load lua module");

        // Test fail cases
        let fail_case1 = r#"
            local timer = _G.Timer

            timer.AddBeatCallback()
        "#;
        let fail_case2 = r#"
            local timer = _G.Timer

            timer.AddBeatCallback("My Callback")
        "#;
        let fail_case3 = r#"
            local timer = _G.Timer

            timer.AddBeatCallback("My Callback", 2)
        "#;
        let fail_case4 = r#"
            local timer = _G.Timer

            timer.AddBeatCallback("My Callback", 2, 2)
        "#;
        let fail_case5 = r#"
            local timer = _G.Timer

            local function test()

            end

            timer.AddBeatCallback("My Callback", false, test)
        "#;

        assert!(lua.load(fail_case1).exec().is_err());
        assert!(lua.load(fail_case2).exec().is_err());
        assert!(lua.load(fail_case3).exec().is_err());
        assert!(lua.load(fail_case4).exec().is_err());
        assert!(lua.load(fail_case5).exec().is_err());

        // Test success case
        let success_case = r#"
            local timer = _G.Timer

            local function userCallFunction()

            end

            timer.AddBeatCallback("UserCall", 1.0, userCallFunction)
        "#;

        assert!(lua.load(success_case).exec().is_ok());

        // Test global now
        let timer: Table = globals.get("Timer").expect("Timer table not found");
        let callbacks: Table = timer.get("_Callbacks").expect("Callback table not found");
        let user_call: Table = callbacks
            .get("UserCall")
            .expect("Didn't find user callback");

        assert_eq!(
            user_call.get::<String>("type").expect("Didn't find type"),
            "beat"
        );
    }

    #[test]
    fn test_get_callback_type() {
        let lua = Lua::new();
        let globals = lua.globals();

        lua.load(timer::LUA_MODULE)
            .exec()
            .expect("Failed to load lua module");

        // Test success case
        let success_case = r#"
            local timer = _G.Timer

            local function userCallFunction()

            end

            timer.AddTickCallback("TickCall", userCallFunction)
            timer.AddBeatCallback("BeatCall", 1.0, userCallFunction)

            _G.TestValue_Tick = timer.GetCallbackType("TickCall")
            _G.TestValue_Beat = timer.GetCallbackType("BeatCall")
            _G.TestValue_Nil = timer.GetCallbackType("NilCallback")
        "#;

        assert!(lua.load(success_case).exec().is_ok());

        // Test global now
        assert_eq!(
            globals
                .get::<String>("TestValue_Tick")
                .expect("Didn't find type"),
            "tick"
        );
        assert_eq!(
            globals
                .get::<String>("TestValue_Beat")
                .expect("Didn't find type"),
            "beat"
        );
        assert!(
            !globals
                .contains_key("TestValue_Nil")
                .expect("Error checking for key")
        );
    }

    #[test]
    fn test_get_callback_freq() {
        let lua = Lua::new();
        let globals = lua.globals();

        lua.load(timer::LUA_MODULE)
            .exec()
            .expect("Failed to load lua module");

        // Test success case
        let success_case = r#"
            local timer = _G.Timer

            local function userCallFunction()

            end

            timer.AddTickCallback("TickCall", userCallFunction)
            timer.AddBeatCallback("BeatCall", 1.0, userCallFunction)

            _G.TestValue_Tick = timer.GetCallbackFreq("TickCall")
            _G.TestValue_Beat = timer.GetCallbackFreq("BeatCall")
            _G.TestValue_Nil = timer.GetCallbackFreq("NilCallback")
        "#;

        assert!(lua.load(success_case).exec().is_ok());

        // Test global now
        assert!(
            !globals
                .contains_key("TestValue_Tick")
                .expect("Error checking for key")
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue_Beat")
                .expect("Didn't find freq"),
            1.0
        );
        assert!(
            !globals
                .contains_key("TestValue_Nil")
                .expect("Error checking for key")
        );
    }

    #[test]
    fn test_set_callback_freq() {
        let lua = Lua::new();
        let globals = lua.globals();

        lua.load(timer::LUA_MODULE)
            .exec()
            .expect("Failed to load lua module");

        // Initialize environment
        let init_code = r#"
            local timer = _G.Timer

            local function userCallFunction()

            end

            timer.AddTickCallback("TickCall", userCallFunction)
            timer.AddBeatCallback("BeatCall", 1.0, userCallFunction)
        "#;

        assert!(lua.load(init_code).exec().is_ok());

        // Error Cases
        let fail_case1 = r#"
            local timer = _G.Timer

            timer.SetCallbackFreq("NilCall", 1.0)
        "#;
        let fail_case2 = r#"
            local timer = _G.Timer

            timer.SetCallbackFreq("TickCall", 1.0)
        "#;
        let fail_case3 = r#"
            local timer = _G.Timer

            timer.SetCallbackFreq("BeatCall", 0.0)
        "#;

        assert!(lua.load(fail_case1).exec().is_err());
        assert!(lua.load(fail_case2).exec().is_err());
        assert!(lua.load(fail_case3).exec().is_err());

        // Success Case
        let success_case = r#"
            local timer = _G.Timer

            timer.SetCallbackFreq("BeatCall", 3.0)
        "#;

        // Test global now
        let timer: Table = globals.get("Timer").expect("Timer table not found");
        let callbacks: Table = timer.get("_Callbacks").expect("Callback table not found");
        let user_call: Table = callbacks
            .get("BeatCall")
            .expect("Didn't find beat callback");

        assert_eq!(user_call.get::<f64>("freq").expect("Didn't find freq"), 1.0);
        assert!(lua.load(success_case).exec().is_ok());
        assert_eq!(user_call.get::<f64>("freq").expect("Didn't find freq"), 3.0);
    }
}
