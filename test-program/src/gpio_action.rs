use once_cell::sync::OnceCell;

use crate::{flex_io::FlexIo, gpioAction};

pub static currentGpio: OnceCell<u32> = OnceCell::new();

pub async fn gpio_action(pin: gpioAction, io: &mut FlexIo<'_>) {
    
    embassy_time::Timer::after_millis(2000).await;
}