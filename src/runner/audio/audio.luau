-- Local Utility Functions --

local function play(net_id: Number, duration: Number)
	return _audio_command_handler("audio;play;"..tostring(net_id)..";"..tostring(duration))
end

-- Metatables --

_SequenceEvent = {}
_SequenceEvent._mt = {}

function _SequenceEvent.new(event_id: String)
	-- TODO: This should do sanity checking to confirm that it is a valid
	-- event_id
	
	return nil
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

	return _SequenceEvent.new(play(net_id, duration))
end
