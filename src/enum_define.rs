#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);
impl ColorCode {
    pub fn new(foreground_color: Color, background_color: Color) -> ColorCode {
        ColorCode((background_color as u8) << 4 | (foreground_color as u8))
    }
}


pub enum ControlKey {
	ESC,  F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12, PRTSCR,SCROLL,PAUSE,
	TAB,		                       BACKSPACE, INSERT, HOME,PAGEUP,
	CAPSLOCK,		                       ENTER, DELETE,END,PAGEDOWN,
	LSHIFT,          			          RSHIFT,		  UP,
	LCTRL,LALT,      SPACE,           RALT,RCTRL, 	LEFT,DOWN,RIGHT,
	
}
pub enum Key{
	Character(u8),
	Control(ControlKey),
	Nothing
}
pub struct KeyBoardMapper {
	caps_lock : bool,
	lowercase_keys:&'static [u8],
	uppercase_keys:&'static [u8],
	hold_shift: bool,
	extend: bool
}
impl KeyBoardMapper {
	pub fn new() -> Self {
		KeyBoardMapper { 
			caps_lock: false,
			lowercase_keys: "__1234567890-=__qwertyuiop[]__asdfghjkl;'`_\\zxcvbnm,./_*_ ________________-_5_+_________________________________________________".as_bytes(),
			uppercase_keys: "__!@#$%^&*()_+__QWERTYUIOP{}__ASDFGHJKL:\"~_|ZXCVBNM<>?_*_ ________________-_5_+_________________________________________________".as_bytes(),
			hold_shift : false,
			extend: false

		}
	}
	pub fn scancode_to_char(&mut self,scancode : u8) -> Key {
		
		match scancode {
			0x01 => Key::Control(ControlKey::ESC),
			0x0e => Key::Character(8),// \b不识别
			0x0f => Key::Character(b'\t'),
			0x3a => {
				self.caps_lock =  !self.caps_lock;
				Key::Nothing
			}
			0x1c => Key::Character(b'\n'),
			0x2a | 0x36 => {
				self.hold_shift = true;
				Key::Nothing
			},
			0xaa | 0xb6 => {
				self.hold_shift = false;
				Key::Nothing
			},
			0x1d => {
				// CTRL
				if self.extend {
					self.extend =false;
					Key::Control(ControlKey::RCTRL)
				}
				else {
					self.extend =false;
					Key::Control(ControlKey::LCTRL)
				}
			},
			0x38 => {
				if self.extend {
					self.extend =false;
					Key::Control(ControlKey::RALT)
				}
				else {
					self.extend =false;
					Key::Control(ControlKey::LALT)
				}

			},
			0xe0 => {
				self.extend = true;
				Key::Nothing
			},
			0x3b..=0x44 | 0x57 | 0x58 => {
				// Fn
				Key::Nothing
			},
			0x39 => Key::Character(b' '),
			_ => {
				if scancode >= 128 {
					Key::Nothing
				}
				else if self.caps_lock ^ self.hold_shift {
					Key::Character(	self.uppercase_keys[(scancode % 128) as usize])
				}
				else {
					Key::Character(	self.lowercase_keys[(scancode % 128) as usize])
				}
			}
		}
		
	}
	
}

