# Using Constants and Shared variables

### Using Constants

**Constants** are used to implement a constant number within a network. They can be created manually by using the function ``Constant.new(x)``, where `x` can be any decimal or integer number. There also exists automatic `Number` -> `Network` conversion, showcased in the example below.

```lua
local frequency = Constant.new(440.0)
local amplitude = Constant.new(0.5)

local sine_network = (frequency .. Sine) * amplitude

-- Using automatic conversion
local sine_network_auto = (440.0 .. Sine) * 0.5
```

When using automatic conversions of `Number` -> `Constants`, it is important to note that the conversion *only* occurs when interacting with a network.
If you are planning to [stack](./using-networks.md#stacking) two numbers together, it is important to have at least *one* of them be created using `Constant.new()`.

```lua
-- Valid stacking of Constants
local valid_stack = Constant.new(440.0) // Constant.new(2.2)

-- Invalid stacking, Actually just floor divides numbers
local invalid_stack = 440.0 // 2.2
```

### Using Shared

