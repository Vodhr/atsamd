//! Analogue-to-Digital Conversion
use atsamd_hal_macros::hal_cfg;

#[rustfmt::skip]
use crate::gpio::*;
use crate::ehal_02::adc::{Channel, OneShot};

use crate::pac::{adc0, ADC0, ADC1};
use crate::clock::pclk;
use crate::clock::apb;
use crate::clock::types::{Adc0 as Adc0Id, Adc1 as Adc1Id};

use crate::calibration;

/// Samples per reading
pub use adc0::avgctrl::SAMPLENUM_A as SampleRate;
/// Clock frequency relative to the system clock
pub use adc0::ctrlb::PRESCALER_A as Prescaler;
/// Reading resolution in bits
pub use adc0::ctrlc::RESSEL_A as Resolution;
/// Reference voltage (or its source)
pub use adc0::refctrl::REFSEL_A as Reference;

/// An ADC where results are accessible via interrupt servicing.
pub struct InterruptAdc<A: Adc, C: ConversionMode<A>> {
    adc: A,
    m: core::marker::PhantomData<C>,
}

/// Describes how an interrupt-driven ADC should finalize the peripheral
/// upon the completion of a conversion.
pub trait ConversionMode<A: Adc> {
    fn on_start(adc: &mut A);
    fn on_complete(adc: &mut A);
    fn on_stop(adc: &mut A);
}

pub struct SingleConversion<A: Adc> {
    m: core::marker::PhantomData<A>,
}
pub struct FreeRunning<A: Adc> {
    m: core::marker::PhantomData<A>,
}

pub trait Adc: Sized {
    fn synchronous_convert(&mut self) -> u16;
    fn enable_interrupts(&mut self);
    fn disable_interrupts(&mut self);
    fn power_up(&mut self);
    fn power_down(&mut self);
    fn start_conversion(&mut self);
    fn enable_freerunning(&mut self);
    fn disable_freerunning(&mut self);
    fn service_interrupt_ready(&mut self) -> Option<u16>;
    fn mux<PIN: Channel<Self, ID=u8>>(&mut self, _pin: &mut PIN);
}

