// keyboard and joystick support
extern crate minifb;

use minifb::*;
use c64::cia;

/*
 C64 keyboard map:

 Bit |    7      6   5     4      3     2        1       0
-----+------------------------------------------------------
  7  | RUNSTOP   Q   C=  SPACE    2    CTRL     <-       1
  6  |    /      ^   =   RSHIFT  HOME   ;       *      POUND
  5  |    ,      @   "     .      -     L       P        +
  4  |    N      O   K     M      0     J       I        9
  3  |    V      U   H     B      8     G       Y        7
  2  |    X      T   F     C      6     D       R        5
  1  |  LSHIFT   E   S     Z      4     A       W        3
  0  |  CRSR-DN  F5  F3    F1     F7  CRSR-RT  RETURN  DELETE
*/

pub struct IO {
    keyboard_state: [bool; 0xFF], // key states, including shift presses
    joystick_state: [bool; 0x0A], // 9 directions (num-pad) + 1 fire button
    joy_port1: bool,  // is joystick plugged to port 1?
}

impl IO {
    pub fn new() -> IO {
        IO {
            keyboard_state: [false; 0xFF],
            joystick_state: [false; 0x0A],
            joy_port1: false
        }
    }


    pub fn update(&mut self, window: &Window, cia1: &mut cia::CIAShared) {
        // keyboard processing
        // iterating over all keys is crawling-slow, so check individual keys
        self.process_key(window.is_key_down(Key::Key0), Key::Key0, cia1);
        self.process_key(window.is_key_down(Key::Key1), Key::Key1, cia1);
        self.process_key(window.is_key_down(Key::Key2), Key::Key2, cia1);
        self.process_key(window.is_key_down(Key::Key3), Key::Key3, cia1);
        self.process_key(window.is_key_down(Key::Key4), Key::Key4, cia1);
        self.process_key(window.is_key_down(Key::Key5), Key::Key5, cia1);
        self.process_key(window.is_key_down(Key::Key6), Key::Key6, cia1);
        self.process_key(window.is_key_down(Key::Key7), Key::Key7, cia1);
        self.process_key(window.is_key_down(Key::Key8), Key::Key8, cia1);
        self.process_key(window.is_key_down(Key::Key9), Key::Key9, cia1);

        self.process_key(window.is_key_down(Key::A), Key::A, cia1);
        self.process_key(window.is_key_down(Key::B), Key::B, cia1);
        self.process_key(window.is_key_down(Key::C), Key::C, cia1);
        self.process_key(window.is_key_down(Key::D), Key::D, cia1);
        self.process_key(window.is_key_down(Key::E), Key::E, cia1);
        self.process_key(window.is_key_down(Key::F), Key::F, cia1);
        self.process_key(window.is_key_down(Key::G), Key::G, cia1);
        self.process_key(window.is_key_down(Key::H), Key::H, cia1);
        self.process_key(window.is_key_down(Key::I), Key::I, cia1);
        self.process_key(window.is_key_down(Key::J), Key::J, cia1);
        self.process_key(window.is_key_down(Key::K), Key::K, cia1);
        self.process_key(window.is_key_down(Key::L), Key::L, cia1);
        self.process_key(window.is_key_down(Key::M), Key::M, cia1);
        self.process_key(window.is_key_down(Key::N), Key::N, cia1);
        self.process_key(window.is_key_down(Key::O), Key::O, cia1);
        self.process_key(window.is_key_down(Key::P), Key::P, cia1);
        self.process_key(window.is_key_down(Key::Q), Key::Q, cia1);
        self.process_key(window.is_key_down(Key::R), Key::R, cia1);
        self.process_key(window.is_key_down(Key::S), Key::S, cia1);
        self.process_key(window.is_key_down(Key::T), Key::T, cia1);
        self.process_key(window.is_key_down(Key::U), Key::U, cia1);
        self.process_key(window.is_key_down(Key::V), Key::V, cia1);
        self.process_key(window.is_key_down(Key::W), Key::W, cia1);
        self.process_key(window.is_key_down(Key::X), Key::X, cia1);
        self.process_key(window.is_key_down(Key::Y), Key::Y, cia1);
        self.process_key(window.is_key_down(Key::Z), Key::Z, cia1);
        self.process_key(window.is_key_down(Key::F1), Key::F1, cia1);
        self.process_key(window.is_key_down(Key::F2), Key::F2, cia1);
        self.process_key(window.is_key_down(Key::F3), Key::F3, cia1);
        self.process_key(window.is_key_down(Key::F4), Key::F4, cia1);
        self.process_key(window.is_key_down(Key::F5), Key::F5, cia1);
        self.process_key(window.is_key_down(Key::F6), Key::F6, cia1);
        self.process_key(window.is_key_down(Key::F7), Key::F7, cia1);
        self.process_key(window.is_key_down(Key::F8), Key::F8, cia1);

        self.process_key(window.is_key_down(Key::Down),  Key::Down,  cia1);
        self.process_key(window.is_key_down(Key::Up),    Key::Up,    cia1);
        self.process_key(window.is_key_down(Key::Right), Key::Right, cia1);
        self.process_key(window.is_key_down(Key::Left),  Key::Left,  cia1);
        self.process_key(window.is_key_down(Key::Space),  Key::Space,  cia1);
        self.process_key(window.is_key_down(Key::Comma),  Key::Comma,  cia1);
        self.process_key(window.is_key_down(Key::Period), Key::Period, cia1);
        self.process_key(window.is_key_down(Key::Slash),  Key::Slash,  cia1);
        self.process_key(window.is_key_down(Key::Enter),  Key::Enter,  cia1);
        self.process_key(window.is_key_down(Key::Backspace),  Key::Backspace,  cia1);
        self.process_key(window.is_key_down(Key::Backquote),  Key::Backquote,  cia1);
        self.process_key(window.is_key_down(Key::LeftShift),  Key::LeftShift,  cia1);
        self.process_key(window.is_key_down(Key::RightShift), Key::RightShift, cia1);
        self.process_key(window.is_key_down(Key::Escape),     Key::Escape,     cia1);
        self.process_key(window.is_key_down(Key::Minus),  Key::Minus,  cia1);
        self.process_key(window.is_key_down(Key::Equal),  Key::Equal,  cia1);
        self.process_key(window.is_key_down(Key::Insert), Key::Insert, cia1);
        self.process_key(window.is_key_down(Key::Home),   Key::Home,   cia1);
        self.process_key(window.is_key_down(Key::LeftBracket),  Key::LeftBracket,  cia1);
        self.process_key(window.is_key_down(Key::RightBracket), Key::RightBracket, cia1);
        self.process_key(window.is_key_down(Key::Delete),     Key::Delete,     cia1);
        self.process_key(window.is_key_down(Key::Semicolon),  Key::Semicolon,  cia1);
        self.process_key(window.is_key_down(Key::Apostrophe), Key::Apostrophe, cia1);
        self.process_key(window.is_key_down(Key::Backslash),  Key::Backslash,  cia1);
        self.process_key(window.is_key_down(Key::Tab),        Key::Tab,        cia1);
        self.process_key(window.is_key_down(Key::LeftCtrl),   Key::LeftCtrl,   cia1);

        // joystick processing
        self.process_joystick(window.is_key_down(Key::NumPad1), Key::NumPad1, cia1);
        self.process_joystick(window.is_key_down(Key::NumPad2), Key::NumPad2, cia1);
        self.process_joystick(window.is_key_down(Key::NumPad3), Key::NumPad3, cia1);
        self.process_joystick(window.is_key_down(Key::NumPad4), Key::NumPad4, cia1);
        self.process_joystick(window.is_key_down(Key::NumPad5), Key::NumPad5, cia1);
        self.process_joystick(window.is_key_down(Key::NumPad6), Key::NumPad6, cia1);
        self.process_joystick(window.is_key_down(Key::NumPad7), Key::NumPad7, cia1);
        self.process_joystick(window.is_key_down(Key::NumPad8), Key::NumPad8, cia1);
        self.process_joystick(window.is_key_down(Key::NumPad9), Key::NumPad9, cia1);
        self.process_joystick(window.is_key_down(Key::NumPad0), Key::NumPad0, cia1);

        // helper keys
        // toggle between joystick ports 1 and 2
        if window.is_key_pressed(Key::NumLock, KeyRepeat::No) {
            self.joy_port1 = !self.joy_port1;

            if self.joy_port1 {
                cia1.borrow_mut().joystick_2 = 0xFF;
            }
            else {
                cia1.borrow_mut().joystick_1 = 0xFF;
            }
            
            println!("Using joystick in port {}", if self.joy_port1 { "1" } else { "2" });
        }
    }


