-- Local Utility Functions --

-- Shared Commands
local function sharedExists(name: String)
	return _dsp_command_handler("shared_exists;"..name) == "true"
end

local function sharedSetValue(name: String, value: Number)
	local ret = _dsp_command_handler("shared_set;"..name..";"..tostring(value))
	return tonumber(ret)
end

local function sharedGetValue(name: String)
	local ret = _dsp_command_handler("shared_get;"..name)

	if ret == "nil" then
		return nil
	else
		return tonumber(ret)
	end
end

local function sharedGetNet(name: String)
	local ret = _dsp_command_handler("shared_get_net;"..name)

	if ret == "nil" then
		return nil
	else
		return tonumber(ret)
	end
end

-- Network Management Commands
local function netExists(id: Number)
	return _dsp_command_handler("net_exists;"..tostring(id)) == "true"
end

local function netClone(id: Number)
	local ret = _dsp_command_handler("net_clone;"..tostring(id))

	if ret == "nil" then
		return nil
	else
		return tonumber(ret)
	end
end

local function netConstant(value: Number)
	return _dsp_command_handler("net_constant;"..tostring(value))
end

local function netVectorLength()
	return _dsp_command_handler("net_vector_length")
end

-- Network Proxy Commands
local function netDefault(node_type: String)
	local ret = _dsp_command_handler("net_default;"..node_type)

	if ret == "nil" then
		return nil
	else
		return tonumber(ret)
	end
end

local function netProduct(id1: Number, id2: Number)
	local ret = _dsp_command_handler("net_product;"..tostring(id1)..";"..tostring(id2))

	if ret == "nil" then
		return nil
	else
		return tonumber(ret)
	end
end

local function netBus(id1: Number, id2: Number)
	local ret = _dsp_command_handler("net_bus;"..tostring(id1)..";"..tostring(id2))

	if ret == "nil" then
		return nil
	else
		return tonumber(ret)
	end
end

local function netPipe(id1: Number, id2: Number)
	local ret = _dsp_command_handler("net_pipe;"..tostring(id1)..";"..tostring(id2))

	if ret == "nil" then
		return nil
	else
		return tonumber(ret)
	end
end

local function netCommit(id: Number)
	_dsp_command_handler("net_commit;"..id)
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
--
Net = {}
Net._mt = {}
-- Contains:
-- 		_net_id: Number

-- Metamethods
-- Bus two networks together
function Net._mt.__add(a, b)
	return Net.from(netBus(a._net_id, b._net_id))
end

function Net._mt.__mul(a, b)
	return Net.from(netProduct(a._net_id, b._net_id))
end

function Net._mt.__concat(a, b)
	return Net.from(netPipe(a._net_id, b._net_id))
end

-- User facing functions
function Net.from(net_id: Number)
	local new_net = {}

	setmetatable(new_net, Net._mt)
	new_net._net_id = net_id

	return new_net
end

--	Constant
--		Used to house a utility function for creating
--		constant networks
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
function Shared._mt.__add(a, b)
	return Net.from(netBus(a._net_id, b._net_id))
end

function Shared._mt.__mul(a, b)
	return Net.from(netProduct(a._net_id, b._net_id))
end

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

-- Setup Defaults --
_G.Hammond = Net.from(netDefault("hammond"))
_G.Organ = Net.from(netDefault("organ"))
_G.Saw = Net.from(netDefault("saw"))
_G.Sine = Net.from(netDefault("sine"))
_G.SoftSaw = Net.from(netDefault("softsaw"))
_G.Square = Net.from(netDefault("square"))
_G.Triangle = Net.from(netDefault("triangle"))
