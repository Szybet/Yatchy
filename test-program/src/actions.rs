use log::info;

use crate::{currentAction, gpioAction, outputGpios};

pub async fn reset_actions(io: &mut outputGpios<'_>) {
    io.gpio0.set_low();
    io.gpio1.set_low();
    io.gpio2.set_low();
    io.gpio3.set_low();
    io.gpio4.set_low();
    io.gpio5.set_low();
    io.gpio6.set_low();
    io.gpio7.set_low();
    io.gpio8.set_low();
    io.gpio10.set_low();
    io.gpio11.set_low();
    io.gpio14.set_low();
    io.gpio15.set_low();
    io.gpio18.set_low();
    io.gpio19.set_low();
    io.gpio20.set_low();
    io.gpio21.set_low();
    io.gpio22.set_low();
    io.gpio23.set_low();
}

async fn gpio_action(pin: gpioAction, io: &mut outputGpios<'_>) {
    match pin {
        gpioAction::gpio0 => {
            info!("Toggling pin gpio0");
            io.gpio0.toggle();
        }
        gpioAction::gpio1 => {
            info!("Toggling pin gpio1");
            io.gpio1.toggle();
        }
        gpioAction::gpio2 => {
            info!("Toggling pin gpio2");
            io.gpio2.toggle();
        }
        gpioAction::gpio3 => {
            info!("Toggling pin gpio3");
            io.gpio3.toggle();
        }
        gpioAction::gpio4 => {
            info!("Toggling pin gpio4");
            io.gpio4.toggle();
        }
        gpioAction::gpio5 => {
            info!("Toggling pin gpio5");
            io.gpio5.toggle();
        }
        gpioAction::gpio6 => {
            info!("Toggling pin gpio6");
            io.gpio6.toggle();
        }
        gpioAction::gpio7 => {
            info!("Toggling pin gpio7");
            io.gpio7.toggle();
        }
        gpioAction::gpio8 => {
            info!("Toggling pin gpio8");
            io.gpio8.toggle();
        }
        gpioAction::gpio10 => {
            info!("Toggling pin gpio10");
            io.gpio10.toggle();
        }
        gpioAction::gpio11 => {
            info!("Toggling pin gpio11");
            io.gpio11.toggle();
        }
        gpioAction::gpio14 => {
            info!("Toggling pin gpio14");
            io.gpio14.toggle();
        }
        gpioAction::gpio15 => {
            info!("Toggling pin gpio15");
            io.gpio15.toggle();
        }
        gpioAction::gpio18 => {
            info!("Toggling pin gpio18");
            io.gpio18.toggle();
        }
        gpioAction::gpio19 => {
            info!("Toggling pin gpio19");
            io.gpio19.toggle();
        }
        gpioAction::gpio20 => {
            info!("Toggling pin gpio20");
            io.gpio20.toggle();
        }
        gpioAction::gpio21 => {
            info!("Toggling pin gpio21");
            io.gpio21.toggle();
        }
        gpioAction::gpio22 => {
            info!("Toggling pin gpio22");
            io.gpio22.toggle();
        }
        gpioAction::gpio23 => {
            info!("Toggling pin gpio23");
            io.gpio23.toggle();
        }
    }
    embassy_time::Timer::after_millis(2000).await;
}

pub async fn manage_action(act: Option<currentAction>, io: &mut outputGpios<'_>) {
    if let Some(ract) = act {
        match ract {
            currentAction::gpio(pin) => {
                gpio_action(pin, io).await;
            }
        }
    }
}
