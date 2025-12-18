# Using Networks

### Table of Contents

- [Basics](#basics)
- [Arithmetic](#arithmetic)
- [Piping](#piping)
- [Stacking](#stacking)

### Basics

A **Network** is the fundamental unit of outputting sound within OCTMM.
It represents everything from a simple Sine wave, to a complex signal processor mixing multiple waveforms, filters, and effects.

In order to make networks, several default [waveforms](../references/generators.md), [filters](../references/filters.md), and [effects](../references/effects.md) are provided. Each of these defaults can be thought of as a building block for more complex networks, with each of them being able to interact with one another through **Inputs** and **Outputs**. The number of **Inputs** and **Outputs** varies per node, with some requiring two inputs for one output, or vice versa. For more information on using networks with multiple inputs/outputs, see [Stacking](#stacking)

### Piping

**Piping** is used to take a output of one network (e.g. a [constant](../guides/using-shared.md)), and "pipe" it into the input of another, like a Sine generator. The operand used to pipe is ``NetworkA .. NetworkB``, where the outputs of Network A are fed into the inputs of Network B.

```lua
-- Piping a constant frequency into a generator
local basic_piped_network = Constant.new(440.0) .. Sine

-- Piping a pre-existing network through multiple generators
local many_piped_network = basic_piped_network .. Saw .. Triangle
```

### Arithmetic

There are 3 ways to combine networks, **Addition (+)**, **Subtraction (-)**, and **Multiplication (\*)**. All of these operands behave as it would in any other programming language. There is no division operand.

```lua
local freq = Constant.new(440.0)

local added_network = (freq .. Sine) + (freq .. Saw)
local subtracted_network = (freq .. Saw) - (freq .. Square)
local multiplied_network = added_network * subtracted_network
```

### Stacking

**Stacking** is an operand used to "stack" networks on top of each other. The operand for stacking is ``NetworkA // NetworkB``, where Network A is the lower network, and Network B is the upper network.

There are two primary use cases for stacking networks.
1. Controlling Networks with multiple inputs
    - Networks with multiple inputs, such as the [Pulse](../references/generators.md) generator, require multiple inputs to determine not only frequency, but pulse width as well.

2. Separating Left and Right audio channel
    - Internally, all played networks are output into a stereo audio stream, and all mono networks are converted to stereo automatically. Therefore, you can stack two networks together to create a unique left / right channel sound, assuming both networks only have 1 output.

```lua
-- Example of using Pulse generator
local frequency = Constant.new(440.0)
local duty_cycle = Constant.new(5.0) .. Sine

local pulse_network = (frequency // duty_cycle) .. Pulse

-- Using two networks for unique stereo sound
local left_channel = frequency .. Sine
local right_channel = frequency .. Saw

local stereo_network = left_channel // right_channel
```

### Playing Networks

For playing networks, please see [Audio Utilities/Playback](../../audio/references/playback.md).
