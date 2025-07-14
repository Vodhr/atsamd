//! Working with timer counter hardware
use atsamd_hal_macros::hal_cfg;

use crate::pac::tc0::Count16 as Count16Reg;
use crate::pac;

use crate::clock::types;
use crate::clock::types::{Tc0, Tc0Tc1, Tc2Tc3, Tc4};
#[hal_cfg("tc7")]
use crate::clock::types::{Tc5, Tc6, Tc7};

use crate::clock::pclk;
use crate::time::Hertz;

mod common;
pub use common::Count16;

#[cfg(feature = "async")]
mod async_api;

#[cfg(feature = "async")]
pub use async_api::*;
use crate::clock::apb;
use crate::clock::apb::ApbId;
use crate::clock::pclk::Pclk;
// Note:
// TC3 + TC4 can be paired to make a 32-bit counter
// TC5 + TC6 can be paired to make a 32-bit counter

/// A generic hardware timer counter.
///
/// The counters are exposed in 16-bit mode only.
/// The hardware allows configuring the 8-bit mode
/// and pairing up some instances to run in 32-bit
/// mode, but that functionality is not currently
/// exposed by this hal implementation.
/// TimerCounter implements both the `Periodic` and
/// the `CountDown` embedded_hal timer traits.
/// Before a hardware timer can be used, it must first
/// have a clock configured.
pub struct TimerCounter<TC, A: ApbId = Tc0> {
    freq: Hertz,
    apb_clk: apb::ApbClk<A>,
    tc: TC,
}
impl<TC> TimerCounter<TC>
where
    TC: Count16,
{
    /// Starts the 16-bit timer, counting up in periodic mode.
    fn start_timer(&mut self, divider: u16, cycles: u16) {
        // Disable the timer while we reconfigure it
        self.disable();

        let count = self.tc.count_16();

        // Now that we have a clock routed to the peripheral, we
        // can ask it to perform a reset.
        count.ctrla().write(|w| w.swrst().set_bit());
        while count.syncbusy().read().swrst().bit_is_set() {}

        count.ctrlbset().write(|w| {
            // Count up when the direction bit is zero
            w.dir().clear_bit();
            // Periodic
            w.oneshot().clear_bit()
        });

        // Set TOP value for mfrq mode
        count.cc(0).write(|w| unsafe { w.cc().bits(cycles) });

        // Enable Match Frequency Waveform generation
        count.wave().modify(|_, w| w.wavegen().mfrq());

        count.ctrla().modify(|_, w| {
            match divider {
                1 => w.prescaler().div1(),
                2 => w.prescaler().div2(),
                4 => w.prescaler().div4(),
                8 => w.prescaler().div8(),
                16 => w.prescaler().div16(),
                64 => w.prescaler().div64(),
                256 => w.prescaler().div256(),
                1024 => w.prescaler().div1024(),
                _ => unreachable!(),
            };
            w.enable().set_bit();
            w.runstdby().set_bit()
        });
    }

    /// Disable the timer
    fn disable(&mut self) {
        let count = self.tc.count_16();

        count.ctrla().modify(|_, w| w.enable().clear_bit());
        while count.syncbusy().read().enable().bit_is_set() {}
    }
}

macro_rules! tc {
    ($($TYPE:ident: ($TC:ident, $pclk_id:ident, $apb_id:ident),)+) => {
        $(
pub type $TYPE = TimerCounter<pac::$TC>;

impl Count16 for pac::$TC {
    fn count_16(&self) -> &Count16Reg {
        self.count16()
    }
}

impl TimerCounter<pac::$TC, types::$TC> {
    pub fn new<S: pclk::PclkSourceId>(
        apb_clk: apb::ApbClk<types::$apb_id>, tc: pac::$TC, pclk: &Pclk<$pclk_id, S>) -> Self {
        // this is safe because we're constrained to just the tc3 bit
        {
            let count = tc.count16();

            // Disable the timer while we reconfigure it
            count.ctrla().modify(|_, w| w.enable().clear_bit());
            while count.syncbusy().read().enable().bit_is_set()  {}
        }
        Self {
            freq: pclk.freq(),
            apb_clk,
            tc,
        }
    }
}
        )+
    }
}

tc! {
    TimerCounter0: (Tc0, Tc0Tc1, Tc0),
    TimerCounter1: (Tc1, Tc0Tc1, Tc1),
    TimerCounter2: (Tc2, Tc2Tc3, Tc2),
    TimerCounter3: (Tc3, Tc2Tc3, Tc3),
    TimerCounter4: (Tc4, Tc4, Tc4),
}

#[hal_cfg("tc7")]
tc! {
    TimerCounter3: (Tc5, Tc5, Tc5),
    TimerCounter4: (Tc6, Tc6, Tc6),
    TimerCounter4: (Tc7, Tc7, Tc7),
}
