#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use actions::{check_actions, manage_action, reset_actions};
use core::{fmt::Write, mem::MaybeUninit};
use esp_hal_low::gpio::any_pin::AnyPin;
use esp_hal_low::uart::Uart;
use esp_println::logger::{init_logger, init_logger_from_env};
use flex_io::FlexIo;
use gpio_action::gpio_reset;
use heapless::{String, Vec};
use log::{debug, error, info};
use static_cell::StaticCell;

use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal_low::gpio::{Flex, Level};
use esp_hal_low::{
    clock::ClockControl,
    entry,
    gpio::{GpioPin, Io, Output},
    macros::main,
    peripherals::Peripherals,
    system::SystemControl,
    timer::{timg::TimerGroup, ErasedTimer, OneShotTimer},
    usb_serial_jtag::UsbSerialJtag,
};

mod actions;
mod flex_io;
mod gpio_action;

#[global_allocator]
pub(crate) static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn heap_init() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

const MAX_BUFFER_SIZE: usize = 512;

macro_rules! iterable_enum {
    ($visibility:vis, $name:ident, $($member:tt),*) => {
        $visibility enum $name {$($member),*}
        impl $name {
            fn iterate() -> Vec<$name> {
                vec![$($name::$member,)*]
            }
        }
    };
    ($name:ident, $($member:tt),*) => {
        iterable_enum!(, $name, $($member),*)
    };
}

/*
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum gpioAction {
    gpio0,
    gpio1,
    gpio2,
    gpio3,
    gpio4,
    gpio5,
    gpio6,
    gpio7,
    //io9, // gpio 9! This pin must be tested before even connecting to usb
    gpio10, // probably TXD0
    gpio11, // probbaly RXD0
    // io12 and io13 are USB
    gpio14,
    gpio15,
    gpio18,
    gpio19,
    gpio20,
    gpio21,
    gpio22,
    gpio23,
}
*/

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum currentAction {
    Gpio(u32),
    SelfCheckGpio(),
}

