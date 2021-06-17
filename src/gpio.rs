use core::convert::{From, Into};
use core::ptr::{read_volatile, write_volatile};
pub const GPBASE: usize = crate::MMIO_BASE + 0x20_0000;

const GPFSEL: usize = GPBASE + 0x0;
const GPSET: usize = GPBASE + 0x1c;
const GPCLR: usize = GPBASE + 0x28;
const GPPUPD : usize = GPBASE + 0xe4;

pub enum PullMode {
    PullNone,
    PullUp,
    PullDown,
}
impl From<PullMode> for u32 {
    fn from(val: PullMode) -> u32 {
        match val {
            PullMode::PullDown => 0b10,
            PullMode::PullNone => 0b0,
            PullMode::PullUp => 0b1,
        }
    }
}
impl core::clone::Clone for PullMode {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::marker::Copy for PullMode {}

pub enum GpioFunction {
    INPUT,
    OUTPUT,
    ALTF0,
    ALTF1,
    ALTF2,
    ALTF3,
    ALTF4,
    ALTF5,
}

impl From<GpioFunction> for u32 {
    fn from(val: GpioFunction) -> u32 {
        match val {
            GpioFunction::INPUT => 0,
            GpioFunction::OUTPUT => 1,
            GpioFunction::ALTF0 => 0b100,
            GpioFunction::ALTF1 => 0b101,
            GpioFunction::ALTF2 => 0b110,
            GpioFunction::ALTF3 => 0b111,
            GpioFunction::ALTF4 => 0b011,
            GpioFunction::ALTF5 => 0b010,
        }
    }
}

impl core::clone::Clone for GpioFunction {
    fn clone(&self) -> Self {
        *self
    }
}

impl core::marker::Copy for GpioFunction {}

/// struct for gpio pin
pub struct GpioPin {
    pin: u32,
}

impl GpioPin {
    /// make new instance witout init
    pub fn new(pin: u32) -> Self {
        Self { pin }
    }

    pub const fn rxd() -> Self{
        Self{pin : 15}
    }

    pub const fn txd() -> Self{
        Self{pin: 14}
    }

    pub fn set_function(&self, func: GpioFunction) {
        gpio_ctrl(self.pin, func.into(), GPFSEL, 3);
    }

    pub fn set_high(&self) {
        gpio_ctrl(self.pin, 1, GPSET, 1);
    }

    pub fn set_low(&self) {
        gpio_ctrl(self.pin, 1, GPCLR, 1);
    }

    pub fn set_pupd(&self, mode : PullMode){
        gpio_ctrl(self.pin, mode.into(), GPPUPD, 2);
    }
}

pub fn gpio_ctrl(pin_num: u32, value: u32, base: usize, width: usize) {
    let frame = 32 / width;
    let reg = (base + (pin_num as usize / frame) * 4) as *mut u32;
    let shift = ((pin_num as usize % frame) * width) as u32;
    let val = value << shift;
    let mask = ((1 << width as u32) - 1) << shift;

    unsafe {
        let tmp = read_volatile(reg); // read the previous value
        write_volatile(reg, (tmp & !mask) | val);
    }
}
