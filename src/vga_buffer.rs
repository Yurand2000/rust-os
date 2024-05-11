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

impl Color {
    fn to_u8(color: Self, bg_color: Self, blinking: bool) -> u8 {
        (blinking as u8) << 7 | bg_color.background_color() | color.font_color()
    }

    #[inline(always)]
    fn font_color(self) -> u8 {
        self as u8
    }

    #[inline(always)]
    fn background_color(self) -> u8 {
        (self as u8 & 0x7) << 4
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
    ch: u8,
    color: u8,
}

impl ScreenChar {
    pub fn new(ch: u8, color: u8) -> Self {
        Self { ch, color }
    }
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

type VgaBuffer = [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT];

pub struct Writer {
    column_position: usize,
    color: u8,
    buffer: &'static mut VgaBuffer,
}

impl Writer {
    pub fn new() -> Self {
        Self {
            column_position: 0,
            color: Color::to_u8(Color::White, Color::Blue, false),
            buffer: unsafe { &mut *(0xb8000 as *mut VgaBuffer) },
        }
    }

    pub fn new_line(&mut self) {
        for row in 0..(BUFFER_HEIGHT - 1) {
            for col in 0..BUFFER_WIDTH {
                self.buffer[row][col] = self.buffer[row + 1][col];
            }
        }

        let last_row = BUFFER_HEIGHT - 1;
        let color = self.color;
        for col in 0..BUFFER_WIDTH {
            self.buffer[last_row][col] = ScreenChar::new(b' ', color);
        }

        self.column_position = 0;
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.column_position >= BUFFER_WIDTH {
                    self.column_position = 0;
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color = self.color;

                self.buffer[row][col] = ScreenChar::new(byte, color);
                self.column_position += 1;
            }
        }
    }

    pub fn write(&mut self, str: &str) {
        for byte in str.bytes() {
            match byte {
                 // printable ASCII byte or newline
                 0x20..=0x7e | b'\n' => self.write_byte(byte),
                 // not part of printable ASCII range
                 _ => self.write_byte(0xfe),
            }
        }
    }

    pub fn write_line(&mut self, str: &str) {
        self.new_line();
        self.write(str);
    }
}

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer::new());
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
pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}