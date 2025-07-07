#[doc = "Register `DIRSET` reader"]
pub type R = crate::R<DirsetSpec>;
#[doc = "Register `DIRSET` writer"]
pub type W = crate::W<DirsetSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data Direction Set\n\nYou can [`read`](crate::Reg::read) this register and get [`dirset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirsetSpec;
impl crate::RegisterSpec for DirsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dirset::R`](R) reader structure"]
impl crate::Readable for DirsetSpec {}
#[doc = "`write(|w| ..)` method takes [`dirset::W`](W) writer structure"]
impl crate::Writable for DirsetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIRSET to value 0"]
impl crate::Resettable for DirsetSpec {
    const RESET_VALUE: u32 = 0;
}
