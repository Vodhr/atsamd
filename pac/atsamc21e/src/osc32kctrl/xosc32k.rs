#[doc = "Register `XOSC32K` reader"]
pub struct R(crate::R<XOSC32K_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XOSC32K_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XOSC32K_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XOSC32K_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XOSC32K` writer"]
pub struct W(crate::W<XOSC32K_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XOSC32K_SPEC>;
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
impl From<crate::W<XOSC32K_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XOSC32K_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `XTALEN` reader - Crystal Oscillator Enable"]
pub struct XTALEN_R(crate::FieldReader<bool, bool>);
impl XTALEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        XTALEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XTALEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XTALEN` writer - Crystal Oscillator Enable"]
pub struct XTALEN_W<'a> {
    w: &'a mut W,
}
impl<'a> XTALEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `EN32K` reader - 32kHz Output Enable"]
pub struct EN32K_R(crate::FieldReader<bool, bool>);
impl EN32K_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN32K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN32K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN32K` writer - 32kHz Output Enable"]
pub struct EN32K_W<'a> {
    w: &'a mut W,
}
impl<'a> EN32K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `EN1K` reader - 1kHz Output Enable"]
pub struct EN1K_R(crate::FieldReader<bool, bool>);
impl EN1K_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN1K_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN1K_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN1K` writer - 1kHz Output Enable"]
pub struct EN1K_W<'a> {
    w: &'a mut W,
}
impl<'a> EN1K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub struct RUNSTDBY_R(crate::FieldReader<bool, bool>);
impl RUNSTDBY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RUNSTDBY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RUNSTDBY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub struct RUNSTDBY_W<'a> {
    w: &'a mut W,
}
impl<'a> RUNSTDBY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub struct ONDEMAND_R(crate::FieldReader<bool, bool>);
impl ONDEMAND_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ONDEMAND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ONDEMAND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub struct ONDEMAND_W<'a> {
    w: &'a mut W,
}
impl<'a> ONDEMAND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Oscillator Start-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STARTUP_A {
    #[doc = "0: 0.122 ms"]
    CYCLE1 = 0,
    #[doc = "1: 1.068 ms"]
    CYCLE32 = 1,
    #[doc = "2: 62.6 ms"]
    CYCLE2048 = 2,
    #[doc = "3: 125 ms"]
    CYCLE4096 = 3,
    #[doc = "4: 500 ms"]
    CYCLE16384 = 4,
    #[doc = "5: 1000 ms"]
    CYCLE32768 = 5,
    #[doc = "6: 2000 ms"]
    CYCLE65536 = 6,
    #[doc = "7: 4000 ms"]
    CYCLE131072 = 7,
}
impl From<STARTUP_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STARTUP` reader - Oscillator Start-Up Time"]
pub struct STARTUP_R(crate::FieldReader<u8, STARTUP_A>);
impl STARTUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        STARTUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTUP_A {
        match self.bits {
            0 => STARTUP_A::CYCLE1,
            1 => STARTUP_A::CYCLE32,
            2 => STARTUP_A::CYCLE2048,
            3 => STARTUP_A::CYCLE4096,
            4 => STARTUP_A::CYCLE16384,
            5 => STARTUP_A::CYCLE32768,
            6 => STARTUP_A::CYCLE65536,
            7 => STARTUP_A::CYCLE131072,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLE1`"]
    #[inline(always)]
    pub fn is_cycle1(&self) -> bool {
        **self == STARTUP_A::CYCLE1
    }
    #[doc = "Checks if the value of the field is `CYCLE32`"]
    #[inline(always)]
    pub fn is_cycle32(&self) -> bool {
        **self == STARTUP_A::CYCLE32
    }
    #[doc = "Checks if the value of the field is `CYCLE2048`"]
    #[inline(always)]
    pub fn is_cycle2048(&self) -> bool {
        **self == STARTUP_A::CYCLE2048
    }
    #[doc = "Checks if the value of the field is `CYCLE4096`"]
    #[inline(always)]
    pub fn is_cycle4096(&self) -> bool {
        **self == STARTUP_A::CYCLE4096
    }
    #[doc = "Checks if the value of the field is `CYCLE16384`"]
    #[inline(always)]
    pub fn is_cycle16384(&self) -> bool {
        **self == STARTUP_A::CYCLE16384
    }
    #[doc = "Checks if the value of the field is `CYCLE32768`"]
    #[inline(always)]
    pub fn is_cycle32768(&self) -> bool {
        **self == STARTUP_A::CYCLE32768
    }
    #[doc = "Checks if the value of the field is `CYCLE65536`"]
    #[inline(always)]
    pub fn is_cycle65536(&self) -> bool {
        **self == STARTUP_A::CYCLE65536
    }
    #[doc = "Checks if the value of the field is `CYCLE131072`"]
    #[inline(always)]
    pub fn is_cycle131072(&self) -> bool {
        **self == STARTUP_A::CYCLE131072
    }
}
impl core::ops::Deref for STARTUP_R {
    type Target = crate::FieldReader<u8, STARTUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTUP` writer - Oscillator Start-Up Time"]
pub struct STARTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTUP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "0.122 ms"]
    #[inline(always)]
    pub fn cycle1(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE1)
    }
    #[doc = "1.068 ms"]
    #[inline(always)]
    pub fn cycle32(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE32)
    }
    #[doc = "62.6 ms"]
    #[inline(always)]
    pub fn cycle2048(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE2048)
    }
    #[doc = "125 ms"]
    #[inline(always)]
    pub fn cycle4096(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE4096)
    }
    #[doc = "500 ms"]
    #[inline(always)]
    pub fn cycle16384(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE16384)
    }
    #[doc = "1000 ms"]
    #[inline(always)]
    pub fn cycle32768(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE32768)
    }
    #[doc = "2000 ms"]
    #[inline(always)]
    pub fn cycle65536(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE65536)
    }
    #[doc = "4000 ms"]
    #[inline(always)]
    pub fn cycle131072(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE131072)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u16 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub struct WRTLOCK_R(crate::FieldReader<bool, bool>);
impl WRTLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRTLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRTLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub struct WRTLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> WRTLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&self) -> XTALEN_R {
        XTALEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 32kHz Output Enable"]
    #[inline(always)]
    pub fn en32k(&self) -> EN32K_R {
        EN32K_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1kHz Output Enable"]
    #[inline(always)]
    pub fn en1k(&self) -> EN1K_R {
        EN1K_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RUNSTDBY_R {
        RUNSTDBY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> ONDEMAND_R {
        ONDEMAND_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WRTLOCK_R {
        WRTLOCK_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&mut self) -> XTALEN_W {
        XTALEN_W { w: self }
    }
    #[doc = "Bit 3 - 32kHz Output Enable"]
    #[inline(always)]
    pub fn en32k(&mut self) -> EN32K_W {
        EN32K_W { w: self }
    }
    #[doc = "Bit 4 - 1kHz Output Enable"]
    #[inline(always)]
    pub fn en1k(&mut self) -> EN1K_W {
        EN1K_W { w: self }
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&mut self) -> RUNSTDBY_W {
        RUNSTDBY_W { w: self }
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&mut self) -> ONDEMAND_W {
        ONDEMAND_W { w: self }
    }
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W {
        STARTUP_W { w: self }
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&mut self) -> WRTLOCK_W {
        WRTLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32kHz External Crystal Oscillator (XOSC32K) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xosc32k](index.html) module"]
pub struct XOSC32K_SPEC;
impl crate::RegisterSpec for XOSC32K_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [xosc32k::R](R) reader structure"]
impl crate::Readable for XOSC32K_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xosc32k::W](W) writer structure"]
impl crate::Writable for XOSC32K_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XOSC32K to value 0x80"]
impl crate::Resettable for XOSC32K_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
