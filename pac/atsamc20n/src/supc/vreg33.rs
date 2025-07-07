#[doc = "Register `VREG33` reader"]
pub type R = crate::R<Vreg33Spec>;
#[doc = "Register `VREG33` writer"]
pub type W = crate::W<Vreg33Spec>;
#[doc = "Field `ENABLE` reader - VREG33 Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - VREG33 Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENRDY` reader - VREG33 Ready Enable"]
pub type EnrdyR = crate::BitReader;
#[doc = "Field `ENRDY` writer - VREG33 Ready Enable"]
pub type EnrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPASS` reader - VREG33 Bypass"]
pub type BypassR = crate::BitReader;
#[doc = "Field `BYPASS` writer - VREG33 Bypass"]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOEN` reader - Isolation Enable"]
pub type IsoenR = crate::BitReader;
#[doc = "Field `ISOEN` writer - Isolation Enable"]
pub type IsoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - VREG33 Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VREG33 Ready Enable"]
    #[inline(always)]
    pub fn enrdy(&self) -> EnrdyR {
        EnrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VREG33 Bypass"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Isolation Enable"]
    #[inline(always)]
    pub fn isoen(&self) -> IsoenR {
        IsoenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - VREG33 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<Vreg33Spec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - VREG33 Ready Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enrdy(&mut self) -> EnrdyW<Vreg33Spec> {
        EnrdyW::new(self, 2)
    }
    #[doc = "Bit 3 - VREG33 Bypass"]
    #[inline(always)]
    #[must_use]
    pub fn bypass(&mut self) -> BypassW<Vreg33Spec> {
        BypassW::new(self, 3)
    }
    #[doc = "Bit 4 - Isolation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn isoen(&mut self) -> IsoenW<Vreg33Spec> {
        IsoenW::new(self, 4)
    }
}
#[doc = "VREG33 Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vreg33::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vreg33::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vreg33Spec;
impl crate::RegisterSpec for Vreg33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vreg33::R`](R) reader structure"]
impl crate::Readable for Vreg33Spec {}
#[doc = "`write(|w| ..)` method takes [`vreg33::W`](W) writer structure"]
impl crate::Writable for Vreg33Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREG33 to value 0x10"]
impl crate::Resettable for Vreg33Spec {
    const RESET_VALUE: u32 = 0x10;
}
