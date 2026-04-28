![Bird Pro header image](https://codeberg.org/vimae/birdpro/raw/branch/main/.forgejo/assets/birdpro-header.png)

# Bird Pro

Bird Pro is custom feature-rich text to speech software designed to
give a voice to those unable to speak.

Features:
  - Beautiful user interface
  - Multiple TTS voice providers (Piper, Microsoft Edge, ElevenLabs, Windows)
  - VRChat chatbox integration over OSC
  - Output to a txt file (for use in OBS subtitles)
  - Native Linux support

# Download

**Bird Pro is an ongoing work in progress and may be unstable.** Please keep this in mind if you want to try it out.

[**Download Release**](https://codeberg.org/vimae/birdpro/releases)

Development builds for Windows, macOS and Linux can be downloaded from [GitHub Actions](https://github.com/vimaexd/birdpro) currently.

### Please note:
- AppImages may require `LD_PRELOAD=/usr/lib/libwayland-client.so` in wayland setups due to an upstream Tauri bug

# Develop

- Install [Rust](https://rust-lang.org/tools/install/), [Bun](https://bun.sh/) and [LLVM](https://releases.llvm.org/)
- Clone the repo with submodules: `git clone --recursive https://codeberg.org/vimae/birdpro`
- Download the [Tauri System Dependencies](https://v2.tauri.app/start/prerequisites/) required for your operating system
- Run `bun install` to install JS dependencies
- Run `bunx tauri dev` to install Rust dependencies and start Bird Pro

# Credits
Bird Pro is a [vimae](https://mae.wtf) project

The name "Bird Pro" was inspired by [IvyNyabula](https://ivynyabula.cc/) and her bird self

# License
Bird Pro is licensed under [GPLv3](https://www.gnu.org/licenses/gpl-3.0.en.html).
