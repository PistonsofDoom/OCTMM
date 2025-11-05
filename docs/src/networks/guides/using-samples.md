## Using Samples

### Loading Samples

In order to use samples within a [project](../../project.md), it has to exist under the `samples` folder.
Every valid sample under this folder will be loaded upon project play / export, therefore care should be taken to only insert samples being used.
Samples are allowed to be organized by folders, however the folder name is not included in the sample name upon loading. 

Any audio format supported by the Rust crate [symphonia](https://docs.rs/symphonia/latest/symphonia/) is valid.

An example of a sample folder structure would be the following:
```
samples/kicks/kick01.wav
samples/kicks/clap01.wav
samples/effects/riser.wav
```

### Using Samples

When writing your project samples can be used by calling `Sample.new(name, loop_point, channel)`, where `name` is the name of your audio file, `loop_point` is the point in seconds at which the sample loops back to after reaching the end, and `channel` is the channel to use for the network starting from 1.
If `loop_point` is left empty or nil, the sample will not loop.
If `channel` is left empty or nil, all channels of the sample will be used for the network.

After using `Sample.new(x, y, z)`, a network will be created using the sample as the data within it. 
The input to the network controls the speed, where `1.0` is normal, and `2.0` is twice the speed, `0.5` is half the speed, etc. 
The output(s) of the network can vary, as if `channel` is left empty, it will have as many outputs as the sample provides in channels. 
If `channel` is configured, it will only have the one output.

```lua
-- Only name given, no loop nor channel
local kick01_sample = Sample.new("kick01")
local kick01_network = Constant.new(1.0) .. kick01_sample

-- Name, loop point, and channel
local clap01_sample = Sample.new("clap01", 0.0, 1)
local clap01_network = Constant.new(1.5) .. clap01_sample

-- Name, no loop, channel
local riser_sample = Sample.new("riser", nil, 2)
local riser_network = Constant.new(0.75) .. riser_network .. Sine 

-- You can also feed samples into generators and effects!
local cool_network = ((riser_network .. Sine) // Constant.new(7000.0)) .. Lowpole
