use log::{debug, error, info};
use esp_hal::gpio::{InputConfig, Pull};

use crate::{CurrentAction, flex_io::FlexIo};

pub async fn gpio_check(io: &mut FlexIo<'_>) {
    //info!("entered gpio debug");
    for pin in 0..24 {
        if let Some(out_pin) = io.current_output {
            if pin == out_pin {
                continue;
            }
        }
        if let Some(in_pin) = io.get_pin(pin) {
            // Pin 13 is always high, even if we disabled JTAG. That's why we ignore the error
            if pin == 13 {
                continue;
            }
            if pin == 6 {
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
    for pin in 0..24 {
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
        let flex_pin = io.get_pin(pin).unwrap();
        flex_pin.set_input_enable(false);
        flex_pin.set_output_enable(true);
    }
    info!("Toggling pin {}", pin);
    io.get_pin(pin).unwrap().toggle();
    embassy_time::Timer::after_millis(2000).await;
}

pub async fn self_check_gpio(act: &mut Option<CurrentAction>, io: &mut FlexIo<'_>) {
    for pin in 0..24 {
        if let Some(_flex_pin) = io.get_pin(pin) {
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