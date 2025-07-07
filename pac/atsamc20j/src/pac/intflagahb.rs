#[doc = "Register `INTFLAGAHB` reader"]
pub type R = crate::R<IntflagahbSpec>;
#[doc = "Register `INTFLAGAHB` writer"]
pub type W = crate::W<IntflagahbSpec>;
#[doc = "Field `FLASH_` reader - FLASH"]
pub type Flash_R = crate::BitReader;
#[doc = "Field `FLASH_` writer - FLASH"]
pub type Flash_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSRAMCM0P_` reader - HSRAMCM0P"]
pub type Hsramcm0p_R = crate::BitReader;
#[doc = "Field `HSRAMCM0P_` writer - HSRAMCM0P"]
pub type Hsramcm0p_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSRAMDSU_` reader - HSRAMDSU"]
pub type Hsramdsu_R = crate::BitReader;
#[doc = "Field `HSRAMDSU_` writer - HSRAMDSU"]
pub type Hsramdsu_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPB1_` reader - HPB1"]
pub type Hpb1_R = crate::BitReader;
#[doc = "Field `HPB1_` writer - HPB1"]
pub type Hpb1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPB0_` reader - HPB0"]
pub type Hpb0_R = crate::BitReader;
#[doc = "Field `HPB0_` writer - HPB0"]
pub type Hpb0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPB2_` reader - HPB2"]
pub type Hpb2_R = crate::BitReader;
#[doc = "Field `HPB2_` writer - HPB2"]
pub type Hpb2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPRAMDMAC_` reader - LPRAMDMAC"]
pub type Lpramdmac_R = crate::BitReader;
#[doc = "Field `LPRAMDMAC_` writer - LPRAMDMAC"]
pub type Lpramdmac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVAS_` reader - DIVAS"]
pub type Divas_R = crate::BitReader;
#[doc = "Field `DIVAS_` writer - DIVAS"]
pub type Divas_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    pub fn flash_(&self) -> Flash_R {
        Flash_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HSRAMCM0P"]
    #[inline(always)]
    pub fn hsramcm0p_(&self) -> Hsramcm0p_R {
        Hsramcm0p_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSRAMDSU"]
    #[inline(always)]
    pub fn hsramdsu_(&self) -> Hsramdsu_R {
        Hsramdsu_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HPB1"]
    #[inline(always)]
    pub fn hpb1_(&self) -> Hpb1_R {
        Hpb1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HPB0"]
    #[inline(always)]
    pub fn hpb0_(&self) -> Hpb0_R {
        Hpb0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HPB2"]
    #[inline(always)]
    pub fn hpb2_(&self) -> Hpb2_R {
        Hpb2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPRAMDMAC"]
    #[inline(always)]
    pub fn lpramdmac_(&self) -> Lpramdmac_R {
        Lpramdmac_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DIVAS"]
    #[inline(always)]
    pub fn divas_(&self) -> Divas_R {
        Divas_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FLASH"]
    #[inline(always)]
    #[must_use]
    pub fn flash_(&mut self) -> Flash_W<IntflagahbSpec> {
        Flash_W::new(self, 0)
    }
    #[doc = "Bit 1 - HSRAMCM0P"]
    #[inline(always)]
    #[must_use]
    pub fn hsramcm0p_(&mut self) -> Hsramcm0p_W<IntflagahbSpec> {
        Hsramcm0p_W::new(self, 1)
    }
    #[doc = "Bit 2 - HSRAMDSU"]
    #[inline(always)]
    #[must_use]
    pub fn hsramdsu_(&mut self) -> Hsramdsu_W<IntflagahbSpec> {
        Hsramdsu_W::new(self, 2)
    }
    #[doc = "Bit 3 - HPB1"]
    #[inline(always)]
    #[must_use]
    pub fn hpb1_(&mut self) -> Hpb1_W<IntflagahbSpec> {
        Hpb1_W::new(self, 3)
    }
    #[doc = "Bit 4 - HPB0"]
    #[inline(always)]
    #[must_use]
    pub fn hpb0_(&mut self) -> Hpb0_W<IntflagahbSpec> {
        Hpb0_W::new(self, 4)
    }
    #[doc = "Bit 5 - HPB2"]
    #[inline(always)]
    #[must_use]
    pub fn hpb2_(&mut self) -> Hpb2_W<IntflagahbSpec> {
        Hpb2_W::new(self, 5)
    }
    #[doc = "Bit 6 - LPRAMDMAC"]
    #[inline(always)]
    #[must_use]
    pub fn lpramdmac_(&mut self) -> Lpramdmac_W<IntflagahbSpec> {
        Lpramdmac_W::new(self, 6)
    }
    #[doc = "Bit 7 - DIVAS"]
    #[inline(always)]
    #[must_use]
    pub fn divas_(&mut self) -> Divas_W<IntflagahbSpec> {
        Divas_W::new(self, 7)
    }
}
#[doc = "Bridge interrupt flag status\n\nYou can [`read`](crate::Reg::read) this register and get [`intflagahb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflagahb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagahbSpec;
impl crate::RegisterSpec for IntflagahbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflagahb::R`](R) reader structure"]
impl crate::Readable for IntflagahbSpec {}
#[doc = "`write(|w| ..)` method takes [`intflagahb::W`](W) writer structure"]
impl crate::Writable for IntflagahbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFLAGAHB to value 0"]
impl crate::Resettable for IntflagahbSpec {
    const RESET_VALUE: u32 = 0;
}
