//support Rust's formatting macros, like integers
use core::fmt::{Write, Result, Arguments};
use lazy_static::lazy_static;
use spin::Mutex;
use volatile::Volatile;
//This attribute disables warnings isuued by the compiler for any unused variant in the Color enum 
#[allow(dead_code)]
//This attribute enables copy semantics for the type and make it printable and comparable
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//This attribute makes each enum variant stored as an u8
#[repr(u8)]
//use C-like enum to explicitly specify the number for each color
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
//ensures that the ColorCode has the exact same data layout as an u8
#[repr(transparent)]
//the ColorCode struct contains the full color byte, containing foreground and background colors
struct ColorCode(u8);

//implement a full ColorCode that specifies foreground and background color
impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
//This attribute guarantees that the struct's fields are laid out exactly like in a C struct and
//thus guarantees the correct field ordering
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

//use the attribute to ensure that it has the same memory layout as its single field
#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

//create the Writer type 
pub struct Writer {
    //the column_position field keeps track of the current position in the last row
    column_position: usize,
    //the current foreground and background colors are specifies by color_code
    color_code: ColorCode,
    //a reference to the VGA buffer is stored in Buffer,the 'static lifetime specifies that the
    //reference is valid for the whole program run time
    buffer: &'static mut Buffer,
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Blue, Color::Yellow),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}

impl Writer {
    //first, create a method to write a single ascii byte
    pub fn write_byte(&mut self, byte:u8) {
        match byte {
            //if the byte is \n, the Writer does not print anything, instead it calls new_line
            //method
            b'\n' => self.new_line(),
            byte => {
                //the Writer checks if the current line is full, in that case, call new_line method
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;

                //write a new ScreenChar to the Buffer at the current position
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code,
                });

                //advance the current column_position by 1
                self.column_position += 1;
            }
        }
    }

    //create a method to print whole strings
    fn write_string(&mut self, s: &str){
        //convert the string to bytes and print them one-by-one
        for byte in s.bytes() {
            //use a match to differentiate printable ASCII bytes and unprintable bytes
            match byte {
                //between the space character and the ~ character or the newline \n are printable
                //ASCII bytes
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                //for unprintable bytes, the Writer prints the character whose hex code is 0xfe on
                //the VGA hardware
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


//implement the core::fmt::Write trait. The only required method of this trait is write_str, so
//as to support different types, like integers or floats
impl Write for Writer {
    fn write_str(&mut self, s: &str) -> Result {
        self.write_string(s);
        //Ok containing the () type.
        Ok(())
    }
}

/*
pub fn print_something() {
    //create a new Writer
    let mut writer = Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Blue, Color::Yellow),
        //an unsafe block, since the compiler can’t guarantee that the raw pointer is valid.
        buffer: unsafe {
            //First, we cast the integer 0xb8000 as a mutable raw pointer. Then we convert it to a mutable reference by dereferencing it (through *) and immediately borrowing it again (through &mut). 
            &mut *(0xb8000 as *mut Buffer)
        },
    };

    //write the byte b'T', the b prefix creates a byte literal, which represents an ASCII character.
    writer.write_string("This is a ");
    writer.write_string("minircore!");
    writer.write_string("GWU");
    write!(writer, "Yue Lei's test").unwrap();
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}
*/

// ----------------------------------------------------------------------------------

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
pub fn _print(args: Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}

