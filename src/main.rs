#![no_std]
#![no_main]
#![feature(asm, global_asm)]
#![feature(panic_info_message)]
#![feature(format_args_nl)]

mod boot;
mod mutex;
mod uart;
mod interface;
mod gpio;
mod print;
mod driver;

const MMIO_BASE: usize = 0xFE00_0000;

fn main() {
    let mut count = 0;
    let led = gpio::GpioPin::new(16);
    led.set_function(gpio::GpioFunction::OUTPUT);
    
    while count < 1000{
        println!("Hacky Hello World");
        led.set_high();
        wait(10000);
        led.set_low();
        wait(10000);        
    }
}

fn wait(dur : usize) {
    for _ in 0..dur{
        unsafe{
            asm!("");
        }
    }
}

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info : &PanicInfo) -> !{
    match _info.message(){
        Some(args) => println!("Kernel Panic! : {}", args),
        None => println!("Kernel Panic!")        
    }
    loop{}
}