    pub fn check_restore_key(&self, window: &Window) -> bool {
        // End will serve as the Restore key
        window.is_key_pressed(Key::End, KeyRepeat::No)
    }


    // *** private functions *** //

    fn process_key(&mut self, key_pressed: bool, keycode: Key, cia1: &mut cia::CIAShared) {   
        if key_pressed {
            self.on_key_press(keycode, cia1);
        }
        else {
            self.on_key_release(keycode, cia1);
        }
    }    


    fn on_key_press(&mut self, keycode: Key, cia1: &mut cia::CIAShared) {
        let c64_keycode = self.keycode_to_c64(keycode);

        if self.keyboard_state[c64_keycode as usize] || c64_keycode == 0xFF
        {
            return
        }

        self.keyboard_state[c64_keycode as usize] = true;

        let c64_bit  = c64_keycode & 7;
        let c64_byte = (c64_keycode >> 3) & 7;
        
        // key is shifted?
        if (c64_keycode & 0x80) != 0
        {
            cia1.borrow_mut().key_matrix[6] &= 0xEF;
            cia1.borrow_mut().rev_matrix[4] &= 0xBF;
        }
        
        cia1.borrow_mut().key_matrix[c64_byte as usize] &= !(1 << c64_bit);
        cia1.borrow_mut().rev_matrix[c64_bit as usize]  &= !(1 << c64_byte);
    }


