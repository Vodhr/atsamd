#[doc = "Register `STATUSD` reader"]
pub type R = crate::R<StatusdSpec>;
#[doc = "Field `SERCOM6_` reader - SERCOM6 APB Protect Enable"]
pub type Sercom6_R = crate::BitReader;
#[doc = "Field `SERCOM7_` reader - SERCOM7 APB Protect Enable"]
pub type Sercom7_R = crate::BitReader;
#[doc = "Field `TC5_` reader - TC5 APB Protect Enable"]
pub type Tc5_R = crate::BitReader;
#[doc = "Field `TC6_` reader - TC6 APB Protect Enable"]
pub type Tc6_R = crate::BitReader;
#[doc = "Field `TC7_` reader - TC7 APB Protect Enable"]
pub type Tc7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SERCOM6 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom6_(&self) -> Sercom6_R {
        Sercom6_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM7 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom7_(&self) -> Sercom7_R {
        Sercom7_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TC5 APB Protect Enable"]
    #[inline(always)]
    pub fn tc5_(&self) -> Tc5_R {
        Tc5_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TC6 APB Protect Enable"]
    #[inline(always)]
    pub fn tc6_(&self) -> Tc6_R {
        Tc6_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TC7 APB Protect Enable"]
    #[inline(always)]
    pub fn tc7_(&self) -> Tc7_R {
        Tc7_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge D\n\nYou can [`read`](crate::Reg::read) this register and get [`statusd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusdSpec;
impl crate::RegisterSpec for StatusdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusd::R`](R) reader structure"]
impl crate::Readable for StatusdSpec {}
#[doc = "`reset()` method sets STATUSD to value 0"]
impl crate::Resettable for StatusdSpec {
    const RESET_VALUE: u32 = 0;
}
