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

**Piping** is used to take a output of one network (e.g. a [constant](../guides/using-shared.md)), and "pipe" it into the input of another, like a Sine generator.

```lua
-- Piping a constant frequency into a generator
local basic_piped_network = Constant.new(440.0) .. Sine

-- Piping a pre-existing network through multiple generators
local many_piped_network = basic_piped_network .. Saw .. Triangle
```

### Arithmetic

There are 3 ways to combine networks, Addition (+), Subtraction (-), and Multiplication (\*).

```lua
local freq = Constant.new(440.0)

local adding_networks = (freq .. Sine) + (freq .. Saw)
local subtracting_networks = (freq .. Saw) - (freq .. Square)

local multiplying_networks = adding_networks * subtracting_networks
```

### Stacking



<!--
How to stack, why stacking is needed for certain things
-->

### Playing Networks

For playing networks, please see [Audio Utilities/Playback](../../audio/references/playback.md).
