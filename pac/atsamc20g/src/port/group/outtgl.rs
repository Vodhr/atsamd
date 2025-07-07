#[doc = "Register `OUTTGL` reader"]
pub type R = crate::R<OuttglSpec>;
#[doc = "Register `OUTTGL` writer"]
pub type W = crate::W<OuttglSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data Output Value Toggle\n\nYou can [`read`](crate::Reg::read) this register and get [`outtgl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outtgl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OuttglSpec;
impl crate::RegisterSpec for OuttglSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outtgl::R`](R) reader structure"]
impl crate::Readable for OuttglSpec {}
#[doc = "`write(|w| ..)` method takes [`outtgl::W`](W) writer structure"]
impl crate::Writable for OuttglSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUTTGL to value 0"]
impl crate::Resettable for OuttglSpec {
    const RESET_VALUE: u32 = 0;
}
