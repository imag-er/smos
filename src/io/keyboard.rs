


#[derive(Debug)]
pub enum ControlKey {
	ESC,  F1,F2,F3,F4,F5,F6,F7,F8,F9,F10,F11,F12, PRTSCR,SCROLL,PAUSE,
	TAB,		                       BACKSPACE, INSERT, HOME,PAGEUP,
	CAPSLOCK,		                       ENTER, DELETE,END,PAGEDOWN,
	LSHIFT,          			          RSHIFT,		  UP,
	LCTRL,LALT,      SPACE,           RALT,RCTRL, 	LEFT,DOWN,RIGHT,
	
	_Undefined
}
#[derive(Debug)]
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
	hold_ctrl: bool,
	hold_alt:bool,
	extend: bool
}
impl KeyBoardMapper {
	pub fn new() -> Self {
		KeyBoardMapper { 
			caps_lock: false,
			lowercase_keys: "__1234567890-=_\tqwertyuiop[]\n_asdfghjkl;'`_\\zxcvbnm,./_*_ ________________-_5_+_________________________________________________".as_bytes(),
			uppercase_keys: "__!@#$%^&*()_+_\tQWERTYUIOP{}\n_ASDFGHJKL:\"~_|ZXCVBNM<>?_*_ ________________-_5_+_________________________________________________".as_bytes(),
			hold_shift : false,
			hold_ctrl : false,
			hold_alt :false,
			extend: false

		}
	}
	pub fn scancode_to_char(&mut self,scancode : u8) -> Key {
		if self.extend {
			self.extend = false;
			match scancode {
				0x1d => return Key::Control(ControlKey::RCTRL),
				0x38 => return Key::Control(ControlKey::RALT),
				0x46 => return Key::Control(ControlKey::LEFT),
				0x4d => return Key::Control(ControlKey::RIGHT),
				0x48 => return Key::Control(ControlKey::UP),
				0x50 => return Key::Control(ControlKey::DOWN),
				0x52 => return Key::Control(ControlKey::INSERT),
				0x47 => return Key::Control(ControlKey::HOME),
				0x49 => return Key::Control(ControlKey::PAGEUP),
				0x53 => return Key::Control(ControlKey::DELETE),
				0x4F => return Key::Control(ControlKey::END),
				0x51 => return Key::Control(ControlKey::PAGEDOWN),				
				_ => return Key::Nothing
			}
		}
		// 有字符的返回字符 没字符的返回控制
		match scancode {
			0x01 => Key::Control(ControlKey::ESC),
			0x46 => Key::Control(ControlKey::SCROLL),
			0x1d => {
				self.hold_ctrl = true;
				Key::Control(ControlKey::LCTRL)
			},
			0x9d => {
				self.hold_ctrl = false;
				Key::Nothing
			},
			0x38 => {
				self.hold_alt = true;
				Key::Control(ControlKey::LALT)
			} ,
			0xb8 => {
				self.hold_alt = false;
				Key::Nothing
			} ,

			0x39 => Key::Character(b' '),
			0x0e => Key::Character(8),// \b不识别

			0x3a => {
				self.caps_lock = !self.caps_lock;
				Key::Control(ControlKey::CAPSLOCK)
			}
			0x1c => Key::Character(b'\n'),
			0x2a => {
				self.hold_shift = true;
				Key::Control(ControlKey::LSHIFT)
			},
			0x36 => {
				self.hold_shift = true;
				Key::Control(ControlKey::RSHIFT)
			},
			0xaa | 0xb6 => {
				self.hold_shift = false;
				Key::Nothing
			},
			0xe0 => {
				self.extend = true;
				Key::Nothing
			},
			0x3b..=0x44 | 0x57 | 0x58 => 
				match scancode {
					0x3b => Key::Control(ControlKey::F1),
					0x3c => Key::Control(ControlKey::F2),
					0x3d => Key::Control(ControlKey::F3),
					0x3e => Key::Control(ControlKey::F4),
					0x3f => Key::Control(ControlKey::F5),
					0x40 => Key::Control(ControlKey::F6),
					0x41 => Key::Control(ControlKey::F7),
					0x42 => Key::Control(ControlKey::F8),
					0x43 => Key::Control(ControlKey::F9),
					0x44 => Key::Control(ControlKey::F10),
					0x57 => Key::Control(ControlKey::F11),
					0x58 => Key::Control(ControlKey::F12),
					_ => Key::Nothing
				},
			_ => match scancode {
				128..=u8::MAX => Key::Nothing,
				_ if self.hold_alt | self.hold_ctrl => Key::Control(ControlKey::_Undefined),
				_ => {
					if self.caps_lock ^ self.hold_shift {
						Key::Character(	self.uppercase_keys[(scancode % 128) as usize])
					}
					else {
						Key::Character(	self.lowercase_keys[(scancode % 128) as usize])
					}
				} 
			}
			
			
			// 	else if self.caps_lock ^ self.hold_shift {
			// 		Key::Character(	self.uppercase_keys[(scancode % 128) as usize])
			// 	}
			// 	else {
			// 		Key::Character(	self.lowercase_keys[(scancode % 128) as usize])
			// 	}
			// }
		}
		
	}
	
}

