pub mod pin;

use embedded_hal_02::prelude::_embedded_hal_blocking_spi_Write;
#[cfg(feature = "async")]
use super::{FutureAdc, async_api};

use super::{
    ADC_SETTINGS_INTERNAL_READ, Accumulation, Adc, AdcInstance, AdcSettings, CpuVoltageSource,
    Error, Flags, PrimaryAdc, SampleCount,
};
use crate::{calibration, dmac, pac};
use crate::dmac::{TriggerAction, TriggerSource};

/// ADC instance 0
pub struct Adc0 {
    _adc: pac::Adc0,
}

impl PrimaryAdc for Adc0 {}

impl AdcInstance for Adc0 {
    type Instance = pac::Adc0;

    type ClockId = crate::clock::v2::types::Adc0;

    #[cfg(feature = "async")]
    type Interrupt = crate::async_hal::interrupts::ADC0;

    #[inline]
    fn peripheral_reg_block(p: &mut pac::Peripherals) -> &pac::adc0::RegisterBlock {
        &p.adc0
    }

    #[inline]
    fn calibrate(instance: &Self::Instance) {
        instance.calib().write(|w| unsafe {
            w.biascomp().bits(calibration::adc0_biascomp_scale_cal());
            w.biasrefbuf().bits(calibration::adc0_biasref_scale_cal())
        });
    }

    #[cfg(feature = "async")]
    #[inline]
    fn waker() -> &'static embassy_sync::waitqueue::AtomicWaker {
        use super::async_api;
        &async_api::ADC_WAKERS[0]
    }

    const DMA_TRIGGER_SOURCE: TriggerSource = TriggerSource::Adc0Resrdy;
}

/// ADC instance 0
pub struct Adc1 {
    _adc: pac::Adc1,
}

impl AdcInstance for Adc1 {
    type Instance = pac::Adc1;

    type ClockId = crate::clock::v2::pclk::ids::Adc1;

    #[cfg(feature = "async")]
    type Interrupt = crate::async_hal::interrupts::ADC1;

    #[inline]
    fn peripheral_reg_block(p: &mut pac::Peripherals) -> &pac::adc0::RegisterBlock {
        &p.adc1
    }

    #[inline]
    fn calibrate(instance: &Self::Instance) {
        instance.calib().write(|w| unsafe {
            w.biascomp().bits(calibration::adc1_biascomp_scale_cal());
            w.biasrefbuf().bits(calibration::adc1_biasref_scale_cal())
        });
    }

    #[cfg(feature = "async")]
    #[inline]
    fn waker() -> &'static embassy_sync::waitqueue::AtomicWaker {
        use super::async_api;
        &async_api::ADC_WAKERS[1]
    }

    const DMA_TRIGGER_SOURCE: TriggerSource = TriggerSource::Adc1Resrdy;
}

