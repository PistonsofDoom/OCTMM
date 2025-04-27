-- Constants 

-- Timer Callback Types
local TIMER_CALLBACK_TICK = "tick"
local TIMER_CALLBACK_BEAT = "beat"

-- Metatables

-- Initialize Globals
_G.Timer = {};
_G.Timer._Callbacks = {};

-- Local utility functions


-- 	addCallback(
--		callback_name - [String] name of the callback, should be unique
--
--		callback_type - [String] type, should be one of 
--								the TIMER_CALLBACK_xxxx
--		callback_freq - If type is TIMER_CALLBACK_BEAT, should be number
--						otherwise, should be nil
--
--		callback_func - [Function (number)] - Callback function, number is
--						the delta time between calls
-- 	)
-- 
local function addCallback(callback_name: string, callback_type: string, callback_freq, callback_func: (number))
	if _G.Timer._Callbacks[callback_name] ~= nil then
		error("Callback already exists!")
	end

	_G.Timer._Callbacks[callback_name] = {}
	_G.Timer._Callbacks[callback_name]["type"] = callback_type
	_G.Timer._Callbacks[callback_name]["freq"] = callback_freq
	_G.Timer._Callbacks[callback_name]["func"] = callback_func
end


-- Global Functions

--	AddTickCallback(
--		callback_name - [String] Name of the callback function,
--						needs to be unique
--
--		callback_func - [Function (number)] Function that gets called,
--						can be used within multiple callbacks
--	)
--
--	Add a callback function that gets called every 'tick'
_G.Timer.AddTickCallback = function(callback_name: string, callback_func: (number))
	addCallback(callback_name, TIMER_CALLBACK_TICK, nil, callback_func)
end

--	AddBeatCallback(
--		callback_name - [String] Name of the callback function,
--						needs to be unique
--
--		callback_freq - [Number] Number that represents the number of
--						'beats' that must pass before each
--						call. Has to be greater than 0, but can
--						have a decimal
--
--		callback_func - [Function (number)] Function to actually call back to,
--						Can be used within multiple callbacks
--	)
--
--	Add a callback function that gets called at a certain frequency 
--	per beat
_G.Timer.AddBeatCallback = function(callback_name: string, callback_freq: number, callback_func: (number))
	if callback_freq <= 0 then
		error("Tried to AddBeatCallback " + callback_name + " with a frequency below 0")
	end
	addCallback(callback_name, TIMER_CALLBACK_BEAT, callback_freq, callback_func)
end


