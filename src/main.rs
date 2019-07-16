#![no_std]
#![no_main]

mod delay;
mod gpio;
mod uart;

use crate::{delay::delay, gpio::OutputPin, uart::Uart};
use core::{fmt::Write, panic::PanicInfo, time::Duration};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn app_main() {
    let mut uart = Uart::new(uart::Which::Uart0);
    let mut led = OutputPin::new(2);

    loop {
        led.set_low();
        delay(Duration::from_secs(1));

        uart.write_str("This is a test string.\n").unwrap();
        led.set_high();
        delay(Duration::from_secs(1));
    }
}
