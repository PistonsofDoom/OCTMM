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

	-- Force clamp duration
	if duration < 0.03 then
		print("Play duration too small, clamping to 0.03")
		duration = 0.03
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

--[[ Instrument Utilities ]]--

local uid = 0

Instrument = {}
Instrument._methods = {}
-- Contains:
-- 		_voices: {{
--				freq = Shared Net
--				amp = Shared Net
--				net = Voice net
--				end_time = Time in which this voice "ends". 
--					AKA, if end_time <= GetTime(), this voice
--					is open.
--				event_id = The event id returned by audio Play(),
--				           not guaranteed to exist
-- 			}, 
-- 			..
-- 			} 

function Instrument._methods._FindBestVoice(self)
	local time = GetTime()
	-- Assume best case first
	local best_voice = nil
	local lowest_time = nil

	for index,voice in ipairs(self._voices) do
		local end_time = voice.end_time
		-- If the voice isn't playing, just return it now.
		if end_time < time then
			return {["index"] = index, ["forced"] = false}
		end
		
		-- Otherwise, check if this voice has a lower time.
		if lowest_time == nil or lowest_time > end_time then
			best_voice = index
			lowest_time = end_time
		end
	end

	return {["index"] = best_voice, ["forced"] = true}
end

function Instrument._methods._PlayChord(self, notes, duration, amp)
	local voice_return = {}
	for _,v in ipairs(notes) do
		table.insert(voice_return, self:Play(v, duration, amp))
	end

	return voice_return
end

function Instrument._methods.SetFreq(self, index: Number, new_freq: Number)
	-- If new_freq isn't specified, presume index is actually the new frequency
	-- and the user wants to adjust voice 1. This allows instruments with
	-- only one voice to be controlled easily.
	if new_freq == nil then
		new_freq = index
		index = 1
	end
	self._voices[index].freq(new_freq)
end

function Instrument._methods.SetAmp(self, index: Number, new_amp: Number)
	-- Same as with SetFreq, if new_amp is nil, presume the user is only
	-- adjusting the first voice channel
	if new_amp == nil then
		new_amp = index
		index = 1
	end
	self._voices[index].amp(new_amp)
end

function Instrument._methods.Play(self, note, duration, amp)
	-- Process duration
	if duration == nil then
		duration = 99999
	end

	-- Get frequency
	local freq = 0

	if typeof(note) == "number" then
		freq = note
	elseif typeof(note) == "table" then
		-- If _freq exists, just use that.
		if note._freq then
			freq = note._freq
		-- Otherwise, force use the function _PlayChord(notes, amp)
		else
			return self:_PlayChord(note, duration, amp)
		end
	end

	-- Get voice
	local voice = self:_FindBestVoice()

	if voice.forced then
		self:Stop(voice.index)
	end

	self:SetFreq(voice.index, freq)
	-- If applicable, set amplification
	if amp then
		self:SetAmp(voice.index, amp)
	end

	-- Play the sound
	self._voices[voice.index].event_id = Play(self._voices[voice.index].net, duration)
	self._voices[voice.index].end_time = GetTime() + duration

	return voice.index
end

function Instrument._methods.Stop(self, index: Number)
	local event_id = self._voices[index].event_id
	
	if event_id then
		Stop(event_id)
	end

	print("Force stopping: "..tostring(index))
	self._voices[index].end_time = -1.0
end


-- voice_func - The function called whenever a voice is created.
--              It should always return a Net, and has three inputs.
--              Frequency, Amplification, and Voice Index
-- voices - The number of voices the instrument should have.
--          If a user tries to play a note and there are no more
--          available voices, it will prematurely stop and then play
--          the voice that is closest to ending
function Instrument.new(voice_func: Function, voices: Number)
	local new_instrument = {}
	new_instrument._voices = {}

	-- Sanity check
	if voices < 1 then
		error("Tried to create an instrument with "..tostring(voices)..". Which is less than 1")
	end

	-- Initialize all voices
	for i = 1,voices do
		local voice = {}
		
		voice.freq = Shared.new(0.0)
		voice.amp = Shared.new(0.5)
		voice.net = voice_func(voice.freq, voice.amp, i)
		voice.end_time = -1.0

		new_instrument._voices[i] = voice
	end
	
	-- Setup methods
	new_instrument._FindBestVoice = Instrument._methods._FindBestVoice
	new_instrument._PlayChord = Instrument._methods._PlayChord
	new_instrument.SetFreq = Instrument._methods.SetFreq
	new_instrument.SetAmp = Instrument._methods.SetAmp
	new_instrument.Play = Instrument._methods.Play
	new_instrument.Stop = Instrument._methods.Stop

	return new_instrument
end
