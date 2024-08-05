use esp_hal_low::gpio::InputPin;
use esp_hal_low::gpio::{Flex};
use log::info;

use crate::{currentAction, gpio_action, FlexIo};
use crate::gpio_action::{gpio_action, gpio_check, gpio_reset, self_check_gpio};

pub async fn reset_actions(io: &mut FlexIo<'_>) {
    gpio_reset(io).await;
}

pub async fn check_actions(io: &mut FlexIo<'_>) {
    gpio_check(io).await;
}

pub async fn manage_action(act: &mut Option<currentAction>, io: &mut FlexIo<'_>) {
    if let Some(ract) = act {
        match ract {
            currentAction::Gpio(pin) => {
                gpio_action(*pin, io).await;
            }
            currentAction::SelfCheckGpio() => {
                self_check_gpio(act, io).await;
            }
        }
    }
}
