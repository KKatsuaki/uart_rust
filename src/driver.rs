use crate::MMIO_BASE;
use crate::gpio;
use crate::gpio::GPBASE;
use crate::gpio::GpioPin;

use core::ptr::{read_volatile, write_volatile};

const AUX_BASE     : usize = MMIO_BASE + 0x215000;
const AUX_ENABLE   : usize = AUX_BASE  + 0x04;
const AUX_IO_REG   : usize = AUX_BASE  + 0x40;
const AUX_IER_REG  : usize = AUX_BASE  + 0x44;
const AUX_IIR_REG  : usize = AUX_BASE  + 0x48;
const AUX_LCR_REG  : usize = AUX_BASE  + 0x4c;
const AUX_MCR_REG  : usize = AUX_BASE  + 0x50;
const AUX_LSR_REG  : usize = AUX_BASE  + 0x54;
const AUX_CTL_REG  : usize = AUX_BASE  + 0x60;
const AUX_BAUD_REG : usize = AUX_BASE  + 0x68;


pub struct MiniUartRaw{
    rxd : gpio::GpioPin,
    txd : gpio::GpioPin
}

impl MiniUartRaw{
    pub const fn new() -> Self{
        Self{
            rxd : gpio::GpioPin::rxd(),
            txd : gpio::GpioPin::txd(),            
        }
    }

    pub fn setup(&self) {
        unsafe{
            // setup 4 gpio regs
            self.txd.set_function(gpio::GpioFunction::ALTF5);
            self.rxd.set_function(gpio::GpioFunction::ALTF5);
            self.txd.set_pupd(gpio::PullMode::PullNone);
            self.rxd.set_pupd(gpio::PullMode::PullNone);            

            // config 4 aux regs
            write_volatile(AUX_ENABLE as *mut u32, 1);
            write_volatile(AUX_IER_REG as *mut u32, 0);            
            write_volatile(AUX_CTL_REG as *mut u32, 0);
            write_volatile(AUX_LCR_REG as *mut u32, 3);
            write_volatile(AUX_MCR_REG as *mut u32, 0);
            write_volatile(AUX_IER_REG as *mut u32, 0);            
            write_volatile(AUX_IIR_REG as *mut u32, 0xC6);
            write_volatile(AUX_BAUD_REG as *mut u32, 541);
            write_volatile(AUX_CTL_REG as *mut u32, 0b11);
        }
    }

    fn is_ready(&self) -> bool{
        unsafe{
            read_volatile(AUX_LSR_REG as *const u32) & 0x20 != 0
        }
    }

    pub fn putchar(&self, ch : char) {
        while !self.is_ready(){}
        unsafe{
            write_volatile(AUX_IO_REG as *mut u32 , ch as u32 );
        }
            
    }

    pub fn getchar(&self) -> char {
        unsafe{                                                
            let ch = read_volatile(AUX_IO_REG as *const u8);
            ch as char
        }                                                      
    }
}
