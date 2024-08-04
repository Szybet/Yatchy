use esp_hal_low::gpio::any_pin::AnyPin;
use esp_hal_low::gpio::{self, Flex, GpioPin, Io};

pub struct FlexIo<'a> {
    pub gpio0: Flex<'a, AnyPin<'a>>,
    pub gpio1: Flex<'a, AnyPin<'a>>,
    pub gpio2: Flex<'a, AnyPin<'a>>,
    pub gpio3: Flex<'a, AnyPin<'a>>,
    pub gpio4: Flex<'a, AnyPin<'a>>,
    pub gpio5: Flex<'a, AnyPin<'a>>,
    pub gpio6: Flex<'a, AnyPin<'a>>,
    pub gpio7: Flex<'a, AnyPin<'a>>,
    pub gpio8: Flex<'a, AnyPin<'a>>,
    pub gpio10: Flex<'a, AnyPin<'a>>,
    pub gpio11: Flex<'a, AnyPin<'a>>,
    pub gpio14: Flex<'a, AnyPin<'a>>,
    pub gpio15: Flex<'a, AnyPin<'a>>,
    pub gpio18: Flex<'a, AnyPin<'a>>,
    pub gpio19: Flex<'a, AnyPin<'a>>,
    pub gpio20: Flex<'a, AnyPin<'a>>,
    pub gpio21: Flex<'a, AnyPin<'a>>,
    pub gpio22: Flex<'a, AnyPin<'a>>,
    pub gpio23: Flex<'a, AnyPin<'a>>,
}

impl<'a> FlexIo<'a> {
    pub fn new(io: Io) -> Self {
        FlexIo {
            gpio0: Flex::new(AnyPin::new(io.pins.gpio0)),
            gpio1: Flex::new(AnyPin::new(io.pins.gpio1)),
            gpio2: Flex::new(AnyPin::new(io.pins.gpio2)),
            gpio3: Flex::new(AnyPin::new(io.pins.gpio3)),
            gpio4: Flex::new(AnyPin::new(io.pins.gpio4)),
            gpio5: Flex::new(AnyPin::new(io.pins.gpio5)),
            gpio6: Flex::new(AnyPin::new(io.pins.gpio6)),
            gpio7: Flex::new(AnyPin::new(io.pins.gpio7)),
            gpio8: Flex::new(AnyPin::new(io.pins.gpio8)),
            gpio10: Flex::new(AnyPin::new(io.pins.gpio10)),
            gpio11: Flex::new(AnyPin::new(io.pins.gpio11)),
            gpio14: Flex::new(AnyPin::new(io.pins.gpio14)),
            gpio15: Flex::new(AnyPin::new(io.pins.gpio15)),
            gpio18: Flex::new(AnyPin::new(io.pins.gpio18)),
            gpio19: Flex::new(AnyPin::new(io.pins.gpio19)),
            gpio20: Flex::new(AnyPin::new(io.pins.gpio20)),
            gpio21: Flex::new(AnyPin::new(io.pins.gpio21)),
            gpio22: Flex::new(AnyPin::new(io.pins.gpio22)),
            gpio23: Flex::new(AnyPin::new(io.pins.gpio23)),
        }
    }

    pub fn get_pin(&mut self, index: u32) -> &mut Flex<'a, AnyPin<'a>> {
        match index {
            0 => &mut self.gpio0,
            1 => &mut self.gpio1,
            2 => &mut self.gpio2,
            3 => &mut self.gpio3,
            4 => &mut self.gpio4,
            5 => &mut self.gpio5,
            6 => &mut self.gpio6,
            7 => &mut self.gpio7,
            8 => &mut self.gpio8,
            10 => &mut self.gpio10,
            11 => &mut self.gpio11,
            14 => &mut self.gpio14,
            15 => &mut self.gpio15,
            18 => &mut self.gpio18,
            19 => &mut self.gpio19,
            20 => &mut self.gpio20,
            21 => &mut self.gpio21,
            22 => &mut self.gpio22,
            23 => &mut self.gpio23,
            _ => panic!(),
        }
    }
}
