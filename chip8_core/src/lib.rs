
use std::fs;
use rand::{distributions::Standard, Rng};

const START_ADDRESS: u16 = 0x200;
const FONTSET_SIZE: usize = 80;
const FONTSET_START_ADDRESS: usize = 0x50;
pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_WIDTH: usize = 64;
pub const SCALE: usize = 15;

const FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
	0x20, 0x60, 0x20, 0x20, 0x70, // 1
	0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
	0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
	0x90, 0x90, 0xF0, 0x10, 0x10, // 4
	0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
	0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
	0xF0, 0x10, 0x20, 0x40, 0x40, // 7
	0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
	0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
	0xF0, 0x90, 0xF0, 0x90, 0x90, // A
	0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
	0xF0, 0x80, 0x80, 0x80, 0xF0, // C
	0xE0, 0x90, 0x90, 0x90, 0xE0, // D
	0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
	0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];


pub struct Chip8 {
    registers: [u8; 16],
    memory: [u8; 4096],
    i_register: u16,
    program_counter: u16,
    stack: [u16; 16],
    stack_pointer: u8,
    delay_timer: u8,
    sound_timer: u8,
    keypad: [bool; 16],
    display: [bool; 64 * 32],

}

impl Chip8 {
    pub fn new() -> Chip8{
        let mut new_chip8: Chip8 = Chip8 {
            registers: [0; 16],
            memory: [0; 4096],
            i_register: 0,
            program_counter: START_ADDRESS,
            stack: [0; 16],
            stack_pointer: 0,
            delay_timer: 0,
            sound_timer: 0,
            keypad: [false; 16],
            display: [false; 64 * 32],
        };

        for i in 0..FONTSET_SIZE {
            new_chip8.memory[FONTSET_START_ADDRESS + i] = FONTSET[i];
        }

        return new_chip8;
    }

    pub fn load(&mut self, data: &[u8]){
        let start = START_ADDRESS as usize;
        let end = (START_ADDRESS as usize) + data.len();
        self.memory[start..end].copy_from_slice(data);
    }


    fn push(&mut self, val:u16) {
        self.stack[self.stack_pointer as usize] = val;
        self.stack_pointer += 1;
    }

    fn pop(&mut self) -> u16 {
        self.stack_pointer -= 1;
        return self.stack[self.stack_pointer as usize];
    }

    pub fn cycle(&mut self) {
        // Fetch
        let higher_byte = self.memory[self.program_counter as usize] as u16;
        let lower_byte = self.memory[(self.program_counter + 1) as usize] as u16;
        let op = ( higher_byte << 8 | lower_byte );

        // Increment the PC before we execute anything
        self.program_counter += 2;

        // execute
        self.execute(op);

        // Decrement the delay timer if it's been set
	    if (self.delay_timer > 0)
	    {
	    	self.delay_timer -= 1;
	    }

	    // Decrement the sound timer if it's been set
	    if (self.sound_timer > 0)
	    {
	    	self.sound_timer -= 1;
	    }

    }

    pub fn get_display(&self) -> &[bool] {
        &self.display
    }

    pub fn keypress(&mut self, id: usize, pressed: bool) {
        self.keypad[id] = pressed;
    }

