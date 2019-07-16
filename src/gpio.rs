use core::convert::TryInto;
use esp32_sys::{
    gpio_mode_t_GPIO_MODE_OUTPUT, gpio_pad_select_gpio, gpio_set_direction, gpio_set_level,
};

pub struct OutputPin {
    which: u32,
}

impl OutputPin {
    pub fn new(which: u32) -> Self {
        unsafe {
            gpio_pad_select_gpio(which.try_into().unwrap());
            gpio_set_direction(which, gpio_mode_t_GPIO_MODE_OUTPUT);
        }

        Self { which }
    }

    pub fn set_high(&mut self) {
        unsafe {
            gpio_set_level(self.which, 1);
        }
    }

    pub fn set_low(&mut self) {
        unsafe {
            gpio_set_level(self.which, 0);
        }
    }
}
