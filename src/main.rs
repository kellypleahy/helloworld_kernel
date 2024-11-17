#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(helloworld_kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use helloworld_kernel::{hlt_loop, println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    helloworld_kernel::init();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    hlt_loop()
}

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    helloworld_kernel::test_panic_handler(info)
}