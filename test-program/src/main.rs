#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use actions::{manage_action, reset_actions};
use core::{fmt::Write, mem::MaybeUninit};
use esp_println::logger::{init_logger, init_logger_from_env};
use heapless::{String, Vec};
use log::{debug, error, info};
use static_cell::StaticCell;

use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal_low::gpio::Level;
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
    gpio8,
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum currentAction {
    gpio(gpioAction),
}

pub struct outputGpios<'a> {
    gpio0: Output<'a, GpioPin<0>>,
    gpio1: Output<'a, GpioPin<1>>,
    gpio2: Output<'a, GpioPin<2>>,
    gpio3: Output<'a, GpioPin<3>>,
    gpio4: Output<'a, GpioPin<4>>,
    gpio5: Output<'a, GpioPin<5>>,
    gpio6: Output<'a, GpioPin<6>>,
    gpio7: Output<'a, GpioPin<7>>,
    gpio8: Output<'a, GpioPin<8>>,
    gpio10: Output<'a, GpioPin<10>>,
    gpio11: Output<'a, GpioPin<11>>,
    gpio14: Output<'a, GpioPin<14>>,
    gpio15: Output<'a, GpioPin<15>>,
    gpio18: Output<'a, GpioPin<18>>,
    gpio19: Output<'a, GpioPin<19>>,
    gpio20: Output<'a, GpioPin<20>>,
    gpio21: Output<'a, GpioPin<21>>,
    gpio22: Output<'a, GpioPin<22>>,
    gpio23: Output<'a, GpioPin<23>>,
}

#[main]
async fn main(_spawner: Spawner) {
    init_logger(log::LevelFilter::Info);
    //init_logger(log::LevelFilter::Debug);
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

    let mut gpio_out = outputGpios {
        gpio0: Output::new(io.pins.gpio0, Level::Low),
        gpio1: Output::new(io.pins.gpio1, Level::Low),
        gpio2: Output::new(io.pins.gpio2, Level::Low),
        gpio3: Output::new(io.pins.gpio3, Level::Low),
        gpio4: Output::new(io.pins.gpio4, Level::Low),
        gpio5: Output::new(io.pins.gpio5, Level::Low),
        gpio6: Output::new(io.pins.gpio6, Level::Low),
        gpio7: Output::new(io.pins.gpio7, Level::Low),
        gpio8: Output::new(io.pins.gpio8, Level::Low),
        gpio10: Output::new(io.pins.gpio10, Level::Low),
        gpio11: Output::new(io.pins.gpio11, Level::Low),
        gpio14: Output::new(io.pins.gpio14, Level::Low),
        gpio15: Output::new(io.pins.gpio15, Level::Low),
        gpio18: Output::new(io.pins.gpio18, Level::Low),
        gpio19: Output::new(io.pins.gpio19, Level::Low),
        gpio20: Output::new(io.pins.gpio20, Level::Low),
        gpio21: Output::new(io.pins.gpio21, Level::Low),
        gpio22: Output::new(io.pins.gpio22, Level::Low),
        gpio23: Output::new(io.pins.gpio23, Level::Low),
    };    

    info!("Initializing usb serial");
    // https://github.com/esp-rs/esp-hal/blob/main/examples/src/bin/usb_serial.rs
    let (mut tx, mut rx) = UsbSerialJtag::new_async(peripherals.USB_DEVICE).split();
    let mut rbuf = [0u8; MAX_BUFFER_SIZE];
    let mut string_buffer: heapless::Vec<u8, MAX_BUFFER_SIZE> = heapless::Vec::new();

    let mut cur_act: Option<currentAction> = None;
    let mut started_typing = false;
    loop {
        debug!("Iterating...");
        // Read from serial until space is detected
        let mut is_newline = false;
        let r = rx.read_byte();
        if let Ok(byte) = r {
            string_buffer.push(byte).unwrap();
            debug!("Received character: {}", byte);
            tx.write_char(byte as char).unwrap();
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
                    cur_act = Some(currentAction::gpio(gpioAction::gpio0));
                } else if action == "gpio1" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio1));
                } else if action == "gpio2" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio2));
                } else if action == "gpio3" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio3));
                } else if action == "gpio4" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio4));
                } else if action == "gpio5" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio5));
                } else if action == "gpio6" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio6));
                } else if action == "gpio7" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio7));
                } else if action == "gpio8" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio8));
                } else if action == "gpio10" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio10));
                } else if action == "gpio11" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio11));
                } else if action == "gpio14" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio14));
                } else if action == "gpio15" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio15));
                } else if action == "gpio18" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio18));
                } else if action == "gpio19" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio19));
                } else if action == "gpio20" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio20));
                } else if action == "gpio21" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio21));
                } else if action == "gpio22" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio22));
                } else if action == "gpio23" {
                    cur_act = Some(currentAction::gpio(gpioAction::gpio23));
                } else {
                    error!("Specify the gpio number as gpioX");
                    cur_act = None;
                }
            } else if action == "exit" {
                info!("Exiting the current action");
                reset_actions(&mut gpio_out).await;
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

        manage_action(cur_act, &mut gpio_out).await;
    }
}
