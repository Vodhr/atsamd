#[doc = "Register `INTENCLR` reader"]
pub struct R(crate::R<INTENCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTENCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTENCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTENCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTENCLR` writer"]
pub struct W(crate::W<INTENCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTENCLR_SPEC>;
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
impl From<crate::W<INTENCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTENCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BODVDDRDY` reader - BODVDD Ready"]
pub struct BODVDDRDY_R(crate::FieldReader<bool, bool>);
impl BODVDDRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BODVDDRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODVDDRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODVDDRDY` writer - BODVDD Ready"]
pub struct BODVDDRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BODVDDRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `BODVDDDET` reader - BODVDD Detection"]
pub struct BODVDDDET_R(crate::FieldReader<bool, bool>);
impl BODVDDDET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BODVDDDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODVDDDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODVDDDET` writer - BODVDD Detection"]
pub struct BODVDDDET_W<'a> {
    w: &'a mut W,
}
impl<'a> BODVDDDET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `BVDDSRDY` reader - BODVDD Synchronization Ready"]
pub struct BVDDSRDY_R(crate::FieldReader<bool, bool>);
impl BVDDSRDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BVDDSRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BVDDSRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BVDDSRDY` writer - BODVDD Synchronization Ready"]
pub struct BVDDSRDY_W<'a> {
    w: &'a mut W,
}
impl<'a> BVDDSRDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `VREG33RDY` reader - VREG33 Ready"]
pub struct VREG33RDY_R(crate::FieldReader<bool, bool>);
impl VREG33RDY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VREG33RDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREG33RDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREG33RDY` writer - VREG33 Ready"]
pub struct VREG33RDY_W<'a> {
    w: &'a mut W,
}
impl<'a> VREG33RDY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - BODVDD Ready"]
    #[inline(always)]
    pub fn bodvddrdy(&self) -> BODVDDRDY_R {
        BODVDDRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - BODVDD Detection"]
    #[inline(always)]
    pub fn bodvdddet(&self) -> BODVDDDET_R {
        BODVDDDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - BODVDD Synchronization Ready"]
    #[inline(always)]
    pub fn bvddsrdy(&self) -> BVDDSRDY_R {
        BVDDSRDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - VREG33 Ready"]
    #[inline(always)]
    pub fn vreg33rdy(&self) -> VREG33RDY_R {
        VREG33RDY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BODVDD Ready"]
    #[inline(always)]
    pub fn bodvddrdy(&mut self) -> BODVDDRDY_W {
        BODVDDRDY_W { w: self }
    }
    #[doc = "Bit 1 - BODVDD Detection"]
    #[inline(always)]
    pub fn bodvdddet(&mut self) -> BODVDDDET_W {
        BODVDDDET_W { w: self }
    }
    #[doc = "Bit 2 - BODVDD Synchronization Ready"]
    #[inline(always)]
    pub fn bvddsrdy(&mut self) -> BVDDSRDY_W {
        BVDDSRDY_W { w: self }
    }
    #[doc = "Bit 6 - VREG33 Ready"]
    #[inline(always)]
    pub fn vreg33rdy(&mut self) -> VREG33RDY_W {
        VREG33RDY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intenclr](index.html) module"]
pub struct INTENCLR_SPEC;
impl crate::RegisterSpec for INTENCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intenclr::R](R) reader structure"]
impl crate::Readable for INTENCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intenclr::W](W) writer structure"]
impl crate::Writable for INTENCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for INTENCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
