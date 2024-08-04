use esp_hal_low::gpio::InputPin;
use esp_hal_low::gpio::{Flex};
use log::info;

use crate::{currentAction, gpioAction, gpio_action, FlexIo};
use crate::gpio_action::gpio_action;

pub fn reset_actions(io: &mut FlexIo<'_>) {
    io.gpio0.set_as_input(esp_hal_low::gpio::Pull::Down);
}

pub fn check_actions(io: &mut FlexIo<'_>) {
    
}

pub async fn manage_action(act: Option<currentAction>, io: &mut FlexIo<'_>) {
    if let Some(ract) = act {
        match ract {
            currentAction::gpio(pin) => {
                gpio_action(pin, io).await;
            }
        }
    }
}
