#[doc = "Register `PBLDATA0` reader"]
pub type R = crate::R<Pbldata0Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Page Buffer Load Data 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbldata0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pbldata0Spec;
impl crate::RegisterSpec for Pbldata0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbldata0::R`](R) reader structure"]
impl crate::Readable for Pbldata0Spec {}
#[doc = "`reset()` method sets PBLDATA0 to value 0"]
impl crate::Resettable for Pbldata0Spec {
    const RESET_VALUE: u32 = 0;
}
