pub const LUA_MODULE: &str = include_str!("timer.luau");

pub struct TimerModule {}

impl TimerModule {}

#[cfg(test)]
mod tests {
    use crate::runner::timer;
    use mlua::*;

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
        let user_call: Table = callbacks.get("UserCall").expect("Didn't find user callback");

        assert_eq!(user_call.get::<String>("type").expect("Didn't find type"), "tick");
        assert!(user_call.get::<Function>("func").is_ok());
    }
}
