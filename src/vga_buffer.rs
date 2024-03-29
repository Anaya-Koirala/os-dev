use core::fmt;
use lazy_static::lazy_static;
use volatile::Volatile;
use spin::Mutex;


lazy_static!{
    pub static ref BLUE: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightBlue, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

lazy_static!{
    pub static ref CYAN: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Cyan, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

lazy_static!{
    pub static ref PINK: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Pink, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

lazy_static!{   
    pub static ref MAGENTA: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Magenta, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

lazy_static!{
    pub static ref BROWN: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Brown, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}


lazy_static!{
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::LightGray, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

lazy_static!{
    pub static ref RED: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Red, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

lazy_static!{
    pub static ref GREEN: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Green, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

lazy_static!{
    pub static ref YELLOW: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

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
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }
    fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}


#[macro_export]
macro_rules! error {
    
    ($($arg:tt)*) => ($crate::vga_buffer::_error(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _error(args: fmt::Arguments){
    use core::fmt::Write;
    RED.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! success {
    
    ($($arg:tt)*) => ($crate::vga_buffer::_success(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _success(args: fmt::Arguments){
    use core::fmt::Write;
    GREEN.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! warn {
    
    ($($arg:tt)*) => ($crate::vga_buffer::warn(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _warn(args: fmt::Arguments){
    use core::fmt::Write;
    YELLOW.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! blue_print {
    
    ($($arg:tt)*) => ($crate::vga_buffer::_blue_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _blue_print(args: fmt::Arguments){
    use core::fmt::Write;
    BLUE.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! cyan_print {
    
    ($($arg:tt)*) => ($crate::vga_buffer::_cyan_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _cyan_print(args: fmt::Arguments){
    use core::fmt::Write;
    CYAN.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! magenta_print {
    
    ($($arg:tt)*) => ($crate::vga_buffer::_magenta_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _magenta_print(args: fmt::Arguments){
    use core::fmt::Write;
    MAGENTA.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! brown_print {
    
    ($($arg:tt)*) => ($crate::vga_buffer::_brown_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _brown_print(args: fmt::Arguments){
    use core::fmt::Write;
    BROWN.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! pink_print {
    
    ($($arg:tt)*) => ($crate::vga_buffer::_pink_print(format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _pink_print(args: fmt::Arguments){
    use core::fmt::Write;
    PINK.lock().write_fmt(args).unwrap();
}