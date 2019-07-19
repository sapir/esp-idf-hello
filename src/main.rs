#![no_std]
#![no_main]

mod delay;

use crate::delay::delay;
use core::{fmt::Write as _, panic::PanicInfo, time::Duration};
use embedded_hal::{digital::v2::OutputPin as _, serial::Write};
use esp32_hal::{
    gpio::OutputPin,
    serial::{self, Uart0},
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn app_main() {
    let mut uart = unsafe { Uart0::new(17, 16) };
    let mut led = unsafe { OutputPin::new(2) };

    let uart = &mut uart as &mut dyn Write<u8, Error = serial::Error>;

    loop {
        led.set_low().unwrap();
        delay(Duration::from_secs(1));

        write!(uart, "Writing with esp32-hal!\n").unwrap();
        led.set_high().unwrap();
        delay(Duration::from_secs(1));
    }
}
