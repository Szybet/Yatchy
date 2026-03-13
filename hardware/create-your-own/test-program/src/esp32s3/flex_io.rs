use esp_hal::gpio::{Flex};
use crate::CurrentAction;

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
    pub gpio8: Flex<'a>,
    pub gpio9: Flex<'a>,
    pub gpio10: Flex<'a>,
    pub gpio11: Flex<'a>,
    pub gpio12: Flex<'a>,
    pub gpio13: Flex<'a>,
    pub gpio14: Flex<'a>,
    pub gpio15: Flex<'a>,
    pub gpio16: Flex<'a>,
    pub gpio17: Flex<'a>,
    pub gpio18: Flex<'a>,
    pub gpio19: Flex<'a>,
    pub gpio20: Flex<'a>,
    pub gpio21: Flex<'a>,
    pub gpio38: Flex<'a>,
    pub gpio39: Flex<'a>,
    pub gpio40: Flex<'a>,
    pub gpio41: Flex<'a>,
    pub gpio42: Flex<'a>,
    pub gpio43: Flex<'a>,
    pub gpio44: Flex<'a>,
    pub gpio45: Flex<'a>,
    pub gpio46: Flex<'a>,
    pub gpio47: Flex<'a>,
    pub gpio48: Flex<'a>,
}

impl<'a> FlexIo<'a> {
    pub fn handle_command(&self, action: &str) -> Option<CurrentAction> {
        if action.starts_with("gpio") {
            let num_str = &action[4..];
            if let Ok(num) = num_str.parse::<u32>() {
                return Some(CurrentAction::Gpio(num));
            }
        }
        if action == "self_check_gpio" {
            return Some(CurrentAction::SelfCheckGpio());
        }
        None
    }

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
            8 => Some(&mut self.gpio8),
            9 => Some(&mut self.gpio9),
            10 => Some(&mut self.gpio10),
            11 => Some(&mut self.gpio11),
            12 => Some(&mut self.gpio12),
            13 => Some(&mut self.gpio13),
            14 => Some(&mut self.gpio14),
            15 => Some(&mut self.gpio15),
            16 => Some(&mut self.gpio16),
            17 => Some(&mut self.gpio17),
            18 => Some(&mut self.gpio18),
            19 => Some(&mut self.gpio19),
            20 => Some(&mut self.gpio20),
            21 => Some(&mut self.gpio21),
            38 => Some(&mut self.gpio38),
            39 => Some(&mut self.gpio39),
            40 => Some(&mut self.gpio40),
            41 => Some(&mut self.gpio41),
            42 => Some(&mut self.gpio42),
            43 => Some(&mut self.gpio43),
            44 => Some(&mut self.gpio44),
            45 => Some(&mut self.gpio45),
            46 => Some(&mut self.gpio46),
            47 => Some(&mut self.gpio47),
            48 => Some(&mut self.gpio48),
            _ => None,
        }
    }
}
