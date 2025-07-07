#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `STARTEI` reader - Start Conversion Event Input Enable"]
pub type StarteiR = crate::BitReader;
#[doc = "Field `STARTEI` writer - Start Conversion Event Input Enable"]
pub type StarteiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTINV` reader - Start Conversion Event Invert Enable"]
pub type StartinvR = crate::BitReader;
#[doc = "Field `STARTINV` writer - Start Conversion Event Invert Enable"]
pub type StartinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINEO` reader - Window Monitor Event Out"]
pub type WineoR = crate::BitReader;
#[doc = "Field `WINEO` writer - Window Monitor Event Out"]
pub type WineoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start Conversion Event Input Enable"]
    #[inline(always)]
    pub fn startei(&self) -> StarteiR {
        StarteiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start Conversion Event Invert Enable"]
    #[inline(always)]
    pub fn startinv(&self) -> StartinvR {
        StartinvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Window Monitor Event Out"]
    #[inline(always)]
    pub fn wineo(&self) -> WineoR {
        WineoR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Conversion Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn startei(&mut self) -> StarteiW<EvctrlSpec> {
        StarteiW::new(self, 0)
    }
    #[doc = "Bit 1 - Start Conversion Event Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn startinv(&mut self) -> StartinvW<EvctrlSpec> {
        StartinvW::new(self, 1)
    }
    #[doc = "Bit 2 - Window Monitor Event Out"]
    #[inline(always)]
    #[must_use]
    pub fn wineo(&mut self) -> WineoW<EvctrlSpec> {
        WineoW::new(self, 2)
    }
}
#[doc = "Event Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {
    const RESET_VALUE: u8 = 0;
}
