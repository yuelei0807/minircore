use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};
use crate::println;
use crate::print;
use lazy_static::lazy_static;
use crate::gdt;
use crate::hlt_loop;
use pic8259::ChainedPics;
use spin;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    //add Keyboard variant to the InterruptIndex enum
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

//-------------------------------------------------------------------------------------
//IDT
//-------------------------------------------------------------------------------------
//use the lazy_static to create the static IDT
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        //add a handler function for the page fault
        idt.page_fault.set_handler_fn(page_fault_handler);

        //set the stack index for our double fault handler in the IDT
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        //add a handler function for the timer interrupt
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        //add a handler function for the keyboard interrupt
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

//A double fault is a normal exception with an error code, specify a fouble_fault_handler function
extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

//create a breakpoint exception test
#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}

//-------------------------------------------------------------------------------------
//PIC
//-------------------------------------------------------------------------------------


pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

//the ChainedPics struct represents the primary/secondary PIC layout
pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");
    //The notify_end_of_interrupt figures out whether the primary or secondary PIC sent the interrupt
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    //print a k and send the end of interrupt signal to the interrupt controller
    //print!("k");
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;
    use x86_64::instructions::port::Port;

    //use the lazy_static macro to create a static Keyboard object protected by a Mutex
    lazy_static! {
        //initialize the Keyboard with a US keyboard layout and the scancode set 1
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = //use the Ignore option to handle the ctrl like normal keys
            Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
            );
    }

    let mut keyboard = KEYBOARD.lock();

    //read from the I/O port with the number 0x60
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    //On each interrupt, lock the Mutex, read the scancode from the keyboard controller, and pass it to the add_byte method, which translates the scancode into an Option<KeyEvent>
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        //pass the key_event to the process_keyevent method, which translates the key event to a character
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    //translates keypresses of the number keys 0-9 and ignores all other keys
    //let key = match scancode {
        //0x02 => Some('1'),
        //0x03 => Some('2'),
        //0x04 => Some('3'),
        //0x05 => Some('4'),
        //0x06 => Some('5'),
        //0x07 => Some('6'),
        //0x08 => Some('7'),
        //0x09 => Some('8'),
        //0x0a => Some('9'),
        //0x0b => Some('0'),
        //_ => None,
    //};
    //if let Some(key) = key {
    //    print!("{}", key);
    //}

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

//The PageFaultErrorCode type provides more information about the type of memory access that caused the page fault
extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame,error_code: PageFaultErrorCode,) {
    //The CR2 register is automatically set by the CPU on a page fault and contains the accessed virtual address that caused the page fault
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    //use the Cr2::read function of the x86_64 crate to read and print it
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    //avoid continuing execution without resolving the page fault
    hlt_loop();
}
