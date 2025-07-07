#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `SWRST` reader - Software Reset Synchroniation Busy bit"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `GENCTRL0` reader - Generic Clock Generator Control 0 Synchronization Busy bits"]
pub type Genctrl0R = crate::BitReader;
#[doc = "Field `GENCTRL1` reader - Generic Clock Generator Control 1 Synchronization Busy bits"]
pub type Genctrl1R = crate::BitReader;
#[doc = "Field `GENCTRL2` reader - Generic Clock Generator Control 2 Synchronization Busy bits"]
pub type Genctrl2R = crate::BitReader;
#[doc = "Field `GENCTRL3` reader - Generic Clock Generator Control 3 Synchronization Busy bits"]
pub type Genctrl3R = crate::BitReader;
#[doc = "Field `GENCTRL4` reader - Generic Clock Generator Control 4 Synchronization Busy bits"]
pub type Genctrl4R = crate::BitReader;
#[doc = "Field `GENCTRL5` reader - Generic Clock Generator Control 5 Synchronization Busy bits"]
pub type Genctrl5R = crate::BitReader;
#[doc = "Field `GENCTRL6` reader - Generic Clock Generator Control 6 Synchronization Busy bits"]
pub type Genctrl6R = crate::BitReader;
#[doc = "Field `GENCTRL7` reader - Generic Clock Generator Control 7 Synchronization Busy bits"]
pub type Genctrl7R = crate::BitReader;
#[doc = "Field `GENCTRL8` reader - Generic Clock Generator Control 8 Synchronization Busy bits"]
pub type Genctrl8R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software Reset Synchroniation Busy bit"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Generic Clock Generator Control 0 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl0(&self) -> Genctrl0R {
        Genctrl0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generic Clock Generator Control 1 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl1(&self) -> Genctrl1R {
        Genctrl1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Generic Clock Generator Control 2 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl2(&self) -> Genctrl2R {
        Genctrl2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Generic Clock Generator Control 3 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl3(&self) -> Genctrl3R {
        Genctrl3R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Generic Clock Generator Control 4 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl4(&self) -> Genctrl4R {
        Genctrl4R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Generic Clock Generator Control 5 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl5(&self) -> Genctrl5R {
        Genctrl5R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Generic Clock Generator Control 6 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl6(&self) -> Genctrl6R {
        Genctrl6R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Generic Clock Generator Control 7 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl7(&self) -> Genctrl7R {
        Genctrl7R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Generic Clock Generator Control 8 Synchronization Busy bits"]
    #[inline(always)]
    pub fn genctrl8(&self) -> Genctrl8R {
        Genctrl8R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