    fn on_key_release(&mut self, keycode: Key, cia1: &mut cia::CIAShared) {
        let c64_keycode = self.keycode_to_c64(keycode);

        if !self.keyboard_state[c64_keycode as usize] || c64_keycode == 0xFF {
            return;
        }
        
        self.keyboard_state[c64_keycode as usize] = false;

        let c64_bit  = c64_keycode & 7;
        let c64_byte = (c64_keycode >> 3) & 7;
        
        // key is shifted?
        if (c64_keycode & 0x80) != 0 {
            cia1.borrow_mut().key_matrix[6] |= 0x10;
            cia1.borrow_mut().rev_matrix[4] |= 0x40;
        }
        
        cia1.borrow_mut().key_matrix[c64_byte as usize] |= 1 << c64_bit;
        cia1.borrow_mut().rev_matrix[c64_bit as usize]  |= 1 << c64_byte;
    }


    fn process_joystick(&mut self, key_pressed: bool, keycode: Key, cia1: &mut cia::CIAShared) {
        if key_pressed {
            self.on_joy_press(keycode, cia1);
        }
        else {
            self.on_joy_release(keycode, cia1);
        }
    }


    fn on_joy_press(&mut self, keycode: Key, cia1: &mut cia::CIAShared) {
        let mut joystate = if self.joy_port1 { cia1.borrow_mut().joystick_1 } else { cia1.borrow_mut().joystick_2 };

        match keycode {
            // down-left
            Key::NumPad1 => { self.joystick_state[0] = true; joystate = (joystate | 0x09) & !0x06; },
            // down
            Key::NumPad2 => { self.joystick_state[1] = true; joystate = (joystate | 0x01) & !0x02; },
            // down-right
            Key::NumPad3 => { self.joystick_state[2] = true; joystate = (joystate | 0x05) & !0x0A; },
            // left
            Key::NumPad4 => { self.joystick_state[3] = true; joystate = (joystate | 0x08) & !0x04; },
            // center
            Key::NumPad5 => { self.joystick_state[4] = true; joystate |= 0x0F; },
            // right
            Key::NumPad6 => { self.joystick_state[5] = true; joystate = (joystate | 0x04) & !0x08; },
            // up-left
            Key::NumPad7 => { self.joystick_state[6] = true; joystate = (joystate | 0x0A) & !0x05; },
            // up
            Key::NumPad8 => { self.joystick_state[7] = true; joystate = (joystate | 0x02) & !0x01; },
            // up-right
            Key::NumPad9 => { self.joystick_state[8] = true; joystate = (joystate | 0x06) & !0x09; },
            // fire button
            Key::NumPad0 => { self.joystick_state[9] = true; joystate &= !0x10; },
            _ => (),
        }

        if self.joy_port1 {
            cia1.borrow_mut().joystick_1 = joystate;
        }
        else {
            cia1.borrow_mut().joystick_2 = joystate;
        }
    }


    fn on_joy_release(&mut self, keycode: Key, cia1: &mut cia::CIAShared) {
        let mut joystate = if self.joy_port1 { cia1.borrow_mut().joystick_1 } else { cia1.borrow_mut().joystick_2 };

        if joystate == 0xFF {
            return;
        }
        
        match keycode {
            // down-left
            Key::NumPad1 => if self.joystick_state[0] { joystate |= 0x06; self.joystick_state[0] = false; },
            // down
            Key::NumPad2 => if self.joystick_state[1] { joystate |= 0x02; self.joystick_state[1] = false; },
            // down-right
            Key::NumPad3 => if self.joystick_state[2] { joystate |= 0x0A; self.joystick_state[2] = false; },
            // left
            Key::NumPad4 => if self.joystick_state[3] { joystate |= 0x04; self.joystick_state[3] = false; },
            // center
            Key::NumPad5 => self.joystick_state[4] = false,
            // right
            Key::NumPad6 => if self.joystick_state[5] { joystate |= 0x08; self.joystick_state[5] = false; },
            // up-left
            Key::NumPad7 => if self.joystick_state[6] { joystate |= 0x05; self.joystick_state[6] = false; },
            // up
            Key::NumPad8 => if self.joystick_state[7] { joystate |= 0x01; self.joystick_state[7] = false; },
            // up-right
            Key::NumPad9 => if self.joystick_state[8] { joystate |= 0x09; self.joystick_state[8] = false; },
            // fire button
            Key::NumPad0 => if self.joystick_state[9] { joystate |= 0x10; self.joystick_state[9] = false; },
            _ => (),
        }

        if self.joy_port1 {
            cia1.borrow_mut().joystick_1 = joystate;
            cia1.borrow_mut().joystick_2 = 0xFF;
        }
        else {
            cia1.borrow_mut().joystick_1 = 0xFF;
            cia1.borrow_mut().joystick_2 = joystate;
        }
    }


