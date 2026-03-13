#![no_std]
#![no_main]

use actions::{manage_action, reset_actions};
use core::{fmt::Write, mem::MaybeUninit};
use esp_println::logger::init_logger;
use flex_io::FlexIo;
use gpio_action::gpio_reset;
use log::{debug, error, info};

use embassy_executor::Spawner;
use esp_backtrace as _;
use esp_hal::gpio::{Flex};
use esp_hal::{
    gpio::Io,
    timer::{timg::TimerGroup},
    usb_serial_jtag::UsbSerialJtag,
    interrupt::software::SoftwareInterruptControl,
};

mod actions;
mod flex_io;
mod gpio_action;

esp_bootloader_esp_idf::esp_app_desc!();

fn heap_init() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        esp_alloc::HEAP.add_region(esp_alloc::HeapRegion::new(
            core::ptr::addr_of_mut!(HEAP) as *mut u8,
            HEAP_SIZE,
            esp_alloc::MemoryCapability::Internal.into(),
        ));
    }
}

const MAX_BUFFER_SIZE: usize = 512;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CurrentAction {
    Gpio(u32),
    SelfCheckGpio(),
}

#[esp_rtos::main]
async fn main(_spawner: Spawner) {
    init_logger(log::LevelFilter::Info);
    // init_logger(log::LevelFilter::Debug);
    
    info!("Heap init");
    heap_init();

    info!("Getting the hardware");
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, sw_int.software_interrupt0);

    let _io = Io::new(peripherals.IO_MUX);

    let mut gpios = FlexIo {
        current_output: None,
        gpio0: Flex::new(peripherals.GPIO0),
        gpio1: Flex::new(peripherals.GPIO1),
        gpio2: Flex::new(peripherals.GPIO2),
        gpio3: Flex::new(peripherals.GPIO3),
        gpio4: Flex::new(peripherals.GPIO4),
        gpio5: Flex::new(peripherals.GPIO5),
        gpio6: Flex::new(peripherals.GPIO6),
        gpio7: Flex::new(peripherals.GPIO7),
        gpio10: Flex::new(peripherals.GPIO10),
        gpio11: Flex::new(peripherals.GPIO11),
        #[cfg(feature = "uart")]
        gpio12: Flex::new(peripherals.GPIO12),
        #[cfg(feature = "uart")]
        gpio13: Flex::new(peripherals.GPIO13),
        gpio14: Flex::new(peripherals.GPIO14),
        gpio15: Flex::new(peripherals.GPIO15),
        gpio18: Flex::new(peripherals.GPIO18),
        gpio19: Flex::new(peripherals.GPIO19),
        gpio20: Flex::new(peripherals.GPIO20),
        gpio21: Flex::new(peripherals.GPIO21),
        #[cfg(feature = "i2c")]
        gpio22: Flex::new(peripherals.GPIO22),
        #[cfg(feature = "i2c")]
        gpio23: Flex::new(peripherals.GPIO23),
    };

    gpio_reset(&mut gpios).await;
    embassy_time::Timer::after_millis(1000).await; // To make sure all pins are low

    #[cfg(feature = "usb_jtag")]
    info!("Initializing usb jtag communication");

    #[cfg(feature = "uart")]
    info!("Initializing uart communication");

    // https://github.com/esp-rs/esp-hal/blob/main/examples/src/bin/usb_serial.rs
    #[cfg(feature = "uart")]
    let mut serial = esp_hal::uart::Uart::new(peripherals.UART0, esp_hal::uart::Config::default()).unwrap()
        .with_rx(peripherals.GPIO17)
        .with_tx(peripherals.GPIO16)
        .into_async();

    #[cfg(feature = "usb_jtag")]
    let (mut rx, mut tx) = UsbSerialJtag::new(peripherals.USB_DEVICE).into_async().split();

    let mut string_buffer: heapless::Vec<u8, MAX_BUFFER_SIZE> = heapless::Vec::new();

    let mut cur_act: Option<CurrentAction> = None;
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
        actions::check_actions(&mut gpios).await;

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
                    cur_act = Some(CurrentAction::Gpio(0));
                } else if action == "gpio1" {
                    cur_act = Some(CurrentAction::Gpio(1));
                } else if action == "gpio2" {
                    cur_act = Some(CurrentAction::Gpio(2));
                } else if action == "gpio3" {
                    cur_act = Some(CurrentAction::Gpio(3));
                } else if action == "gpio4" {
                    cur_act = Some(CurrentAction::Gpio(4));
                } else if action == "gpio5" {
                    cur_act = Some(CurrentAction::Gpio(5));
                } else if action == "gpio6" {
                    cur_act = Some(CurrentAction::Gpio(6));
                } else if action == "gpio7" {
                    cur_act = Some(CurrentAction::Gpio(7));
                } else if action == "gpio10" {
                    cur_act = Some(CurrentAction::Gpio(10));
                } else if action == "gpio11" {
                    cur_act = Some(CurrentAction::Gpio(11));
                } else if action == "gpio12" {
                    #[cfg(feature = "uart")]
                    {
                        cur_act = Some(CurrentAction::Gpio(12));
                    }
                    #[cfg(not(feature = "uart"))]
                    {
                        error!("You dummy, this gpio is used for usb communication, which you are using now!");
                    }
                } else if action == "gpio13" {
                    #[cfg(feature = "uart")]
                    {
                        cur_act = Some(CurrentAction::Gpio(13));
                    }
                    #[cfg(not(feature = "uart"))]
                    {
                        error!("You dummy, this gpio is used for usb communication, which you are using now!");
                    }
                } else if action == "gpio14" {
                    cur_act = Some(CurrentAction::Gpio(14));
                } else if action == "gpio15" {
                    cur_act = Some(CurrentAction::Gpio(15));
                } else if action == "gpio18" {
                    cur_act = Some(CurrentAction::Gpio(18));
                } else if action == "gpio19" {
                    cur_act = Some(CurrentAction::Gpio(19));
                } else if action == "gpio20" {
                    cur_act = Some(CurrentAction::Gpio(20));
                } else if action == "gpio21" {
                    cur_act = Some(CurrentAction::Gpio(21));
                } else if action == "gpio22" {
                    #[cfg(feature = "i2c")]
                    {
                        cur_act = Some(CurrentAction::Gpio(22));
                    }
                    #[cfg(not(feature = "i2c"))]
                    {
                        error!("i2c bad me not like");
                    }
                } else if action == "gpio23" {
                    #[cfg(feature = "i2c")]
                    {
                        cur_act = Some(CurrentAction::Gpio(23));
                    }
                    #[cfg(not(feature = "i2c"))]
                    {
                        error!("i2c bad me not like");
                    }
                } else if action == "self_check_gpio" {
                    info!("Starting self checking gpio");
                    cur_act = Some(CurrentAction::SelfCheckGpio());
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
        }

        if r.is_err() && cur_act.is_none() {
            embassy_time::Timer::after_millis(200).await;
            continue;
        }

        manage_action(&mut cur_act, &mut gpios).await;
    }
}
