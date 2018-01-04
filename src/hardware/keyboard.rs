use sdl2::keyboard::Keycode;

pub struct Keyboard {
    pub keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [false; 16]
        }
    }

    pub fn set_key_status(&mut self, key: Keycode, status: bool) {
        match key {
            Keycode::Num1 => self.keys[0x1] = status,
            Keycode::Num2 => self.keys[0x2] = status,
            Keycode::Num3 => self.keys[0x3] = status,
            Keycode::Num4 => self.keys[0xC] = status,
            Keycode::Q => self.keys[0x4] = status,
            Keycode::W => self.keys[0x5] = status,
            Keycode::E => self.keys[0x6] = status,
            Keycode::R => self.keys[0xd] = status,
            Keycode::A => self.keys[0x7] = status,
            Keycode::S => self.keys[0x8] = status,
            Keycode::D => self.keys[0x9] = status,
            Keycode::F => self.keys[0xe] = status,
            Keycode::Z => self.keys[0xa] = status,
            Keycode::X => self.keys[0x0] = status,
            Keycode::C => self.keys[0xb] = status,
            Keycode::V => self.keys[0xf] = status,
            _ => {}
        }
    }
}