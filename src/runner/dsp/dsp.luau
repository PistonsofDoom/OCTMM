-- Constants

-- Metatables

local Shared = {

}

-- Initialize Globals
_G.DSP = {}

-- Local Utility Functions

-- Shared Commands
local function sharedExists(name: String)
	return _dsp_command_handler("shared_exists;"..name) == "true"
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
-- Global Functions
