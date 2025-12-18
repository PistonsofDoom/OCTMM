# Using Constants and Shared variables

### Using Constants

**Constants** are used to implement an unchanging number within a network.
They can be created manually by using the function ``Constant.new(val)``, where `val` can be any value of type `Number`, and the return is a Network containing only the value `val`. 
There also exists automatic `Number` -> `Network` conversion, showcased in the example below.

```lua
local frequency = Constant.new(440.0)
local amplitude = Constant.new(0.5)

local sine_network = (frequency .. Sine) * amplitude

-- Using automatic conversion
local sine_network_auto = (440.0 .. Sine) * 0.5
```

When using automatic conversions of `Number` -> `Constants`, it is important to note that the conversion *only* occurs when interacting with a network.
If you are planning to [stack](./using-networks.md#stacking) two numbers together, it is required to have at least *one* of them be a network.

```lua
-- Valid stacking of Constants
local valid_stack = Constant.new(440.0) // Constant.new(2.2)

-- Invalid stacking
-- Floor divides instead
local invalid_stack = 440.0 // 2.2
```

### Using Shared

**Shared** networks are used to implement a changing number within a network. They are created by using the function ``Shared.new(val)``, where `val` is the default value, and the return is a network containing only the shared.

Unlike a constant network, shared networks can be `set` and `read` from.

```lua
local my_shared_freq = Shared.new(440.0)
local my_shared_amp = Shared.new(0.5)

local sine_network = (my_shared_freq .. Sine) * my_shared_amp

-- Print current values
print(#my_shared_freq)
print(#my_shared_amp)

-- sine_network would be equal to
-- (440.0 .. Sine) * 0.5

-- Change values
my_shared_freq(220.0)
my_shared_amp(#my_shared_amp * 2.0)

-- Print new values
print(#my_shared_freq)
print(#my_shared_amp)

-- sine_network would be equal to
-- (220.0 .. Sine) * 1.0
```

It is important to note that shared networks should be created separately from where they are used, as the ability to `set` and `read` the shared is lost when used with a network.

```lua
-- INVALID SHARED USAGE
local sine_network (Shared.new(440.0) .. Sine) * 0.5

sine_network(220.0) -- WILL ERROR, SEE PROPER EXAMPLE ABOVE
```
