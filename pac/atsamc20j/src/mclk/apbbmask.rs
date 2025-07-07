#[doc = "Register `APBBMASK` reader"]
pub type R = crate::R<ApbbmaskSpec>;
#[doc = "Register `APBBMASK` writer"]
pub type W = crate::W<ApbbmaskSpec>;
#[doc = "Field `PORT_` reader - PORT APB Clock Enable"]
pub type Port_R = crate::BitReader;
#[doc = "Field `PORT_` writer - PORT APB Clock Enable"]
pub type Port_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSU_` reader - DSU APB Clock Enable"]
pub type Dsu_R = crate::BitReader;
#[doc = "Field `DSU_` writer - DSU APB Clock Enable"]
pub type Dsu_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL APB Clock Enable"]
pub type Nvmctrl_R = crate::BitReader;
#[doc = "Field `NVMCTRL_` writer - NVMCTRL APB Clock Enable"]
pub type Nvmctrl_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMATRIXHS_` reader - HMATRIXHS APB Clock Enable"]
pub type Hmatrixhs_R = crate::BitReader;
#[doc = "Field `HMATRIXHS_` writer - HMATRIXHS APB Clock Enable"]
pub type Hmatrixhs_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PORT APB Clock Enable"]
    #[inline(always)]
    pub fn port_(&self) -> Port_R {
        Port_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSU APB Clock Enable"]
    #[inline(always)]
    pub fn dsu_(&self) -> Dsu_R {
        Dsu_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL APB Clock Enable"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> Nvmctrl_R {
        Nvmctrl_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - HMATRIXHS APB Clock Enable"]
    #[inline(always)]
    pub fn hmatrixhs_(&self) -> Hmatrixhs_R {
        Hmatrixhs_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PORT APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn port_(&mut self) -> Port_W<ApbbmaskSpec> {
        Port_W::new(self, 0)
    }
    #[doc = "Bit 1 - DSU APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dsu_(&mut self) -> Dsu_W<ApbbmaskSpec> {
        Dsu_W::new(self, 1)
    }
    #[doc = "Bit 2 - NVMCTRL APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_(&mut self) -> Nvmctrl_W<ApbbmaskSpec> {
        Nvmctrl_W::new(self, 2)
    }
    #[doc = "Bit 5 - HMATRIXHS APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hmatrixhs_(&mut self) -> Hmatrixhs_W<ApbbmaskSpec> {
        Hmatrixhs_W::new(self, 5)
    }
}
#[doc = "APBB Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`apbbmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbbmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbbmaskSpec;
impl crate::RegisterSpec for ApbbmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbbmask::R`](R) reader structure"]
impl crate::Readable for ApbbmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`apbbmask::W`](W) writer structure"]
impl crate::Writable for ApbbmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBBMASK to value 0x07"]
impl crate::Resettable for ApbbmaskSpec {
    const RESET_VALUE: u32 = 0x07;
}
