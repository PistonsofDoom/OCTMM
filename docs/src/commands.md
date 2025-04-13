
## Create
`UNIMPLEMENTED`

``octmm create <Name> [Path]``

Creates an empty audio project 



### Args
- Name: Name of the project, can only use the characters:
  - a-Z
  - 0-9
  - `-` and `_`
- Path (opt): Specifies what directory the project should be created in
  - Default: Current working directory

### Examples

- ``octmm create MyNewSong``
- ``octmm create MyNewSong ~/Songs/``

## Play
`UNIMPLEMENTED`

``octmm play [Path]``

Plays an audio project live. If a path isn't specified, it will attempt
to play the current working directory.

### Args
- Path (opt): Specifies what directory the project to play is located in
  - Default: Current working directory

### Examples

- ``octmm play``
- ``octmm play ~/Songs/MyNewSong/``

## Export
`UNIMPLEMENTED`

``octmm export <Project Path> <Export Path> [Format]``

Exports an audio project into a specified file type.

### Args
- Project Path: Path to the project directory
- Export Path: Path to the directory the exported file should be stored in
- Format (opt): Audio format the exported project should be
  - Default: `TODO: Set as whatever lossless format is best when implemented`

### Examples

- ``octmm export ./ ~/SongExports/``
- ``octmm export ~/Songs/MyNewSong/ ~/SongExports/ Wave``

