## Effects

### Overview

| Name | Description | Parameters | Inputs |
|---|---|---|---|
| Reverb | Stereo reverb | Room size in meters, Approximate reverberation time until -60 db in beats, High frequency damping from 0 -> 1 | Input Left, Input Right |
| Delay | Fixed delay of specified beats. | Delay in beats | Input |
| Flanger | Mono flanger | Feedback Amount, Minimum Delay, Maximum Delay, Delay Switch Frequency | Input |
| Phaser | Mono phaser. Modifier is designed to be within the range 0 -> 1 | Feedback Amount, Phase Hz, Modifier | Input |
| Pluck | [Karplus-Strong](https://en.wikipedia.org/wiki/Karplus%E2%80%93Strong_string_synthesis) plucked string oscillator | Frequency, Gain per beat, High frequency damping (0 -> 1) | String Excitation |
| Chorus | Mono chorus with 5 voices | Voice separation in beats, Variation delay in beats, Delay modulation frequency | Input |
| ADSR | Attack, Decay, Sustain, Release. If input is positive, the ADSR starts. Output is always between 0 and 1 | Attack in Beats, Decay in Beats, Sustain in Beats, Release in Beats | ADSR Control |
| Clip | Clamp the input between two user defined values. Default minimum is -1, and default maximum is 1 | Minimum, Maximum | Input Signal |

### Examples

#### ADSR
```lua
-- TODO
```

#### Clip
```lua
-- TODO
```
