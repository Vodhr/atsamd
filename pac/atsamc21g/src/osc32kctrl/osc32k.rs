#[doc = "Register `OSC32K` reader"]
pub struct R(crate::R<OSC32K_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC32K_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC32K_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC32K_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC32K` writer"]
pub struct W(crate::W<OSC32K_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC32K_SPEC>;
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
impl From<crate::W<OSC32K_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC32K_SPEC>) -> Self {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Oscillator Start-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STARTUP_A {
    #[doc = "0: 0.092 ms"]
    CYCLE3 = 0,
    #[doc = "1: 0.122 ms"]
    CYCLE4 = 1,
    #[doc = "2: 0.183 ms"]
    CYCLE6 = 2,
    #[doc = "3: 0.305 ms"]
    CYCLE10 = 3,
    #[doc = "4: 0.549 ms"]
    CYCLE18 = 4,
    #[doc = "5: 1.038 ms"]
    CYCLE34 = 5,
    #[doc = "6: 2.014 ms"]
    CYCLE66 = 6,
    #[doc = "7: 3.967 ms"]
    CYCLE130 = 7,
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
            0 => STARTUP_A::CYCLE3,
            1 => STARTUP_A::CYCLE4,
            2 => STARTUP_A::CYCLE6,
            3 => STARTUP_A::CYCLE10,
            4 => STARTUP_A::CYCLE18,
            5 => STARTUP_A::CYCLE34,
            6 => STARTUP_A::CYCLE66,
            7 => STARTUP_A::CYCLE130,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLE3`"]
    #[inline(always)]
    pub fn is_cycle3(&self) -> bool {
        **self == STARTUP_A::CYCLE3
    }
    #[doc = "Checks if the value of the field is `CYCLE4`"]
    #[inline(always)]
    pub fn is_cycle4(&self) -> bool {
        **self == STARTUP_A::CYCLE4
    }
    #[doc = "Checks if the value of the field is `CYCLE6`"]
    #[inline(always)]
    pub fn is_cycle6(&self) -> bool {
        **self == STARTUP_A::CYCLE6
    }
    #[doc = "Checks if the value of the field is `CYCLE10`"]
    #[inline(always)]
    pub fn is_cycle10(&self) -> bool {
        **self == STARTUP_A::CYCLE10
    }
    #[doc = "Checks if the value of the field is `CYCLE18`"]
    #[inline(always)]
    pub fn is_cycle18(&self) -> bool {
        **self == STARTUP_A::CYCLE18
    }
    #[doc = "Checks if the value of the field is `CYCLE34`"]
    #[inline(always)]
    pub fn is_cycle34(&self) -> bool {
        **self == STARTUP_A::CYCLE34
    }
    #[doc = "Checks if the value of the field is `CYCLE66`"]
    #[inline(always)]
    pub fn is_cycle66(&self) -> bool {
        **self == STARTUP_A::CYCLE66
    }
    #[doc = "Checks if the value of the field is `CYCLE130`"]
    #[inline(always)]
    pub fn is_cycle130(&self) -> bool {
        **self == STARTUP_A::CYCLE130
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
    #[doc = "0.092 ms"]
    #[inline(always)]
    pub fn cycle3(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE3)
    }
    #[doc = "0.122 ms"]
    #[inline(always)]
    pub fn cycle4(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE4)
    }
    #[doc = "0.183 ms"]
    #[inline(always)]
    pub fn cycle6(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE6)
    }
    #[doc = "0.305 ms"]
    #[inline(always)]
    pub fn cycle10(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE10)
    }
    #[doc = "0.549 ms"]
    #[inline(always)]
    pub fn cycle18(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE18)
    }
    #[doc = "1.038 ms"]
    #[inline(always)]
    pub fn cycle34(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE34)
    }
    #[doc = "2.014 ms"]
    #[inline(always)]
    pub fn cycle66(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE66)
    }
    #[doc = "3.967 ms"]
    #[inline(always)]
    pub fn cycle130(self) -> &'a mut W {
        self.variant(STARTUP_A::CYCLE130)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CALIB` reader - Oscillator Calibration"]
pub struct CALIB_R(crate::FieldReader<u8, u8>);
impl CALIB_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CALIB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALIB_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALIB` writer - Oscillator Calibration"]
pub struct CALIB_W<'a> {
    w: &'a mut W,
}
impl<'a> CALIB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 32kHz Output Enable"]
    #[inline(always)]
    pub fn en32k(&self) -> EN32K_R {
        EN32K_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1kHz Output Enable"]
    #[inline(always)]
    pub fn en1k(&self) -> EN1K_R {
        EN1K_R::new(((self.bits >> 3) & 0x01) != 0)
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
    #[doc = "Bits 16:22 - Oscillator Calibration"]
    #[inline(always)]
    pub fn calib(&self) -> CALIB_R {
        CALIB_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - 32kHz Output Enable"]
    #[inline(always)]
    pub fn en32k(&mut self) -> EN32K_W {
        EN32K_W { w: self }
    }
    #[doc = "Bit 3 - 1kHz Output Enable"]
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
    #[doc = "Bits 16:22 - Oscillator Calibration"]
    #[inline(always)]
    pub fn calib(&mut self) -> CALIB_W {
        CALIB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "32kHz Internal Oscillator (OSC32K) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc32k](index.html) module"]
pub struct OSC32K_SPEC;
impl crate::RegisterSpec for OSC32K_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc32k::R](R) reader structure"]
impl crate::Readable for OSC32K_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc32k::W](W) writer structure"]
impl crate::Writable for OSC32K_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC32K to value 0x003f_0080"]
impl crate::Resettable for OSC32K_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x003f_0080
    }
}
