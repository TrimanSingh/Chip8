# CHIP-8 Emulator

A Rust implementation of a CHIP-8 emulator with a desktop frontend using SDL2.

## What is CHIP-8?

CHIP-8 is an interpreted programming language and virtual machine designed in the mid-1970s. It was originally used on the COSMAC VIP and Telmac 1800 8-bit microcomputers to make video games easier to develop and more portable. Today, CHIP-8 is a popular system for beginners looking to learn about emulator development.

## Project Structure

This project consists of two main components:

- `chip8_core`: A library crate that implements the core CHIP-8 emulation logic
- `desktop`: A desktop application that uses SDL2 to display graphics and handle user input

## Included ROMs

The repository includes several CHIP-8 ROM files for testing and demonstration:

- IBM Logo.ch8: A simple ROM that displays the IBM logo
- OK.ch8: A simple test ROM
- test_opcode.ch8: A ROM for testing CHIP-8 opcodes
- [Tetris [Fran Dachille, 1991].ch8](Tetris%20%5BFran%20Dachille%2C%201991%5D.ch8): A CHIP-8 implementation of Tetris

## Building and Running

### Prerequisites

- Rust and Cargo (install from [rustup.rs](https://rustup.rs))
- SDL2 development libraries

### Build Instructions

1. Clone the repository
2. Build the project with Cargo:

```sh
cargo build --release
```

3. Run the emulator with a ROM file:

```sh
cargo run --release -- path/to/rom.ch8
```

### Controls

The emulator likely maps the original CHIP-8 16-key keypad to your keyboard. The typical mapping is:

```
CHIP-8 Keypad:     Keyboard Mapping:
+-+-+-+-+          +-+-+-+-+
|1|2|3|C|          |1|2|3|4|
+-+-+-+-+          +-+-+-+-+
|4|5|6|D|          |Q|W|E|R|
+-+-+-+-+    ->    +-+-+-+-+
|7|8|9|E|          |A|S|D|F|
+-+-+-+-+          +-+-+-+-+
|A|0|B|F|          |Z|X|C|V|
+-+-+-+-+          +-+-+-+-+
```

## Technical Details

The emulator implements all standard CHIP-8 features, including:

- 4KB memory space
- 16 8-bit registers (V0-VF)
- A program counter (PC)
- An index register (I)
- A stack for subroutine calls
- A delay timer and a sound timer
- A 64×32 pixel monochrome display
- A 16-key hexadecimal keypad
- 35 opcodes

The core emulation is separated from the display and input handling, making it easy to port to different platforms.
