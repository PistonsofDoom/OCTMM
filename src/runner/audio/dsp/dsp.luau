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
		return nil
	else
		return tonumber(ret)
	end
end

local function netConstant(value: Number)
	return _audio_command_handler("dsp;net_constant;"..tostring(value))
end

local function netFromSample(value: String, loop_point)
	-- Parse loop_point
	if loop_point == nil or typeof(loop_point) ~= "number" then
		loop_point = ""
	else
		loop_point = ";"..tostring(loop_point)
	end

	-- Return of net_from_sample is always consistently either "nil", or
	-- "net_id;duration"
	local ret = _audio_command_handler("dsp;net_from_sample;"..value..loop_point)

	if ret == "nil" then
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
		return nil
	else
		return tonumber(ret)
	end
end

local function netProduct(id1: Number, id2: Number)
	local ret = _audio_command_handler("dsp;net_product;"..tostring(id1)..";"..tostring(id2))

	if ret == "nil" then
		return nil
	else
		return tonumber(ret)
	end
end

local function netBus(id1: Number, id2: Number)
	local ret = _audio_command_handler("dsp;net_bus;"..tostring(id1)..";"..tostring(id2))

	if ret == "nil" then
		return nil
	else
		return tonumber(ret)
	end
end

local function netPipe(id1: Number, id2: Number)
	local ret = _audio_command_handler("dsp;net_pipe;"..tostring(id1)..";"..tostring(id2))

	if ret == "nil" then
		return nil
	else
		return tonumber(ret)
	end
end

local function netStack(id1: Number, id2: Number)
	local ret = _audio_command_handler("dsp;net_stack;"..tostring(id1)..";"..tostring(id2))

	if ret == "nil" then
		return nil
	else
		return tonumber(ret)
	end
end

local function netCommit(id: Number)
	_audio_command_handler("dsp;net_commit;"..id)
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
	return Net.from(netBus(a._net_id, b._net_id))
end

function Net._mt.__mul(a, b)
	return Net.from(netProduct(a._net_id, b._net_id))
end

function Net._mt.__concat(a, b)
	return Net.from(netPipe(a._net_id, b._net_id))
end

function Net._mt.__idiv(a, b)
	return Net.from(netStack(a._net_id, b._net_id))
end

-- User facing functions
function Net.from(net_id: Number)
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
function Shared.new(name:String , value:Number)
	local new_shared = {}

	setmetatable(new_shared, Shared._mt)
	new_shared._name = name
	new_shared._value = value
	new_shared._net_id = sharedSetValue(name, value)

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
Sample._mt.__concat = Net._mt.__concat

-- Return duration of sample
function Sample._mt.__len(table)
	return table._duration
end

-- name: Name of the sample, as represented in the /samples/ directory
-- loop_point: Point at which the sample should loop back to after reaching
-- 	the end. If left nil, no loop point is given. Should be in seconds.
--
function Sample.new(name: String, loop_point)
	local new_sample = {}

	local sample_data = netFromSample(name, loop_point)
	if sample_data == nil then
		error("Cannot find or create sample.. "..name)
	end

	setmetatable(new_sample, Sample._mt)
	new_sample._net_id = sample_data.net_id
	new_sample._name = name
	new_sample._duration = sample_data.duration

	return new_sample
end

-- Setup Defaults --
_G.Hammond = Net.from(netDefault("hammond"))
_G.Organ = Net.from(netDefault("organ"))
_G.Saw = Net.from(netDefault("saw"))
_G.Sine = Net.from(netDefault("sine"))
_G.SoftSaw = Net.from(netDefault("softsaw"))
_G.Square = Net.from(netDefault("square"))
_G.Triangle = Net.from(netDefault("triangle"))
