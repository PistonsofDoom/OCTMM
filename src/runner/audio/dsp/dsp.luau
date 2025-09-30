-- Local Utility Functions --

-- Shared Commands
local function sharedExists(name: String)
	return _audio_command_handler("dsp;shared_exists;"..name) == "true"
end

local function sharedSetValue(name: String, value: Number)
	local ret = _audio_command_handler("dsp;shared_set;"..name..";"..tostring(value))
	return tonumber(ret)
end

local function sharedGetValue(name: String)
	local ret = _audio_command_handler("dsp;shared_get;"..name)

	if ret == "nil" then
		return nil
	else
		return tonumber(ret)
	end
end

local function sharedGetNet(name: String)
	local ret = _audio_command_handler("dsp;shared_get_net;"..name)

	if ret == "nil" then
		return nil
	else
		return tonumber(ret)
	end
end

-- Network Management Commands
local function netExists(id: Number)
	return _audio_command_handler("dsp;net_exists;"..tostring(id)) == "true"
end

local function netClone(id: Number)
	local ret = _audio_command_handler("dsp;net_clone;"..tostring(id))

	if ret == "nil" then
		print("Net clone error")
		return nil
	else
		return tonumber(ret)
	end
end

local function netConstant(value: Number)
	return _audio_command_handler("dsp;net_constant;"..tostring(value))
end

local function netFromSample(value: String, loop_point, channel)
	-- Parse loop_point
	if loop_point == nil or typeof(loop_point) ~= "number" then
		loop_point = ";nil"
	else
		loop_point = ";"..tostring(loop_point)
	end

	-- Parse channel 
	if channel == nil or typeof(channel) ~= "number" then
		channel = ""
	else
		channel = ";"..tostring(channel)
	end

	-- Return of net_from_sample is always consistently either "nil", or
	-- "net_id;duration"
	local ret = _audio_command_handler("dsp;net_from_sample;"..value..loop_point..channel)

	if ret == "nil" then
		print("Net from sample error")
		return nil
	end
	
	-- Separate return into two parts, the network id, and the duration
	local ret_array = {}
	local keys = {"net_id", "duration"}
	local key_index = 1

	for num_str in string.gmatch(ret, "[^;]+") do
		ret_array[keys[key_index]] = tonumber(num_str)
		key_index += 1
	end

	return ret_array
end

local function netVectorLength()
	return _audio_command_handler("dsp;net_vector_length")
end

-- Network Proxy Commands
local function netDefault(node_type: String)
	local ret = _audio_command_handler("dsp;net_default;"..node_type)

	if ret == "nil" then
		print("Net default error")
		return nil
	else
		return tonumber(ret)
	end
end

-- Takes in effect type, and a "variadic", which should contain
-- all the arguments requried for that net type
local function netEffect(effect_type, ...)
	local args = { ... }

	-- Convert variadic args into a string, with ; as a separator between values
	local args_as_string = ""
	for _,v in pairs(args) do
		args_as_string = args_as_string..";"..tostring(v)
	end

	print(args_as_string)

	local ret = _audio_command_handler("dsp;net_effect;"..effect_type..args_as_string)

	if ret == "nil" then
		print("Net effect error for effect "..effect_type)
		return nil
	else
		return tonumber(ret)
	end
end

local function netProduct(id1: Number, id2: Number)
	local ret = _audio_command_handler("dsp;net_product;"..tostring(id1)..";"..tostring(id2))

	if ret == "nil" then
		print("Net product error")
		return nil
	else
		return tonumber(ret)
	end
end

local function netBus(id1: Number, id2: Number)
	local ret = _audio_command_handler("dsp;net_bus;"..tostring(id1)..";"..tostring(id2))

	if ret == "nil" then
		print("Net bus error")
		return nil
	else
		return tonumber(ret)
	end
end

local function netPipe(id1: Number, id2: Number)
	local ret = _audio_command_handler("dsp;net_pipe;"..tostring(id1)..";"..tostring(id2))

	if ret == "nil" then
		print("Net pipe error")
		return nil
	else
		return tonumber(ret)
	end
end

local function netStack(id1: Number, id2: Number)
	local ret = _audio_command_handler("dsp;net_stack;"..tostring(id1)..";"..tostring(id2))

	if ret == "nil" then
		print("Net stack error")
		return nil
	else
		return tonumber(ret)
	end
end

