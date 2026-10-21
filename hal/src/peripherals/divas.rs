use crate::clock::ahb::AhbClk;
use crate::clock::types;
use crate::pac;

pub struct Divas {
    divas: pac::Divas,
    ahbclk: AhbClk<types::Divas>,
}

impl Divas {
    pub fn new(divas: pac::Divas, ahbclk: AhbClk<types::Divas>) -> Self {
        Self {
            divas, ahbclk,
        }
    }

    #[inline]
    pub fn sqrt(&self, num: u32) -> (u32, u32) {
        self.divas.sqrnum().write(|w| unsafe { w.bits(num) });
        self.wait();
        (self.divas.result().read().bits(), self.divas.rem().read().bits())
    }

    #[inline]
    pub fn checked_div_unsigned(&self, dividend: u32, divisor: u32) -> Option<(u32, u32)> {
        match divisor {
            0 => None,
            _ => {
                self.divas.ctrla().write(|w| w
                    .dlz().set_bit()
                    .signed().clear_bit());

                self.divas.dividend().write(|w| unsafe { w.bits(dividend) });
                self.divas.divisor().write(|w| unsafe { w.bits(divisor) });

                self.wait();

                Some((self.divas.result().read().bits(), self.divas.rem().read().bits()))
            }
        }
    }

    #[inline]
    pub fn checked_div_signed(&self, dividend: i32, divisor: i32) -> Option<(i32, i32)> {
        match divisor {
            0 => None,
            _ => {
                self.divas.ctrla().write(|w| w
                    .dlz().set_bit()
                    .signed().set_bit());

                self.divas.dividend().write(|w| unsafe { w.bits(dividend as u32) });

                self.divas.divisor().write(|w| unsafe { w.bits(divisor as u32) });

                self.wait();

                Some((self.divas.result().read().bits() as i32,
                      self.divas.rem().read().bits() as i32))
            }
        }
    }


    #[inline]
    fn wait(&self) {
        while self.divas.status().read().busy().bit_is_set() {}
    }
}