#[doc = "Register `DIVIDEND` reader"]
pub type R = crate::R<DividendSpec>;
#[doc = "Register `DIVIDEND` writer"]
pub type W = crate::W<DividendSpec>;
#[doc = "Field `DIVIDEND` reader - DIVIDEND"]
pub type DividendR = crate::FieldReader<u32>;
#[doc = "Field `DIVIDEND` writer - DIVIDEND"]
pub type DividendW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DIVIDEND"]
    #[inline(always)]
    pub fn dividend(&self) -> DividendR {
        DividendR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DIVIDEND"]
    #[inline(always)]
    #[must_use]
    pub fn dividend(&mut self) -> DividendW<DividendSpec> {
        DividendW::new(self, 0)
    }
}
#[doc = "Dividend\n\nYou can [`read`](crate::Reg::read) this register and get [`dividend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dividend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DividendSpec;
impl crate::RegisterSpec for DividendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dividend::R`](R) reader structure"]
impl crate::Readable for DividendSpec {}
#[doc = "`write(|w| ..)` method takes [`dividend::W`](W) writer structure"]
impl crate::Writable for DividendSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIVIDEND to value 0"]
impl crate::Resettable for DividendSpec {
    const RESET_VALUE: u32 = 0;
}
