#[doc = "Register `DIRTGL` reader"]
pub type R = crate::R<DirtglSpec>;
#[doc = "Register `DIRTGL` writer"]
pub type W = crate::W<DirtglSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data Direction Toggle\n\nYou can [`read`](crate::Reg::read) this register and get [`dirtgl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirtgl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirtglSpec;
impl crate::RegisterSpec for DirtglSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dirtgl::R`](R) reader structure"]
impl crate::Readable for DirtglSpec {}
#[doc = "`write(|w| ..)` method takes [`dirtgl::W`](W) writer structure"]
impl crate::Writable for DirtglSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIRTGL to value 0"]
impl crate::Resettable for DirtglSpec {
    const RESET_VALUE: u32 = 0;
}
