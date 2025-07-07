#[doc = "Register `RESULT` reader"]
pub type R = crate::R<ResultSpec>;
#[doc = "Field `RESULT` reader - Result Value"]
pub type ResultR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - Result Value"]
    #[inline(always)]
    pub fn result(&self) -> ResultR {
        ResultR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "Result\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResultSpec;
impl crate::RegisterSpec for ResultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result::R`](R) reader structure"]
impl crate::Readable for ResultSpec {}
#[doc = "`reset()` method sets RESULT to value 0"]
impl crate::Resettable for ResultSpec {
    const RESET_VALUE: u32 = 0;
}
