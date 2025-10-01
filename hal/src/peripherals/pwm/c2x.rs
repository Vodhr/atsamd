#![allow(non_snake_case)]

use atsamd_hal_macros::hal_cfg;

use crate::clock::{pclk, apb};
use crate::clock::types::{Tc0Tc1, Tc2Tc3, Tc4, Tcc0Tcc1, Tcc2};
use crate::clock::types;
use crate::gpio::*;
use crate::gpio::{AlternateE, AnyPin, Pin};

use crate::time::Hertz;
use crate::timer_params::TimerParams;

// Timer/Counter (TCx)

/// This is a major syntax hack.
///
/// The previous Pinout types were enums that took specific v1::Pin types. As a
/// result, there was no way to make that implementation simultaneously
/// compatible with both v1::Pin and Pin.
///
/// BUT, the enum variant syntax is the same as the namespaced function syntax.
/// I converted the enums to structs, and I created constructor methods with the
/// same names as the previous enum variants. By constructing Pinout types with
/// functions rather than enum variants, you can make it generic over v1::Pin
/// and Pin types.
///
/// This is (mostly) backwards compatible with the current syntax, and all the
/// existing calls compile. The only incompatible change is the requirement of
/// type parameters on the Pwm types. Most of the type, the type parameters can
/// be inferred, so this is mostly backwards compatible as well. But there were
/// one or two instances where I had to add explicit type parameters to existing
/// BSP code.
macro_rules! impl_tc_pinout {
    (
        $Type:ident: [ $(
            $( #[$attr:meta] )?
            ($func: ident, $Id: ident)
        ),+ ]
    ) => {
        pub struct $Type<I: PinId> {
            _pin: Pin<I, AlternateE>,
        }

        $(
            $( #[$attr] )?
            impl $Type<$Id> {
                #[inline]
                pub fn $func(pin: impl AnyPin<Id = $Id>) -> Self {
                    let _pin = pin.into().into_alternate();
                    Self { _pin }
                }
            }
        )+
    };
}

#[cfg(feature = "samc21n")]
#[hal_cfg("tc0")]
impl_tc_pinout!(TC0Pinout: [
    #[hal_cfg("pa08")]
    (Pa8, PA08),
    #[hal_cfg("pa09")]
    (Pa9, PA09),
    #[hal_cfg("pb30")]
    (Pb30, PB30),
    #[hal_cfg("pb31")]
    (Pb31, PB31)
]);

#[cfg(any(feature = "samc21e", feature = "samc21g", feature = "samc21j"))]
#[hal_cfg("tc0")]
impl_tc_pinout!(TC0Pinout: [
    #[hal_cfg("pb08")]
    (Pb8, PB08),
    #[hal_cfg("pb09")]
    (Pb9, PB09),
    #[hal_cfg("pb12")]
    (Pb12, PB12),
    #[hal_cfg("pb13")]
    (Pb13, PB13),
    #[hal_cfg("pa22")]
    (Pa22, PA22),
    #[hal_cfg("pa23")]
    (Pa23, PA23)
]);

#[cfg(feature = "samc21n")]
#[hal_cfg("tc1")]
impl_tc_pinout!(TC1Pinout: [
    #[hal_cfg("pa10")]
    (Pa10, PA10),
    #[hal_cfg("pa11")]
    (Pa11, PA11),
    #[hal_cfg("pa30")]
    (Pa30, PA30),
    #[hal_cfg("pa31")]
    (Pa31, PA31)
]);

#[cfg(any(feature = "samc21e", feature = "samc21g", feature = "samc21j"))]
#[hal_cfg("tc1")]
impl_tc_pinout!(TC1Pinout: [
    #[hal_cfg("pb10")]
    (Pb10, PB10),
    #[hal_cfg("pb11")]
    (Pb11, PB11),
    #[hal_cfg("pb14")]
    (Pb14, PB14),
    #[hal_cfg("pb15")]
    (Pb15, PB15),
    #[hal_cfg("pa24")]
    (Pa24, PA24),
    #[hal_cfg("pa25")]
    (Pa25, PA25)
]);

#[cfg(feature = "samc21n")]
#[hal_cfg("tc2")]
impl_tc_pinout!(TC2Pinout: [
    #[hal_cfg("pa00")]
    (Pa0, PA00),
    #[hal_cfg("pa01")]
    (Pa1, PA01),
    #[hal_cfg("pa12")]
    (Pa12, PA12),
    #[hal_cfg("pa13")]
    (Pa13, PA13),
    #[hal_cfg("pa16")]
    (Pa16, PA16),
    #[hal_cfg("pa17")]
    (Pa17, PA17)
]);

#[cfg(any(feature = "samc21e", feature = "samc21g", feature = "samc21j"))]
#[hal_cfg("tc2")]
impl_tc_pinout!(TC2Pinout: [
    #[hal_cfg("pb16")]
    (Pb16, PB16),
    #[hal_cfg("pb17")]
    (Pb17, PB17),
    #[hal_cfg("pb02")]
    (Pb2, PB02)
]);

#[cfg(feature = "samc21n")]
#[hal_cfg("tc3")]
impl_tc_pinout!(TC3Pinout: [
    #[hal_cfg("pa14")]
    (Pa14, PA14),
    #[hal_cfg("pa15")]
    (Pa15, PA15),
    #[hal_cfg("pa18")]
    (Pa18, PA18),
    #[hal_cfg("pa19")]
    (Pa19, PA19)
]);

#[cfg(any(feature = "samc21e", feature = "samc21g", feature = "samc21j"))]
#[hal_cfg("tc3")]
impl_tc_pinout!(TC3Pinout: [
    #[hal_cfg("pa20")]
    (Pa20, PA20),
    #[hal_cfg("pa21")]
    (Pa21, PA21),
    #[hal_cfg("pb22")]
    (Pb22, PB22),
    #[hal_cfg("pb23")]
    (Pb23, PB23),
    #[hal_cfg("pb00")]
    (Pb0, PB00),
    #[hal_cfg("pb01")]
    (Pb1, PB01)
]);

#[cfg(feature = "samc21n")]
#[hal_cfg("tc4")]
impl_tc_pinout!(TC4Pinout: [
    #[hal_cfg("pb12")]
    (Pb12, PB12),
    #[hal_cfg("pb13")]
    (Pb13, PB13),
    #[hal_cfg("pa22")]
    (Pa22, PA22),
    #[hal_cfg("pa23")]
    (Pa23, PA23)
]);

#[cfg(any(feature = "samc21e", feature = "samc21g", feature = "samc21j"))]
#[hal_cfg("tc4")]
impl_tc_pinout!(TC4Pinout: [
    #[hal_cfg("pa14")]
    (Pa14, PA14),
    #[hal_cfg("pa15")]
    (Pa15, PA15),
    #[hal_cfg("pa18")]
    (Pa18, PA18)
]);

#[cfg(feature = "samc21n")]
#[hal_cfg("tc5")]
impl_tc_pinout!(TC5Pinout: [
    #[hal_cfg("pb10")]
    (Pb10, PB10),
    #[hal_cfg("pb11")]
    (Pb11, PB11),
    #[hal_cfg("pb14")]
    (Pb14, PB14),
    #[hal_cfg("pb15")]
    (Pb15, PB15),
    #[hal_cfg("pa24")]
    (Pa24, PA24),
    #[hal_cfg("pa25")]
    (Pa25, PA25)
]);

#[cfg(feature = "samc21n")]
#[hal_cfg("tc6")]
impl_tc_pinout!(TC6Pinout: [
    #[hal_cfg("pb16")]
    (Pb16, PB16),
    #[hal_cfg("pb17")]
    (Pb17, PB17),
    #[hal_cfg("pb02")]
    (Pb2, PB02),
    #[hal_cfg("pb03")]
    (Pb3, PB03)
]);

#[cfg(feature = "samc21n")]
#[hal_cfg("tc7")]
impl_tc_pinout!(TC7Pinout: [
    #[hal_cfg("pa20")]
    (Pa20, PA20),
    #[hal_cfg("pa21")]
    (Pa21, PA21),
    #[hal_cfg("pb22")]
    (Pb22, PB22),
    #[hal_cfg("pb23")]
    (Pb23, PB23),
    #[hal_cfg("pb00")]
    (Pb0, PB00),
    #[hal_cfg("pb01")]
    (Pb1, PB01)
]);

macro_rules! pwm {
    ($($TYPE:ident: ($TC:ident, $pinout:ident, $pclk_id:ident, $apb_id:ident, $wrapper:ident)),+) => {
        $(

pub struct $TYPE<I: PinId> {
    freq: Hertz,
    apbclk: apb::ApbClk<types::$apb_id>,
    tc: crate::pac::$TC,
    #[allow(dead_code)]
    pinout: $pinout<I>,
}

impl<I: PinId> $TYPE<I> {
    #[inline]
    pub fn new<S: pclk::PclkSourceId>(
        pclk: pclk::Pclk<$pclk_id, S>,
        apbclk: apb::ApbClk<types::$apb_id>,
        freq: Hertz,
        tc: crate::pac::$TC,
        pinout: $pinout<I>,
    ) -> (Self, pclk::Pclk<$pclk_id, S>) {
        let count = tc.count16();
        let params = TimerParams::new(freq.convert(), pclk.freq());

        count.ctrla().write(|w| w.swrst().set_bit());
        while count.ctrla().read().bits() & 1 != 0 {}
        count.ctrla().modify(|_, w| w.enable().clear_bit());
        while count.syncbusy().read().enable().bit_is_set() {}
        count.ctrla().modify(|_, w| {
            match params.divider {
                1 => w.prescaler().div1(),
                2 => w.prescaler().div2(),
                4 => w.prescaler().div4(),
                8 => w.prescaler().div8(),
                16 => w.prescaler().div16(),
                64 => w.prescaler().div64(),
                256 => w.prescaler().div256(),
                1024 => w.prescaler().div1024(),
                _ => unreachable!(),
            }
        });
        count.wave().write(|w| w.wavegen().mpwm());
        count.cc(0).write(|w| unsafe { w.cc().bits(params.cycles as u16) });
        while count.syncbusy().read().cc0().bit_is_set() {}
        count.cc(1).write(|w| unsafe { w.cc().bits(0) });
        while count.syncbusy().read().cc1().bit_is_set() {}
        count.ctrla().modify(|_, w| w.enable().set_bit());
        while count.syncbusy().read().enable().bit_is_set() {}

        (Self {
            freq: pclk.freq(),
            apbclk: apbclk,
            tc: tc,
            pinout: pinout,
        },
        pclk)
    }

    #[inline]
    pub fn get_period(&self) -> Hertz {
        let count = self.tc.count16();
        let divisor = count.ctrla().read().prescaler().bits();
        let top = count.cc(0).read().cc().bits();
        self.freq / divisor as u32 / (top + 1) as u32
    }

    #[inline]
    pub fn set_period(&mut self, period: Hertz)
    {
        let period = period.into();
        let params = TimerParams::new(period, self.freq);
        let count = self.tc.count16();
        count.ctrla().modify(|_, w| w.enable().clear_bit());
        while count.syncbusy().read().enable().bit_is_set() {}
        count.ctrla().modify(|_, w| {
                match params.divider {
                    1 => w.prescaler().div1(),
                    2 => w.prescaler().div2(),
                    4 => w.prescaler().div4(),
                    8 => w.prescaler().div8(),
                    16 => w.prescaler().div16(),
                    64 => w.prescaler().div64(),
                    256 => w.prescaler().div256(),
                    1024 => w.prescaler().div1024(),
                    _ => unreachable!(),
                }
            });
        count.ctrla().modify(|_, w| w.enable().set_bit());
        while count.syncbusy().read().enable().bit_is_set() {}
        count.cc(0).write(|w| unsafe { w.cc().bits(params.cycles as u16) });
        while count.syncbusy().read().cc0().bit_is_set() {}
    }
}

impl<I: PinId> $crate::ehal::pwm::ErrorType for$TYPE<I> {
    type Error = ::core::convert::Infallible;
}

impl<I: PinId> $crate::ehal::pwm::SetDutyCycle for $TYPE<I> {
    #[inline]
    fn max_duty_cycle(&self) -> u16 {
        let count = self.tc.count16();
        let top = count.cc(0).read().cc().bits();
        top
    }

    #[inline]
    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        let count = self.tc.count16();
        unsafe { count.ccbuf(1).write(|w| w.ccbuf().bits(duty)); }
        Ok(())
    }
}

impl<I: PinId> $crate::ehal_02::PwmPin for $TYPE<I> {
    type Duty = u16;

    #[inline]
    fn disable(&mut self) {
        let count = self.tc.count16();
        count.ctrla().modify(|_, w| w.enable().clear_bit());
        while count.syncbusy().read().enable().bit_is_set() {}
    }

    #[inline]
    fn enable(&mut self) {
        let count = self.tc.count16();
        count.ctrla().modify(|_, w| w.enable().set_bit());
        while count.syncbusy().read().enable().bit_is_set() {}
    }


    #[inline]
    fn get_duty(&self) -> Self::Duty {
        let count = self.tc.count16();
        let duty: u16 = count.ccbuf(1).read().ccbuf().bits();
        duty
    }

    #[inline]
    fn get_max_duty(&self) -> Self::Duty {
        use $crate::ehal::pwm::SetDutyCycle;
        self.max_duty_cycle()
    }

    #[inline]
    fn set_duty(&mut self, duty: Self::Duty) {
        use $crate::ehal::pwm::SetDutyCycle;
        let _ignore_infaillible = self.set_duty_cycle(duty);
    }
}

)+}}

#[hal_cfg("tc0")]
pwm! { Pwm0: (Tc0, TC0Pinout, Tc0Tc1, Tc0, Pwm0Wrapper) }
#[hal_cfg("tc1")]
pwm! { Pwm1: (Tc1, TC1Pinout, Tc0Tc1, Tc1, Pwm1Wrapper) }
#[hal_cfg("tc2")]
pwm! { Pwm2: (Tc2, TC2Pinout, Tc2Tc3, Tc2, Pwm2Wrapper) }
#[hal_cfg("tc3")]
pwm! { Pwm3: (Tc3, TC3Pinout, Tc2Tc3, Tc3, Pwm3Wrapper) }
#[hal_cfg("tc4")]
pwm! { Pwm4: (Tc4, TC4Pinout, Tc4, Tc4, Pwm4Wrapper) }

// Timer/Counter for Control Applications (TCCx)

#[derive(Copy, Clone)]
pub enum Channel {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
}

/// This is a major syntax hack.
///
/// The previous Pinout types were enums that took specific v1::Pin types. As a
/// result, there was no way to make that implementation simultaneously
/// compatible with both v1::Pin and Pin.
///
/// BUT, the enum variant syntax is the same as the namespaced function syntax.
/// I converted the enums to structs, and I created constructor methods with the
/// same names as the previous enum variants. By constructing Pinout types with
/// functions rather than enum variants, you can make it generic over v1::Pin
/// and Pin types.
///
/// This is (mostly) backwards compatible with the current syntax, and all the
/// existing calls compile. The only incompatible change is the requirement of
/// type parameters on the Pwm types. Most of the type, the type parameters can
/// be inferred, so this is mostly backwards compatible as well. But there were
/// one or two instances where I had to add explicit type parameters to existing
/// BSP code.
macro_rules! impl_tcc_pinout {
    (
        $Type:ident: [ $(
            $( #[$attr:meta] )?
            ($func: ident, $Id: ident, $Mode:ident)
        ),+ ]
    ) => {
        pub struct $Type<I: PinId, M: PinMode> {
            _pin: Pin<I, M>,
        }

        $(
            $( #[$attr] )?
            impl $Type<$Id, $Mode> {
                #[inline]
                pub fn $func(pin: impl AnyPin<Id = $Id>) -> Self {
                    let _pin = pin.into().into_alternate();
                    Self { _pin }
                }
            }
        )+
    };
}

// todo: add samc20 pins
#[cfg(feature = "samc21n")]
#[hal_cfg("tcc0")]
impl_tcc_pinout!(TCC0Pinout: [
    #[hal_cfg("pa08")]
    (Pa8, PA08, AlternateF),
    #[hal_cfg("pa09")]
    (Pa9, PA09, AlternateF),
    #[hal_cfg("pa10")]
    (Pa10, PA10, AlternateF),
    #[hal_cfg("pa11")]
    (Pa11, PA11, AlternateF),
    #[hal_cfg("pb11")]
    (Pb11, PB11, AlternateF),
    #[hal_cfg("pb12")]
    (Pb12, PB12, AlternateF),
    #[hal_cfg("pb13")]
    (Pb13, PB13, AlternateF),
    #[hal_cfg("pa12")]
    (Pa12, PA12, AlternateF),
    #[hal_cfg("pa13")]
    (Pa13, PA13, AlternateF)
]);

#[cfg(any(feature = "samc21e", feature = "samc21g", feature = "samc21j"))]
#[hal_cfg("tcc0")]
impl_tcc_pinout!(TCC0Pinout: [
    #[hal_cfg("pa04")]
    (Pa4, PA04, AlternateE),
    #[hal_cfg("pa05")]
    (Pa5, PA05, AlternateE),
    #[hal_cfg("pa08")]
    (Pa8, PA08, AlternateE),
    #[hal_cfg("pa09")]
    (Pa9, PA09, AlternateE),
    #[hal_cfg("pb30")]
    (Pb30, PB30, AlternateE),
    #[hal_cfg("pb31")]
    (Pb31, PB31, AlternateE),
    #[hal_cfg("pa10")]
    (Pa10, PA10, AlternateF),
    #[hal_cfg("pa11")]
    (Pa11, PA11, AlternateF),
    #[hal_cfg("pb10")]
    (Pb10, PB10, AlternateF),
    #[hal_cfg("pb11")]
    (Pb11, PB11, AlternateF),
    #[hal_cfg("pb12")]
    (Pb12, PB12, AlternateF),
    #[hal_cfg("pb13")]
    (Pb13, PB13, AlternateF),
    #[hal_cfg("pa12")]
    (Pa12, PA12, AlternateF),
    #[hal_cfg("pa13")]
    (Pa13, PA13, AlternateF),
    #[hal_cfg("pa14")]
    (Pa14, PA14, AlternateF),
    #[hal_cfg("pa15")]
    (Pa15, PA15, AlternateF),
    #[hal_cfg("pa16")]
    (Pa16, PA16, AlternateF),
    #[hal_cfg("pa17")]
    (Pa17, PA17, AlternateF),
    #[hal_cfg("pa18")]
    (Pa18, PA18, AlternateF),
    #[hal_cfg("pa19")]
    (Pa19, PA19, AlternateF),
    #[hal_cfg("pb16")]
    (Pb16, PB13, AlternateF),
    #[hal_cfg("pb17")]
    (Pb17, PB17, AlternateF),
    #[hal_cfg("pa20")]
    (Pa20, PA20, AlternateF),
    #[hal_cfg("pa21")]
    (Pa21, PA21, AlternateF),
    #[hal_cfg("pa22")]
    (Pa22, PA22, AlternateF),
    #[hal_cfg("pa23")]
    (Pa23, PA23, AlternateF)
]);

#[cfg(feature = "samc21n")]
#[hal_cfg("tcc1")]
impl_tcc_pinout!(TCC1Pinout: [
    #[hal_cfg("pa16")]
    (Pa16, PA16, AlternateF),
    #[hal_cfg("pa17")]
    (Pa17, PA17, AlternateF),
    #[hal_cfg("pa18")]
    (Pa18, PA18, AlternateF),
    #[hal_cfg("pa19")]
    (Pa19, PA19, AlternateF),
    #[hal_cfg("pa22")]
    (Pa22, PA22, AlternateF),
    #[hal_cfg("pa23")]
    (Pa23, PA23, AlternateF),
    #[hal_cfg("pb22")]
    (Pa22, PA22, AlternateF),
    #[hal_cfg("pb23")]
    (Pa23, PA23, AlternateF)
]);

#[cfg(any(feature = "samc21e", feature = "samc21g", feature = "samc21j"))]
#[hal_cfg("tcc1")]
impl_tcc_pinout!(TCC1Pinout: [
    #[hal_cfg("pa06")]
    (Pa6, PA06, AlternateE),
    #[hal_cfg("pa07")]
    (Pa7, PA07, AlternateE),
    #[hal_cfg("pa10")]
    (Pa10, PA10, AlternateE),
    #[hal_cfg("pa11")]
    (Pa11, PA11, AlternateE),
    #[hal_cfg("pa30")]
    (Pa30, PA30, AlternateE),
    #[hal_cfg("pa31")]
    (Pa31, PA31, AlternateE),
    #[hal_cfg("pa08")]
    (Pa8, PA08, AlternateF),
    #[hal_cfg("pa09")]
    (Pa9, PA09, AlternateF),
    #[hal_cfg("pa24")]
    (Pa24, PA24, AlternateF),
    #[hal_cfg("pa25")]
    (Pa25, PA25, AlternateF),
    #[hal_cfg("pb30")]
    (Pb30, PB30, AlternateF),
    #[hal_cfg("pb31")]
    (Pb31, PB31, AlternateF)
]);

#[cfg(feature = "samc21n")]
#[hal_cfg("tcc2")]
impl_tcc_pinout!(TCC2Pinout: [
    #[hal_cfg("pc03")]
    (Pc3, PC03, AlternateF),
    #[hal_cfg("pc05")]
    (Pc5, PC05, AlternateF),
    #[hal_cfg("pa20")]
    (Pa20, PA20, AlternateF),
    #[hal_cfg("pa21")]
    (Pa21, PA21, AlternateF),
    #[hal_cfg("pa24")]
    (Pa24, PA24, AlternateF),
    #[hal_cfg("pa25")]
    (Pa25, PA25, AlternateF)
]);

#[cfg(any(feature = "samc21e", feature = "samc21g", feature = "samc21j"))]
#[hal_cfg("tcc2")]
impl_tcc_pinout!(TCC2Pinout: [
    #[hal_cfg("pa00")]
    (Pa0, PA00, AlternateE),
    #[hal_cfg("pa01")]
    (Pa1, PA01, AlternateE),
    #[hal_cfg("pa12")]
    (Pa12, PA12, AlternateE),
    #[hal_cfg("pa13")]
    (Pa13, PA13, AlternateE),
    #[hal_cfg("pa16")]
    (Pa16, PA16, AlternateE),
    #[hal_cfg("pa17")]
    (Pa17, PA17, AlternateE)
]);

macro_rules! pwm_tcc {
    ($($TYPE:ident: ($TCC:ident, $pinout:ident, $pclk_id:ident, $apb_id:ident, $wrapper:ident)),+) => {
        $(

pub struct $TYPE<I: PinId, M: PinMode, S: pclk::PclkSourceId> {
    pclk: pclk::Pclk<$pclk_id, S>,
    apbclk: apb::ApbClk<types::$apb_id>,
    tcc: crate::pac::$TCC,
    #[allow(dead_code)]
    pinout: $pinout<I, M>,
}

impl<I: PinId, M: PinMode, S: pclk::PclkSourceId> $TYPE<I, M, S> {
    #[inline]
    pub fn new(
        pclk: pclk::Pclk<$pclk_id, S>,
        apbclk: apb::ApbClk<types::$apb_id>,
        freq: Hertz,
        tcc: crate::pac::$TCC,
        pinout: $pinout<I, M>,
    ) -> Self {
        let params = TimerParams::new(freq.convert(), pclk.freq());

        // TODO Disabled due to previous configuration actions
        // tcc.ctrla().write(|w| w.swrst().set_bit());
        // while tcc.syncbusy().read().swrst().bit_is_set() {}
        tcc.ctrlbclr().write(|w| w.dir().set_bit() );
        while tcc.syncbusy().read().ctrlb().bit_is_set() {}
        tcc.ctrla().modify(|_, w| w.enable().clear_bit());
        while tcc.syncbusy().read().enable().bit_is_set() {}
        tcc.ctrla().modify(|_, w| {
            match params.divider {
                1 => w.prescaler().div1(),
                2 => w.prescaler().div2(),
                4 => w.prescaler().div4(),
                8 => w.prescaler().div8(),
                16 => w.prescaler().div16(),
                64 => w.prescaler().div64(),
                256 => w.prescaler().div256(),
                1024 => w.prescaler().div1024(),
                _ => unreachable!(),
            }
        });
        tcc.wave().write(|w| w.wavegen().npwm());
        while tcc.syncbusy().read().wave().bit_is_set() {}
        tcc.per().write(|w| unsafe { w.bits(params.cycles as u32) });
        while tcc.syncbusy().read().per().bit_is_set() {}

        tcc.ctrlbclr().write(|w| w.lupd().set_bit());

        tcc.ctrla().modify(|_, w| w.enable().set_bit());
        while tcc.syncbusy().read().enable().bit_is_set() {}

        Self {
            pclk,
            apbclk,
            tcc,
            pinout,
        }
    }

    #[inline]
    pub fn set_duty_3x(&mut self, channel0: Channel, channel1: Channel, channel2: Channel, duty0: u32, duty1: u32, duty2: u32) {
        self.tcc.ccbuf(channel0 as usize).write(|w| unsafe { w.ccbuf().bits(duty0) });
        self.tcc.ccbuf(channel1 as usize).write(|w| unsafe { w.ccbuf().bits(duty1) });
        self.tcc.ccbuf(channel2 as usize).write(|w| unsafe { w.ccbuf().bits(duty2) });
    }
}

impl<I: PinId, M: PinMode, S: pclk::PclkSourceId> $crate::ehal_02::Pwm for $TYPE<I, M, S> {
    type Channel = Channel;
    type Time = Hertz;
    type Duty = u32;

    #[inline]
    fn disable(&mut self, _channel: Self::Channel) {
        self.tcc.ctrla().modify(|_, w| w.enable().clear_bit());
        while self.tcc.syncbusy().read().enable().bit_is_set() {}
    }

    #[inline]
    fn enable(&mut self, _channel: Self::Channel) {
        self.tcc.ctrla().modify(|_, w| w.enable().set_bit());
        while self.tcc.syncbusy().read().enable().bit_is_set() {}
    }

    #[inline]
    fn get_period(&self) -> Self::Time {
        let divisor = self.tcc.ctrla().read().prescaler().bits();
        let top = self.tcc.per().read().bits();
        self.pclk.freq() / divisor as u32 / (top + 1) as u32
    }

    #[inline]
    fn get_duty(&self, channel: Self::Channel) -> Self::Duty {
        let cc = &self.tcc.cc(channel as usize);
        let duty = cc.read().cc().bits();
        duty
    }

    #[inline]
    fn get_max_duty(&self) -> Self::Duty {
        let top = self.tcc.per().read().bits();
        top
    }

    #[inline]
    fn set_duty(&mut self, channel: Self::Channel, duty: Self::Duty) {
        self.tcc.cc(channel as usize).write(|w| unsafe { w.cc().bits(duty) });
        while self.tcc.syncbusy().read().bits() != 0 {}
    }

    #[inline]
    fn set_period<P>(&mut self, period: P)
    where
        P: Into<Self::Time>,
    {
        let params = TimerParams::new(period.into().convert(), self.pclk.freq());
        self.tcc.ctrla().modify(|_, w| w.enable().clear_bit());
        while self.tcc.syncbusy().read().enable().bit_is_set() {}
        self.tcc.ctrla().modify(|_, w| {
            match params.divider {
                1 => w.prescaler().div1(),
                2 => w.prescaler().div2(),
                4 => w.prescaler().div4(),
                8 => w.prescaler().div8(),
                16 => w.prescaler().div16(),
                64 => w.prescaler().div64(),
                256 => w.prescaler().div256(),
                1024 => w.prescaler().div1024(),
                _ => unreachable!(),
            }
        });
        self.tcc.ctrla().modify(|_, w| w.enable().set_bit());
        while self.tcc.syncbusy().read().enable().bit_is_set() {}
        self.tcc.per().write(|w| unsafe { w.bits(params.cycles as u32) });
        while self.tcc.syncbusy().read().per().bit() {}
    }
}
        )+
    };
}

#[hal_cfg("tcc0")]
pwm_tcc! { Tcc0Pwm: (Tcc0, TCC0Pinout, Tcc0Tcc1, Tcc0, TccPwm0Wrapper) }
#[hal_cfg("tcc1")]
pwm_tcc! { Tcc1Pwm: (Tcc1, TCC1Pinout, Tcc0Tcc1, Tcc1, TccPwm1Wrapper) }
#[hal_cfg("tcc2")]
pwm_tcc! { Tcc2Pwm: (Tcc2, TCC2Pinout, Tcc2, Tcc2, TccPwm2Wrapper) }