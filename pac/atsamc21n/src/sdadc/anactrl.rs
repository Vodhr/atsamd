#[doc = "Register `ANACTRL` reader"]
pub type R = crate::R<AnactrlSpec>;
#[doc = "Register `ANACTRL` writer"]
pub type W = crate::W<AnactrlSpec>;
#[doc = "Field `CTRSDADC` reader - SDADC Control"]
pub type CtrsdadcR = crate::FieldReader;
#[doc = "Field `CTRSDADC` writer - SDADC Control"]
pub type CtrsdadcW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ONCHOP` reader - Chopper"]
pub type OnchopR = crate::BitReader;
#[doc = "Field `ONCHOP` writer - Chopper"]
pub type OnchopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUFTEST` reader - BUFTEST"]
pub type BuftestR = crate::BitReader;
#[doc = "Field `BUFTEST` writer - BUFTEST"]
pub type BuftestW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - SDADC Control"]
    #[inline(always)]
    pub fn ctrsdadc(&self) -> CtrsdadcR {
        CtrsdadcR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - Chopper"]
    #[inline(always)]
    pub fn onchop(&self) -> OnchopR {
        OnchopR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BUFTEST"]
    #[inline(always)]
    pub fn buftest(&self) -> BuftestR {
        BuftestR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - SDADC Control"]
    #[inline(always)]
    #[must_use]
    pub fn ctrsdadc(&mut self) -> CtrsdadcW<AnactrlSpec> {
        CtrsdadcW::new(self, 0)
    }
    #[doc = "Bit 6 - Chopper"]
    #[inline(always)]
    #[must_use]
    pub fn onchop(&mut self) -> OnchopW<AnactrlSpec> {
        OnchopW::new(self, 6)
    }
    #[doc = "Bit 7 - BUFTEST"]
    #[inline(always)]
    #[must_use]
    pub fn buftest(&mut self) -> BuftestW<AnactrlSpec> {
        BuftestW::new(self, 7)
    }
}
#[doc = "Analog Control\n\nYou can [`read`](crate::Reg::read) this register and get [`anactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`anactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnactrlSpec;
impl crate::RegisterSpec for AnactrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`anactrl::R`](R) reader structure"]
impl crate::Readable for AnactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`anactrl::W`](W) writer structure"]
impl crate::Writable for AnactrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ANACTRL to value 0"]
impl crate::Resettable for AnactrlSpec {
    const RESET_VALUE: u8 = 0;
}
