![Bird Pro header image](https://codeberg.org/vimae/birdpro/raw/branch/main/.forgejo/assets/birdpro-header.png)

# Bird Pro

Bird Pro is custom feature-rich text to speech software designed to
give a voice to those unable to speak.

Features:
  - Beautiful user interface
  - Multiple TTS voice providers (Windows, Microsoft Edge, Elevenlabs)
  - VRChat chatbox integration over OSC
  - Output to a txt file (for use in OBS subtitles)
  - Native Linux support

# Download

**Bird Pro is currently work in progress and has not reached release state.** Please keep this in mind if you want to try it out.

Latest builds for Windows and Linux can be downloaded from [Codeberg Actions](https://codeberg.org/vimae/birdpro/actions) currently.

### Please note:
- AppImages are currently not working due to an upstream bug

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
