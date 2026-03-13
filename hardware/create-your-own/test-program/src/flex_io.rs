use esp_hal::gpio::{Flex};

// https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf
/*
ESP32-C6 has five strapping pins:
• MTMS
• MTDI
• GPIO9
• GPIO15
*/

pub struct FlexIo<'a> {
    pub current_output: Option<u32>,
    pub gpio0: Flex<'a>,
    pub gpio1: Flex<'a>,
    pub gpio2: Flex<'a>,
    pub gpio3: Flex<'a>,
    pub gpio4: Flex<'a>,
    pub gpio5: Flex<'a>,
    pub gpio6: Flex<'a>,
    pub gpio7: Flex<'a>,
    pub gpio10: Flex<'a>,
    pub gpio11: Flex<'a>,
    #[cfg(feature = "uart")]
    pub gpio12: Flex<'a>,
    #[cfg(feature = "uart")]
    pub gpio13: Flex<'a>,
    pub gpio14: Flex<'a>,
    pub gpio15: Flex<'a>,
    pub gpio18: Flex<'a>,
    pub gpio19: Flex<'a>,
    pub gpio20: Flex<'a>,
    pub gpio21: Flex<'a>,
    #[cfg(feature = "i2c")]
    pub gpio22: Flex<'a>,
    #[cfg(feature = "i2c")]          
    pub gpio23: Flex<'a>,
}

impl<'a> FlexIo<'a> {
    pub fn get_pin(&mut self, index: u32) -> Option<&mut Flex<'a>> {
        match index {
            0 => Some(&mut self.gpio0),
            1 => Some(&mut self.gpio1),
            2 => Some(&mut self.gpio2),
            3 => Some(&mut self.gpio3),
            4 => Some(&mut self.gpio4),
            5 => Some(&mut self.gpio5),
            6 => Some(&mut self.gpio6),
            7 => Some(&mut self.gpio7),
            10 => Some(&mut self.gpio10),
            11 => Some(&mut self.gpio11),
            #[cfg(feature = "uart")]
            12 => Some(&mut self.gpio12),
            #[cfg(feature = "uart")]
            13 => Some(&mut self.gpio13),
            14 => Some(&mut self.gpio14),
            15 => Some(&mut self.gpio15),
            18 => Some(&mut self.gpio18),
            19 => Some(&mut self.gpio19),
            20 => Some(&mut self.gpio20),
            21 => Some(&mut self.gpio21),
            #[cfg(feature = "i2c")]
            22 => Some(&mut self.gpio22),
            #[cfg(feature = "i2c")]          
            23 => Some(&mut self.gpio23),
            _ => None,
        }
    }
}
