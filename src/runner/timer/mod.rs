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

    fn type_from_string(from_str: String) -> Option<CallbackType> {
        match from_str.as_str() {
            "tick" => Some(CallbackType::Tick),
            "beat" => Some(CallbackType::Beat),
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

        timer
            .set("_Time", time.clone())
            .expect("Unable to set Time");

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
                .get("function")
                .expect(format!("Invalid callback function on callback {}:", name).as_str());

            match call_type {
                CallbackType::Beat => {
                    let call_freq: f64 = value.get("frequency").expect(
                        format!("Invalid callback frequency on callback {}:", name).as_str(),
                    );
                    let call_time: f64 = value.get("time").unwrap_or(0.0);

                    if time - call_time >= 0.0 {
                        let time = time.clone();

                        value.set("time", time + (60.0 / bpm) * call_freq).expect(
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
                }
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

        timer.init(&lua);

        // Success
        let test_program = r#"
            _G.TestValue_Tick = 0
            _G.TestValue_Beat = 0

            SetBPM(60)

            local tick_timer = Timer.new(TICK)
            local beat_timer = Timer.new(BEAT, nil, 1.0)

            local function tick_callback()
                _G.TestValue_Tick += 1
                if _G.TestValue_Tick == 4 then
                    tick_timer:SetEnabled(false)
                end

                _G.TickEnabled = tick_timer:GetEnabled()
            end

            local function beat_callback()
                _G.TestValue_Beat += 1
                if _G.TestValue_Beat == 2 then
                    beat_timer:Disable()
                end

                _G.BeatEnabled = beat_timer:GetEnabled()
            end

            tick_timer:SetCallback(tick_callback)
            beat_timer:SetCallback(beat_callback)

            tick_timer:SetEnabled(true)
            beat_timer:Enable()

        "#;

        lua.load(test_program)
            .exec()
            .expect("Failed to run program:");

        timer.update(&0.0, &lua);
        assert_eq!(
            globals
                .get::<f64>("TestValue_Tick")
                .expect("Didn't find value"),
            1.0
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue_Beat")
                .expect("Didn't find value"),
            1.0
        );

        timer.update(&0.0, &lua);
        assert_eq!(
            globals
                .get::<f64>("TestValue_Tick")
                .expect("Didn't find value"),
            2.0
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue_Beat")
                .expect("Didn't find value"),
            1.0
        );
        assert_eq!(
            globals
                .get::<bool>("TickEnabled")
                .expect("Didn't find value"),
            true
        );
        assert_eq!(
            globals
                .get::<bool>("BeatEnabled")
                .expect("Didn't find value"),
            true
        );

        timer.update(&1.0, &lua);
        assert_eq!(
            globals
                .get::<f64>("TestValue_Tick")
                .expect("Didn't find value"),
            3.0
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue_Beat")
                .expect("Didn't find value"),
            2.0
        );
        assert_eq!(
            globals
                .get::<bool>("TickEnabled")
                .expect("Didn't find value"),
            true
        );
        assert_eq!(
            globals
                .get::<bool>("BeatEnabled")
                .expect("Didn't find value"),
            false
        );

        timer.update(&3.0, &lua);
        assert_eq!(
            globals
                .get::<f64>("TestValue_Tick")
                .expect("Didn't find value"),
            4.0
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue_Beat")
                .expect("Didn't find value"),
            2.0
        );
        assert_eq!(
            globals
                .get::<bool>("TickEnabled")
                .expect("Didn't find value"),
            false
        );
        assert_eq!(
            globals
                .get::<bool>("BeatEnabled")
                .expect("Didn't find value"),
            false
        );

        timer.update(&5.0, &lua);
        assert_eq!(
            globals
                .get::<f64>("TestValue_Tick")
                .expect("Didn't find value"),
            4.0
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue_Beat")
                .expect("Didn't find value"),
            2.0
        );

        // Failures
        let test_program = r#"
            local timer = Timer.new(TICK)

            timer:Enable()
        "#;
        assert!(lua.load(test_program).exec().is_err());
        let test_program = r#"
            local function my_func()

            end
            local timer = Timer.new(BEAT, my_func)

            timer:Enable()
        "#;
        assert!(lua.load(test_program).exec().is_err());

        timer.end(&lua);
    }

    // LUA CODE TESTS
    #[test]
    fn test_bpm_utilities() {
        let lua = Lua::new();
        let globals = lua.globals();
        let timer: &mut dyn PollingModule = &mut TimerModule::new();

        timer.init(&lua);

        // Test failure case
        let fail_case1 = r#"
            SetBPM(0)
        "#;
        let fail_case2 = r#"
            SetBPM("1")
        "#;

        assert!(lua.load(fail_case1).exec().is_err());
        assert!(lua.load(fail_case2).exec().is_err());

        // Test success case
        let success_case = r#"
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

        timer.end(&lua);
    }

    #[test]
    fn test_time_utilities() {
        let lua = Lua::new();
        let globals = lua.globals();
        let timer: &mut dyn PollingModule = &mut TimerModule::new();

        timer.init(&lua);

        // Test success case
        let test_program = r#"
            local function test_func()
                _G.TestValue = GetTime()
            end

            local timer = Timer.new(TICK, test_func)
            timer:Enable()
        "#;

        lua.load(test_program).exec().expect("Failed to load test program: ");

        // Test global now
        timer.update(&0.1, &lua);
        assert_eq!(
            globals
                .get::<f64>("TestValue")
                .expect("Didn't find time value"),
            0.1
        );

        timer.update(&5.4, &lua);
        assert_eq!(
            globals
                .get::<f64>("TestValue")
                .expect("Didn't find time value"),
            5.4
        );

        timer.end(&lua);
    }

    #[test]
    fn test_tick_callbacks() {
        let lua = Lua::new();
        let globals = lua.globals();
        let timer: &mut dyn PollingModule = &mut TimerModule::new();

        timer.init(&lua);

        // Init environment
        let test_program = r#"
            _G.TestValue = 0

            local function my_callback()
                _G.TestValue += 1
            end

            local my_timer = Timer.new(TICK, my_callback)

            my_timer:Enable()
        "#;

        lua.load(test_program).exec().expect("Error occured");

        // Update timer twice, this should call
        // the Tick Callback twice, and
        // the Beat Callback once
        timer.update(&0.5, &lua);
        timer.update(&0.5, &lua);
        timer.update(&1.0, &lua);

        assert_eq!(
            globals
                .get::<f64>("TestValue")
                .expect("Didn't find TIMER TestValue value"),
            3.0
        );

        timer.end(&lua);
    }

    #[test]
    fn test_beat_callbacks() {
        let lua = Lua::new();
        let globals = lua.globals();
        let timer: &mut dyn PollingModule = &mut TimerModule::new();

        timer.init(&lua);

        // Success
        let test_program = r#"
            SetBPM(60.0)

            _G.TestValue1 = 0
            _G.TestValue2 = 0
            _G.TestValue3 = 0

            -- No delay, no offset
            local timer1 = Timer.new(BEAT, nil, 1.0, 0.0)
            -- Delay, no offset
            local timer2 = Timer.new(BEAT, nil, 1.0, 0.0)
            -- Delay, offset
            local timer3 = Timer.new(BEAT, nil, 1.0, 1.0)

            local function call1()
                _G.TestValue1 += 1
            end
            local function call2()
                _G.TestValue2 += 1
            end
            local function call3()
                _G.TestValue3 += 1
            end

            timer1:SetCallback(call1)
            timer2:SetCallback(call2)
            timer3:SetCallback(call3)

            timer1:Enable()
            timer2:Enable(true)
            timer3:Enable(true)
        "#;

        lua.load(test_program).exec().expect("Error occured");

        // Update timer twice, this should call
        // the Tick Callback twice, and
        // the Beat Callback once
        timer.update(&0.0, &lua);
        assert_eq!(
            globals
                .get::<f64>("TestValue1")
                .expect("Didn't find TIMER TestValue value"),
            1.0
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue2")
                .expect("Didn't find TIMER TestValue value"),
            0.0
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue3")
                .expect("Didn't find TIMER TestValue value"),
            0.0
        );
        timer.update(&0.5, &lua);
        assert_eq!(
            globals
                .get::<f64>("TestValue1")
                .expect("Didn't find TIMER TestValue value"),
            1.0
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue2")
                .expect("Didn't find TIMER TestValue value"),
            0.0
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue3")
                .expect("Didn't find TIMER TestValue value"),
            0.0
        );

        timer.update(&1.0, &lua);
        assert_eq!(
            globals
                .get::<f64>("TestValue1")
                .expect("Didn't find TIMER TestValue value"),
            2.0
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue2")
                .expect("Didn't find TIMER TestValue value"),
            1.0
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue3")
                .expect("Didn't find TIMER TestValue value"),
            0.0
        );

        timer.update(&1.9, &lua);
        assert_eq!(
            globals
                .get::<f64>("TestValue1")
                .expect("Didn't find TIMER TestValue value"),
            2.0
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue2")
                .expect("Didn't find TIMER TestValue value"),
            1.0
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue3")
                .expect("Didn't find TIMER TestValue value"),
            0.0
        );

        timer.update(&2.0, &lua);
        assert_eq!(
            globals
                .get::<f64>("TestValue1")
                .expect("Didn't find TIMER TestValue value"),
            3.0
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue2")
                .expect("Didn't find TIMER TestValue value"),
            2.0
        );
        assert_eq!(
            globals
                .get::<f64>("TestValue3")
                .expect("Didn't find TIMER TestValue value"),
            1.0
        );

        // Failures

        // SetOffset failure
        let test_program = r#"
            local timer = Timer.new(TICK)
            timer:SetOffset(true)
        "#;
        assert!(lua.load(test_program).exec().is_err());

        // SetFreq failures
        let test_program = r#"
            local timer = Timer.new(TICK)
            timer:SetFreq(true)
        "#;
        assert!(lua.load(test_program).exec().is_err());
        let test_program = r#"
            local timer = Timer.new(TICK)
            timer:SetFreq(0)
        "#;
        assert!(lua.load(test_program).exec().is_err());
        let test_program = r#"
            local timer = Timer.new(TICK)
            timer:SetFreq(-1)
        "#;
        assert!(lua.load(test_program).exec().is_err());

        timer.end(&lua);
    }
}
