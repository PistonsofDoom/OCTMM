--[[ Generic Audio Utilities ]]--
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

--[[ Note Utilities ]]--

--[[
-- How should this be done?
-- Easily accessible by user (e.g. C4 should refer to the note C4. C4S should refer to C4 sharp)
--
--
--]]

-- Note Prefix, whether its sharp, base frequency
local note_base = {
	{"C", false, 16.35},
	{"C", true, 17.32},
	{"D", false, 18.35},
	{"D", true, 19.45},
	{"E", false, 20.60},
	{"F", false, 21.83},
	{"F", true, 23.12},
	{"G", false, 24.50},
	{"G", true, 25.96},
	{"A", false, 27.50},
	{"A", true, 29.14},
	{"B", false, 30.87}
}

-- Default global scales
_G.MAJOR = {
	0,
	2,
	4,
	5,
	7,
	9,
	11
}

_G.MINOR = {
	0,
	2,
	3,
	5,
	7,
	8,
	10
}

-- Initialize Notes
Note = {}
Note._mt = {}
Note._methods = {}
-- Contains:
-- 		_index: Number
-- 		_freq: Number

-- TODO: GetFreq(), Scale(scale array, index), Chord(chord type)

-- Return the "sharp" equivalent of the current note.
function Note._mt.__len(table)
	return Note.new(table._index + 1)
end

function Note._methods.GetFreq(self)
	return self._freq
end

function Note._methods.Offset(self, offset)
	return Note.new(self._index + offset)
end

function Note._methods.Scale(self, scale, index: Number)
	if typeof(scale) ~= "table" then
		error("Tried to use a non-table as a Scale")
	end
	
	local note_index = (index % (#scale)) + 1
	local octave_index = math.floor(index/ (#scale))

	return Note.new(self._index + scale[note_index] + (octave_index * 12))
end

function Note._methods.Chord(self, chord)
	if typeof(chord) ~= "table" then
		error("Tried to use a non-table as a Chord")
	end

	local new_chord = {}

	for _,v in ipairs(chord) do
		table.insert(new_chord, self:Offset(v))
	end

	return new_chord
end

function Note.new(index)
	-- Calculate frequency
	local start = 1 + ((index-1) % 12)
	local octave = math.floor(index / 12)

	-- Create table
	local new_note = {}

	setmetatable(new_note, Note._mt)

	-- Add Generic Methods
	new_note.Offset = Note._methods.Offset
	new_note.Scale = Note._methods.Scale
	new_note.Chord = Note._methods.Chord

	new_note.GetFreq = Note._methods.GetFreq

	new_note._index = index
	new_note._freq = note_base[start][3] * math.pow(2, octave)

	return new_note
end

-- 12 notes per octave, 10 octaves
-- TODO: Implement this
for i = 1,(12 * 10) do
	--Note.new(i)
end