impl<I: AdcInstance> Adc<I> {
    #[inline]
    /// Configures the ADC.
    pub(crate) fn configure(&mut self, cfg: AdcSettings) {
        if cfg != self.cfg {
            // Set discard flag for next read
            self.discard = true;
        }
        // Stop ADC
        self.power_down();
        self.sync();

        self.adc.ctrla().write(|w| w.swrst().set_bit());
        self.sync();

        I::calibrate(&self.adc);
        self.sync();
        self.adc
            .ctrlb()
            .modify(|_, w| w.prescaler().variant(cfg.clk_divider));
        self.sync();
        self.adc
            .ctrlc()
            .modify(|_, w| w.ressel().variant(cfg.accumulation.resolution()));
        self.sync();

        self.adc
            .sampctrl()
            .modify(|_, w| unsafe { w.samplen().bits(cfg.sample_clock_cycles.saturating_sub(1)) }); // sample length
        self.sync();
        self.adc.inputctrl().modify(|_, w| w.muxneg().gnd()); // No negative input (internal gnd)
        self.sync();

        self.adc
            .ctrlc().modify(|_, w| w.diffmode().clear_bit());
        self.sync();

        let (sample_cnt, adjres) = match cfg.accumulation {
            // 1 sample to be used as is
            Accumulation::Single(_) => (SampleCount::_1, 0),
            // A total of `adc_sample_count` elements will be averaged by the ADC
            // before it returns the result
            // Table 45-3 SAMx5x datasheet
            Accumulation::Average(cnt) => (cnt, core::cmp::min(cnt as u8, 0x04)),
            // A total of `adc_sample_count` elements will be summed by the ADC
            // before it returns the result
            Accumulation::Summed(cnt) => (cnt, 0),
        };
        self.adc.avgctrl().modify(|_, w| {
            w.samplenum().variant(sample_cnt);
            unsafe { w.adjres().bits(adjres) }
        });
        self.sync();
        self.set_reference(cfg.vref);
        self.sync();
        // self.adc.refctrl().modify(|_, w| w.refcomp().set_bit());
        // self.sync();

        // self.adc.sampctrl().modify(|_, w| w.offcomp().set_bit());
        // self.sync();

        self.adc.ctrla().modify(|_, w| w.runstdby().set_bit());
        self.sync();
        self.adc.evctrl().modify(|_, w| w.startei().set_bit());
        self.sync();
        self.adc.evctrl().modify(|_, w| w.resrdyeo().set_bit());
        self.sync();
        // self.enable_freerunning();
        // self.adc.ctrla().modify(|_, w| w.enable().set_bit());
        // self.sync();

        self.reset_sequence();

        self.cfg = cfg;
        self.power_up();
    }
}

impl<I: AdcInstance + PrimaryAdc> Adc<I> {
    #[inline]
    pub fn read_cpu_voltage(&mut self, src: CpuVoltageSource) -> u16 {
        let voltage = self.with_specific_settings(ADC_SETTINGS_INTERNAL_READ, |adc| {
            let res = adc.read_channel(src as u8);
            adc.reading_to_f32(res) * 3.3 * 4.0 // x4 since the voltages are 1/4 scaled
        });
        (voltage * 1000.0) as u16
    }
}

impl<I: AdcInstance> Adc<I> {
    #[inline]
    pub(super) fn sync(&self) {
        // Slightly more performant than checking the individual bits
        // since we avoid an extra instruction to bit shift
        while self.adc.syncbusy().read().bits() != 0 {
            core::hint::spin_loop();
        }
    }

    #[inline]
    pub(super) fn flush(&mut self) {
        self.adc.swtrig().modify(|_, w| w.flush().set_bit());
    }

    #[inline]
    pub(super) fn power_up(&mut self) {
        self.adc.ctrla().modify(|_, w| w.enable().set_bit());
        self.sync();
    }

    #[inline]
    #[allow(dead_code)]
    pub(super) fn power_down(&mut self) {
        self.adc.ctrla().modify(|_, w| w.enable().clear_bit());
        self.sync();
    }

    #[inline]
    pub fn start_conversion(&mut self) {
        self.adc.swtrig().modify(|_, w| w.start().set_bit());
    }

    #[inline]
    pub(super) fn enable_freerunning(&mut self) {
        self.adc.ctrlc().modify(|_, w| w.freerun().set_bit());
        self.sync();
    }

    #[inline]
    pub(super) fn disable_freerunning(&mut self) {
        self.adc.ctrlc().modify(|_, w| w.freerun().clear_bit());
        self.sync();
    }

    #[inline]
    pub(super) fn read_flags(&self) -> Flags {
        let bits = self.adc.intflag().read().bits();
        Flags::from_bits_truncate(bits)
    }

    #[cfg(feature="async")]
    /// Clear the specified interrupt flags
    #[inline]
    pub(super) fn clear_flags(&mut self, flags: &Flags) {
        unsafe {
            self.adc.intflag().write(|w| w.bits(flags.bits()));
        }
    }

    /// Clear all interrupt flags
    #[inline]
    pub(super) fn clear_all_flags(&mut self) {
        unsafe {
            self.adc.intflag().write(|w| w.bits(0b111));
        }
    }

    /// Check whether the provided flags contain an `OVERRUN` error
    #[inline]
    pub(super) fn check_overrun(&mut self, flags: &Flags) -> Result<(), Error> {
        if flags.contains(Flags::OVERRUN) {
            Err(Error::BufferOverrun)
        } else {
            Ok(())
        }
    }

