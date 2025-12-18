## Generators

### Overview

| Name | Description | Inputs |
|---|---|---|
| Hammond | Hammond oscillator | Frequency | 
| Organ | Organ oscillator | Frequency |
| Sine | Sine oscillator | Frequency |
| Saw | Saw oscillator | Frequency |
| SoftSaw | Softened saw oscillator | Frequency |
| Square | Square oscillator | Frequency |
| Triangle | Triangle oscillator | Frequency |
| Lorenz | [Lornez system](https://en.wikipedia.org/wiki/Lorenz_system) oscillator | Frequency |
| Rossler | [Rossler](https://en.wikipedia.org/wiki/R%C3%B6ssler_attractor) oscillator. Exhibits peaks at multiples of the frequency. | Frequency |
| Ramp | Generates a ramp within the range of 0 -> 1 | Frequency |
| Pulse | Pulse oscillator. Duty cycle ranges from 0 -> 1| Frequency, Duty Cycle |
| MLS | White noise with only -1 and 1 values | |
| White | White noise | |
| Pink | Pink noise | |
| Brown | Brown noise | |

### Examples

<!-- TODO: CONFIRM THE EXAMPLES WORK -->
#### Single input 

```lua
local freq_sine = Constant.new(440)

local sine_net = freq_sine .. Sine

Play(sine_net)
```
#### Dual input

```lua
local pulse_duty = Constant.new(0.5)
local pulse_freq = Constant.new(440)

local pulse_net = (pulse_freq // pulse_duty) .. Pulse

Play(pulse_net)
```

#### No input

```lua
local freq_saw = Constant.new(220)

local saw_with_noise = MLS * (freq_saw .. Saw)

Play(saw_with_noise)
```
