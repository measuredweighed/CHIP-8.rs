# 🎮 CHIP-8.rs
A [CHIP-8](https://en.wikipedia.org/wiki/CHIP-8) emulator written in Rust as a means of learning more about emulation and the Rust language itself. Initial code was pieced together by reading [various articles](http://emulator101.com/) and then improved by looking to [other Rust implementations](https://github.com/starrhorne) for tips on best practices.

![Screenshot of CHIP-8 running SPACE INVADERS](https://github.com/measuredweighed/CHIP-8.rs/blob/master/screenshot.png)

# Usage
Run via `cargo` like so:
```cargo run games/tetris.ch8```

# Stuff that's missing
* Although the opcodes and behaviour of the `sound_timer` are correctly emulated, there's currently no support for playing sound.
* Better error handling, which seems like a particularly egregious sin for a Rust implementation of... anything.
