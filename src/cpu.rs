use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use resources;

extern crate rand;

pub struct CPU {
    opcode: u16,
    v: [u8; 16], // 16 8-bit registers: V0 => VF
    i: u16, 
    pc: usize,

    memory: [u8; 4096],
    pub screen: [[u8; 64]; 32],

    stack: [u16; 16],
    sp: u16,

    delay_timer: u8,
    sound_timer: u8,

    pub key_state: [bool; 16],
    prev_key_state: [bool; 16],
    waiting_for_key_press:bool
}

impl CPU {
    pub fn new() -> CPU {
        let mut cpu = CPU {
            opcode: 0,
            v: [0; 16],
            i: 0,
            pc: 0x200,

            memory: [0; 4096],
            screen: [[0; 64]; 32],

            stack: [0; 16],
            sp: 0,

            delay_timer: 0,
            sound_timer: 0,

            key_state: [false; 16],
            prev_key_state: [false; 16],
            waiting_for_key_press: false
        };

        for i in 0..resources::FONT.len() {
            cpu.memory[i] = resources::FONT[i];
        }

        return cpu;
    }

    pub fn load(&mut self, path: &Path) {
        let mut f = match File::open(&path) {
            Err(reason) => panic!("Failed to open {:?}: {}", path, reason),
            Ok(file) => file
        };

        // Fill memory starting at 0x200
        match f.read(&mut self.memory[0x200..]) {
            Err(reason) => panic!("Failed to read application memory: {}", reason),
            Ok(_) => println!("All done")
        }
    }

    pub fn cycle(&mut self) {

        self.opcode = (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc+1] as u16);

        //print!("{:04X} ", self.opcode);
        