#[main]
async fn main(_spawner: Spawner) {
    init_logger(log::LevelFilter::Info);
    // init_logger(log::LevelFilter::Debug);
    
    info!("Heap init");
    heap_init();

    info!("Getting the hardware");
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);

    let clocks = ClockControl::max(system.clock_control).freeze();

    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks, None);

    static ONE_SHOT_TIMER: StaticCell<[OneShotTimer<ErasedTimer>; 1]> = StaticCell::new();
    esp_hal_embassy::init(
        &clocks,
        ONE_SHOT_TIMER.init([OneShotTimer::new(timg0.timer0.into())]),
    );
    let mut io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    // This disables USB JTAG interface
    // https://github.com/esp-rs/esp-hal/issues/279
    #[cfg(feature = "uart")]
    peripherals.USB_DEVICE.conf0().modify(|_,w| w.usb_pad_enable().clear_bit());

    let mut gpios = FlexIo {
        current_output: None,
        gpio0: Flex::new(AnyPin::new(io.pins.gpio0)),
        gpio1: Flex::new(AnyPin::new(io.pins.gpio1)),
        gpio2: Flex::new(AnyPin::new(io.pins.gpio2)),
        gpio3: Flex::new(AnyPin::new(io.pins.gpio3)),
        gpio4: Flex::new(AnyPin::new(io.pins.gpio4)),
        gpio5: Flex::new(AnyPin::new(io.pins.gpio5)),
        gpio6: Flex::new(AnyPin::new(io.pins.gpio6)),
        gpio7: Flex::new(AnyPin::new(io.pins.gpio7)),
        gpio10: Flex::new(AnyPin::new(io.pins.gpio10)),
        gpio11: Flex::new(AnyPin::new(io.pins.gpio11)),
        #[cfg(feature = "uart")]
        gpio12: Flex::new(AnyPin::new(io.pins.gpio12)),
        #[cfg(feature = "uart")]
        gpio13: Flex::new(AnyPin::new(io.pins.gpio13)),
        gpio14: Flex::new(AnyPin::new(io.pins.gpio14)),
        gpio15: Flex::new(AnyPin::new(io.pins.gpio15)),
        gpio18: Flex::new(AnyPin::new(io.pins.gpio18)),
        gpio19: Flex::new(AnyPin::new(io.pins.gpio19)),
        gpio20: Flex::new(AnyPin::new(io.pins.gpio20)),
        gpio21: Flex::new(AnyPin::new(io.pins.gpio21)),
        #[cfg(feature = "i2c")]
        gpio22: Flex::new(AnyPin::new(io.pins.gpio22)),
        #[cfg(feature = "i2c")]
        gpio23: Flex::new(AnyPin::new(io.pins.gpio23)),
    };

    gpio_reset(&mut gpios).await;
    embassy_time::Timer::after_millis(1000).await; // To make sure all pins are low

    info!("Initializing usb serial");
    // https://github.com/esp-rs/esp-hal/blob/main/examples/src/bin/usb_serial.rs
    #[cfg(feature = "uart")]
    let mut serial = Uart::new(peripherals.UART0, &clocks, io.pins.gpio16, io.pins.gpio17).unwrap();

    #[cfg(feature = "usb_jtag")]
    let (mut tx, mut rx) = UsbSerialJtag::new_async(peripherals.USB_DEVICE).split();

    let mut rbuf = [0u8; MAX_BUFFER_SIZE];
    let mut string_buffer: heapless::Vec<u8, MAX_BUFFER_SIZE> = heapless::Vec::new();

    let mut cur_act: Option<currentAction> = None;
    let mut started_typing = false;
    loop {
        //debug!("Iterating...");
        // Read from serial until space is detected
        let mut is_newline = false;
        #[cfg(feature = "usb_jtag")]
        let r = rx.read_byte();
        #[cfg(feature = "uart")]
        let r = serial.read_byte();
        if let Ok(byte) = r {
            string_buffer.push(byte).unwrap();
            debug!("Received character: {}", byte);
            #[cfg(feature = "usb_jtag")]
            tx.write_char(byte as char).unwrap();
            #[cfg(feature = "uart")]
            serial.write_bytes(&[byte]).unwrap();
            // Cariage return
            // https://www.asciitable.com/
            if byte == 13 {
                debug!("Received a new line");
                is_newline = true;
                started_typing = false;
            } else {
                started_typing = true;
                continue;
            }
        }

        #[cfg(feature = "init_test")]
        check_actions(&mut gpios).await;

        if started_typing && !is_newline {
            embassy_time::Timer::after_millis(50).await;
            continue;
        }

        if is_newline {
            string_buffer.pop();
            let action = heapless::String::from_utf8(string_buffer.clone()).unwrap();
            string_buffer.clear();
            debug!("Received string: \"{}\"", action);
            let cur_act_tmp = cur_act;
            // What a horrible day to see this, but I can't create a function that takes an argument of String<_>
            if action.contains("gpio") {
                if action == "gpio0" {
                    cur_act = Some(currentAction::Gpio(0));
                } else if action == "gpio1" {
                    cur_act = Some(currentAction::Gpio(1));
                } else if action == "gpio2" {
                    cur_act = Some(currentAction::Gpio(2));
                } else if action == "gpio3" {
                    cur_act = Some(currentAction::Gpio(3));
                } else if action == "gpio4" {
                    cur_act = Some(currentAction::Gpio(4));
                } else if action == "gpio5" {
                    cur_act = Some(currentAction::Gpio(5));
                } else if action == "gpio6" {
                    cur_act = Some(currentAction::Gpio(6));
                } else if action == "gpio7" {
                    cur_act = Some(currentAction::Gpio(7));
                } else if action == "gpio10" {
                    cur_act = Some(currentAction::Gpio(10));
                } else if action == "gpio11" {
                    cur_act = Some(currentAction::Gpio(11));
                } else if action == "gpio12" {
                    #[cfg(feature = "uart")]
                    {
                        cur_act = Some(currentAction::Gpio(12));
                    }
                    #[cfg(not(feature = "uart"))]
                    {
                        error!("You dummy, this gpio is used for usb communication, which you are using now!");
                    }
                } else if action == "gpio13" {
                    #[cfg(feature = "uart")]
                    {
                        cur_act = Some(currentAction::Gpio(13));
                    }
                    #[cfg(not(feature = "uart"))]
                    {
                        error!("You dummy, this gpio is used for usb communication, which you are using now!");
                    }
                } else if action == "gpio14" {
                    cur_act = Some(currentAction::Gpio(14));
                } else if action == "gpio15" {
                    cur_act = Some(currentAction::Gpio(15));
                } else if action == "gpio18" {
                    cur_act = Some(currentAction::Gpio(18));
                } else if action == "gpio19" {
                    cur_act = Some(currentAction::Gpio(19));
                } else if action == "gpio20" {
                    cur_act = Some(currentAction::Gpio(20));
                } else if action == "gpio21" {
                    cur_act = Some(currentAction::Gpio(21));
                } else if action == "gpio22" {
                    #[cfg(feature = "i2c")]
                    {
                        cur_act = Some(currentAction::Gpio(22));
                    }
                    #[cfg(not(feature = "i2c"))]
                    {
                        error!("i2c bad me not like");
                    }
                } else if action == "gpio23" {
                    #[cfg(feature = "i2c")]
                    {
                        cur_act = Some(currentAction::Gpio(23));
                    }
                    #[cfg(not(feature = "i2c"))]
                    {
                        error!("i2c bad me not like");
                    }
                } else if action == "self_check_gpio" {
                    info!("Starting self checking gpio");
                    cur_act = Some(currentAction::SelfCheckGpio());
                } else {
                    error!("Specify the gpio number as gpioX");
                    cur_act = None;
                }
            } else if action == "exit" {
                info!("Exiting the current action");
                reset_actions(&mut gpios).await;
                cur_act = None;
            } else {
                error!("Unknown action");
                //cur_act = None;
            }

            if cur_act_tmp != cur_act {
                info!("Current action: {:?}", cur_act);
            }
            is_newline = false;
        }

        if r.is_err() && cur_act.is_none() {
            embassy_time::Timer::after_millis(200).await;
            continue;
        }

        manage_action(&mut cur_act, &mut gpios).await;
    }
}
