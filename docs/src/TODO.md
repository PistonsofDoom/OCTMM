
### BEFORE RC/0.1.0 is merged
- Check DSP module for unused functions
    - Print a message for any invalid network combinations!
- Timer module should have a "schedule" function, which refers to scheduling a function to be called after so many beats, once
- Note Utilities
    - Synth Utility
    - Sample Utility
    - Sequence Utility
        - Simply provides what amounts to an array with a .step() function that returns the value, and whether or not it looped

- Document all internal lua commands
- Create template Luau file
  - Update test_project_load to confirm template luau file is created
- Create example project
    - Remove todo in audio_module test
- Update program documentation to be more user friendly
- Investigate GitHub actions for automatic MdBook compilation (to either github wiki or github pages?)

### AFTER RC/0.1.0 is released
- Refactor DSP module net functions to reduce repeat code.
- Look into wavech_at, see if its a good idea to implement for the program

- Investigate making "time" be based in beats, where a higher BPM = a larger increase in time per update

- 'Log' struct, which should handle all Error, Warning, and Information messages
