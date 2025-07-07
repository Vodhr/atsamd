#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "Window Monitor Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Winmodeselect {
    #[doc = "0: No window mode (default)"]
    Disable = 0,
    #[doc = "1: VALUE greater than WINLT"]
    Above = 1,
    #[doc = "2: VALUE less than WINUT"]
    Below = 2,
    #[doc = "3: VALUE greater than WINLT and VALUE less than WINUT"]
    Inside = 3,
    #[doc = "4: VALUE less than WINLT or VALUE greater than WINUT"]
    Outside = 4,
    #[doc = "5: VALUE greater than WINUT with hysteresis to WINLT"]
    HystAbove = 5,
    #[doc = "6: VALUE less than WINLST with hysteresis to WINUT"]
    HystBelow = 6,
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
            1 => Some(Winmodeselect::Above),
            2 => Some(Winmodeselect::Below),
            3 => Some(Winmodeselect::Inside),
            4 => Some(Winmodeselect::Outside),
            5 => Some(Winmodeselect::HystAbove),
            6 => Some(Winmodeselect::HystBelow),
            _ => None,
        }
    }
    #[doc = "No window mode (default)"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Winmodeselect::Disable
    }
    #[doc = "VALUE greater than WINLT"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == Winmodeselect::Above
    }
    #[doc = "VALUE less than WINUT"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == Winmodeselect::Below
    }
    #[doc = "VALUE greater than WINLT and VALUE less than WINUT"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == Winmodeselect::Inside
    }
    #[doc = "VALUE less than WINLT or VALUE greater than WINUT"]
    #[inline(always)]
    pub fn is_outside(&self) -> bool {
        *self == Winmodeselect::Outside
    }
    #[doc = "VALUE greater than WINUT with hysteresis to WINLT"]
    #[inline(always)]
    pub fn is_hyst_above(&self) -> bool {
        *self == Winmodeselect::HystAbove
    }
    #[doc = "VALUE less than WINLST with hysteresis to WINUT"]
    #[inline(always)]
    pub fn is_hyst_below(&self) -> bool {
        *self == Winmodeselect::HystBelow
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
    #[doc = "VALUE greater than WINLT"]
    #[inline(always)]
    pub fn above(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Above)
    }
    #[doc = "VALUE less than WINUT"]
    #[inline(always)]
    pub fn below(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Below)
    }
    #[doc = "VALUE greater than WINLT and VALUE less than WINUT"]
    #[inline(always)]
    pub fn inside(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Inside)
    }
    #[doc = "VALUE less than WINLT or VALUE greater than WINUT"]
    #[inline(always)]
    pub fn outside(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::Outside)
    }
    #[doc = "VALUE greater than WINUT with hysteresis to WINLT"]
    #[inline(always)]
    pub fn hyst_above(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::HystAbove)
    }
    #[doc = "VALUE less than WINLST with hysteresis to WINUT"]
    #[inline(always)]
    pub fn hyst_below(self) -> &'a mut crate::W<REG> {
        self.variant(Winmodeselect::HystBelow)
    }
}
#[doc = "Field `FREERUN` reader - Free Running Measurement"]
pub type FreerunR = crate::BitReader;
#[doc = "Field `FREERUN` writer - Free Running Measurement"]
pub type FreerunW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline(always)]
    pub fn winmode(&self) -> WinmodeR {
        WinmodeR::new(self.bits & 7)
    }
    #[doc = "Bit 4 - Free Running Measurement"]
    #[inline(always)]
    pub fn freerun(&self) -> FreerunR {
        FreerunR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Window Monitor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn winmode(&mut self) -> WinmodeW<CtrlcSpec> {
        WinmodeW::new(self, 0)
    }
    #[doc = "Bit 4 - Free Running Measurement"]
    #[inline(always)]
    #[must_use]
    pub fn freerun(&mut self) -> FreerunW<CtrlcSpec> {
        FreerunW::new(self, 4)
    }
}
#[doc = "Control C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlcSpec;
impl crate::RegisterSpec for CtrlcSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CtrlcSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CtrlcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CtrlcSpec {
    const RESET_VALUE: u8 = 0;
}
