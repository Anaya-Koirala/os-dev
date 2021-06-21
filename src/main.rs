#![no_std]
#![no_main]

mod console;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info:&PanicInfo)->!{
    
    console::print("Error! Kernel Panic",console::BackgroundColor::Black,console::BackgroundColor::White);
    loop{}
}




#[no_mangle]
pub extern "C" fn _start() -> ! {
    console::print("Hello World",console::BackgroundColor::Black,console::BackgroundColor::White);
    console::print("Hello",console::BackgroundColor::Black,console::BackgroundColor::White);
    loop {}
}

