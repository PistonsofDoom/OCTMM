use crate::runner::Module;
use mlua::*;

const LUA_MODULE: &str = include_str!("timer.luau");

pub struct TimerModule {}

impl TimerModule {
    pub fn new() -> TimerModule {
        TimerModule {}
    }
}

impl Module for TimerModule {
    fn init(&self, lua: &Lua) {
        println!("Initializing TimerModule");
    }
    fn update(&self, time: &f64, lua: &Lua) {
        let timer: Table = lua
            .globals()
            .get("Timer")
            .expect("Didn't find 'Timer' table");

        let callbacks: Table = timer.get("_Callbacks").expect("Didn't find `Timer._Callbacks`");
        let bpm: f64 = timer.get("BPM").expect("Invalid BPM");
        
        // optimization: use Table::for_each
        for pair in callbacks.pairs::<String, Table>() {
            let (key, value) = pair.expect("Invalid callback");

            let call_type: String = value.get("type").expect("Invalid callback type");
            let call_func: Function = value.get("func").expect("Invalid callback function");

            if call_type == "beat" {
                let call_freq: f64 = value.get("freq").expect("Invalid callback frequency");
                let call_time: f64 = value.get("time").unwrap_or(0.0);

                // If function should be called
                if time - call_time >= (60.0 / bpm) / call_freq {
                    let time = time.clone();

                    value.set("time", time);
                    call_func.call::<()>(time).expect("Error occured while running beat update");
                }
            }
            else {
                let time = time.clone();
                call_func.call::<()>(time).expect("Error occured while running tick update");
            }
        }
    }
    fn end(&self, lua: &Lua) {
        println!("Ending TimerModule")
    }

    fn get_program(&self) -> &str {
        LUA_MODULE
    }

    fn get_name(&self) -> &str {
        "timer module"
    }
}

#[cfg(test)]
mod tests {
    use crate::runner::timer;
    use mlua::*;

    #[test]
    fn test_rust_module() {
        panic!("Unimplemented");
    }

    // LUA CODE TESTS
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
}
