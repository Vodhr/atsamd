#[doc = "Register `OFFSET` reader"]
pub type R = crate::R<OffsetSpec>;
#[doc = "Register `OFFSET` writer"]
pub type W = crate::W<OffsetSpec>;
#[doc = "Field `OFFSETC` reader - Offset Correction"]
pub type OffsetcR = crate::FieldReader<u32>;
#[doc = "Field `OFFSETC` writer - Offset Correction"]
pub type OffsetcW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Offset Correction"]
    #[inline(always)]
    pub fn offsetc(&self) -> OffsetcR {
        OffsetcR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Offset Correction"]
    #[inline(always)]
    #[must_use]
    pub fn offsetc(&mut self) -> OffsetcW<OffsetSpec> {
        OffsetcW::new(self, 0)
    }
}
#[doc = "Offset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`offset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`offset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OffsetSpec;
impl crate::RegisterSpec for OffsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`offset::R`](R) reader structure"]
impl crate::Readable for OffsetSpec {}
#[doc = "`write(|w| ..)` method takes [`offset::W`](W) writer structure"]
impl crate::Writable for OffsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OFFSET to value 0"]
impl crate::Resettable for OffsetSpec {
    const RESET_VALUE: u32 = 0;
}
