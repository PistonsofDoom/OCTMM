
### BEFORE RC/0.1.0 is merged
- Add DSP subtract
- Add DSP 'envelope', which contains a couple custom envelopers a user can create

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

- 'Note Group' lua class, which is created by adding notes together
- Note Utility functions for offsetting frequencies by constant offsets and multipliers
