-- Initialize Globals
_G.Timer = {}
_G.Timer._Callbacks = {}
_G.Timer._Time = 0.0
_G.Timer._BPM = 120
-- Number, acts as a unique id every time its used
_G.Timer._UID = 0

-- Local utility functions

-- Increment the global _UID variable, then
-- return the number as a string
local function getUID()
	Timer._UID += 1
	return tostring(Timer._UID)
end

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
local function addCallback(callback_name: string, callback_type: string, callback_freq, callback_func)
	if _G.Timer._Callbacks[callback_name] ~= nil then
		error("Callback already exists!")
	end
	if typeof(callback_func) ~= "function" then
		error("Tried to create callback to non-function")
	end

	_G.Timer._Callbacks[callback_name] = {}
	_G.Timer._Callbacks[callback_name]["type"] = callback_type
	_G.Timer._Callbacks[callback_name]["freq"] = callback_freq
	_G.Timer._Callbacks[callback_name]["func"] = callback_func
end

-- Global Functions
_G.GetTime = function()
	return Timer._Time
end

-- SetBPM(
-- 		bpm - [Number] New BPM of the song, has to be > 0, can
-- 			  be a decimal.
-- )
-- 
-- Set the BPM of the song
_G.SetBPM = function(bpm: number)
	if bpm <= 0 then
		error("Tried to set the BPM <= 0")
	end

	_G.Timer._BPM = bpm
end

-- GetBPM(
-- ) -> [Number] Current BPM
--
-- Returns the current BPM of the song
_G.GetBPM = function() 
	return _G.Timer._BPM
end

-- Metatables
-- _type = type of callback
-- _name = callback name
-- _function = function to callback to
--
-- Beat Only:
-- _frequency = Amount of beats to wait before being called, decimal value
-- _offset = Offset from which the callback should be called.
--
Timer._methods = {}

-- Generic Methods
-- Enables the callback
-- If 'delay' is true, the frequency of the callback is added as a start
-- delay
--
-- Returns whether or not we succeeded in enabling the timer.
Timer._methods.Enable = function(self, delay)
	if self._type == nil or self._name == nil then
		error("Timer type or name was not found. Did you call the method with '.' instead of ':'?")
	end

	-- If the timer exists, return as we don't need to add any callbacks
	if Timer._Callbacks[self._name] ~= nil then
		return false
	end

	-- BEAT Specific Sanity Checks
	if self._type == BEAT then
		if self._frequency == nil then
			error("Timer frequency was nil. Please set timer frequency")
		elseif self._offset == nil then
			error("Timer offset was nil.")
		end
	end

	-- At this point, we're good to enable the callback
	Timer._Callbacks[self._name] = {}
	Timer._Callbacks[self._name]["type"] = self._type
	Timer._Callbacks[self._name]["function"] = self._function

	if self._type == BEAT then
		local bpm_modifier = 60.0 / GetBPM()
		local enable_time = GetTime() + (bpm_modifier * self._offset)

		if delay then
			enable_time += bpm_modifier * self._frequency
		end

		Timer._Callbacks[self._name]["frequency"] = self._frequency
		Timer._Callbacks[self._name]["time"] = enable_time
	end
end

-- Disables the callback
Timer._methods.Disable = function(self)
	Timer._Callbacks[self._name] = nil
end

-- Wrapper
Timer._methods.SetEnable = function(self, state, delay)
	if state then
		self.Enable(delay)
	else
		self.Disable()
	end
end

Timer._methods.SetFreq = function(self, new_freq)
	
end

Timer._methods.SetOffset = function(self, new_offset)
	
end

-- 
-- Arguments:
-- 	timer_type: Takes in the BEAT or TICK constant global variable
-- 		TICK -> Function will create a "tick" callback
-- 		BEAT -> Function will create a "beat" callback
--
-- 	func: Callback Function
--
-- 	freq: Only used in 'BEAT', refers to the about of beats per callback
--
-- 	offset: Offset in BEATS from which the BEAT should be called.
--
function Timer.new(timer_type: String, func, freq, offset)
	-- Sanity check
	if timer_type ~= BEAT and timer_type ~= TICK or func == nil then
		error("Invalid timer type")
	end

	if func == nil or typeof(func) ~= "function" then
		error("Provided function was invalid")
	end

	-- Create table
	local new_callback = {}

	-- Constant Variables
	new_callback._name = getUID()
	new_callback._type = timer_type
	new_callback._function = func

	-- Add Generic Methods
	new_callback.Enable = Timer._methods.Enable

	-- Add extra functionality for only BEAT types
	if timer_type == BEAT then
		-- These are optional during creation.
		-- If frequency isn't provided, and the timer is attempted to be Enabled, it will throw an error
		if typeof(freq) == "number" then
			new_callback._frequency = freq
		end
		-- If no offset is provided, set it to 0
		if typeof(offset) == "number" then
			new_callback._offset = offset
		else
			new_callback._offset = 0
		end

		new_callback.SetFreq = Timer._methods.SetFreq
		new_callback.SetOffset = Timer._methods.SetOffset
	end

	return new_callback
end
