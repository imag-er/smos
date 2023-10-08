use core::fmt;

use crate::io::color::{ColorCode,Color};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const TABLE_WIDTH: usize = 4;

#[repr(transparent)]
struct Buffer {
    // 二维数组定义
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// 定义writer并实现接口
// 用于向buffer写入
pub struct Writer {
    row_position: usize,
    colume_position: usize,
    color_code: ColorCode,
    // vga buffer全局有效
    buffer: &'static mut Buffer,

}

impl Writer {
    // 写入byte
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\t' => self.colume_position = {
                let mut new_col_pos = ((self.colume_position / TABLE_WIDTH) + 1) * TABLE_WIDTH;
                if new_col_pos >= BUFFER_WIDTH {
                    new_col_pos = BUFFER_WIDTH -1 ;
                }
                new_col_pos
                
            },
            b'\n' => self.new_line(),
            b'\r' => self.colume_position = 0,
            8 => self.colume_position -= 1,// \b
            byte => {
                if self.colume_position >= BUFFER_WIDTH {
                    // 超出行宽度 新行
                    self.new_line();
                }
                // let row = BUFFER_HEIGHT - 1;
                let row = self.row_position;

                let col = self.colume_position;

                let color_code = self.color_code;

                self.buffer.chars[row][col] = ScreenChar {
                    ascii_character: byte,
                    color_code,
                };

                self.colume_position += 1;

                use crate::serial_print;
                serial_print!("\r-------------");
                serial_print!("{:?} {:?}", self.row_position, self.colume_position);
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // 在CodePage437定义内的字符
                0x20..=0x7E | b'\n' | b'\r' | b'\t' | 8 => self.write_byte(byte),
                // 其他字符 打印方块 0xfe
                _ => self.write_byte(0xfe),
            }
        }
    }
    fn new_line(&mut self) {
        
        if self.row_position == 24 {

            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {

                    self.buffer.chars[row - 1][col] = self.buffer.chars[row][col];
                    serial_println!("{} {}\r",row,col);
                }
            }
            
            for col in 0..BUFFER_WIDTH {
                self.buffer.chars[24][col] = ScreenChar {
                    ascii_character: b' ',
                    color_code: ColorCode::new(Color::White, Color::Black),
                };
            }

        } else {
            self.row_position += 1;
        }
        self.colume_position = 0;
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}
use lazy_static::lazy_static;
use spin::Mutex;

use crate::serial_println;

// lazy初始化writer
lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        colume_position: 0,
        row_position: 0,
        color_code: ColorCode::new(Color::White, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

// 定义print macro
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// 修复死锁
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });
}
