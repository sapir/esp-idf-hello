use core::{convert::TryInto, time::Duration};
use esp32_sys::{configTICK_RATE_HZ, ets_delay_us, vTaskDelay};

#[allow(non_upper_case_globals)]
const portTICK_PERIOD_MS: u32 = 1000 / configTICK_RATE_HZ;

pub fn ets_delay(dur: Duration) {
    for _ in 0..dur.as_secs() {
        unsafe {
            ets_delay_us(1_000_000);
        }
    }

    unsafe {
        ets_delay_us(dur.subsec_micros());
    }
}

pub fn delay(dur: Duration) {
    let ticks = dur.as_nanos() / (u128::from(portTICK_PERIOD_MS) * 1_000_000);

    unsafe {
        vTaskDelay(ticks.try_into().unwrap());
    }
}
