## Filters

### Overview

| Name | Description | Inputs |
|---|---|---|
| Resonator | Constant-gain bandpass resonator. Center frequency and bandwith in Hz | <ol><li>Input</li><li>Center Frequency</li><li>Bandwidth</li></ol> |
| Peak | Peaking filter | <ol><li>Input</li><li>Center Frequency</li><li>Q</li> |
| Pinkpass | Pinking filter | <ol><li>Input</li></ol> |
| Panner | Mono to stereo panner. Pan input should between -1 -> 1 | <ol><li>Input</li><li>Pan</li></ol> |
| Notch | Notching filter | <ol><li>Input</li><li>Center Frequency</li><li>Q</li></ol>|
| Moog | Moog resonant lowpass filter | <ol><li>Input</li><li>Cutoff Frequency</li><li>Q</li></ol> |
| Morph | Filter that "morphs" between lowpass, peak, and highpass. Morph should be between -1 -> 1 | <ol><li>Input</li><li>Center Frequency</li><li>Q</li><li>Morph</li> |
| Lowpole | One-pole lowpass filter | <ol><li>Input</li><li>Cutoff Frequency</li></ol> |
| Lowpass | Lowpass filter | <ol><li>Input</li><li>Cutoff Frequency</li><li>Q</li></ol> |
| Lowshelf | Low shelf filter with adjustable gain | <ol><li>Input</li><li>Center Frequency</li><li>Q</li><li>Gain</li></ol> |
| Highpole | One-pole highpass filter | <ol><li>Input</li><li>Cutoff Frequency</li></ol>|
| Highpass | Highpass filter | <ol><li>Input</li><li>Cutoff Frequency</li><li>Q</li></ol>|
| Highshelf | High shelf filter with adjustable gain | <ol><li>Input</li><li>Center Frequency</li><li>Q</li><li>Gain</li></ol>|
| Butterpass | Butterworth lowpass filter | <ol><li>Input</li><li>Cutoff Frequency</li></ol>|
| Bell | Bell filter with adjustable gain | <ol><li>Input</li><li>Center Frequency</li><li>Q</li><li>Gain</li></ol>|
| Bandpass | Bandpass filter | <ol><li>Input</li><li>Center Frequency</li><li>Q</li></ol> |
| Bandrez | Resonant two-pole bandpass filter | <ol><li>Input</li><li>Center Frequency</li><li>Q</li></ol> |
| Allpass | Allpass filter | <ol><li>Input</li><li>Center Frequency</li><li>Q</li></ol> |

### Examples

#### Lowpass

```lua
local organ_network = Constant.new(440.0) .. Organ 
local cutoff_frequency = Constant.new(330.0)
local q_factor = Constant.new(1.0)

local lowpassed_network = (organ_network // cutoff_frequency // q_factor) .. Lowpass
```

#### Panner

```lua
local organ_network = Constant.new(440.0) .. Organ 
local pan_sine_network = Constant.new(5.0) .. Sine

local panning_network = (organ_network // pan_sine_network) .. Panner
```
