# CHIP-8 Emulator

A CHIP-8 interpreter/emulator written in Rust with SDL2 for graphics and input handling.

## Overview

CHIP-8 is an interpreted programming language developed in the 1970s for early microcomputers. This project implements a complete CHIP-8 interpreter with a desktop frontend using SDL2 for rendering and input.

## Project Structure

The project is organized into two main components:

### `chip8_core/`
The core emulator library containing:
- Complete CHIP-8 instruction set implementation
- Memory management (4KB RAM)
- 16 general-purpose registers (V0-VF)
- Timer systems (delay and sound timers)
- Display buffer (64x32 pixels)
- Keypad input handling
- Built-in font set for hexadecimal digits

### `desktop/`
The SDL2-based desktop frontend providing:
- Window management and rendering
- Real-time display updates
- Keyboard input mapping
- Game loop implementation

## Features

### Complete CHIP-8 Instruction Set
- Memory operations (load, store, copy)
- Arithmetic operations (add, subtract, bitwise operations)
- Control flow (jumps, subroutines, conditionals)
- Graphics rendering with collision detection
- Timer operations
- Random number generation
- Keyboard input handling

### Display System
- 64x32 monochrome display
- Configurable scaling (currently 15x)
- Real-time rendering with SDL2
- Sprite drawing with XOR pixel operations

### Input System
- 16-key hexadecimal keypad mapping:
  ```
  1 2 3 C    ->    1 2 3 4
  4 5 6 D    ->    Q W E R
  7 8 9 E    ->    A S D F
  A 0 B F    ->    Z X C V
  ```

## Technical Details

### Memory Layout
- **0x000-0x1FF**: Reserved for interpreter
- **0x050-0x0A0**: Built-in font set
- **0x200-0xFFF**: Program memory (3584 bytes)

### Registers
- **V0-VE**: General purpose 8-bit registers
- **VF**: Flag register (used for carry, borrow, collision)
- **I**: 16-bit index register
- **PC**: Program counter
- **SP**: Stack pointer

### Timers
- **Delay Timer**: Decrements at 60Hz, used for timing events
- **Sound Timer**: Decrements at 60Hz, beeps when non-zero

## Building and Running

### Prerequisites
- Rust (latest stable version)
- SDL2 development libraries

#### Installing SDL2 on Windows
1. Download SDL2 development libraries from [libsdl.org](https://www.libsdl.org/download-2.0.php)
2. Extract and place `SDL2.dll` in the project directory (already included)

#### Installing SDL2 on Linux (Ubuntu/Debian)
```bash
sudo apt-get install libsdl2-dev
```

#### Installing SDL2 on macOS
```bash
brew install sdl2
```

### Building
```bash
# Build the project
cd desktop
cargo build

# Build with optimizations
cargo build --release
```

### Running
```bash
# Run with a ROM file
cargo run /path/to/rom.ch8

# Example with included ROMs
cargo run "../IBM Logo.ch8"
cargo run "../Tetris [Fran Dachille, 1991].ch8"
```

## Included ROMs

The project includes several test ROMs:
- **IBM Logo.ch8**: Displays the IBM logo (classic test ROM)
- **Tetris [Fran Dachille, 1991].ch8**: Tetris game implementation
- **test_opcode.ch8**: Instruction set test ROM
- **OK.ch8**: Simple test program

## Controls

The CHIP-8 keypad is mapped to your keyboard as follows:

| CHIP-8 Key | Keyboard Key |
|------------|--------------|
| 1 2 3 C    | 1 2 3 4      |
| 4 5 6 D    | Q W E R      |
| 7 8 9 E    | A S D F      |
| A 0 B F    | Z X C V      |

## Architecture

### Core Emulation Loop
1. **Fetch**: Read instruction from memory at program counter
2. **Decode**: Parse the 16-bit instruction
3. **Execute**: Perform the corresponding operation
4. **Update**: Decrement timers and update display

### Instruction Format
CHIP-8 instructions are 16-bit (2 bytes) with the following format:
- **nnn**: 12-bit address
- **nn** or **kk**: 8-bit constant
- **n**: 4-bit constant
- **x** and **y**: 4-bit register identifiers

## Dependencies

### Core Library (`chip8_core`)
- `rand = "0.8.5"` - Random number generation

### Desktop Frontend (`desktop`)
- `sdl2 = "^0.34.3"` - Graphics and input handling
- `chip8_core` - Local core library

## Development

### Code Organization
- **lib.rs**: Core CHIP-8 implementation with complete instruction set
- **main.rs**: SDL2 frontend with rendering and input handling

### Key Constants
- `DISPLAY_WIDTH`: 64 pixels
- `DISPLAY_HEIGHT`: 32 pixels
- `SCALE`: 15x scaling factor
- `TICKS_PER_FRAME`: 10 (controls emulation speed)

## Future Enhancements

Potential improvements for the emulator:
- Audio support for the sound timer
- Configurable key mappings
- Save states
- Debugger interface
- Support for CHIP-8 variants (SUPER-CHIP, XO-CHIP)
- ROM compatibility testing suite

## License

This project is open source. Please check individual ROM files for their respective licenses.

## Resources

- [CHIP-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [CHIP-8 Wikipedia](https://en.wikipedia.org/wiki/CHIP-8)
- [SDL2 Documentation](https://wiki.libsdl.org/SDL2/FrontPage)