    fn execute(&mut self, op:u16) {
        let digit1 = (op & 0xF000) >> 12;
        let digit2 = (op & 0x0F00) >> 8;
        let digit3 = (op & 0x00F0) >> 4;
        let digit4 = op & 0x000F;

        match (digit1, digit2, digit3, digit4) {
            // Clears the screen
            (0, 0, 0xE, 0) => {
                self.display = [false; 64 * 32];
            },

            // Return from Subroutine RET
            (0, 0, 0xE, 0xE) => {
                let address = self.pop();
                self.program_counter = address;
            },

            // Jump to location nnn; 1nnn
            (1, _, _, _) => {
                let address: u16 = op & 0xFFF;
                self.program_counter = address;
            },

            // Call subroutine at nnn; 2nnn
            (2, _, _, _) => {
                let address: u16 = op & 0xFFF;
                self.push(self.program_counter);
                self.program_counter = address;
            },

            (3, _, _, _) => {
                let nn: u8 = (op & 0xFF) as u8;
                let x = digit2 as usize;
                if self.registers[x] == nn {
                    self.program_counter += 2;
                }
            },

            //Skip next instruction if Vx != kk
            (4, _, _, _) => {
                let nn: u8 = (op & 0xFF) as u8;
                let x = digit2 as usize;
                if self.registers[x] != nn {
                    self.program_counter += 2;
                }
            },

            // Skip next instruction if Vx != kk.
            (5, _, _, _) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.registers[x] == self.registers[y] {
                    self.program_counter += 2;
                }
            },

            // Set Vx = kk.
            (6, _, _, _) => {
                let nn: u8 = (op & 0xFF) as u8;
                let x = digit2 as usize;
                self.registers[x] = nn;
            },

            // Set Vx += kk.
            (7, _, _, _) => {
                let nn: u8 = (op & 0xFF) as u8;
                let x = digit2 as usize;
                self.registers[x] = self.registers[x].wrapping_add(nn);
            },

            // Set Vx = Vy.
            (8, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                self.registers[x] = self.registers[y];
            },

            // Set Vx = Vx OR Vy.
            (8, _, _, 1) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.registers[x] |= self.registers[y];
            },

