use log::{debug, error, info};
use esp_hal::gpio::{InputConfig, Pull};

use crate::CurrentAction;
use crate::esp32s3::flex_io::FlexIo;

pub async fn gpio_check(io: &mut FlexIo<'_>) {
    // S3 has up to 48 GPIOs, but not all are available
    for pin in 0..49 {
        if let Some(out_pin) = io.current_output {
            if pin == out_pin {
                continue;
            }
        }
        if let Some(in_pin) = io.get_pin(pin) {
            // Exclude common strapping/JTAG pins if needed
            // For S3: GPIO 19, 20 are USB
            if pin == 19 || pin == 20 {
                continue;
            }
            if in_pin.is_high() {
                error!("Gpio {} is high!", pin);
            }
        }
    }
}

pub async fn gpio_reset(io: &mut FlexIo<'_>) {
    debug!("Calling gpio_reset");
    io.current_output = None;
    for pin in 0..49 {
        if let Some(flex_pin) = io.get_pin(pin) {
            flex_pin.set_low();
            flex_pin.set_output_enable(false);
            flex_pin.set_input_enable(true);
            flex_pin.apply_input_config(&InputConfig::default().with_pull(Pull::None));
        }
    }
}

pub async fn gpio_action(pin: u32, io: &mut FlexIo<'_>) {
    let mut reset_needed = true;
    if let Some(out_pin) = io.current_output {
        if out_pin == pin {
            reset_needed = false;
        }
    }
    if reset_needed {
        debug!("Setting pin {} to output", pin);
        gpio_reset(io).await;
        io.current_output = Some(pin);
        if let Some(flex_pin) = io.get_pin(pin) {
            flex_pin.set_input_enable(false);
            flex_pin.set_output_enable(true);
        }
    }
    if let Some(flex_pin) = io.get_pin(pin) {
        info!("Toggling pin {}", pin);
        flex_pin.toggle();
    }
    embassy_time::Timer::after_millis(2000).await;
}

pub async fn self_check_gpio(act: &mut Option<CurrentAction>, io: &mut FlexIo<'_>) {
    for pin in 0..49 {
        if io.get_pin(pin).is_some() {
            info!("Checking pin {}", pin);
            gpio_action(pin, io).await;
            embassy_time::Timer::after_millis(1750).await;
            gpio_check(io).await;
            embassy_time::Timer::after_millis(1750).await;
        }
    }
    gpio_reset(io).await;
    *act = None;
    info!("Finished self checking gpio");
}
