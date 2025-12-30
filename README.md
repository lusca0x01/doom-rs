# doom-rs

Port of classic Doom to Rust.

## Manual step required for audio (submodule)

To build with the Rust audio backend, you must manually edit the following file:

	native/doomgeneric/doomgeneric/i_sound.c

Change the preprocessor line to:

	#if defined(FEATURE_SOUND) && !defined(__DJGPP__) && !defined(DG_SOUND_RUST)

Make sure the `DG_SOUND_RUST` flag is defined to avoid audio implementation conflicts.

**This change will NOT be committed to this repository, as the file is part of a submodule. If you want this to be permanent, please open a PR in the original project.**

## Features
- Graphics in Win32 window (GDI)
- Sound effects support (rodio)
- Classic and WASD key mapping
- Music structure ready (stubs)
- C/Rust integration via FFI

## Default Controls
- Arrow/Ctrl/Alt/Shift: movement, strafe, run
- Spacebar: use/open
- Ctrl: fire
- Tab: map
- Esc: menu
- 1–7: weapon selection
- WASD: modern alternative

## Build
Requirements:
- Rust (edition 2021+)
- MSVC toolchain
- [rodio](https://crates.io/crates/rodio)
- Windows SDK

To build:
```sh
cargo build --release
```

## Structure
- `src/`: main Rust code
- `native/`: legacy C code (doomgeneric)
- `build.rs`: integrates C into Rust build

## Future Improvements
- Real music playback implementation (currently only stubs)
- More platform support
- Code cleanup and optimizations

Contributions are welcome! Feel free to open issues or pull requests.

## Credits
- Original Doom: id Software
- DoomGeneric: Simon Howard
- Rust port: Lucas Wagner Fernandes

## License
GPL v2 or later (as per DoomGeneric)
