use crate::{interface::Write, uart};
use core::fmt;

pub fn _print(args : fmt::Arguments) {
    uart::console().write_fmt(args).unwrap();
}

#[macro_export]
#[warn(unused_must_use)]
macro_rules! println {
    () => ($crate::print!("\n\r"));
    ($($arg:tt)*) => ({
        crate::print::_print(format_args_nl!($($arg)*));
        $crate::print!("\r");
    })
}

#[macro_export]
#[warn(unused_must_use)]
macro_rules! print {
    ($($arg:tt)*) => (crate::print::_print(format_args!($($arg)*)));
}


