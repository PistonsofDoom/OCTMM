## Effects

### Overview

| Name | Description | Parameters | Inputs |
|---|---|---|---|
| Reverb | Stereo reverb | <ul><li>Room size in meters</li><li>Approximate reverb time until -60 db in beats</li><li>High frequency damping (0 -> 1)</li></ul> | <ol><li>Input Left</li><li>Input Right</li></ol> |
| Delay | Fixed delay of specified beats. | <ul><li>Delay in beats</li></ul> | <ol><li>Input</li></ol> |
| Flanger | Mono flanger | <ul><li>Feedback Amount</li><li>Minimum Delay</li><li>Maximum Delay</li><li>Delay Switch Frequency</li></ul> | <ol><li>Input</li></ol> |
| Phaser | Mono phaser. Modifier is designed to be within the range 0 -> 1 | <ul><li>Feedback Amount</li><li>Phase Hz</li><li>Modifier</li></ul> | <ol><li>Input</li></ol> |
| Pluck | [Karplus-Strong](https://en.wikipedia.org/wiki/Karplus%E2%80%93Strong_string_synthesis) plucked string oscillator | <ul><li>Frequency</li><li>Gain per beat</li><li>High frequency damping (0 -> 1)</li></ul> | <ol><li>String Excitation</li></ol> |
| Chorus | Mono chorus with 5 voices | <ul><li>Voice separation in beats</li><li>Variation delay in beats</li><li>Delay modulation frequency</li></ul> | <ol><li>Input</li></ol> |
| ADSR | Attack, Decay, Sustain, Release. If input is positive, the ADSR starts. Output is always between 0 and 1 | <ul><li>Attack in Beats</li><li>Decay in Beats</li><li>Sustain in Beats</li><li>Release in Beats</li></ul> | <ol><li>ADSR Control</li></ol> |
| Clip | Clamp the input between two user defined values. Default minimum is -1, and default maximum is 1 | <ul><li>Minimum</li><li>Maximum</li></ul> | <ol><li>Input Signal</li></ol> |

### Examples

#### ADSR
```lua
-- TODO
```

#### Clip
```lua
-- TODO
```
