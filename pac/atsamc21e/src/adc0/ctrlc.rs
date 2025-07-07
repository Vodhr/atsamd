#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "Field `DIFFMODE` reader - Differential Mode"]
pub type DiffmodeR = crate::BitReader;
#[doc = "Field `DIFFMODE` writer - Differential Mode"]
pub type DiffmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEFTADJ` reader - Left-Adjusted Result"]
pub type LeftadjR = crate::BitReader;
#[doc = "Field `LEFTADJ` writer - Left-Adjusted Result"]
pub type LeftadjW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREERUN` reader - Free Running Mode"]
pub type FreerunR = crate::BitReader;
#[doc = "Field `FREERUN` writer - Free Running Mode"]
pub type FreerunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORREN` reader - Digital Correction Logic Enable"]
pub type CorrenR = crate::BitReader;
#[doc = "Field `CORREN` writer - Digital Correction Logic Enable"]
pub type CorrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Conversion Result Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Resselselect {
    #[doc = "0: 12-bit"]
    _12bit = 0,
    #[doc = "1: 16-bit averaging mode"]
    _16bit = 1,
    #[doc = "2: 10-bit"]
    _10bit = 2,
    #[doc = "3: 8-bit"]
    _8bit = 3,
}
impl From<Resselselect> for u8 {
    #[inline(always)]
    fn from(variant: Resselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Resselselect {
    type Ux = u8;
}
impl crate::IsEnum for Resselselect {}
#[doc = "Field `RESSEL` reader - Conversion Result Resolution"]
pub type ResselR = crate::FieldReader<Resselselect>;
impl ResselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Resselselect {
        match self.bits {
            0 => Resselselect::_12bit,
            1 => Resselselect::_16bit,
            2 => Resselselect::_10bit,
            3 => Resselselect::_8bit,
            _ => unreachable!(),
        }
    }
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn is_12bit(&self) -> bool {
        *self == Resselselect::_12bit
    }
    #[doc = "16-bit averaging mode"]
    #[inline(always)]
    pub fn is_16bit(&self) -> bool {
        *self == Resselselect::_16bit
    }
    #[doc = "10-bit"]
    #[inline(always)]
    pub fn is_10bit(&self) -> bool {
        *self == Resselselect::_10bit
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn is_8bit(&self) -> bool {
        *self == Resselselect::_8bit
    }
}
#[doc = "Field `RESSEL` writer - Conversion Result Resolution"]
pub type ResselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Resselselect, crate::Safe>;
impl<'a, REG> ResselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn _12bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resselselect::_12bit)
    }
    #[doc = "16-bit averaging mode"]
    #[inline(always)]
    pub fn _16bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resselselect::_16bit)
    }
    #[doc = "10-bit"]
    #[inline(always)]
    pub fn _10bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resselselect::_10bit)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn _8bit(self) -> &'a mut crate::W<REG> {
        self.variant(Resselselect::_8bit)
    }
}
#[doc = "Field `R2R` reader - Rail-to-Rail mode enable"]
pub type R2rR = crate::BitReader;
#[doc = "Field `R2R` writer - Rail-to-Rail mode enable"]
pub type R2rW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Window Monitor Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Winmodeselect {
    #[doc = "0: No window mode (default)"]
    Disable = 0,
    #[doc = "1: RESULT > WINLT"]
    Mode1 = 1,
    #[doc = "2: RESULT &lt; WINUT"]
    Mode2 = 2,
    #[doc = "3: WINLT &lt; RESULT &lt; WINUT"]
    Mode3 = 3,
    #[doc = "4: !(WINLT &lt; RESULT &lt; WINUT)"]
    Mode4 = 4,
}
impl From<Winmodeselect> for u8 {
    #[inline(always)]
    fn from(variant: Winmodeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Winmodeselect {
    type Ux = u8;
}
impl crate::IsEnum for Winmodeselect {}
#[doc = "Field `WINMODE` reader - Window Monitor Mode"]
pub type WinmodeR = crate::FieldReader<Winmodeselect>;
impl WinmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Winmodeselect> {
        match self.bits {
            0 => Some(Winmodeselect::Disable),
            1 => Some(Winmodeselect::Mode1),
            2 => Some(Winmodeselect::Mode2),
            3 => Some(Winmodeselect::Mode3),
            4 => Some(Winmodeselect::Mode4),
            _ => None,
        }
    }
    #[doc = "No window mode (default)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Winmodeselect::Disable
    }
    #[doc = "RESULT > WINLT"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == Winmodeselect::Mode1
    }
    #[doc = "RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn is_mode2(&self) -> bool {
        *self == Winmodeselect::Mode2
    }
    #[doc = "WINLT &lt; RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn is_mode3(&self) -> bool {
        *self == Winmodeselect::Mode3
    }
    #[doc = "!(WINLT &lt; RESULT &lt; WINUT)"]
    #[inline(always)]
    pub fn is_mode4(&self) -> bool {
        *self == Winmodeselect::Mode4
    }
}
#[doc = "Field `WINMODE` writer - Window Monitor Mode"]
pub type WinmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Winmodeselect>;
impl<'a, REG> WinmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No window mode (default)"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Disable)
    }
    #[doc = "RESULT > WINLT"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Mode1)
    }
    #[doc = "RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn mode2(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Mode2)
    }
    #[doc = "WINLT &lt; RESULT &lt; WINUT"]
    #[inline(always)]
    pub fn mode3(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Mode3)
    }
    #[doc = "!(WINLT &lt; RESULT &lt; WINUT)"]
    #[inline(always)]
    pub fn mode4(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Mode4)
    }
}
#[doc = "Dual Mode Trigger Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dualselselect {
    #[doc = "0: Start event or software trigger will start a conversion on both ADCs"]
    Both = 0,
    #[doc = "1: START event or software trigger will alternately start a conversion on ADC0 and ADC1"]
    Interleave = 1,
}
impl From<Dualselselect> for u8 {
    #[inline(always)]
    fn from(variant: Dualselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dualselselect {
    type Ux = u8;
}
impl crate::IsEnum for Dualselselect {}
#[doc = "Field `DUALSEL` reader - Dual Mode Trigger Selection"]
pub type DualselR = crate::FieldReader<Dualselselect>;
impl DualselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dualselselect> {
        match self.bits {
            0 => Some(Dualselselect::Both),
            1 => Some(Dualselselect::Interleave),
            _ => None,
        }
    }
    #[doc = "Start event or software trigger will start a conversion on both ADCs"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Dualselselect::Both
    }
    #[doc = "START event or software trigger will alternately start a conversion on ADC0 and ADC1"]
    #[inline(always)]
    pub fn is_interleave(&self) -> bool {
        *self == Dualselselect::Interleave
    }
}
#[doc = "Field `DUALSEL` writer - Dual Mode Trigger Selection"]
pub type DualselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dualselselect>;
impl<'a, REG> DualselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Start event or software trigger will start a conversion on both ADCs"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Dualselselect::Both)
    }
    #[doc = "START event or software trigger will alternately start a conversion on ADC0 and ADC1"]
    #[inline(always)]
    pub fn interleave(self) -> &'a mut crate::W<REG> {
        self.variant(Dualselselect::Interleave)
    }
}
impl R {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    pub fn diffmode(&self) -> DiffmodeR {
        DiffmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Left-Adjusted Result"]
    #[inline(always)]
    pub fn leftadj(&self) -> LeftadjR {
        LeftadjR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Free Running Mode"]
    #[inline(always)]
    pub fn freerun(&self) -> FreerunR {
        FreerunR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Digital Correction Logic Enable"]
    #[inline(always)]
    pub fn corren(&self) -> CorrenR {
        CorrenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Conversion Result Resolution"]
    #[inline(always)]
    pub fn ressel(&self) -> ResselR {
        ResselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Rail-to-Rail mode enable"]
    #[inline(always)]
    pub fn r2r(&self) -> R2rR {
        R2rR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WinmodeR {
        WinmodeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Dual Mode Trigger Selection"]
    #[inline(always)]
    pub fn dualsel(&self) -> DualselR {
        DualselR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Differential Mode"]
    #[inline(always)]
    #[must_use]
    pub fn diffmode(&mut self) -> DiffmodeW<CtrlcSpec> {
        DiffmodeW::new(self, 0)
    }
    #[doc = "Bit 1 - Left-Adjusted Result"]
    #[inline(always)]
    #[must_use]
    pub fn leftadj(&mut self) -> LeftadjW<CtrlcSpec> {
        LeftadjW::new(self, 1)
    }
    #[doc = "Bit 2 - Free Running Mode"]
    #[inline(always)]
    #[must_use]
    pub fn freerun(&mut self) -> FreerunW<CtrlcSpec> {
        FreerunW::new(self, 2)
    }
    #[doc = "Bit 3 - Digital Correction Logic Enable"]
    #[inline(always)]
    #[must_use]
    pub fn corren(&mut self) -> CorrenW<CtrlcSpec> {
        CorrenW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Conversion Result Resolution"]
    #[inline(always)]
    #[must_use]
    pub fn ressel(&mut self) -> ResselW<CtrlcSpec> {
        ResselW::new(self, 4)
    }
    #[doc = "Bit 7 - Rail-to-Rail mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn r2r(&mut self) -> R2rW<CtrlcSpec> {
        R2rW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Window Monitor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn winmode(&mut self) -> WinmodeW<CtrlcSpec> {
        WinmodeW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Dual Mode Trigger Selection"]
    #[inline(always)]
    #[must_use]
    pub fn dualsel(&mut self) -> DualselW<CtrlcSpec> {
        DualselW::new(self, 12)
    }
}
#[doc = "Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlcSpec;
impl crate::RegisterSpec for CtrlcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CtrlcSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CtrlcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CtrlcSpec {
    const RESET_VALUE: u16 = 0;
}
