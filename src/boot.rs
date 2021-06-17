use crate::main;
use crate::driver;

global_asm!(include_str!("boot.S"));

#[no_mangle]
fn _rust_start() {
    init_bss();
    init_uart();
    main();
}

extern "C" {
    static __bss_start: core::cell::UnsafeCell<u64>;
    static __bss_end: core::cell::UnsafeCell<u64>;
}

fn init_bss() {
    let mut ptr = unsafe { __bss_start.get() };
    let end = unsafe { __bss_end.get() };
    while ptr <= end {
        unsafe {
            core::ptr::write_volatile(ptr, 0);
            ptr = ptr.offset(1);
        }
    }
}

fn init_uart(){
    let _tmp = driver::MiniUartRaw::new();
    _tmp.setup();
}
