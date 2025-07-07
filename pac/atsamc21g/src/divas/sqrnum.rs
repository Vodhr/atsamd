#[doc = "Register `SQRNUM` reader"]
pub type R = crate::R<SqrnumSpec>;
#[doc = "Register `SQRNUM` writer"]
pub type W = crate::W<SqrnumSpec>;
#[doc = "Field `SQRNUM` reader - Square Root Input"]
pub type SqrnumR = crate::FieldReader<u32>;
#[doc = "Field `SQRNUM` writer - Square Root Input"]
pub type SqrnumW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Square Root Input"]
    #[inline(always)]
    pub fn sqrnum(&self) -> SqrnumR {
        SqrnumR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Square Root Input"]
    #[inline(always)]
    #[must_use]
    pub fn sqrnum(&mut self) -> SqrnumW<SqrnumSpec> {
        SqrnumW::new(self, 0)
    }
}
#[doc = "Square Root Input\n\nYou can [`read`](crate::Reg::read) this register and get [`sqrnum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqrnum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SqrnumSpec;
impl crate::RegisterSpec for SqrnumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sqrnum::R`](R) reader structure"]
impl crate::Readable for SqrnumSpec {}
#[doc = "`write(|w| ..)` method takes [`sqrnum::W`](W) writer structure"]
impl crate::Writable for SqrnumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SQRNUM to value 0"]
impl crate::Resettable for SqrnumSpec {
    const RESET_VALUE: u32 = 0;
}
