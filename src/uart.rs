use core::ptr;
use esp32_sys::{
    gpio_num_t_GPIO_NUM_16, gpio_num_t_GPIO_NUM_17, uart_config_t, uart_driver_install,
    uart_hw_flowcontrol_t_UART_HW_FLOWCTRL_DISABLE, uart_param_config,
    uart_parity_t_UART_PARITY_DISABLE, uart_port_t_UART_NUM_0, uart_port_t_UART_NUM_1,
    uart_port_t_UART_NUM_2, uart_set_pin, uart_stop_bits_t_UART_STOP_BITS_1,
    uart_word_length_t_UART_DATA_8_BITS, uart_write_bytes, UART_PIN_NO_CHANGE,
};

const ECHO_TEST_TXD: i32 = gpio_num_t_GPIO_NUM_17 as i32;
const ECHO_TEST_RXD: i32 = gpio_num_t_GPIO_NUM_16 as i32;
const BUF_SIZE: i32 = 1024;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Which {
    Uart0,
    Uart1,
    Uart2,
}

impl From<Which> for u32 {
    fn from(value: Which) -> Self {
        use Which::*;

        match value {
            Uart0 => uart_port_t_UART_NUM_0,
            Uart1 => uart_port_t_UART_NUM_1,
            Uart2 => uart_port_t_UART_NUM_2,
        }
    }
}

pub struct Uart {
    which: Which,
}

impl Uart {
    pub fn new(which: Which) -> Self {
        /* Configure parameters of an UART driver,
         * communication pins and install the driver */
        let uart_config = uart_config_t {
            baud_rate: 115200,
            data_bits: uart_word_length_t_UART_DATA_8_BITS,
            parity: uart_parity_t_UART_PARITY_DISABLE,
            stop_bits: uart_stop_bits_t_UART_STOP_BITS_1,
            flow_ctrl: uart_hw_flowcontrol_t_UART_HW_FLOWCTRL_DISABLE,
            rx_flow_ctrl_thresh: 0,
            use_ref_tick: false,
        };

        unsafe {
            uart_param_config(u32::from(which), &uart_config);
            uart_set_pin(
                u32::from(which),
                ECHO_TEST_TXD,
                ECHO_TEST_RXD,
                UART_PIN_NO_CHANGE, // RTS
                UART_PIN_NO_CHANGE, // CTS
            );
            uart_driver_install(u32::from(which), BUF_SIZE * 2, 0, 0, ptr::null_mut(), 0);
        }

        Self { which }
    }
}

impl core::fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            uart_write_bytes(u32::from(self.which), s.as_ptr() as *const _, s.len());
        }
        Ok(())
    }
}
