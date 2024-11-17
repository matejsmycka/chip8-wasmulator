# Chip-8 WASM Emulator

This is a minimal one-file Chip-8 emulator written in Rust, ported to WebAssembly. It is based on the [sdl2 minimal emulator](https://github.com/matejsmycka/chip8-min-emulator-rs).

JavaScript is used to load the WebAssembly module and handle the user interface. Rust is only emulating the Chip-8 machine and returning the screen buffer which is rendered on a canvas.

![screenshot](https://i.imgur.com/GxrIKBm.png)

## Controls

The Chip-8 has a 16-key hexadecimal keypad, which is mapped to the following keys:

```
1 2 3 C
4 5 6 D
7 8 9 E
A 0 B F
```
