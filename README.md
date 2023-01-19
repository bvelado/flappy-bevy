# Flappy bevy

A flappy bird clone made with bevy for learning purposes.

## Building and running
### Windows
Run development (with dynamic linking)
```sh
cargo run --features bevy/dynamic --target=x86_64-pc-windows-msvc
```

Release build
```sh
cargo build --release --target=x86_64-pc-windows-msvc
```

### Web (WASM)
Run development (no dynamic linking with WASM)
```sh
trunk serve
```

Release build
```sh
trunk build
```

## Tools and assets

Written in [rust](https://www.rust-lang.org/fr) with
[bevy game engine](https://bevyengine.org/).

Sprite exports made with [LDtk](https://ldtk.io/) from
[Deepnight games](https://deepnight.net/).

[Pixel platformer](https://kenney.nl/assets/pixel-platformer) art from
[Kenney](https://kenney.itch.io/kenney-donation).

[Dogica](https://www.dafont.com/fr/dogica.font) font from
[Roberto Mocci](https://www.patreon.com/rmocci).

[Trunk](https://trunkrs.dev/) bundler for WASM builds.