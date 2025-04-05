
## Description

Overly Complicating The Music Maker is a program designed
to provide a CLI tool that allows a user to produce
audio files through writing Lua scripts, rather than through
a traditional piano roll or tracker interface.

The main features this program aims to provide is a CLI
interface for running Lua scripts, with Lua functions for:
- Sample playback with resampling for different pitches
- Synthesizer creation on the fly using the `fundsp` crate
- Sequencing for samples / synthesizers
