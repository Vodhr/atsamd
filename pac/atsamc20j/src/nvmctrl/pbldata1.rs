#[doc = "Register `PBLDATA1` reader"]
pub type R = crate::R<Pbldata1Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Page Buffer Load Data 1\n\nYou can [`read`](crate::Reg::read) this register and get [`pbldata1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pbldata1Spec;
impl crate::RegisterSpec for Pbldata1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbldata1::R`](R) reader structure"]
impl crate::Readable for Pbldata1Spec {}
#[doc = "`reset()` method sets PBLDATA1 to value 0"]
impl crate::Resettable for Pbldata1Spec {
    const RESET_VALUE: u32 = 0;
}
