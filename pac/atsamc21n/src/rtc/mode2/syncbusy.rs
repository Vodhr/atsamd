#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRST` reader - Software Reset Bit Busy"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `ENABLE` reader - Enable Bit Busy"]
pub type EnableR = crate::BitReader;
#[doc = "Field `FREQCORR` reader - FREQCORR Register Busy"]
pub type FreqcorrR = crate::BitReader;
#[doc = "Field `CLOCK` reader - CLOCK Register Busy"]
pub type ClockR = crate::BitReader;
#[doc = "Field `ALARM0` reader - ALARM 0 Register Busy"]
pub type Alarm0R = crate::BitReader;
#[doc = "Field `MASK0` reader - MASK 0 Register Busy"]
pub type Mask0R = crate::BitReader;
#[doc = "Field `CLOCKSYNC` reader - Clock Read Synchronization Enable Bit Busy"]
pub type ClocksyncR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software Reset Bit Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Bit Busy"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FREQCORR Register Busy"]
    #[inline(always)]
    pub fn freqcorr(&self) -> FreqcorrR {
        FreqcorrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CLOCK Register Busy"]
    #[inline(always)]
    pub fn clock(&self) -> ClockR {
        ClockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - ALARM 0 Register Busy"]
    #[inline(always)]
    pub fn alarm0(&self) -> Alarm0R {
        Alarm0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 11 - MASK 0 Register Busy"]
    #[inline(always)]
    pub fn mask0(&self) -> Mask0R {
        Mask0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Clock Read Synchronization Enable Bit Busy"]
    #[inline(always)]
    pub fn clocksync(&self) -> ClocksyncR {
        ClocksyncR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "MODE2 Synchronization Busy Status\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {
    const RESET_VALUE: u32 = 0;
}
