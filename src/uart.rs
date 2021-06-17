use core::fmt;
use core::fmt::Write;
use crate::driver::MiniUartRaw;
use crate::interface;
use crate::mutex::{Mutex, NullLock};

struct MiniUartInner {       
    raw : MiniUartRaw,
    written_chars : usize,
    read_chars : usize   
}                        

impl MiniUartInner{
    const fn new() -> Self{
        Self{
            raw : MiniUartRaw::new(),
            written_chars : 0,
            read_chars : 0
        }
    }

    fn putchar(&self, ch : char){
        self.raw.putchar(ch);
    }

    fn getchar(&self) -> char{
        self.raw.getchar()
    }
}

impl fmt::Write for MiniUartInner {
    fn write_str(&mut self, s : &str) -> fmt::Result{
        for ch in s.chars(){
            self.putchar(ch)
        }
        Ok(())
    }
}

pub struct MiniUart{
    inner : NullLock<MiniUartInner>
}

impl MiniUart{
    pub const fn new() -> Self{
        Self{
            inner : NullLock::new(MiniUartInner::new())
        }
    }
}

pub static MINI_UART : MiniUart = MiniUart::new();

pub fn console() -> &'static impl interface::Write {
    &MINI_UART
}

impl interface::Write for MiniUart{
    fn write_fmt(&self, args : fmt::Arguments) -> fmt::Result {
        self.inner.lock(|inner| inner.write_fmt(args))
    }
}