        match self.opcode & 0xF000 {
            0x0000 => self.op_0xxx(),
            0x1000 => self.op_1xxx(),
            0x2000 => self.op_2xxx(),
            0x3000 => self.op_3xxx(),
            0x4000 => self.op_4xxx(),
            0x5000 => self.op_5xxx(),
            0x6000 => self.op_6xxx(),
            0x7000 => self.op_7xxx(),
            0x8000 => self.op_8xxx(),
            0x9000 => self.op_9xxx(),
            0xA000 => self.op_axxx(),
            0xB000 => self.op_bxxx(),
            0xC000 => self.op_cxxx(),
            0xD000 => self.op_dxxx(),
            0xE000 => self.op_exxx(),
            0xF000 => self.op_fxxx(),

            _ => {
                panic!("Undefined opcode: {:x}", self.opcode);
            }
        }

    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1; 
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1; 
        }
    }

    fn op_0xxx(&mut self) {
        match self.opcode & 0x00FF {

            // Clears the screen
            0xE0 => {
                //println!("CLS");
                self.screen = [[0; 64]; 32];
            },

            // Returns from a subroutine
            0xEE => {
                //println!("RTS");
                self.sp = self.sp.wrapping_sub(1);
                self.pc = self.stack[self.sp as usize] as usize;
            },
            _ => panic!("Unknown 0x00: {}", self.memory[self.pc+1])
        }
        
        self.pc += 2;
    }

    // JMP $NNN
    fn op_1xxx(&mut self) {
        //println!("JUMP ${:03x}", self.op_nnn());
        self.pc = self.op_nnn() as usize;
    }

    // Call subroutine at NNN
    fn op_2xxx(&mut self) {
        //println!("CALL ${:03x}", self.op_nnn());

        self.stack[self.sp as usize] = self.pc as u16;
        self.sp = self.sp.wrapping_add(1);
        self.pc = self.op_nnn() as usize;
    }

    // if VX == NN skip next instruction
    fn op_3xxx(&mut self) {
        //println!("SKIP.EQ V{:01X},#${:02x}", self.op_x(), self.op_nn());
        if self.v[self.op_x()] == self.op_nn() {
            self.pc += 2;
        }
        self.pc += 2;
    }

    // if VX != NN skip next instruction
    fn op_4xxx(&mut self) {
        //println!("SKIP.NE V{:01X},#${:02x}", self.op_x(), self.op_nn());

        if self.v[self.op_x()] != self.op_nn() {
            self.pc += 2;
        }
        self.pc += 2;
    }

    // if VX == VY skip next instruction
    fn op_5xxx(&mut self) {
        //println!("SKIP.EQ V{:01X},#${:02x}", self.op_x(), self.op_y());

        if self.v[self.op_x()] == self.v[self.op_y()] {
            self.pc += 2;
        }
        self.pc += 2;
    }

    // SET VX = NN
    fn op_6xxx(&mut self) {
        //println!("MVI V{:01X},#${:02x}", self.op_x(), self.op_nn());
        self.v[self.op_x()] = self.op_nn();
        self.pc += 2;
    }

    // SET VX += NN
    fn op_7xxx(&mut self) {
        //println!("ADI V{:01X},#${:02X}", self.op_x(), self.op_nn());

        let vx = self.v[self.op_x()];
        let nn = self.op_nn();
        self.v[self.op_x()] = vx.wrapping_add(nn);
        self.pc += 2;
    }

    fn op_8xxx(&mut self) {
        let y = self.v[self.op_y()];
        match self.opcode & 0x000F {

            // VX = VY
            0 => self.v[self.op_x()] = y,

            // VX = VX | VY
            1 => self.v[self.op_x()] |= y,

            // VX = VX & VY
            2 => self.v[self.op_x()] &= y,

            // VX = VX ^ VY
            3 => self.v[self.op_x()] ^= y,

            // Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't.
            4 => {
                self.v[self.op_x()] = self.v[self.op_x()].wrapping_add(self.v[self.op_y()]);
                if self.v[self.op_x()] < self.v[self.op_y()] {
                    self.v[15] = 1;
                } else {
                    self.v[15] = 0;
                }
            },

            // VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.
            5 => {
                if self.v[self.op_y()] > self.v[self.op_x()] {
                    self.v[15] = 0;
                } else {
                    self.v[15] = 1;
                }
                self.v[self.op_x()] = self.v[self.op_x()].wrapping_sub(self.v[self.op_y()]);
            },

            // Shifts VY right by one and copies the result to VX. VF is set to the value of the least significant bit of VY before the shift.
            6 => {
                self.v[15] = self.v[self.op_x()] & 0x1;
                self.v[self.op_x()] >>= 1;
            },

            7 => {
                if self.v[self.op_x()] > self.v[self.op_y()] {
                    self.v[15] = 0;
                } else {
                    self.v[15] = 1;
                }
                self.v[self.op_x()] = self.v[self.op_y()].wrapping_sub(self.v[self.op_x()]);
            },
            0xE => {
                self.v[15] = self.v[self.op_x()] >> 7;
                self.v[self.op_x()] <<= 1;
            },

            _ => panic!("Unrecognized 0x08 opcode: {:X}", self.opcode)

        }

        self.pc += 2;
    }

    // if VX != VY skip next instruction
    fn op_9xxx(&mut self) {
        if self.v[self.op_x()] != self.v[self.op_y()] {
            self.pc += 2;
        }
        self.pc += 2;
    }

    // I = NNN
    fn op_axxx(&mut self) {
        //println!("MVI I,#${:03X}", self.op_nnn());
        self.i = self.op_nnn();
        self.pc += 2;
    }

    // pc = V0 + NNN
    fn op_bxxx(&mut self) {
        self.pc = (self.op_nnn() + (self.v[0] as u16)) as usize;
    }

    // VX = rand() & NN
    fn op_cxxx(&mut self) {
        //println!("RNDMSK V{:01X},#${:02X}", self.op_x(), self.op_nn());

        let rand = rand::random::<u8>();
        self.v[self.op_x()] = rand & self.op_nn();
        self.pc += 2;
    }

    // Draw sprite at coordinate VX, VY with a width of 8xN pixels
    fn op_dxxx(&mut self) {

        //println!("spRITE V{:01X},V{:01X},#${:01x}", self.memory[self.pc] & 0xF, self.memory[self.pc+1] >> 4, self.memory[self.pc+1] & 0xF);

        let x = self.v[self.op_x()];
        let y = self.v[self.op_y()];
        let from = self.i as usize;
        let to = from + (self.op_n() as usize);

        let sprite = &self.memory[from..to];
        let mut collision:u8 = 0;

        for y2 in 0..sprite.len() {
            for x2 in 0..8 {
                let py = ((y as usize + y2) % 32) as usize;
                let px = ((x as usize + x2) % 64) as usize;

                if (sprite[y2] & (0x80 >> x2)) != 0 {
                    if self.screen[py][px] == 1 { 
                        collision = 1;
                    }
                    self.screen[py][px] ^= 1;
                }
            }
        }
        
        self.v[15] = collision;
        self.pc += 2;
    }

    fn op_exxx(&mut self) {
        let vx = self.v[self.op_x()] as usize;
        match self.memory[self.pc+1] {

            // Skips the next instruction if the key stored in VX is pressed.
            0x9E => {

                if self.key_state[vx] {
                    self.pc += 2;
                }
            },

            // Skips the next instruction if the key stored in VX isn't pressed.
            0xA1 => {

                if !self.key_state[vx] {
                    self.pc += 2;
                }

            },
            _ => {
                panic!("Undefined 0x0E opcode: {}", self.memory[self.pc+1]);
            }
        }

        self.pc += 2;
    }

    fn op_fxxx(&mut self) {
        match self.opcode & 0x00FF {

            // Sets VX to the value of the delay timer.
            0x07 => {
                //println!("MOV V{:01X},DELAY", self.op_x());
                self.v[self.op_x()] = self.delay_timer;
            },

            // A key press is awaited, and then stored in VX. (Blocking Operation. All instruction halted until next key event)
            0x0A => {

                if !self.waiting_for_key_press {

                    // Mark the CPU as waiting on a keypress
                    self.waiting_for_key_press = true;
                    self.prev_key_state = self.key_state;

                } else {

                    // Check whether any of our key states have changed
                    for i in 0..16 {
                        if !self.prev_key_state[i] && self.key_state[i] {
                            self.waiting_for_key_press = false;
                            self.v[self.op_x()] = i as u8;
                            self.pc += 2;
                            return;
                        }

                        self.prev_key_state[i] = self.key_state[i];
                    }

                }

                // Skip incrementing the pc
                return;
            },

            // Sets the delay timer to VX.
            0x15 => {
                self.delay_timer = self.v[self.op_x()];
            },

            // Sets the sound timer to VX.
            0x18 => {
                self.sound_timer = self.v[self.op_x()];
            },

            // Adds VX to I.
            0x1E => {
                self.i += self.v[self.op_x()] as u16;
                //println!("ADI I,V{:01X}", self.op_x());
            },

            // Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.
            0x29 => {
                self.i = (self.v[self.op_x()] as u16) * 5;
            },

            // Stores the binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.)
            0x33 => {
                self.memory[(self.i as usize)] = self.v[self.op_x()] / 100;
                self.memory[(self.i as usize)+1] = (self.v[self.op_x()] / 10) % 10;
                self.memory[(self.i as usize)+2] = (self.v[self.op_x()] % 100) % 10;
            }

            // Stores V0 to VX (including VX) in memory starting at address I. I is increased by 1 for each value written.
            0x55 => {
                for i in 0..(self.op_x()+1) {
                    let index = (self.i as usize)+i;
                    self.memory[index] = self.v[i];
                }
                self.i += (self.op_x() as u16)+1;
            },

            // Fills V0 to VX (including VX) with values from memory starting at address I. I is increased by 1 for each value written.
            0x65 => {
                for i in 0..(self.op_x()+1) {
                    let index = (self.i as usize)+i;
                    self.v[i] = self.memory[index];
                }
                self.i += (self.op_x() as u16)+1;
            },

            _ => {
                panic!("Undefined 0x0F opcode: {:x}", self.opcode);
            }
        }

        self.pc += 2;
    }

    fn op_x(&mut self) -> usize { ((self.opcode & 0x0F00) >> 8) as usize }
    fn op_y(&mut self) -> usize { ((self.opcode & 0x00F0) >> 4) as usize }
    fn op_n(&mut self) -> u8 { (self.opcode & 0x000F) as u8 }
    fn op_nn(&mut self) -> u8 { (self.opcode & 0x00FF) as u8 }
    fn op_nnn(&mut self) -> u16 { self.opcode & 0x0FFF }

}