-- Local Utility Functions --

local function play(net_id: Number, duration: Number)
	return _audio_command_handler("audio;play;"..tostring(net_id)..";"..tostring(duration))
end

local function stop(event_id: String)
	return _audio_command_handler("audio;stop;"..tostring(event_id)) == "true"
end

-- User facing functions

-- Takes in a "Net" metatable, and (optionally) a duration.
-- If no duration is provided, play forever
function Play(net: Table, duration)
	local net_id = net._net_id
	if duration == nil then
		-- Not 'forever', more than 10 days though.
		duration = 99999
	end

	return play(net_id, duration)
end

-- Takes in the "event id" string returned by Play()
function Stop(event_id: String)
	return stop(event_id)
end
