#[doc = "Register `DIRCLR` reader"]
pub type R = crate::R<DirclrSpec>;
#[doc = "Register `DIRCLR` writer"]
pub type W = crate::W<DirclrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Data Direction Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`dirclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dirclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirclrSpec;
impl crate::RegisterSpec for DirclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dirclr::R`](R) reader structure"]
impl crate::Readable for DirclrSpec {}
#[doc = "`write(|w| ..)` method takes [`dirclr::W`](W) writer structure"]
impl crate::Writable for DirclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIRCLR to value 0"]
impl crate::Resettable for DirclrSpec {
    const RESET_VALUE: u32 = 0;
}
