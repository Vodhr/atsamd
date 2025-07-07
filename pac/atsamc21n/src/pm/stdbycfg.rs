#[doc = "Register `STDBYCFG` reader"]
pub type R = crate::R<StdbycfgSpec>;
#[doc = "Register `STDBYCFG` writer"]
pub type W = crate::W<StdbycfgSpec>;
#[doc = "Voltage Regulator Standby mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vregsmodselect {
    #[doc = "0: Automatic mode"]
    Auto = 0,
    #[doc = "1: Performance oriented"]
    Performance = 1,
    #[doc = "2: Low Power oriented"]
    Lp = 2,
}
impl From<Vregsmodselect> for u8 {
    #[inline(always)]
    fn from(variant: Vregsmodselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vregsmodselect {
    type Ux = u8;
}
impl crate::IsEnum for Vregsmodselect {}
#[doc = "Field `VREGSMOD` reader - Voltage Regulator Standby mode"]
pub type VregsmodR = crate::FieldReader<Vregsmodselect>;
impl VregsmodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vregsmodselect> {
        match self.bits {
            0 => Some(Vregsmodselect::Auto),
            1 => Some(Vregsmodselect::Performance),
            2 => Some(Vregsmodselect::Lp),
            _ => None,
        }
    }
    #[doc = "Automatic mode"]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == Vregsmodselect::Auto
    }
    #[doc = "Performance oriented"]
    #[inline(always)]
    pub fn is_performance(&self) -> bool {
        *self == Vregsmodselect::Performance
    }
    #[doc = "Low Power oriented"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        *self == Vregsmodselect::Lp
    }
}
#[doc = "Field `VREGSMOD` writer - Voltage Regulator Standby mode"]
pub type VregsmodW<'a, REG> = crate::FieldWriter<'a, REG, 2, Vregsmodselect>;
impl<'a, REG> VregsmodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Automatic mode"]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(Vregsmodselect::Auto)
    }
    #[doc = "Performance oriented"]
    #[inline(always)]
    pub fn performance(self) -> &'a mut crate::W<REG> {
        self.variant(Vregsmodselect::Performance)
    }
    #[doc = "Low Power oriented"]
    #[inline(always)]
    pub fn lp(self) -> &'a mut crate::W<REG> {
        self.variant(Vregsmodselect::Lp)
    }
}
#[doc = "Field `BBIASHS` reader - Back Bias for HMCRAMCHS"]
pub type BbiashsR = crate::FieldReader;
#[doc = "Field `BBIASHS` writer - Back Bias for HMCRAMCHS"]
pub type BbiashsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 6:7 - Voltage Regulator Standby mode"]
    #[inline(always)]
    pub fn vregsmod(&self) -> VregsmodR {
        VregsmodR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Back Bias for HMCRAMCHS"]
    #[inline(always)]
    pub fn bbiashs(&self) -> BbiashsR {
        BbiashsR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 6:7 - Voltage Regulator Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn vregsmod(&mut self) -> VregsmodW<StdbycfgSpec> {
        VregsmodW::new(self, 6)
    }
    #[doc = "Bits 10:11 - Back Bias for HMCRAMCHS"]
    #[inline(always)]
    #[must_use]
    pub fn bbiashs(&mut self) -> BbiashsW<StdbycfgSpec> {
        BbiashsW::new(self, 10)
    }
}
#[doc = "Standby Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`stdbycfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stdbycfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StdbycfgSpec;
impl crate::RegisterSpec for StdbycfgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`stdbycfg::R`](R) reader structure"]
impl crate::Readable for StdbycfgSpec {}
#[doc = "`write(|w| ..)` method takes [`stdbycfg::W`](W) writer structure"]
impl crate::Writable for StdbycfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets STDBYCFG to value 0x0400"]
impl crate::Resettable for StdbycfgSpec {
    const RESET_VALUE: u16 = 0x0400;
}