macro_rules! adc_hal {
    ($($Adc:ident: ($ADC:ident, $pclk_id:ident, $apb_id:ident, $compcal:ident, $refcal:ident),)+) => {
        $(
pub struct $Adc<S: pclk::PclkSourceId> {
    adc: $ADC,
    pclk: pclk::Pclk<$pclk_id, S>,
    apbclk: apb::ApbClk<$apb_id>,
}

impl<S: pclk::PclkSourceId> $Adc<S> {
    pub fn new(adc: $ADC, pclk: pclk::Pclk<$pclk_id, S>, apbclk: apb::ApbClk<$apb_id>) -> Self {
        adc.ctrlb.modify(|_, w| w.prescaler().div32());
        adc.ctrlc.modify(|_, w| w.ressel()._12bit());
        while adc.syncbusy.read().ctrlc().bit_is_set() {}
        adc.sampctrl.modify(|_, w| unsafe {w.samplen().bits(5)}); // sample length
        while adc.syncbusy.read().sampctrl().bit_is_set() {}
        adc.inputctrl.modify(|_, w| w.muxneg().gnd()); // No negative input (internal gnd)
        while adc.syncbusy.read().inputctrl().bit_is_set() {}

        adc.calib.write(|w| unsafe {
            w.biascomp().bits(calibration::$compcal());
            w.biasrefbuf().bits(calibration::$refcal())
        });

        let mut newadc = Self { adc, pclk, apbclk };

        newadc.samples(SampleRate::_1);
        newadc.reference(Reference::INTREF);

        newadc
    }

    pub fn samples(&mut self, samples: SampleRate) {
        self.adc.avgctrl.modify(|_, w| {
            w.samplenum().variant(samples);
            unsafe {
                // Table 38-2 (38.6.2.10) specifies the adjres
                // values necessary for each SAMPLENUM value.
                w.adjres().bits(match samples {
                    SampleRate::_1 => 0,
                    SampleRate::_2 => 1,
                    SampleRate::_4 => 2,
                    SampleRate::_8 => 3,
                    _ => 4,
                })
            }
        });
        while self.adc.syncbusy.read().avgctrl().bit_is_set() {}
    }

    /// Set the voltage reference
    pub fn reference(&mut self, reference: Reference) {
        self.adc
            .refctrl
            .modify(|_, w| w.refsel().variant(reference));
        // Note there is no syncbusy for refctrl
    }

    /// Set the prescaler for adjusting the clock relative to the system clock
    pub fn prescaler(&mut self, prescaler: Prescaler) {
        self.adc
            .ctrlb
            .modify(|_, w| w.prescaler().variant(prescaler));
        // Note there is no syncbusy for ctrlb
    }

    /// Set the input resolution
    pub fn resolution(&mut self, resolution: Resolution) {
        self.adc
            .ctrlc
            .modify(|_, w| w.ressel().variant(resolution));
        while self.adc.syncbusy.read().ctrlc().bit_is_set() {}
    }
}

impl<S: pclk::PclkSourceId> Adc for $Adc<S> {
    fn power_up(&mut self) {
        while self.adc.syncbusy.read().enable().bit_is_set() {}
        self.adc.ctrla.modify(|_, w| w.enable().set_bit());
        while self.adc.syncbusy.read().enable().bit_is_set() {}
    }

    fn power_down(&mut self) {
        while self.adc.syncbusy.read().enable().bit_is_set() {}
        self.adc.ctrla.modify(|_, w| w.enable().clear_bit());
        while self.adc.syncbusy.read().enable().bit_is_set() {}
    }

    #[inline(always)]
    fn start_conversion(&mut self) {
        // start conversion
        self.adc.swtrig.modify(|_, w| w.start().set_bit());
        // do it again because the datasheet tells us to
        // self.adc.swtrig.modify(|_, w| w.start().set_bit());
    }

    fn enable_freerunning(&mut self) {
        self.adc.ctrlc.modify(|_, w| w.freerun().set_bit());
        while self.adc.syncbusy.read().ctrlc().bit_is_set() {}
    }

    fn disable_freerunning(&mut self) {
        self.adc.ctrlc.modify(|_, w| w.freerun().set_bit());
        while self.adc.syncbusy.read().ctrlc().bit_is_set() {}
    }

    fn synchronous_convert(&mut self) -> u16 {
        self.start_conversion();
        while self.adc.intflag.read().resrdy().bit_is_clear() {}

        self.adc.result.read().result().bits()
    }

    /// Enables an interrupt when conversion is ready.
    fn enable_interrupts(&mut self) {
        self.adc.intflag.write(|w| w.resrdy().set_bit());
        self.adc.intenset.write(|w| w.resrdy().set_bit());
    }

    /// Disables the interrupt for when conversion is ready.
    fn disable_interrupts(&mut self) {
        self.adc.intenclr.write(|w| w.resrdy().set_bit());
    }

    fn service_interrupt_ready(&mut self) -> Option<u16> {
        if self.adc.intflag.read().resrdy().bit_is_set() {
            self.adc.intflag.write(|w| w.resrdy().set_bit());

            Some(self.adc.result.read().result().bits())
        } else {
            None
        }
    }

    /// Sets the mux to a particular pin. The pin mux is enabled-protected,
    /// so must be called while the peripheral is disabled.
    fn mux<PIN: Channel<$Adc<S>, ID=u8>>(&mut self, _pin: &mut PIN) {
        let chan = PIN::channel();
        while self.adc.syncbusy.read().inputctrl().bit_is_set() {}
        self.adc.inputctrl.modify(|_, w| unsafe{ w.muxpos().bits(chan) });
    }
}

impl<WORD, PIN, S: pclk::PclkSourceId> OneShot<$Adc<S>, WORD, PIN> for $Adc<S>
where
   WORD: From<u16>,
   PIN: Channel<$Adc<S>, ID=u8>,
{
    type Error = ();

    fn read(&mut self, pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
        self.mux(pin);
        self.power_up();
        let result = self.synchronous_convert();
        self.power_down();
        Ok(result.into())
    }
}
   )+
    }
}

adc_hal! {
    Adc0: (ADC0, Adc0Id, Adc0Id, adc0_biascomp_scale_cal, adc0_biasref_scale_cal),
    Adc1: (ADC1, Adc1Id, Adc1Id, adc1_biascomp_scale_cal, adc1_biasref_scale_cal),
}

impl<A: Adc> ConversionMode<A> for SingleConversion<A>  {
    fn on_start(_adc: &mut A) {

    }
    fn on_complete(adc: &mut A) {
        adc.disable_interrupts();
        adc.power_down();
    }

    fn on_stop(_adc: &mut A) {

    }
}

impl<A: Adc> ConversionMode<A> for FreeRunning<A> {
    fn on_start(adc: &mut A) {
        adc.enable_freerunning();
    }
    fn on_complete(_adc: &mut A) {

    }
    fn on_stop(adc: &mut A) {
        adc.disable_interrupts();
        adc.power_down();
        adc.disable_freerunning();
    }
}

impl<A: Adc, C: ConversionMode<A>> InterruptAdc<A, C> {
    pub fn service_interrupt_ready(&mut self) -> Option<u16> {
        if let Some(res) = self.adc.service_interrupt_ready() {
            C::on_complete(&mut self.adc);
            Some(res)
        } else {
            None
        }
    }

    /// Starts a conversion sampling the specified pin.
    pub fn start_conversion<PIN: Channel<A, ID=u8>>(&mut self, pin: &mut PIN) {
        self.adc.mux(pin);
        self.adc.power_up();
        C::on_start(&mut self.adc);
        self.adc.enable_interrupts();
        self.adc.start_conversion();
    }