-- If the input is a table, gets _net_id or returns an error
-- If the input is a number, create a new constant
local function getNetId(input)
	if typeof(input) == "number" then
		return Constant.new(input)._net_id
	elseif typeof(input) == "table" then
		return input._net_id
	else
		error("Tried to use type '"..typeof(input).."' as a network")
	end
end

local uid_number = 0
local function getUID()
	uid_number += 1
	return tostring(uid_number)
end

-- Metatables --

--	Net
--		Serves as a sort of "proxy" for the rust
--		DSP module's 'net' utility functions.
--
--		Arithmetic:
--			net1 + net2  - Bus
--			net1 * net2  - Product
--			net1 .. net2 - Pipe
--			net1 // net2 - Stack
--
Net = {}
Net._mt = {}
-- Contains:
-- 		_net_id: Number

-- Metamethods
function Net._mt.__add(a, b)
	return Net.from(netBus(getNetId(a), getNetId(b)))
end

function Net._mt.__mul(a, b)
	return Net.from(netProduct(getNetId(a), getNetId(b)))
end

function Net._mt.__concat(a, b)
	return Net.from(netPipe(getNetId(a), getNetId(b)))
end

function Net._mt.__idiv(a, b)
	return Net.from(netStack(getNetId(a), getNetId(b)))
end

-- User facing functions
function Net.from(net_id: Number)
	if net_id == nil then
		error("Tried creating network with no network id")
	end

	local new_net = {}

	setmetatable(new_net, Net._mt)
	new_net._net_id = net_id

	return new_net
end

function Net.clone(network)
	if not network._net_id then
		return nil
	end

	return Net.from(netClone(network._net_id))
end

--	Constant
--		Used to house a utility function for creating
--		constant networks
--		
--		Example:
--			local my_constant = Constant.new(440)
--
--			local my_synth = my_constant..Sine
--
Constant = {}
function Constant.new(value: Number)
	return Net.from(netConstant(value))
end

