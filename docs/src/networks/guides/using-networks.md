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

**Piping** is used to take a output of one network, e.g. a [constant](../references/using-shared.md), and "pipe" it into the input of another, like a Sine generator.

```lua
-- Very basic piped network
local basic_piped_network = Constant.new(440.0) .. Sine

-- You can take a pre-existing network and pipe it into more generators
local many_piped_network = basic_piped_network .. Saw .. Triangle

-- Or even itself
local double_piped_network = basic_piped_network .. basic_piped_network
```

### Arithmetic

<!--
Should cover:
- Addition
- Subtraction
- Multiplication
- Tips about Constants, and things to watch out for.
-->

### Stacking

<!--
How to stack, why stacking is needed for certain things
-->

### Playing Networks

For playing networks, please see [Audio Utilities/Playback](../../audio/references/playback.md).
