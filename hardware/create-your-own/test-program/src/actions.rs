use crate::{CurrentAction, FlexIo};

#[cfg(feature = "init_test")]
pub async fn check_actions(io: &mut FlexIo<'_>) {
    //info!("Checking actions...");
    for pin in 0..24 {
        if let Some(flex_pin) = io.get_pin(pin) {
            if pin == 13 || pin == 6 {
                continue;
            }
            if flex_pin.is_high() {
                //info!("Pin {} is high!", pin);
            }
        }
    }
}

pub async fn reset_actions(io: &mut FlexIo<'_>) {
    crate::gpio_action::gpio_reset(io).await;
}

pub async fn manage_action(act: &mut Option<CurrentAction>, io: &mut FlexIo<'_>) {
    if let Some(action) = act {
        match action {
            CurrentAction::Gpio(pin) => {
                crate::gpio_action::gpio_action(*pin, io).await;
            }
            CurrentAction::SelfCheckGpio() => {
                crate::gpio_action::self_check_gpio(act, io).await;
            }
        }
    }
}