-- Constants 

-- Timer Callback Types
local TIMER_CALLBACK_TICK = "tick"
local TIMER_CALLBACK_BEAT = "beat"

-- Metatables

-- Initialize Globals
_G.Timer = {};
_G.Timer._Callbacks = {};
_G.Timer._BPM = 120;

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

-- SetBPM(
-- 		bpm - [Number] New BPM of the song, has to be > 0, can
-- 			  be a decimal.
-- )
-- 
-- Set the BPM of the song
_G.Timer.SetBPM = function(bpm: number)
	if bpm <= 0 then
		error("Tried to set the BPM <= 0")
	end

	_G.Timer._BPM = bpm
end

-- GetBPM(
-- ) -> [Number] Current BPM
--
-- Returns the current BPM of the song
_G.Timer.GetBPM = function() 
	return _G.Timer._BPM
end

--	AddTickCallback(
--		callback_name - [String] Name of the callback function,
--						needs to be unique
--
--		callback_func - [Function (number)] Function that gets called,
--						can be used within multiple callbacks
--	)
--
--	Add a callback function that gets called every 'tick'
_G.Timer.AddTickCallback = function(callback_name: string, callback_func)
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
_G.Timer.AddBeatCallback = function(callback_name: string, callback_freq: number, callback_func)
	if callback_freq <= 0 then
		error("Tried to AddBeatCallback "..callback_name.." with a frequency below 0")
	end
	addCallback(callback_name, TIMER_CALLBACK_BEAT, callback_freq, callback_func)
end

--  RemoveCallback(
--  	callback_name - [String] Name of the callback function
--  					to remove.
--  ) -> [Bool] - Whether or not a callback was removed
--
--  Remove a callback, and return whether one existed before being
--  removed.
_G.Timer.RemoveCallback = function(callback_name: string)
	local removed_callback = false

	if _G.Timer.IsCallback(callback_name) then
		removed_callback = true
	end

	_G.Timer._Callbacks[callback_name] = nil

	return removed_callback
end

--  IsCallback(
--  	callback_name - [String] Name of the callback functon to
--  					check for.
--  ) -> [Bool] - Whether or not the callback exists
--
--  Returns whether or not the specified callback exists
_G.Timer.IsCallback = function(callback_name: string)
	if not _G.Timer._Callbacks[callback_name] then
		return false
	end

	return true
end

--  GetCallbackType(
--  	callback_name - [String] Name of the callback function to
--  					check the type of
--  ) -> [String] Type of the callback, or nil if no callback was found
--
--  Get the type of a callback, or nil if no callback exists
_G.Timer.GetCallbackType = function(callback_name: string)
	if not _G.Timer._Callbacks[callback_name] then
		return nil
	end

	return _G.Timer._Callbacks[callback_name]["type"]
end

--  GetCallbackFreq(
--  	callback_name - [String] Name of the callback function to
--  					check the type of
--  ) -> [Number] Frequency of the callback, or nil if no callback was found,
--  	or the callback was not of type "beat"
--
--  Get the frequency of a callback, or nil if no callback exists, or
--  the callback type is not "beat"
_G.Timer.GetCallbackFreq = function(callback_name: string)
	if not _G.Timer._Callbacks[callback_name] then
		return nil
	end

	if _G.Timer._Callbacks[callback_name]["type"] ~= "beat" then
		return nil
	end

	return _G.Timer._Callbacks[callback_name]["freq"]
end

--  SetCallbackFreq(
--  	callback_name - [String] Name of the callback function to
--  					check the type of
--  	callback_freq - [Number] Target frequency of the callback
--  )
--
--  Set the frequency of a callback, throws an error if the callback
--  doesn't exist, or the callback type isn't "beat"
_G.Timer.SetCallbackFreq = function(callback_name: string, callback_freq: number)
	
	if not _G.Timer._Callbacks[callback_name] then
		error("Callback "..callback_name.." doesn't exist")
	end

	if _G.Timer._Callbacks[callback_name]["type"] ~= "beat" then
		error("Callback "..callback_name.." type isn't beat")
	end

	if callback_freq <= 0 then
		error("Tried to set frequency for callback "..callback_name.." below 0")
	end

	_G.Timer._Callbacks[callback_name]["freq"] = callback_freq
end
