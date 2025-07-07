#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRST` reader - SWRST Synchronization Busy"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `ENABLE` reader - ENABLE Synchronization Busy"]
pub type EnableR = crate::BitReader;
#[doc = "Field `INPUTCTRL` reader - INPUTCTRL Synchronization Busy"]
pub type InputctrlR = crate::BitReader;
#[doc = "Field `CTRLC` reader - CTRLC Synchronization Busy"]
pub type CtrlcR = crate::BitReader;
#[doc = "Field `AVGCTRL` reader - AVGCTRL Synchronization Busy"]
pub type AvgctrlR = crate::BitReader;
#[doc = "Field `SAMPCTRL` reader - SAMPCTRL Synchronization Busy"]
pub type SampctrlR = crate::BitReader;
#[doc = "Field `WINLT` reader - WINLT Synchronization Busy"]
pub type WinltR = crate::BitReader;
#[doc = "Field `WINUT` reader - WINUT Synchronization Busy"]
pub type WinutR = crate::BitReader;
#[doc = "Field `GAINCORR` reader - GAINCORR Synchronization Busy"]
pub type GaincorrR = crate::BitReader;
#[doc = "Field `OFFSETCORR` reader - OFFSETCTRL Synchronization Busy"]
pub type OffsetcorrR = crate::BitReader;
#[doc = "Field `SWTRIG` reader - SWTRG Synchronization Busy"]
pub type SwtrigR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SWRST Synchronization Busy"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ENABLE Synchronization Busy"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - INPUTCTRL Synchronization Busy"]
    #[inline(always)]
    pub fn inputctrl(&self) -> InputctrlR {
        InputctrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CTRLC Synchronization Busy"]
    #[inline(always)]
    pub fn ctrlc(&self) -> CtrlcR {
        CtrlcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AVGCTRL Synchronization Busy"]
    #[inline(always)]
    pub fn avgctrl(&self) -> AvgctrlR {
        AvgctrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SAMPCTRL Synchronization Busy"]
    #[inline(always)]
    pub fn sampctrl(&self) -> SampctrlR {
        SampctrlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WINLT Synchronization Busy"]
    #[inline(always)]
    pub fn winlt(&self) -> WinltR {
        WinltR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - WINUT Synchronization Busy"]
    #[inline(always)]
    pub fn winut(&self) -> WinutR {
        WinutR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GAINCORR Synchronization Busy"]
    #[inline(always)]
    pub fn gaincorr(&self) -> GaincorrR {
        GaincorrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OFFSETCTRL Synchronization Busy"]
    #[inline(always)]
    pub fn offsetcorr(&self) -> OffsetcorrR {
        OffsetcorrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SWTRG Synchronization Busy"]
    #[inline(always)]
    pub fn swtrig(&self) -> SwtrigR {
        SwtrigR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {
    const RESET_VALUE: u16 = 0;
}
