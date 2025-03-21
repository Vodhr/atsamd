#[doc = "Register `NMICTRL` reader"]
pub struct R(crate::R<NMICTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NMICTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NMICTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NMICTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NMICTRL` writer"]
pub struct W(crate::W<NMICTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NMICTRL_SPEC>;
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
impl From<crate::W<NMICTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NMICTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "NMI Input Sense Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NMISENSE_A {
    #[doc = "0: No detection"]
    NONE = 0,
    #[doc = "1: Rising-edge detection"]
    RISE = 1,
    #[doc = "2: Falling-edge detection"]
    FALL = 2,
    #[doc = "3: Both-edges detection"]
    BOTH = 3,
    #[doc = "4: High-level detection"]
    HIGH = 4,
    #[doc = "5: Low-level detection"]
    LOW = 5,
}
impl From<NMISENSE_A> for u8 {
    #[inline(always)]
    fn from(variant: NMISENSE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NMISENSE` reader - NMI Input Sense Configuration"]
pub struct NMISENSE_R(crate::FieldReader<u8, NMISENSE_A>);
impl NMISENSE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        NMISENSE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NMISENSE_A> {
        match self.bits {
            0 => Some(NMISENSE_A::NONE),
            1 => Some(NMISENSE_A::RISE),
            2 => Some(NMISENSE_A::FALL),
            3 => Some(NMISENSE_A::BOTH),
            4 => Some(NMISENSE_A::HIGH),
            5 => Some(NMISENSE_A::LOW),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == NMISENSE_A::NONE
    }
    #[doc = "Checks if the value of the field is `RISE`"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        **self == NMISENSE_A::RISE
    }
    #[doc = "Checks if the value of the field is `FALL`"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        **self == NMISENSE_A::FALL
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == NMISENSE_A::BOTH
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == NMISENSE_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == NMISENSE_A::LOW
    }
}
impl core::ops::Deref for NMISENSE_R {
    type Target = crate::FieldReader<u8, NMISENSE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMISENSE` writer - NMI Input Sense Configuration"]
pub struct NMISENSE_W<'a> {
    w: &'a mut W,
}
impl<'a> NMISENSE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMISENSE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No detection"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(NMISENSE_A::NONE)
    }
    #[doc = "Rising-edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut W {
        self.variant(NMISENSE_A::RISE)
    }
    #[doc = "Falling-edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut W {
        self.variant(NMISENSE_A::FALL)
    }
    #[doc = "Both-edges detection"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(NMISENSE_A::BOTH)
    }
    #[doc = "High-level detection"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(NMISENSE_A::HIGH)
    }
    #[doc = "Low-level detection"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(NMISENSE_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u8 & 0x07);
        self.w
    }
}
#[doc = "Field `NMIFILTEN` reader - NMI Filter Enable"]
pub struct NMIFILTEN_R(crate::FieldReader<bool, bool>);
impl NMIFILTEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NMIFILTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NMIFILTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMIFILTEN` writer - NMI Filter Enable"]
pub struct NMIFILTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIFILTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u8 & 0x01) << 3);
        self.w
    }
}
#[doc = "NMI Asynchronous edge Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NMIASYNCH_A {
    #[doc = "0: Edge detection is clock synchronously operated"]
    SYNC = 0,
    #[doc = "1: Edge detection is clock asynchronously operated"]
    ASYNC = 1,
}
impl From<NMIASYNCH_A> for bool {
    #[inline(always)]
    fn from(variant: NMIASYNCH_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMIASYNCH` reader - NMI Asynchronous edge Detection Enable"]
pub struct NMIASYNCH_R(crate::FieldReader<bool, NMIASYNCH_A>);
impl NMIASYNCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NMIASYNCH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMIASYNCH_A {
        match self.bits {
            false => NMIASYNCH_A::SYNC,
            true => NMIASYNCH_A::ASYNC,
        }
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        **self == NMIASYNCH_A::SYNC
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        **self == NMIASYNCH_A::ASYNC
    }
}
impl core::ops::Deref for NMIASYNCH_R {
    type Target = crate::FieldReader<bool, NMIASYNCH_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NMIASYNCH` writer - NMI Asynchronous edge Detection Enable"]
pub struct NMIASYNCH_W<'a> {
    w: &'a mut W,
}
impl<'a> NMIASYNCH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NMIASYNCH_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Edge detection is clock synchronously operated"]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(NMIASYNCH_A::SYNC)
    }
    #[doc = "Edge detection is clock asynchronously operated"]
    #[inline(always)]
    pub fn async_(self) -> &'a mut W {
        self.variant(NMIASYNCH_A::ASYNC)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u8 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - NMI Input Sense Configuration"]
    #[inline(always)]
    pub fn nmisense(&self) -> NMISENSE_R {
        NMISENSE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - NMI Filter Enable"]
    #[inline(always)]
    pub fn nmifilten(&self) -> NMIFILTEN_R {
        NMIFILTEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - NMI Asynchronous edge Detection Enable"]
    #[inline(always)]
    pub fn nmiasynch(&self) -> NMIASYNCH_R {
        NMIASYNCH_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - NMI Input Sense Configuration"]
    #[inline(always)]
    pub fn nmisense(&mut self) -> NMISENSE_W {
        NMISENSE_W { w: self }
    }
    #[doc = "Bit 3 - NMI Filter Enable"]
    #[inline(always)]
    pub fn nmifilten(&mut self) -> NMIFILTEN_W {
        NMIFILTEN_W { w: self }
    }
    #[doc = "Bit 4 - NMI Asynchronous edge Detection Enable"]
    #[inline(always)]
    pub fn nmiasynch(&mut self) -> NMIASYNCH_W {
        NMIASYNCH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NMI Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nmictrl](index.html) module"]
pub struct NMICTRL_SPEC;
impl crate::RegisterSpec for NMICTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [nmictrl::R](R) reader structure"]
impl crate::Readable for NMICTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nmictrl::W](W) writer structure"]
impl crate::Writable for NMICTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NMICTRL to value 0"]
impl crate::Resettable for NMICTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