            // Set Vx = Vx AND Vy.
            (8, _, _, 2) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.registers[x] &= self.registers[y];
            },

            // Set Vx = Vx XOR Vy.
            (8, _, _, 3) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                self.registers[x] ^= self.registers[y];
            },

            

            // Set Vx = Vx + Vy, set VF = carry.
            (8, _, _, 4) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let sum = self.registers[x] as u16 + self.registers[y] as u16;


                if sum > 255 {
                    self.registers[0xF] = 1
                } else {
                    self.registers[0xF] = 0
                }

                self.registers[x] = (sum & 0xFF) as u8;
            },

            // Set Vx = Vx - Vy, set VF = NOT borrow.
            (8, _, _, 5) => {
                let  x = digit2 as usize;
                let y = digit3 as usize;

                if (self.registers[x] > self.registers[y]) {
                    self.registers[0xF] = 1;
                } else {
                    self.registers[0xF] = 0;
                }

                (self.registers[x], _) = self.registers[x].overflowing_sub(self.registers[y]);

            },

            // Set Vx = Vx SHR 1.
            // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
            (8, _, _, 6) => {
                let x = digit2 as usize;

                self.registers[0xF] = self.registers[x] & 0x1;      // Store last bit(LSB) of Vx to VF
                self.registers[x] >>= 1;                            // Divide Vx by 2 using bitwise right shift by 1
            },

            // Set Vx = Vy - Vx, set VF = NOT borrow.
            (8, _, _, 7) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                if (self.registers[y] > self.registers[x]) {
                    self.registers[0xF] = 1;
                } else {
                    self.registers[0xF] = 0;
                }

                self.registers[x] = self.registers[y] - self.registers[x];
            },

            // Set Vx = Vx SHL 1.
            // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
            (8, _, _, 0xE) => {
                let x = digit2 as usize;

                self.registers[0xF] = (self.registers[x] & 0x80) >> 7;     // Store first bit(MSB) of Vx to VF
                self.registers[x] <<= 1;                            // multiply Vx by 2 using bitwise left shift by 1
            },

            // Skip next instruction if Vx != Vy.
            (9, _, _, 0) => {
                let x = digit2 as usize;
                let y = digit3 as usize;
                if self.registers[x] != self.registers[y] {
                    self.program_counter += 2;
                }
            },

            // set index to nnn
            (0xA, _, _, _) => {
                let nnn = op & 0xFFF;
                self.i_register = nnn;
            },

            // Jump to location nnn + V0.
            (0xB, _, _, _) => {
                let nnn = op & 0xFFF;
                self.program_counter = self.registers[0] as u16 + nnn;
            },

            // Set Vx = random byte AND nn
            (0xC, _, _, _) => {
                let x = digit2 as usize;
                let nn = (op & 0xFF) as u8;
                let rng: u8 = rand::thread_rng().gen();
                self.registers[x] = rng & nn;
            },

            // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            (0xD, _, _, _) => {
                let x = digit2 as usize;
                let y = digit3 as usize;

                let height = digit4;

                let xPos = self.registers[x] as usize % DISPLAY_WIDTH;
                let yPos = self.registers[y] as usize % DISPLAY_HEIGHT;

                self.registers[0xF] = 0;

                for row in 0..height {
                    let spriteByte = self.memory[(row + self.i_register) as usize];

                    for col in 0..8 {
                        let spritePixel = spriteByte & (0x80 >> col);
                        let screenPixel = ((xPos + col) + ((yPos + row as usize) * DISPLAY_WIDTH)) as usize;

                        if (spritePixel != 0) {
                            if (self.display[screenPixel]) {
                                self.registers[0xF] = 1;
                            }
                            self.display[screenPixel] ^= true;
                        }
                    }
                }
            },

            // Skip next instruction if key with the value of Vx is pressed.
            (0xE, _, 9, 0xE) => {
                let x = digit2 as usize;
                let Vx = self.registers[x];
                let key = self.keypad[Vx as usize];

                if (key){
                    self.program_counter += 2;
                }
            },

            // Skip next instruction if key with the value of Vx is not pressed.
            (0xE, _, 0xA, 1) => {
                let x = digit2 as usize;
                let Vx = self.registers[x];
                let key = self.keypad[Vx as usize];

                if (!key){
                    self.program_counter += 2;
                }
            },    

            // Set Vx = delay timer value.
            (0xF, _, 0, 7) => {
                let x = digit2 as usize;
                self.registers[x] = self.delay_timer;
            },       

            // Wait for a key press, store the value of the key in Vx.
            (0xF, _, 0, 0xA) => {
                let x = digit2 as usize;
                let mut pressed = false;
                for i in 0..self.keypad.len() {
                    if self.keypad[i] {
                        self.registers[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    //Redo Opcode
                    self.program_counter -= 2;
                }
            },

            // Set delay timer = Vx.
            (0xF, _, 1, 5) => {
                let x = digit2 as usize;
                self.delay_timer = self.registers[x];
            
            },

            // Set sound timer = Vx.
            (0xF, _, 1, 8) => {
                let x = digit2 as usize;
                self.sound_timer = self.registers[x];
            
            },

            // Set I = I + Vx.
            (0xF, _, 1, 0xE) => {
                let x = digit2 as usize;
                self.i_register += self.registers[x] as u16;
            
            },

            // Set I = location of sprite for digit Vx.
            (0xF, _, 2, 9) => {
                let x = digit2 as usize;
                let digit = self.registers[x];
                
                self.i_register = (FONTSET_START_ADDRESS  + (5 * digit) as usize ) as u16;
            
            },

            // Store BCD representation of Vx in memory locations I, I+1, and I+2.
            // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
            (0xF, _, 3, 3) => {
                let x = digit2 as usize;
                let mut value = self.registers[x];

                // ones place
                self.memory[self.i_register as usize  + 2] = value % 10;
                value /= 10;
                // tens place
                self.memory[self.i_register as usize + 1] = value % 10;
                value /= 10;
                // hundreds place
                self.memory[self.i_register as usize + 0] = value % 10;            
            },

            // Store registers V0 through Vx in memory starting at location I.
            (0xF, _, 5, 5) => {
                let x = digit2 as usize;

                for i in 0..=x {
                    self.registers[i] = self.memory[self.i_register as usize + i];
                }
            }

            // Read registers V0 through Vx from memory starting at location I.
            (0xF, _, 6, 5) => {
                let x = digit2 as usize;
                
                for i in 0 ..=x {
                    self.registers[x] = self.memory[self.i_register as usize + i];
                }
            
            },
            (_, _, _, _) => unimplemented!("Unimplemented opcode: {:#04x}", op),
        }
    }

}


