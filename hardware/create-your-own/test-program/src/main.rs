#![no_std]
#![no_main]

use core::mem::MaybeUninit;
use esp_println::logger::init_logger;
use log::{debug, error, info};

use embassy_executor::Spawner;
use esp_backtrace as _;
#[cfg(feature = "usb_jtag")]
use esp_hal::usb_serial_jtag::UsbSerialJtag;
use esp_hal::{interrupt::software::SoftwareInterruptControl, timer::timg::TimerGroup};
#[cfg(feature = "esp32c6")]
mod esp32c6;
#[cfg(feature = "esp32c6")]
use esp32c6::{actions, actions::reset_actions, flex_io::FlexIo, gpio_action::gpio_reset};

#[cfg(feature = "esp32s3")]
mod esp32s3;
#[cfg(feature = "esp32s3")]
use esp32s3::{actions, actions::reset_actions, flex_io::FlexIo, gpio_action::gpio_reset};

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

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(_spawner: Spawner) {
    init_logger(log::LevelFilter::Info);

    info!("Heap init");
    heap_init();

    info!("Getting the hardware");
    let peripherals = esp_hal::init(esp_hal::Config::default());

    let _sw_int = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    let timg0 = TimerGroup::new(peripherals.TIMG0);
    #[cfg(target_arch = "riscv32")]
    esp_rtos::start(timg0.timer0, _sw_int.software_interrupt0);
    #[cfg(not(target_arch = "riscv32"))]
    esp_rtos::start(timg0.timer0);

    let mut gpios = FlexIo {
        current_output: None,
        #[cfg(feature = "esp32c6")]
        gpio0: esp_hal::gpio::Flex::new(peripherals.GPIO0),
        #[cfg(feature = "esp32c6")]
        gpio1: esp_hal::gpio::Flex::new(peripherals.GPIO1),
        #[cfg(feature = "esp32c6")]
        gpio2: esp_hal::gpio::Flex::new(peripherals.GPIO2),
        #[cfg(feature = "esp32c6")]
        gpio3: esp_hal::gpio::Flex::new(peripherals.GPIO3),
        #[cfg(feature = "esp32c6")]
        gpio4: esp_hal::gpio::Flex::new(peripherals.GPIO4),
        #[cfg(feature = "esp32c6")]
        gpio5: esp_hal::gpio::Flex::new(peripherals.GPIO5),
        #[cfg(feature = "esp32c6")]
        gpio6: esp_hal::gpio::Flex::new(peripherals.GPIO6),
        #[cfg(feature = "esp32c6")]
        gpio7: esp_hal::gpio::Flex::new(peripherals.GPIO7),
        #[cfg(feature = "esp32c6")]
        gpio10: esp_hal::gpio::Flex::new(peripherals.GPIO10),
        #[cfg(feature = "esp32c6")]
        gpio11: esp_hal::gpio::Flex::new(peripherals.GPIO11),
        #[cfg(all(feature = "esp32c6", feature = "uart"))]
        gpio12: esp_hal::gpio::Flex::new(peripherals.GPIO12),
        #[cfg(all(feature = "esp32c6", feature = "uart"))]
        gpio13: esp_hal::gpio::Flex::new(peripherals.GPIO13),
        #[cfg(feature = "esp32c6")]
        gpio14: esp_hal::gpio::Flex::new(peripherals.GPIO14),
        #[cfg(feature = "esp32c6")]
        gpio15: esp_hal::gpio::Flex::new(peripherals.GPIO15),
        #[cfg(feature = "esp32c6")]
        gpio18: esp_hal::gpio::Flex::new(peripherals.GPIO18),
        #[cfg(feature = "esp32c6")]
        gpio19: esp_hal::gpio::Flex::new(peripherals.GPIO19),
        #[cfg(feature = "esp32c6")]
        gpio20: esp_hal::gpio::Flex::new(peripherals.GPIO20),
        #[cfg(feature = "esp32c6")]
        gpio21: esp_hal::gpio::Flex::new(peripherals.GPIO21),
        #[cfg(all(feature = "esp32c6", feature = "i2c"))]
        gpio22: esp_hal::gpio::Flex::new(peripherals.GPIO22),
        #[cfg(all(feature = "esp32c6", feature = "i2c"))]
        gpio23: esp_hal::gpio::Flex::new(peripherals.GPIO23),

        #[cfg(feature = "esp32s3")]
        gpio0: esp_hal::gpio::Flex::new(peripherals.GPIO0),
        #[cfg(feature = "esp32s3")]
        gpio1: esp_hal::gpio::Flex::new(peripherals.GPIO1),
        #[cfg(feature = "esp32s3")]
        gpio2: esp_hal::gpio::Flex::new(peripherals.GPIO2),
        #[cfg(feature = "esp32s3")]
        gpio3: esp_hal::gpio::Flex::new(peripherals.GPIO3),
        #[cfg(feature = "esp32s3")]
        gpio4: esp_hal::gpio::Flex::new(peripherals.GPIO4),
        #[cfg(feature = "esp32s3")]
        gpio5: esp_hal::gpio::Flex::new(peripherals.GPIO5),
        #[cfg(feature = "esp32s3")]
        gpio6: esp_hal::gpio::Flex::new(peripherals.GPIO6),
        #[cfg(feature = "esp32s3")]
        gpio7: esp_hal::gpio::Flex::new(peripherals.GPIO7),
        #[cfg(feature = "esp32s3")]
        gpio8: esp_hal::gpio::Flex::new(peripherals.GPIO8),
        #[cfg(feature = "esp32s3")]
        gpio9: esp_hal::gpio::Flex::new(peripherals.GPIO9),
        #[cfg(feature = "esp32s3")]
        gpio10: esp_hal::gpio::Flex::new(peripherals.GPIO10),
        #[cfg(feature = "esp32s3")]
        gpio11: esp_hal::gpio::Flex::new(peripherals.GPIO11),
        #[cfg(feature = "esp32s3")]
        gpio12: esp_hal::gpio::Flex::new(peripherals.GPIO12),
        #[cfg(feature = "esp32s3")]
        gpio13: esp_hal::gpio::Flex::new(peripherals.GPIO13),
        #[cfg(feature = "esp32s3")]
        gpio14: esp_hal::gpio::Flex::new(peripherals.GPIO14),
        #[cfg(feature = "esp32s3")]
        gpio15: esp_hal::gpio::Flex::new(peripherals.GPIO15),
        #[cfg(feature = "esp32s3")]
        gpio16: esp_hal::gpio::Flex::new(peripherals.GPIO16),
        #[cfg(feature = "esp32s3")]
        gpio17: esp_hal::gpio::Flex::new(peripherals.GPIO17),
        #[cfg(feature = "esp32s3")]
        gpio18: esp_hal::gpio::Flex::new(peripherals.GPIO18),
        #[cfg(feature = "esp32s3")]
        gpio19: esp_hal::gpio::Flex::new(peripherals.GPIO19),
        #[cfg(feature = "esp32s3")]
        gpio20: esp_hal::gpio::Flex::new(peripherals.GPIO20),
        #[cfg(feature = "esp32s3")]
        gpio21: esp_hal::gpio::Flex::new(peripherals.GPIO21),
        #[cfg(feature = "esp32s3")]
        gpio34: esp_hal::gpio::Flex::new(peripherals.GPIO34),
        #[cfg(feature = "esp32s3")]
        gpio35: esp_hal::gpio::Flex::new(peripherals.GPIO35),
        #[cfg(feature = "esp32s3")]
        gpio36: esp_hal::gpio::Flex::new(peripherals.GPIO36),
        #[cfg(feature = "esp32s3")]
        gpio37: esp_hal::gpio::Flex::new(peripherals.GPIO37),
        #[cfg(feature = "esp32s3")]
        gpio38: esp_hal::gpio::Flex::new(peripherals.GPIO38),
        #[cfg(feature = "esp32s3")]
        gpio39: esp_hal::gpio::Flex::new(peripherals.GPIO39),
        #[cfg(feature = "esp32s3")]
        gpio40: esp_hal::gpio::Flex::new(peripherals.GPIO40),
        #[cfg(feature = "esp32s3")]
        gpio41: esp_hal::gpio::Flex::new(peripherals.GPIO41),
        #[cfg(feature = "esp32s3")]
        gpio42: esp_hal::gpio::Flex::new(peripherals.GPIO42),
        #[cfg(all(feature = "esp32s3", feature = "usb_jtag"))]
        gpio43: esp_hal::gpio::Flex::new(peripherals.GPIO43),
        #[cfg(all(feature = "esp32s3", feature = "usb_jtag"))]
        gpio44: esp_hal::gpio::Flex::new(peripherals.GPIO44),
        #[cfg(feature = "esp32s3")]
        gpio45: esp_hal::gpio::Flex::new(peripherals.GPIO45),
        #[cfg(feature = "esp32s3")]
        gpio46: esp_hal::gpio::Flex::new(peripherals.GPIO46),
        #[cfg(feature = "esp32s3")]
        gpio47: esp_hal::gpio::Flex::new(peripherals.GPIO47),
        #[cfg(feature = "esp32s3")]
        gpio48: esp_hal::gpio::Flex::new(peripherals.GPIO48),
    };

    gpio_reset(&mut gpios).await;
    embassy_time::Timer::after_millis(1000).await;

    #[cfg(feature = "usb_jtag")]
    info!("Initializing usb jtag communication");

    #[cfg(feature = "uart")]
    info!("Initializing uart communication");

    #[cfg(feature = "uart")]
    let (mut rx, mut tx) = {
        #[cfg(feature = "esp32c6")]
        {
            esp_hal::uart::Uart::new(peripherals.UART0, esp_hal::uart::Config::default())
                .unwrap()
                .with_rx(peripherals.GPIO17)
                .with_tx(peripherals.GPIO16)
                .into_async()
                .split()
        }
        #[cfg(feature = "esp32s3")]
        {
            esp_hal::uart::Uart::new(peripherals.UART0, esp_hal::uart::Config::default())
                .unwrap()
                .with_rx(peripherals.GPIO44)
                .with_tx(peripherals.GPIO43)
                .into_async()
                .split()
        }
    };

    #[cfg(feature = "usb_jtag")]
    let (mut rx, mut tx) = UsbSerialJtag::new(peripherals.USB_DEVICE)
        .into_async()
        .split();

    let mut string_buffer: heapless::Vec<u8, MAX_BUFFER_SIZE> = heapless::Vec::new();

    let mut cur_act: Option<CurrentAction> = None;
    let mut started_typing = false;
    loop {
        let mut is_newline = false;
        let mut readed = false;

        let mut small_buf = [0u8; 1];

        #[cfg(feature = "uart")]
        {
            if rx.read_ready() {
                rx.read(&mut small_buf).ok();
                readed = true;
            }
        }

        #[cfg(feature = "usb_jtag")]
        {
            let amount = rx.drain_rx_fifo(&mut small_buf);
            if amount != 0 {
                readed = true;
            }
        }

        if readed {
            let byte = small_buf[0];
            string_buffer.push(byte).unwrap();
            debug!("Received character: {}", byte);
            let _ = tx.write(&[byte]);

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
            if !string_buffer.is_empty() {
                string_buffer.pop();
            }
            if let Ok(action_str) = core::str::from_utf8(&string_buffer) {
                let cur_act_tmp = cur_act;
                if action_str == "exit" {
                    info!("Exiting the current action");
                    reset_actions(&mut gpios).await;
                    cur_act = None;
                } else if let Some(new_act) = gpios.handle_command(action_str) {
                    cur_act = Some(new_act);
                } else {
                    error!("Unknown action: {}", action_str);
                }

                if cur_act_tmp != cur_act {
                    info!("Current action: {:?}", cur_act);
                }
            }
            string_buffer.clear();
        }

        if !readed && cur_act.is_none() {
            embassy_time::Timer::after_millis(200).await;
            continue;
        }

        actions::manage_action(&mut cur_act, &mut gpios).await;
    }
}
