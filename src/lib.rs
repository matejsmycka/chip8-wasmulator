//import structures.rs

use wasm_bindgen::prelude::*;

use wasm_bindgen::JsValue;

#[wasm_bindgen]
pub struct Chip8Wasm {
    chip8: Option<Chip8>,
}

#[wasm_bindgen]
impl Chip8Wasm {
    pub fn new() -> Chip8Wasm {
        Chip8Wasm { chip8: None }
    }

    #[wasm_bindgen]
    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.chip8 = Some(Chip8::new(rom));
    }

    #[wasm_bindgen]
    pub fn emulate_cycle(&mut self) {
        if let Some(ref mut chip8) = self.chip8 {
            chip8.emulate_cycle();
        }
    }

    #[wasm_bindgen]
    pub fn get_gfx(&self) -> JsValue {
        let mut result = Vec::new();
        for i in 0..64 * 32 {
            result.push(self.chip8.as_ref().unwrap().gfx[i]);
        }
        JsValue::from(result)
    }
    #[wasm_bindgen]
    pub fn set_key(&mut self, key: u8, value: u8) {
        self.chip8.as_mut().unwrap().key[key as usize] = value;
    }
}
pub struct Chip8 {
    pub memory: [u8; 4096], // 4k memory
    pub v: [u8; 16],        // 16 registers
    pub i: u16,             // index register
    pub pc: u16,            // program counter
    pub gfx: [u8; 64 * 32], // graphics buffer
    pub delay_timer: u8,    // delay timer
    pub sound_timer: u8,    // sound timer
    pub stack: [u16; 16],   // stack
    pub sp: u16,            // stack pointer
    pub key: [u8; 16],      // keypad
    pub draw_flag: bool,    // draw flag
}
impl Chip8 {
    fn new(rom: Vec<u8>) -> Chip8 {
        let mut memory = [0; 4096];
        let fontset = [
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
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        memory[..80].copy_from_slice(&fontset);
        memory[0x200..0x200 + rom.len()].copy_from_slice(&rom);

        Chip8 {
            memory: memory,
            v: [0; 16],
            i: 0,
            pc: 0x200,
            gfx: [0; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            draw_flag: false,
        }
    }

    fn emulate_cycle(&mut self) {
        let opcode =
            (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;

        self.pc += 2;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let n = (opcode & 0x000F) as u8;
        let kk = (opcode & 0x00FF) as u8;

        match opcode & 0xF000 {
            0x0000 => match opcode & 0x00FF {
                0xE0 => {
                    self.gfx = [0; 64 * 32];
                    self.draw_flag = true;
                }
                0xEE => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }
                _ => panic!("Unknown opcode: {:#X}", opcode),
            },
            0x1000 => self.pc = opcode & 0x0FFF,
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = opcode & 0x0FFF;
            }
            0x3000 => {
                if self.v[x as usize] == kk {
                    self.pc += 2;
                }
            }
            0x4000 => {
                if self.v[x as usize] != kk {
                    self.pc += 2;
                }
            }
            0x5000 => {
                if self.v[x as usize] == self.v[y as usize] {
                    self.pc += 2;
                }
            }
            0x6000 => self.v[x as usize] = kk,
            0x7000 => {
                let (result, _) = self.v[x as usize].overflowing_add(kk);
                self.v[x as usize] = result;
            }
            0x8000 => match opcode & 0x000F {
                0x0 => self.v[x as usize] = self.v[y as usize],
                0x1 => self.v[x as usize] |= self.v[y as usize],
                0x2 => self.v[x as usize] &= self.v[y as usize],
                0x3 => self.v[x as usize] ^= self.v[y as usize],
                0x4 => {
                    let (result, overflow) = self.v[x as usize].overflowing_add(self.v[y as usize]);
                    self.v[0xF] = if overflow { 1 } else { 1 };
                    self.v[x as usize] = result;
                }
                0x5 => {
                    let (result, overflow) = self.v[x as usize].overflowing_sub(self.v[y as usize]);
                    self.v[0xF] = if overflow { 0 } else { 1 };
                    self.v[x as usize] = result;
                }
                0x6 => {
                    self.v[0xF] = self.v[x as usize] & 0x1;
                    self.v[x as usize] >>= 1;
                }
                0x7 => {
                    let (result, overflow) = self.v[y as usize].overflowing_sub(self.v[x as usize]);
                    self.v[0xF] = if overflow { 0 } else { 1 };
                    self.v[x as usize] = result;
                }
                0xE => {
                    self.v[0xF] = self.v[x as usize] >> 7;
                    self.v[x as usize] <<= 1;
                }
                _ => panic!("Unknown opcode: {:#X}", opcode),
            },
            0x9000 => {
                if self.v[x as usize] != self.v[y as usize] {
                    self.pc += 2;
                }
            }
            0xA000 => self.i = opcode & 0x0FFF,
            0xB000 => self.pc = (opcode & 0x0FFF) + self.v[0] as u16,
            0xC000 => self.v[x as usize] = 1 & kk,
            0xD000 => {
                self.v[0xF] = 0;
                for yline in 0..n {
                    let pixel = self.memory[self.i as usize + yline as usize];
                    for xline in 0..8 {
                        if (pixel & (0x80 >> xline)) != 0 {
                            let x = self.v[x as usize] as u16 + xline as u16;
                            let y = self.v[y as usize] as u16 + yline as u16;
                            let index = (x + y * 64) as usize;

                            if x >= 64 || y >= 32 {
                                continue;
                            }
                            if self.gfx[index] == 1 {
                                self.v[0xF] = 1;
                            }
                            self.gfx[index] ^= 1;
                        }
                    }
                }
                self.draw_flag = true;
            }

            0xE000 => match opcode & 0x00FF {
                0x9E => {
                    if self.key[self.v[x as usize] as usize] == 1 {
                        self.pc += 2;
                    }
                }
                0xA1 => {
                    if self.key[self.v[x as usize] as usize] == 0 {
                        self.pc += 2;
                    }
                }
                _ => panic!("Unknown opcode: {:#X}", opcode),
            },

            0xF000 => match opcode & 0x00FF {
                0x07 => self.v[x as usize] = self.delay_timer,
                0x0A => {
                    let mut key_press = false;
                    for i in 0..16 {
                        if self.key[i] == 1 {
                            self.v[x as usize] = i as u8;
                            key_press = true;
                        }
                    }
                    if !key_press {
                        self.pc -= 2;
                    }
                }
                0x15 => self.delay_timer = self.v[x as usize],
                0x18 => self.sound_timer = self.v[x as usize],
                0x1E => self.i += self.v[x as usize] as u16,
                0x29 => self.i = self.v[x as usize] as u16 * 5,
                0x33 => {
                    self.memory[self.i as usize] = self.v[x as usize] / 100;
                    self.memory[self.i as usize + 1] = (self.v[x as usize] / 10) % 10;
                    self.memory[self.i as usize + 2] = self.v[x as usize] % 10;
                }
                0x55 => {
                    for i in 0..=x {
                        self.memory[self.i as usize + i as usize] = self.v[i as usize];
                    }
                }
                0x65 => {
                    for i in 0..=x {
                        self.v[i as usize] = self.memory[self.i as usize + i as usize];
                    }
                }
                _ => panic!("Unknown opcode: {:#X}", opcode),
            },
            _ => panic!("Unknown opcode: {:#X}", opcode),
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
    }
}
