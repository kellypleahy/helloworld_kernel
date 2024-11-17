#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use helloworld_kernel::{exit_qemu, serial_print_blue, serial_print_green, serial_print_red, serial_print_yellow, QemuExitCode};

#[test_case]
fn should_fail() {
    serial_print_blue!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_print_green!("[ok]\n");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_print_yellow!("Running {} tests\n", tests.len());
    for test in tests {
        test();
        serial_print_red!("[test did not panic]");
        exit_qemu(QemuExitCode::Failed);
    }
    exit_qemu(QemuExitCode::Success);
}