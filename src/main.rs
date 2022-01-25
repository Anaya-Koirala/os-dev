#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga_buffer;


#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World\n");
    println!("how are you doing ? ");
    green_print!("Success\n");
    red_print!("Faliure\n");
    yellow_print!("Warning\n");
    loop {}
}
#[panic_handler]
fn panic(_info:&PanicInfo) -> ! {
    red_print!("{}",_info);
    loop {} 
}

