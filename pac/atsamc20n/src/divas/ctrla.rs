#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SIGNED` reader - Signed"]
pub type SignedR = crate::BitReader;
#[doc = "Field `SIGNED` writer - Signed"]
pub type SignedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLZ` reader - Disable Leading Zero Optimization"]
pub type DlzR = crate::BitReader;
#[doc = "Field `DLZ` writer - Disable Leading Zero Optimization"]
pub type DlzW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Signed"]
    #[inline(always)]
    pub fn signed(&self) -> SignedR {
        SignedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable Leading Zero Optimization"]
    #[inline(always)]
    pub fn dlz(&self) -> DlzR {
        DlzR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Signed"]
    #[inline(always)]
    #[must_use]
    pub fn signed(&mut self) -> SignedW<CtrlaSpec> {
        SignedW::new(self, 0)
    }
    #[doc = "Bit 1 - Disable Leading Zero Optimization"]
    #[inline(always)]
    #[must_use]
    pub fn dlz(&mut self) -> DlzW<CtrlaSpec> {
        DlzW::new(self, 1)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u8 = 0;
}
