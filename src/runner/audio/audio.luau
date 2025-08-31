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

-- base frequency for octave 0, starting at C
local note_base = {
	16.35,
	17.32,
	18.35,
	19.45,
	20.60,
	21.83,
	23.12,
	24.50,
	25.96,
	27.50,
	29.14,
	30.87
}
-- Note Prefix, Whether the note is sharp
local note_names = {
	{"C", false},
	{"C", true},
	{"D", false},
	{"D", true},
	{"E", false},
	{"F", false},
	{"F", true},
	{"G", false},
	{"G", true},
	{"A", false},
	{"A", true},
	{"B", false}
}

-- Default global scales
_G.MAJOR = {0,2,4,5,7,9,11}
_G.MINOR = {0,2,3,5,7,8,10}

-- Default global chords
_G.MAJOR_TRIAD = {0, 4, 7}
_G.MAJOR_SIXTH = {0, 4, 7, 9}
_G.MAJOR_SEVENTH = {0, 4, 7, 11}
_G.DOMINANT_SEVENTH = {0, 4, 7, 10}
_G.AUGMENTED_TRIAD = {0, 4, 8}
_G.AUGMENTED_SEVENTH = {0, 4, 8, 10}
_G.MINOR_TRIAD = {0, 3, 7}
_G.MINOR_SIXTH = {0, 3, 7, 9}
_G.MINOR_SEVENTH = {0, 3, 7, 10}
_G.MINOR_MAJOR_SEVENTH = {0, 3, 7, 11}
_G.DIMINISHED_TRIAD = {0, 3, 6}
_G.DIMINISHED_SEVENTH = {0, 3, 6, 9}
_G.HALF_DIMINISHED_SEVENTH = {0, 3, 6, 10}

-- Initialize Notes
Note = {}
Note._methods = {}
-- Contains:
-- 		_index: Number
-- 		_freq: Number

function Note._methods.GetFreq(self)
	return self._freq
end

-- Returns a new note, based off the offset from
-- the current note.
function Note._methods.Offset(self, offset: Number)
	return Note.new(self._index + offset)
end

-- Returns the next note in a scale, where a scale is
-- represented as a table of offsets, and index determines
-- which offset is used
function Note._methods.Scale(self, scale, index: Number)
	if typeof(scale) ~= "table" then
		error("Tried to use a non-table as a Note Scale")
	end
	
	local note_index = (index % (#scale)) + 1
	local octave_index = math.floor(index / (#scale))

	return Note.new(self._index + scale[note_index] + (octave_index * 12))
end

-- Returns a table of Notes, determined from the "chord" input,
-- which is a table of offsets.
function Note._methods.Chord(self, chord)
	if typeof(chord) ~= "table" then
		error("Tried to use a non-table as a Note Chord")
	end

	local new_chord = {}

	for _,v in ipairs(chord) do
		table.insert(new_chord, self:Offset(v))
	end

	return new_chord
end

function Note.new(index: Number)
	-- Calculate frequency
	local start = 1 + ((index-1) % 12)
	local octave = math.floor(index / 12)

	-- Create table
	local new_note = {}

	-- Add Generic Methods
	new_note.Offset = Note._methods.Offset
	new_note.Scale = Note._methods.Scale
	new_note.Chord = Note._methods.Chord

	new_note.GetFreq = Note._methods.GetFreq

	-- Setup Variables
	new_note._index = index
	new_note._freq = note_base[start] * math.pow(2, octave)

	return new_note
end

-- Initialize all default note globals.
-- e.g. C4, D2, F3s

-- 12 notes per octave, 10 octaves
for i = 1,(12 * 10) do
	local name = note_names[1 + ((i - 1) % 12)]
	local octave = math.floor(i / 12)
	
	if not name[2] then
		name = name[1]..tostring(octave)
	else
		name = name[1].."s"..tostring(octave)
	end

	_G[name] = Note.new(i)
end
