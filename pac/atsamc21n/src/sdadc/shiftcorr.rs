#[doc = "Register `SHIFTCORR` reader"]
pub type R = crate::R<ShiftcorrSpec>;
#[doc = "Register `SHIFTCORR` writer"]
pub type W = crate::W<ShiftcorrSpec>;
#[doc = "Field `SHIFTCORR` reader - Shift Correction Value"]
pub type ShiftcorrR = crate::FieldReader;
#[doc = "Field `SHIFTCORR` writer - Shift Correction Value"]
pub type ShiftcorrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Shift Correction Value"]
    #[inline(always)]
    pub fn shiftcorr(&self) -> ShiftcorrR {
        ShiftcorrR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shift Correction Value"]
    #[inline(always)]
    #[must_use]
    pub fn shiftcorr(&mut self) -> ShiftcorrW<ShiftcorrSpec> {
        ShiftcorrW::new(self, 0)
    }
}
#[doc = "Shift Correction\n\nYou can [`read`](crate::Reg::read) this register and get [`shiftcorr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shiftcorr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShiftcorrSpec;
impl crate::RegisterSpec for ShiftcorrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`shiftcorr::R`](R) reader structure"]
impl crate::Readable for ShiftcorrSpec {}
#[doc = "`write(|w| ..)` method takes [`shiftcorr::W`](W) writer structure"]
impl crate::Writable for ShiftcorrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SHIFTCORR to value 0"]
impl crate::Resettable for ShiftcorrSpec {
    const RESET_VALUE: u8 = 0;
}