--	Shared
--		Kind of like an extension of a Net, but with utility functions
--		for setting & getting Shared values
--		
--		Arithmetic:
--			shared + shared
--			shared + constant
--			net + shared
--			
--			shared * shared
--			shared * constant
--			net * shared
--		NOT	shared * net
--
--		Example:
--			local my_shared = Shared.new("my_shared", 20)
--
--			my_shared(#my_shared + 20)
--
--			print(#my_shared) -- Prints 40
--
Shared = {}
Shared._mt = {}
-- Contains:
-- 		_net_id: Number
-- 		_name: String
-- 		_value: Number

-- Metamethods
Shared._mt.__add = Net._mt.__add
Shared._mt.__mul = Net._mt.__mul
Shared._mt.__idiv = Net._mt.__idiv

-- Set a value on a shared variable by calling
-- my_shared_variable(200)
function Shared._mt.__call(table, value:Number)
	table._value = value
	sharedSetValue(table._name, table._value)
end

-- Get current value of a shared variable
-- local value = #my_shared_variable
function Shared._mt.__len(table)
	return table._value
end

-- User facing functions
function Shared.new(value:Number)
	local new_shared = {}

	setmetatable(new_shared, Shared._mt)
	new_shared._name = getUID()
	new_shared._value = value
	new_shared._net_id = sharedSetValue(new_shared._name, value)

	return new_shared
end

--	Samples
--		Extension of Net, containing a couple extra methods specifically
--		related to samples
--
Sample = {}
Sample._mt = {}
-- Contains:
-- 		_net_id: Number
-- 		_name: Sample Name
-- 		_duration: Number, sample length in time
Sample._mt.__add = Net._mt.__add
Sample._mt.__mul = Net._mt.__mul
Sample._mt.__idiv = Net._mt.__idiv
Sample._mt.__concat = Net._mt.__concat

-- Return duration of sample
function Sample._mt.__len(table)
	return table._duration
end

-- name: Name of the sample, as represented in the /samples/ directory
-- loop_point: Point at which the sample should loop back to after reaching
-- 	the end. If left nil, no loop point is given. Should be in seconds.
-- channel: If specified, will only use that specific channel of the 
-- 	sample. Starts at 1.
--
function Sample.new(name: String, loop_point, channel)
	local new_sample = {}

	local sample_data = netFromSample(name, loop_point, channel - 1)
	if sample_data == nil then
		error("Cannot find or create sample.. "..name)
	end

	setmetatable(new_sample, Sample._mt)
	new_sample._net_id = sample_data.net_id
	new_sample._name = name
	new_sample._duration = sample_data.duration

	return new_sample
end

-- Setup Effect Functions --

-- See fundsp documentation for in-depth description
-- room_size in meters (avg is 10)
-- time (approx reverb time to -60 db)
function Reverb(room_size, time, damping)
	return Net.from(netEffect("reverb", room_size, time * GetBPMModifier() , damping))
end

-- Fixed delay of $time
function Delay(time)
	return Net.from(netEffect("delay", time * GetBPMModifier()))
end

-- feedback_amount (for example, 0.9 or -0.9) where negative inverts feedback phase
-- delay in beats 
-- hz_f is the frequency of the delay function
-- delay_f in beats, for the delay function
function Flanger(feedback_amount, min_delay, max_delay, hz_f, min_delay_f, max_delay_f)
	if min_delay_f == nil then
		min_delay_f = min_delay
	end
	if max_delay_f == nil then
		max_delay_f = max_delay
	end

	min_delay *= GetBPMModifier()
	max_delay *= GetBPMModifier()
	min_delay_f *= GetBPMModifier()
	max_delay_f *= GetBPMModifier()

	return Net.from(netEffect("flanger", feedback_amount, min_delay, max_delay, min_delay_f, max_delay_f, hz_f))
end


-- high_freq_damping is ranged from 0...1
function Pluck(frequency, gain_per_beat, high_freq_damping)
	return Net.from(netEffect("pluck", frequency, gain_per_beat * GetBPMModifier(), high_freq_damping))
end

function Chorus(separation, variation, modulation_freq)
	return Net.from(netEffect("chorus", separation, variation, modulation_freq))
end

function ADSR(attack, decay, sustain, release)
	if not attack then
		attack = 0
	end
	if not decay then
		decay = 0
	end
	if not sustain then
		sustain = 0
	end
	if not release then
		release = 0
	end

	attack *= GetBPMModifier()
	decay *= GetBPMModifier()
	sustain *= GetBPMModifier()
	release *= GetBPMModifier()

	return Net.from(netEffect("adsr_live", attack, decay, sustain, release))
end

function Clip(min, max)
	if not min then
		min = -1
	end
	if not max then
		max = -1
	end
	return Net.from(netEffect("clip_to", min, max))
end

-- Setup Defaults --
_G.Hammond = Net.from(netDefault("hammond"))
_G.Organ = Net.from(netDefault("organ"))
_G.Saw = Net.from(netDefault("saw"))
_G.Sine = Net.from(netDefault("sine"))
_G.SoftSaw = Net.from(netDefault("softsaw"))
_G.Square = Net.from(netDefault("square"))
_G.Triangle = Net.from(netDefault("triangle"))
_G.Lorenz = Net.from(netDefault("lorenz"))
_G.Rossler = Net.from(netDefault("rossler"))
_G.Ramp = Net.from(netDefault("ramp"))
_G.Pulse = Net.from(netDefault("pulse"))
_G.MLS = Net.from(netDefault("mls"))
_G.White = Net.from(netDefault("white"))
_G.Pink = Net.from(netDefault("pink"))
_G.Brown = Net.from(netDefault("brown"))
_G.Resonator = Net.from(netDefault("resonator"))
_G.Peak = Net.from(netDefault("peak"))
_G.Pinkpass = Net.from(netDefault("pinkpass"))
_G.Panner = Net.from(netDefault("panner"))
_G.Notch = Net.from(netDefault("notch"))
_G.Moog = Net.from(netDefault("moog"))
_G.Morph = Net.from(netDefault("morph"))
_G.Lowpole = Net.from(netDefault("lowpole"))
_G.Lowpass = Net.from(netDefault("lowpass"))
_G.Lowshelf = Net.from(netDefault("lowshelf"))
_G.Highpole = Net.from(netDefault("highpole"))
_G.Highpass = Net.from(netDefault("highpass"))
_G.Highshelf = Net.from(netDefault("highshelf"))
_G.Butterpass = Net.from(netDefault("butterpass"))
_G.Bell = Net.from(netDefault("bell"))
_G.Bandpass = Net.from(netDefault("bandpass"))
_G.Bandrez = Net.from(netDefault("bandrez"))
_G.Allpass = Net.from(netDefault("allpass"))