    /// Enables an interrupt when conversion is ready.
    #[inline]
    #[allow(dead_code)]
    pub(super) fn enable_interrupts(&mut self, flags: Flags) {
        unsafe { self.adc.intenset().write(|w| w.bits(flags.bits())) };
    }

    /// Disables the interrupt for when conversion is ready.
    #[inline]
    pub(super) fn disable_interrupts(&mut self, flags: Flags) {
        unsafe { self.adc.intenclr().write(|w| w.bits(flags.bits())) };
    }

    pub(super) fn enable_start_event(&mut self) {
        self.adc.evctrl().modify(|_, w| w.startei().set_bit());
    }

    pub(super) fn disable_start_event(&mut self) {
        self.adc.evctrl().modify(|_, w| w.startei().clear_bit());
    }

    #[inline]
    pub(super) fn conversion_result(&self) -> u16 {
        self.adc.result().read().result().bits()
    }

    #[inline]
    pub(super) fn mux(&mut self, ch: u8) {
        self.adc.inputctrl().modify(|r, w| {
            if r.muxpos().bits() != ch {
                self.discard = true;
            }
            unsafe { w.muxpos().bits(ch) }
        });
        self.sync()
    }

    #[inline]
    pub(super) fn add_channel_to_sequence(&mut self, ch: u8) {
        self.adc.seqctrl().modify(|r, w| unsafe {
            w.seqen().bits(r.seqen().bits() | (1 << ch.min(31)))
        });
    }

    #[inline]
    pub(super) fn get_current_sequence_channel(&self) -> u8 {
        self.adc.seqstatus().read().seqstate().bits()
    }

    #[inline]
    pub(super) fn is_sequence_busy(&self) -> bool {
        self.adc.seqstatus().read().seqbusy().bit_is_set()
    }

    #[inline]
    pub(super) fn reset_sequence(&mut self) {
        self.adc.seqctrl().reset();
    }

    pub fn enable_dma<Id: dmac::ChId, const N: usize>(&mut self,
                                             mut channel: &mut dmac::Channel<Id, dmac::Ready>,
                                             mut buffer: &'static mut [u16; N]) {
        unsafe {
            channel.transfer(
                self,
                &mut buffer,
                I::DMA_TRIGGER_SOURCE,
                TriggerAction::Beat,
                None
            ).unwrap();
        }
    }

    pub fn dma_ready<Id: dmac::ChId>(&self, channel: &mut dmac::Channel<Id, dmac::Ready>) -> bool {
        channel.xfer_complete()
    }
}

unsafe impl<I: AdcInstance> dmac::Buffer for Adc<I> {
    type Beat = u16;

    fn dma_ptr(&mut self) -> *mut Self::Beat {
        self.adc.result().as_ptr()
    }

    fn incrementing(&self) -> bool {
        false
    }

    fn buffer_len(&self) -> usize {
        1
    }
}

struct AdcMuxBuffer(*mut u16);

unsafe impl dmac::Buffer for AdcMuxBuffer {
    type Beat = u16;
    fn dma_ptr(&mut self) -> *mut Self::Beat {
        self.0
    }

    fn incrementing(&self) -> bool {
        false
    }

    fn buffer_len(&self) -> usize {
        1
    }
}

#[cfg(feature = "async")]
impl<I: AdcInstance + PrimaryAdc, F> FutureAdc<I, F>
where
    F: crate::async_hal::interrupts::Binding<I::Interrupt, async_api::InterruptHandler<I>>,
{
    /// Reads a CPU voltage source. Value returned is in millivolts (mV)
    pub async fn read_cpu_voltage(&mut self, src: CpuVoltageSource) -> u16 {
        let old_adc_settings = self.inner.cfg;
        self.inner.configure(ADC_SETTINGS_INTERNAL_READ);

        let res = self.read_channel(src as u8).await;

        // x4 since the voltages are 1/4 scaled
        let voltage = self.inner.reading_to_f32(res) * 3.3 * 4.0;

        self.inner.configure(old_adc_settings);
        (voltage * 1000.0) as u16
    }
}