    pub fn stop_conversion(&mut self) {
        C::on_stop(&mut self.adc);
    }
}

impl<A: Adc, C: ConversionMode<A>> From<A> for InterruptAdc<A, C> {
    fn from(adc: A) -> Self {
        Self {
            adc,
            m: core::marker::PhantomData{},
        }
    }
}

macro_rules! adc_pins {
    (
        $(
            $( #[$cfg:meta] )?
            $PinId:ident: ($ADC:ident, $CHAN:literal)
        ),+
        $(,)?
    ) => {
        $(
            $( #[$cfg] )?
            impl<S: pclk::PclkSourceId> Channel<$ADC<S>> for Pin<$PinId, AlternateB> {
               type ID = u8;
               fn channel() -> u8 { $CHAN }
            }
        )+
    }
}

#[cfg(any(feature = "samc21n", feature = "samc20n"))]
adc_pins! {
    #[hal_cfg("pc00")]
    PC00: (Adc0, 8),
    #[hal_cfg("pc01")]
    PC01: (Adc0, 9),
    #[hal_cfg("pc02")]
    PC02: (Adc0, 10),
    #[hal_cfg("pc03")]
    PC03: (Adc0, 11),
    #[hal_cfg("pa02")]
    PA02: (Adc0, 0),
    #[hal_cfg("pa03")]
    PA03: (Adc0, 1),
    #[hal_cfg("pb08")]
    PB08: (Adc0, 2),
    #[hal_cfg("pb09")]
    PB09: (Adc0, 3),
    #[hal_cfg("pa04")]
    PA04: (Adc0, 4),
    #[hal_cfg("pa05")]
    PA05: (Adc0, 5),
    #[hal_cfg("pa06")]
    PA06: (Adc0, 6),
    #[hal_cfg("pa07")]
    PA07: (Adc0, 7),

    #[hal_cfg("pb04")]
    PB04: (Adc1, 6),
    #[hal_cfg("pb05")]
    PB05: (Adc1, 7),
    #[hal_cfg("pb06")]
    PB06: (Adc1, 8),
    #[hal_cfg("pb07")]
    PB07: (Adc1, 9),
    #[hal_cfg("pb08")]
    PB08: (Adc1, 4),
    #[hal_cfg("pb09")]
    PB09: (Adc1, 5),
    #[hal_cfg("pa08")]
    PA08: (Adc1, 10),
    #[hal_cfg("pa09")]
    PA09: (Adc1, 11),
    #[hal_cfg("pb00")]
    PB00: (Adc1, 0),
    #[hal_cfg("pb01")]
    PB01: (Adc1, 1),
    #[hal_cfg("pb02")]
    PB02: (Adc1, 2),
    #[hal_cfg("pb03")]
    PB03: (Adc1, 3)
}

#[cfg(any(feature = "samc21e", feature = "samc21g", feature = "samc21j",
    feature = "samc20e", feature = "samc20g", feature = "samc20j"))]
adc_pins! {
    #[hal_cfg("pa02")]
    PA02: (Adc0, 0),
    #[hal_cfg("pa03")]
    PA03: (Adc0, 1),
    #[hal_cfg("pb08")]
    PB08: (Adc0, 2),
    #[hal_cfg("pb09")]
    PB09: (Adc0, 3),
    #[hal_cfg("pa04")]
    PA04: (Adc0, 4),
    #[hal_cfg("pa05")]
    PA05: (Adc0, 5),
    #[hal_cfg("pa06")]
    PA06: (Adc0, 6),
    #[hal_cfg("pa07")]
    PA07: (Adc0, 7),
    #[hal_cfg("pa08")]
    PA08: (Adc0, 8),
    #[hal_cfg("pa09")]
    PA09: (Adc0, 9),
    #[hal_cfg("pa10")]
    PA10: (Adc0, 10),
    #[hal_cfg("pa11")]
    PA11: (Adc0, 11),

    #[hal_cfg("pb04")]
    PB04: (Adc1, 6),
    #[hal_cfg("pb05")]
    PB05: (Adc1, 7),
    #[hal_cfg("pb06")]
    PB06: (Adc1, 8),
    #[hal_cfg("pb07")]
    PB07: (Adc1, 9),
    #[hal_cfg("pb08")]
    PB08: (Adc1, 4),
    #[hal_cfg("pb09")]
    PB09: (Adc1, 5),
    #[hal_cfg("pa08")]
    PA08: (Adc1, 10),
    #[hal_cfg("pa09")]
    PA09: (Adc1, 11),
    #[hal_cfg("pb00")]
    PB00: (Adc1, 0),
    #[hal_cfg("pb01")]
    PB01: (Adc1, 1),
    #[hal_cfg("pb02")]
    PB02: (Adc1, 2),
    #[hal_cfg("pb03")]
    PB03: (Adc1, 3)
}