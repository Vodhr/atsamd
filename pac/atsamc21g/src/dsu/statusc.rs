#[doc = "Register `STATUSC` reader"]
pub type R = crate::R<StatuscSpec>;
#[doc = "Field `STATE` reader - State"]
pub type StateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - State"]
    #[inline(always)]
    pub fn state(&self) -> StateR {
        StateR::new(self.bits & 7)
    }
}
#[doc = "Status C\n\nYou can [`read`](crate::Reg::read) this register and get [`statusc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatuscSpec;
impl crate::RegisterSpec for StatuscSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`statusc::R`](R) reader structure"]
impl crate::Readable for StatuscSpec {}
#[doc = "`reset()` method sets STATUSC to value 0"]
impl crate::Resettable for StatuscSpec {
    const RESET_VALUE: u8 = 0;
}
