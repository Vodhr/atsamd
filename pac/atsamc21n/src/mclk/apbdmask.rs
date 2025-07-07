#[doc = "Register `APBDMASK` reader"]
pub type R = crate::R<ApbdmaskSpec>;
#[doc = "Register `APBDMASK` writer"]
pub type W = crate::W<ApbdmaskSpec>;
#[doc = "Field `SERCOM6_` reader - SERCOM6 APB Clock Enable"]
pub type Sercom6_R = crate::BitReader;
#[doc = "Field `SERCOM6_` writer - SERCOM6 APB Clock Enable"]
pub type Sercom6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM7_` reader - SERCOM7 APB Clock Enable"]
pub type Sercom7_R = crate::BitReader;
#[doc = "Field `SERCOM7_` writer - SERCOM7 APB Clock Enable"]
pub type Sercom7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC5_` reader - TC5 APB Clock Enable"]
pub type Tc5_R = crate::BitReader;
#[doc = "Field `TC5_` writer - TC5 APB Clock Enable"]
pub type Tc5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC6_` reader - TC6 APB Clock Enable"]
pub type Tc6_R = crate::BitReader;
#[doc = "Field `TC6_` writer - TC6 APB Clock Enable"]
pub type Tc6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC7_` reader - TC7 APB Clock Enable"]
pub type Tc7_R = crate::BitReader;
#[doc = "Field `TC7_` writer - TC7 APB Clock Enable"]
pub type Tc7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SERCOM6 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom6_(&self) -> Sercom6_R {
        Sercom6_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM7 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom7_(&self) -> Sercom7_R {
        Sercom7_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TC5 APB Clock Enable"]
    #[inline(always)]
    pub fn tc5_(&self) -> Tc5_R {
        Tc5_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TC6 APB Clock Enable"]
    #[inline(always)]
    pub fn tc6_(&self) -> Tc6_R {
        Tc6_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TC7 APB Clock Enable"]
    #[inline(always)]
    pub fn tc7_(&self) -> Tc7_R {
        Tc7_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SERCOM6 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom6_(&mut self) -> Sercom6_W<ApbdmaskSpec> {
        Sercom6_W::new(self, 0)
    }
    #[doc = "Bit 1 - SERCOM7 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom7_(&mut self) -> Sercom7_W<ApbdmaskSpec> {
        Sercom7_W::new(self, 1)
    }
    #[doc = "Bit 2 - TC5 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc5_(&mut self) -> Tc5_W<ApbdmaskSpec> {
        Tc5_W::new(self, 2)
    }
    #[doc = "Bit 3 - TC6 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc6_(&mut self) -> Tc6_W<ApbdmaskSpec> {
        Tc6_W::new(self, 3)
    }
    #[doc = "Bit 4 - TC7 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc7_(&mut self) -> Tc7_W<ApbdmaskSpec> {
        Tc7_W::new(self, 4)
    }
}
#[doc = "APBD Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`apbdmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbdmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbdmaskSpec;
impl crate::RegisterSpec for ApbdmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbdmask::R`](R) reader structure"]
impl crate::Readable for ApbdmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`apbdmask::W`](W) writer structure"]
impl crate::Writable for ApbdmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBDMASK to value 0"]
impl crate::Resettable for ApbdmaskSpec {
    const RESET_VALUE: u32 = 0;
}
