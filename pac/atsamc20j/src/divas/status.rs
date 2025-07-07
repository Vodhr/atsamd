#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `BUSY` reader - DIVAS Accelerator Busy"]
pub type BusyR = crate::BitReader;
#[doc = "Field `BUSY` writer - DIVAS Accelerator Busy"]
pub type BusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBZ` reader - Writing a one to this bit clears DBZ to zero"]
pub type DbzR = crate::BitReader;
#[doc = "Field `DBZ` writer - Writing a one to this bit clears DBZ to zero"]
pub type DbzW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DIVAS Accelerator Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Writing a one to this bit clears DBZ to zero"]
    #[inline(always)]
    pub fn dbz(&self) -> DbzR {
        DbzR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DIVAS Accelerator Busy"]
    #[inline(always)]
    #[must_use]
    pub fn busy(&mut self) -> BusyW<StatusSpec> {
        BusyW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a one to this bit clears DBZ to zero"]
    #[inline(always)]
    #[must_use]
    pub fn dbz(&mut self) -> DbzW<StatusSpec> {
        DbzW::new(self, 1)
    }
}
#[doc = "Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u8 = 0;
}
