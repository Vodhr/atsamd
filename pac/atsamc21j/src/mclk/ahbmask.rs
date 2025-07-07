#[doc = "Register `AHBMASK` reader"]
pub type R = crate::R<AhbmaskSpec>;
#[doc = "Register `AHBMASK` writer"]
pub type W = crate::W<AhbmaskSpec>;
#[doc = "Field `HPB0_` reader - HPB0 AHB Clock Mask"]
pub type Hpb0_R = crate::BitReader;
#[doc = "Field `HPB0_` writer - HPB0 AHB Clock Mask"]
pub type Hpb0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPB1_` reader - HPB1 AHB Clock Mask"]
pub type Hpb1_R = crate::BitReader;
#[doc = "Field `HPB1_` writer - HPB1 AHB Clock Mask"]
pub type Hpb1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPB2_` reader - HPB2 AHB Clock Mask"]
pub type Hpb2_R = crate::BitReader;
#[doc = "Field `HPB2_` writer - HPB2 AHB Clock Mask"]
pub type Hpb2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSU_` reader - DSU AHB Clock Mask"]
pub type Dsu_R = crate::BitReader;
#[doc = "Field `DSU_` writer - DSU AHB Clock Mask"]
pub type Dsu_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMATRIXHS_` reader - HMATRIXHS AHB Clock Mask"]
pub type Hmatrixhs_R = crate::BitReader;
#[doc = "Field `HMATRIXHS_` writer - HMATRIXHS AHB Clock Mask"]
pub type Hmatrixhs_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL AHB Clock Mask"]
pub type Nvmctrl_R = crate::BitReader;
#[doc = "Field `NVMCTRL_` writer - NVMCTRL AHB Clock Mask"]
pub type Nvmctrl_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSRAM_` reader - HSRAM AHB Clock Mask"]
pub type Hsram_R = crate::BitReader;
#[doc = "Field `HSRAM_` writer - HSRAM AHB Clock Mask"]
pub type Hsram_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAC_` reader - DMAC AHB Clock Mask"]
pub type Dmac_R = crate::BitReader;
#[doc = "Field `DMAC_` writer - DMAC AHB Clock Mask"]
pub type Dmac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN0_` reader - CAN0 AHB Clock Mask"]
pub type Can0_R = crate::BitReader;
#[doc = "Field `CAN0_` writer - CAN0 AHB Clock Mask"]
pub type Can0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN1_` reader - CAN1 AHB Clock Mask"]
pub type Can1_R = crate::BitReader;
#[doc = "Field `CAN1_` writer - CAN1 AHB Clock Mask"]
pub type Can1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAC_` reader - PAC AHB Clock Mask"]
pub type Pac_R = crate::BitReader;
#[doc = "Field `PAC_` writer - PAC AHB Clock Mask"]
pub type Pac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NVMCTRL_PICACHU_` reader - NVMCTRL_PICACHU AHB Clock Mask"]
pub type NvmctrlPicachu_R = crate::BitReader;
#[doc = "Field `NVMCTRL_PICACHU_` writer - NVMCTRL_PICACHU AHB Clock Mask"]
pub type NvmctrlPicachu_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVAS_` reader - DIVAS AHB Clock Mask"]
pub type Divas_R = crate::BitReader;
#[doc = "Field `DIVAS_` writer - DIVAS AHB Clock Mask"]
pub type Divas_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - HPB0 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb0_(&self) -> Hpb0_R {
        Hpb0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HPB1 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb1_(&self) -> Hpb1_R {
        Hpb1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HPB2 AHB Clock Mask"]
    #[inline(always)]
    pub fn hpb2_(&self) -> Hpb2_R {
        Hpb2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSU AHB Clock Mask"]
    #[inline(always)]
    pub fn dsu_(&self) -> Dsu_R {
        Dsu_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HMATRIXHS AHB Clock Mask"]
    #[inline(always)]
    pub fn hmatrixhs_(&self) -> Hmatrixhs_R {
        Hmatrixhs_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> Nvmctrl_R {
        Nvmctrl_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSRAM AHB Clock Mask"]
    #[inline(always)]
    pub fn hsram_(&self) -> Hsram_R {
        Hsram_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMAC AHB Clock Mask"]
    #[inline(always)]
    pub fn dmac_(&self) -> Dmac_R {
        Dmac_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CAN0 AHB Clock Mask"]
    #[inline(always)]
    pub fn can0_(&self) -> Can0_R {
        Can0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CAN1 AHB Clock Mask"]
    #[inline(always)]
    pub fn can1_(&self) -> Can1_R {
        Can1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PAC AHB Clock Mask"]
    #[inline(always)]
    pub fn pac_(&self) -> Pac_R {
        Pac_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NVMCTRL_PICACHU AHB Clock Mask"]
    #[inline(always)]
    pub fn nvmctrl_picachu_(&self) -> NvmctrlPicachu_R {
        NvmctrlPicachu_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DIVAS AHB Clock Mask"]
    #[inline(always)]
    pub fn divas_(&self) -> Divas_R {
        Divas_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HPB0 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb0_(&mut self) -> Hpb0_W<AhbmaskSpec> {
        Hpb0_W::new(self, 0)
    }
    #[doc = "Bit 1 - HPB1 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb1_(&mut self) -> Hpb1_W<AhbmaskSpec> {
        Hpb1_W::new(self, 1)
    }
    #[doc = "Bit 2 - HPB2 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hpb2_(&mut self) -> Hpb2_W<AhbmaskSpec> {
        Hpb2_W::new(self, 2)
    }
    #[doc = "Bit 3 - DSU AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dsu_(&mut self) -> Dsu_W<AhbmaskSpec> {
        Dsu_W::new(self, 3)
    }
    #[doc = "Bit 4 - HMATRIXHS AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hmatrixhs_(&mut self) -> Hmatrixhs_W<AhbmaskSpec> {
        Hmatrixhs_W::new(self, 4)
    }
    #[doc = "Bit 5 - NVMCTRL AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_(&mut self) -> Nvmctrl_W<AhbmaskSpec> {
        Nvmctrl_W::new(self, 5)
    }
    #[doc = "Bit 6 - HSRAM AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn hsram_(&mut self) -> Hsram_W<AhbmaskSpec> {
        Hsram_W::new(self, 6)
    }
    #[doc = "Bit 7 - DMAC AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn dmac_(&mut self) -> Dmac_W<AhbmaskSpec> {
        Dmac_W::new(self, 7)
    }
    #[doc = "Bit 8 - CAN0 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn can0_(&mut self) -> Can0_W<AhbmaskSpec> {
        Can0_W::new(self, 8)
    }
    #[doc = "Bit 9 - CAN1 AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn can1_(&mut self) -> Can1_W<AhbmaskSpec> {
        Can1_W::new(self, 9)
    }
    #[doc = "Bit 10 - PAC AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pac_(&mut self) -> Pac_W<AhbmaskSpec> {
        Pac_W::new(self, 10)
    }
    #[doc = "Bit 11 - NVMCTRL_PICACHU AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nvmctrl_picachu_(&mut self) -> NvmctrlPicachu_W<AhbmaskSpec> {
        NvmctrlPicachu_W::new(self, 11)
    }
    #[doc = "Bit 12 - DIVAS AHB Clock Mask"]
    #[inline(always)]
    #[must_use]
    pub fn divas_(&mut self) -> Divas_W<AhbmaskSpec> {
        Divas_W::new(self, 12)
    }
}
#[doc = "AHB Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbmaskSpec;
impl crate::RegisterSpec for AhbmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbmask::R`](R) reader structure"]
impl crate::Readable for AhbmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbmask::W`](W) writer structure"]
impl crate::Writable for AhbmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBMASK to value 0x1cff"]
impl crate::Resettable for AhbmaskSpec {
    const RESET_VALUE: u32 = 0x1cff;
}
