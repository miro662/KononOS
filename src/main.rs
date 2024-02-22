#![no_std]
#![no_main]

mod arch;

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_panic_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn kernel_main(hartid: arch::Hart) -> ! {
    // block threads other than 0
    if hartid != 0 { loop {} }

    // TODO: main thread logic
    loop {
    }
}