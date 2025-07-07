#[doc = "Register `REM` reader"]
pub type R = crate::R<RemSpec>;
#[doc = "Field `REM` reader - REM"]
pub type RemR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - REM"]
    #[inline(always)]
    pub fn rem(&self) -> RemR {
        RemR::new(self.bits)
    }
}
#[doc = "Remainder\n\nYou can [`read`](crate::Reg::read) this register and get [`rem::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RemSpec;
impl crate::RegisterSpec for RemSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rem::R`](R) reader structure"]
impl crate::Readable for RemSpec {}
#[doc = "`reset()` method sets REM to value 0"]
impl crate::Resettable for RemSpec {
    const RESET_VALUE: u32 = 0;
}
