#[doc = "Register `SHIFTCORR` reader"]
pub struct R(crate::R<SHIFTCORR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTCORR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTCORR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTCORR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTCORR` writer"]
pub struct W(crate::W<SHIFTCORR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTCORR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SHIFTCORR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTCORR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHIFTCORR` reader - Shift Correction Value"]
pub struct SHIFTCORR_R(crate::FieldReader<u8, u8>);
impl SHIFTCORR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SHIFTCORR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SHIFTCORR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SHIFTCORR` writer - Shift Correction Value"]
pub struct SHIFTCORR_W<'a> {
    w: &'a mut W,
}
impl<'a> SHIFTCORR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u8 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Shift Correction Value"]
    #[inline(always)]
    pub fn shiftcorr(&self) -> SHIFTCORR_R {
        SHIFTCORR_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Shift Correction Value"]
    #[inline(always)]
    pub fn shiftcorr(&mut self) -> SHIFTCORR_W {
        SHIFTCORR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shift Correction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftcorr](index.html) module"]
pub struct SHIFTCORR_SPEC;
impl crate::RegisterSpec for SHIFTCORR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [shiftcorr::R](R) reader structure"]
impl crate::Readable for SHIFTCORR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftcorr::W](W) writer structure"]
impl crate::Writable for SHIFTCORR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHIFTCORR to value 0"]
impl crate::Resettable for SHIFTCORR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
