use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
    });
}

/// Prints to the host through the serial interface.
#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_color_print {
    ($c:tt, $($arg:tt)*) => {
        $crate::serial::_print(format_args!("\x1B[{}m", $c));
        $crate::serial::_print(format_args!($($arg)*));
        $crate::serial::_print(format_args!("\x1B[0m"))
    }
}

#[macro_export]
macro_rules! serial_print_red {
    ($($arg:tt)*) => { $crate::serial_color_print!(31, $($arg)*); };
}

#[macro_export]
macro_rules! serial_print_green {
    ($($arg:tt)*) => { $crate::serial_color_print!(32, $($arg)*); };
}

#[macro_export]
macro_rules! serial_print_yellow {
    ($($arg:tt)*) => { $crate::serial_color_print!(33, $($arg)*); };
}

#[macro_export]
macro_rules! serial_print_blue {
    ($($arg:tt)*) => { $crate::serial_color_print!(34, $($arg)*); };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(
        concat!($fmt, "\n"), $($arg)*));
}