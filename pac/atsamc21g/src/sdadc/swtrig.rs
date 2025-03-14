#[doc = "Register `SWTRIG` reader"]
pub struct R(crate::R<SWTRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWTRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWTRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWTRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWTRIG` writer"]
pub struct W(crate::W<SWTRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWTRIG_SPEC>;
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
impl From<crate::W<SWTRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWTRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLUSH` reader - SDADC Flush"]
pub struct FLUSH_R(crate::FieldReader<bool, bool>);
impl FLUSH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLUSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLUSH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FLUSH` writer - SDADC Flush"]
pub struct FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLUSH_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u8 & 0x01);
        self.w
    }
}
#[doc = "Field `START` reader - Start SDADC Conversion"]
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - Start SDADC Conversion"]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u8 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SDADC Flush"]
    #[inline(always)]
    pub fn flush(&self) -> FLUSH_R {
        FLUSH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start SDADC Conversion"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SDADC Flush"]
    #[inline(always)]
    pub fn flush(&mut self) -> FLUSH_W {
        FLUSH_W { w: self }
    }
    #[doc = "Bit 1 - Start SDADC Conversion"]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Trigger\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swtrig](index.html) module"]
pub struct SWTRIG_SPEC;
impl crate::RegisterSpec for SWTRIG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [swtrig::R](R) reader structure"]
impl crate::Readable for SWTRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swtrig::W](W) writer structure"]
impl crate::Writable for SWTRIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SWTRIG to value 0"]
impl crate::Resettable for SWTRIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
