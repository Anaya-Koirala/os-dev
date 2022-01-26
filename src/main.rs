#![no_std]
#![no_main]

use core::panic::PanicInfo;
use os::{println,success,error,warn,blue_print,cyan_print,magenta_print,brown_print,pink_print};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World\n");
    os::init();
    blue_print!("Colors\n");
    cyan_print!("Are\n");
    magenta_print!("Cool\n");
    brown_print!("Woah\n");
    pink_print!("Zamn\n");
    loop{}

}
#[panic_handler]
fn panic(_info:&PanicInfo) -> ! {
    error!("{}",_info);
    loop {} 
}

