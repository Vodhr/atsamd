#[doc = "Register `INTFLAGA` reader"]
pub struct R(crate::R<INTFLAGA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFLAGA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFLAGA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFLAGA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFLAGA` writer"]
pub struct W(crate::W<INTFLAGA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFLAGA_SPEC>;
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
impl From<crate::W<INTFLAGA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFLAGA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAC_` reader - PAC"]
pub struct PAC__R(crate::FieldReader<bool, bool>);
impl PAC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PAC_` writer - PAC"]
pub struct PAC__W<'a> {
    w: &'a mut W,
}
impl<'a> PAC__W<'a> {
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
#[doc = "Field `PM_` reader - PM"]
pub struct PM__R(crate::FieldReader<bool, bool>);
impl PM__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PM__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PM__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PM_` writer - PM"]
pub struct PM__W<'a> {
    w: &'a mut W,
}
impl<'a> PM__W<'a> {
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
#[doc = "Field `MCLK_` reader - MCLK"]
pub struct MCLK__R(crate::FieldReader<bool, bool>);
impl MCLK__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MCLK__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCLK__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCLK_` writer - MCLK"]
pub struct MCLK__W<'a> {
    w: &'a mut W,
}
impl<'a> MCLK__W<'a> {
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
#[doc = "Field `RSTC_` reader - RSTC"]
pub struct RSTC__R(crate::FieldReader<bool, bool>);
impl RSTC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RSTC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTC_` writer - RSTC"]
pub struct RSTC__W<'a> {
    w: &'a mut W,
}
impl<'a> RSTC__W<'a> {
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
#[doc = "Field `OSCCTRL_` reader - OSCCTRL"]
pub struct OSCCTRL__R(crate::FieldReader<bool, bool>);
impl OSCCTRL__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSCCTRL__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSCCTRL__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSCCTRL_` writer - OSCCTRL"]
pub struct OSCCTRL__W<'a> {
    w: &'a mut W,
}
impl<'a> OSCCTRL__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `OSC32KCTRL_` reader - OSC32KCTRL"]
pub struct OSC32KCTRL__R(crate::FieldReader<bool, bool>);
impl OSC32KCTRL__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC32KCTRL__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC32KCTRL__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OSC32KCTRL_` writer - OSC32KCTRL"]
pub struct OSC32KCTRL__W<'a> {
    w: &'a mut W,
}
impl<'a> OSC32KCTRL__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `SUPC_` reader - SUPC"]
pub struct SUPC__R(crate::FieldReader<bool, bool>);
impl SUPC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUPC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUPC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUPC_` writer - SUPC"]
pub struct SUPC__W<'a> {
    w: &'a mut W,
}
impl<'a> SUPC__W<'a> {
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
#[doc = "Field `GCLK_` reader - GCLK"]
pub struct GCLK__R(crate::FieldReader<bool, bool>);
impl GCLK__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GCLK__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCLK__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCLK_` writer - GCLK"]
pub struct GCLK__W<'a> {
    w: &'a mut W,
}
impl<'a> GCLK__W<'a> {
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
#[doc = "Field `WDT_` reader - WDT"]
pub struct WDT__R(crate::FieldReader<bool, bool>);
impl WDT__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_` writer - WDT"]
pub struct WDT__W<'a> {
    w: &'a mut W,
}
impl<'a> WDT__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `RTC_` reader - RTC"]
pub struct RTC__R(crate::FieldReader<bool, bool>);
impl RTC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTC_` writer - RTC"]
pub struct RTC__W<'a> {
    w: &'a mut W,
}
impl<'a> RTC__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `EIC_` reader - EIC"]
pub struct EIC__R(crate::FieldReader<bool, bool>);
impl EIC__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EIC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EIC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EIC_` writer - EIC"]
pub struct EIC__W<'a> {
    w: &'a mut W,
}
impl<'a> EIC__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `FREQM_` reader - FREQM"]
pub struct FREQM__R(crate::FieldReader<bool, bool>);
impl FREQM__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FREQM__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREQM__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREQM_` writer - FREQM"]
pub struct FREQM__W<'a> {
    w: &'a mut W,
}
impl<'a> FREQM__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `TSENS_` reader - TSENS"]
pub struct TSENS__R(crate::FieldReader<bool, bool>);
impl TSENS__R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TSENS__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSENS__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSENS_` writer - TSENS"]
pub struct TSENS__W<'a> {
    w: &'a mut W,
}
impl<'a> TSENS__W<'a> {
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
impl R {
    #[doc = "Bit 0 - PAC"]
    #[inline(always)]
    pub fn pac_(&self) -> PAC__R {
        PAC__R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PM"]
    #[inline(always)]
    pub fn pm_(&self) -> PM__R {
        PM__R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - MCLK"]
    #[inline(always)]
    pub fn mclk_(&self) -> MCLK__R {
        MCLK__R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RSTC"]
    #[inline(always)]
    pub fn rstc_(&self) -> RSTC__R {
        RSTC__R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - OSCCTRL"]
    #[inline(always)]
    pub fn oscctrl_(&self) -> OSCCTRL__R {
        OSCCTRL__R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - OSC32KCTRL"]
    #[inline(always)]
    pub fn osc32kctrl_(&self) -> OSC32KCTRL__R {
        OSC32KCTRL__R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SUPC"]
    #[inline(always)]
    pub fn supc_(&self) -> SUPC__R {
        SUPC__R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GCLK"]
    #[inline(always)]
    pub fn gclk_(&self) -> GCLK__R {
        GCLK__R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - WDT"]
    #[inline(always)]
    pub fn wdt_(&self) -> WDT__R {
        WDT__R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RTC"]
    #[inline(always)]
    pub fn rtc_(&self) -> RTC__R {
        RTC__R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EIC"]
    #[inline(always)]
    pub fn eic_(&self) -> EIC__R {
        EIC__R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - FREQM"]
    #[inline(always)]
    pub fn freqm_(&self) -> FREQM__R {
        FREQM__R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TSENS"]
    #[inline(always)]
    pub fn tsens_(&self) -> TSENS__R {
        TSENS__R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PAC"]
    #[inline(always)]
    pub fn pac_(&mut self) -> PAC__W {
        PAC__W { w: self }
    }
    #[doc = "Bit 1 - PM"]
    #[inline(always)]
    pub fn pm_(&mut self) -> PM__W {
        PM__W { w: self }
    }
    #[doc = "Bit 2 - MCLK"]
    #[inline(always)]
    pub fn mclk_(&mut self) -> MCLK__W {
        MCLK__W { w: self }
    }
    #[doc = "Bit 3 - RSTC"]
    #[inline(always)]
    pub fn rstc_(&mut self) -> RSTC__W {
        RSTC__W { w: self }
    }
    #[doc = "Bit 4 - OSCCTRL"]
    #[inline(always)]
    pub fn oscctrl_(&mut self) -> OSCCTRL__W {
        OSCCTRL__W { w: self }
    }
    #[doc = "Bit 5 - OSC32KCTRL"]
    #[inline(always)]
    pub fn osc32kctrl_(&mut self) -> OSC32KCTRL__W {
        OSC32KCTRL__W { w: self }
    }
    #[doc = "Bit 6 - SUPC"]
    #[inline(always)]
    pub fn supc_(&mut self) -> SUPC__W {
        SUPC__W { w: self }
    }
    #[doc = "Bit 7 - GCLK"]
    #[inline(always)]
    pub fn gclk_(&mut self) -> GCLK__W {
        GCLK__W { w: self }
    }
    #[doc = "Bit 8 - WDT"]
    #[inline(always)]
    pub fn wdt_(&mut self) -> WDT__W {
        WDT__W { w: self }
    }
    #[doc = "Bit 9 - RTC"]
    #[inline(always)]
    pub fn rtc_(&mut self) -> RTC__W {
        RTC__W { w: self }
    }
    #[doc = "Bit 10 - EIC"]
    #[inline(always)]
    pub fn eic_(&mut self) -> EIC__W {
        EIC__W { w: self }
    }
    #[doc = "Bit 11 - FREQM"]
    #[inline(always)]
    pub fn freqm_(&mut self) -> FREQM__W {
        FREQM__W { w: self }
    }
    #[doc = "Bit 12 - TSENS"]
    #[inline(always)]
    pub fn tsens_(&mut self) -> TSENS__W {
        TSENS__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral interrupt flag status - Bridge A\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intflaga](index.html) module"]
pub struct INTFLAGA_SPEC;
impl crate::RegisterSpec for INTFLAGA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intflaga::R](R) reader structure"]
impl crate::Readable for INTFLAGA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intflaga::W](W) writer structure"]
impl crate::Writable for INTFLAGA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFLAGA to value 0"]
impl crate::Resettable for INTFLAGA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