    fn keycode_to_c64(&self, keycode: Key) -> u8 {
        // fetch key's bit combination as represented in C64 keyboard matrix
        let to_c64 = |row_bit: u8, col_bit: u8| (row_bit << 3) | col_bit;
        
        match keycode {
            Key::Key0 => to_c64(4, 3),
            Key::Key1 => to_c64(7, 0),
            Key::Key2 => to_c64(7, 3),
            Key::Key3 => to_c64(1, 0),
            Key::Key4 => to_c64(1, 3),
            Key::Key5 => to_c64(2, 0),
            Key::Key6 => to_c64(2, 3),
            Key::Key7 => to_c64(3, 0),
            Key::Key8 => to_c64(3, 3),
            Key::Key9 => to_c64(4, 0),
            Key::A => to_c64(1, 2),
            Key::B => to_c64(3, 4),
            Key::C => to_c64(2, 4),
            Key::D => to_c64(2, 2),
            Key::E => to_c64(1, 6),
            Key::F => to_c64(2, 5),
            Key::G => to_c64(3, 2),
            Key::H => to_c64(3, 5),
            Key::I => to_c64(4, 1),
            Key::J => to_c64(4, 2),
            Key::K => to_c64(4, 5),
            Key::L => to_c64(5, 2),
            Key::M => to_c64(4, 4),
            Key::N => to_c64(4, 7),
            Key::O => to_c64(4, 6),
            Key::P => to_c64(5, 1),
            Key::Q => to_c64(7, 6),
            Key::R => to_c64(2, 1),
            Key::S => to_c64(1, 5),
            Key::T => to_c64(2, 6),
            Key::U => to_c64(3, 6),
            Key::V => to_c64(3, 7),
            Key::W => to_c64(1, 1),
            Key::X => to_c64(2, 7),
            Key::Y => to_c64(3, 1),
            Key::Z => to_c64(1, 4),
            Key::F1 => to_c64(0, 4),
            Key::F2 => to_c64(0, 4) | 0x80,
            Key::F3 => to_c64(0, 5),
            Key::F4 => to_c64(0, 5) | 0x80,
            Key::F5 => to_c64(0, 6),
            Key::F6 => to_c64(0, 6) | 0x80,
            Key::F7 => to_c64(0, 3),
            Key::F8 => to_c64(0, 3) | 0x80,
            Key::Down   => to_c64(0, 7),
            Key::Up     => to_c64(0, 7) | 0x80,
            Key::Right  => to_c64(0, 2),
            Key::Left   => to_c64(0, 2) | 0x80,
            Key::Space  => to_c64(7, 4),
            Key::Comma  => to_c64(5, 7),
            Key::Period => to_c64(5, 4),
            Key::Slash  => to_c64(6, 7),
            Key::Enter     => to_c64(0, 1),
            Key::Backspace => to_c64(0, 0),
            // Left arrow key
            Key::Backquote  => to_c64(7, 1),
            Key::LeftShift  => to_c64(1, 7),
            Key::RightShift => to_c64(6, 4),
            // Run Stop key
            Key::Escape => to_c64(7, 7),
            // Plus key
            Key::Minus  => to_c64(5, 0),
            // Minus key
            Key::Equal  => to_c64(5, 3),
            // Pound key
            Key::Insert => to_c64(6, 0),
            // CLR/Home key
            Key::Home   => to_c64(6, 3),
            // Home key
            Key::Delete => to_c64(6, 6),
            // @ key
            Key::LeftBracket  => to_c64(5, 6),
            // * key
            Key::RightBracket => to_c64(6, 1),
            // Colon key
            Key::Semicolon  => to_c64(5, 5),
            // Semicolon key
            Key::Apostrophe => to_c64(6, 2),
            // Equals key
            Key::Backslash => to_c64(6, 5),
            // Control key
            Key::Tab => to_c64(7, 2),
            // Commodore key
            Key::LeftCtrl => to_c64(7, 5),
            // unknown
            _ => 0xFF
        }
    }
}
