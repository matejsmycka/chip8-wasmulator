# Chip-8 WASM Emulator

Visit deploy here: https://matejsmycka.github.io/chip8-wasmulator/

This minimal one-file Chip-8 emulator is written in Rust and ported to WebAssembly. It is based on the [sdl2 minimal emulator](https://github.com/matejsmycka/chip8-min-emulator-rs).

JavaScript is used to load the WebAssembly module and handle the user interface. Rust only emulates the Chip-8 machine and returns the screen buffer rendered on a canvas.

![screenshot](https://i.imgur.com/GxrIKBm.png)

## Controls

The Chip-8 has a 16-key hexadecimal keypad, which is mapped to the following keys:

```
1 2 3 C
4 5 6 D
7 8 9 E
A 0 B F
```
