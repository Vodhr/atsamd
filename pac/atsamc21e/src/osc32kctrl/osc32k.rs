#[doc = "Register `OSC32K` reader"]
pub type R = crate::R<Osc32kSpec>;
#[doc = "Register `OSC32K` writer"]
pub type W = crate::W<Osc32kSpec>;
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN32K` reader - 32kHz Output Enable"]
pub type En32kR = crate::BitReader;
#[doc = "Field `EN32K` writer - 32kHz Output Enable"]
pub type En32kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN1K` reader - 1kHz Output Enable"]
pub type En1kR = crate::BitReader;
#[doc = "Field `EN1K` writer - 1kHz Output Enable"]
pub type En1kW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Oscillator Start-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Startupselect {
    #[doc = "0: 0.092 ms"]
    Cycle3 = 0,
    #[doc = "1: 0.122 ms"]
    Cycle4 = 1,
    #[doc = "2: 0.183 ms"]
    Cycle6 = 2,
    #[doc = "3: 0.305 ms"]
    Cycle10 = 3,
    #[doc = "4: 0.549 ms"]
    Cycle18 = 4,
    #[doc = "5: 1.038 ms"]
    Cycle34 = 5,
    #[doc = "6: 2.014 ms"]
    Cycle66 = 6,
    #[doc = "7: 3.967 ms"]
    Cycle130 = 7,
}
impl From<Startupselect> for u8 {
    #[inline(always)]
    fn from(variant: Startupselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Startupselect {
    type Ux = u8;
}
impl crate::IsEnum for Startupselect {}
#[doc = "Field `STARTUP` reader - Oscillator Start-Up Time"]
pub type StartupR = crate::FieldReader<Startupselect>;
impl StartupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Startupselect {
        match self.bits {
            0 => Startupselect::Cycle3,
            1 => Startupselect::Cycle4,
            2 => Startupselect::Cycle6,
            3 => Startupselect::Cycle10,
            4 => Startupselect::Cycle18,
            5 => Startupselect::Cycle34,
            6 => Startupselect::Cycle66,
            7 => Startupselect::Cycle130,
            _ => unreachable!(),
        }
    }
    #[doc = "0.092 ms"]
    #[inline(always)]
    pub fn is_cycle3(&self) -> bool {
        *self == Startupselect::Cycle3
    }
    #[doc = "0.122 ms"]
    #[inline(always)]
    pub fn is_cycle4(&self) -> bool {
        *self == Startupselect::Cycle4
    }
    #[doc = "0.183 ms"]
    #[inline(always)]
    pub fn is_cycle6(&self) -> bool {
        *self == Startupselect::Cycle6
    }
    #[doc = "0.305 ms"]
    #[inline(always)]
    pub fn is_cycle10(&self) -> bool {
        *self == Startupselect::Cycle10
    }
    #[doc = "0.549 ms"]
    #[inline(always)]
    pub fn is_cycle18(&self) -> bool {
        *self == Startupselect::Cycle18
    }
    #[doc = "1.038 ms"]
    #[inline(always)]
    pub fn is_cycle34(&self) -> bool {
        *self == Startupselect::Cycle34
    }
    #[doc = "2.014 ms"]
    #[inline(always)]
    pub fn is_cycle66(&self) -> bool {
        *self == Startupselect::Cycle66
    }
    #[doc = "3.967 ms"]
    #[inline(always)]
    pub fn is_cycle130(&self) -> bool {
        *self == Startupselect::Cycle130
    }
}
#[doc = "Field `STARTUP` writer - Oscillator Start-Up Time"]
pub type StartupW<'a, REG> = crate::FieldWriter<'a, REG, 3, Startupselect, crate::Safe>;
impl<'a, REG> StartupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0.092 ms"]
    #[inline(always)]
    pub fn cycle3(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle3)
    }
    #[doc = "0.122 ms"]
    #[inline(always)]
    pub fn cycle4(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle4)
    }
    #[doc = "0.183 ms"]
    #[inline(always)]
    pub fn cycle6(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle6)
    }
    #[doc = "0.305 ms"]
    #[inline(always)]
    pub fn cycle10(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle10)
    }
    #[doc = "0.549 ms"]
    #[inline(always)]
    pub fn cycle18(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle18)
    }
    #[doc = "1.038 ms"]
    #[inline(always)]
    pub fn cycle34(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle34)
    }
    #[doc = "2.014 ms"]
    #[inline(always)]
    pub fn cycle66(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle66)
    }
    #[doc = "3.967 ms"]
    #[inline(always)]
    pub fn cycle130(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle130)
    }
}
#[doc = "Field `WRTLOCK` reader - Write Lock"]
pub type WrtlockR = crate::BitReader;
#[doc = "Field `WRTLOCK` writer - Write Lock"]
pub type WrtlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALIB` reader - Oscillator Calibration"]
pub type CalibR = crate::FieldReader;
#[doc = "Field `CALIB` writer - Oscillator Calibration"]
pub type CalibW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 32kHz Output Enable"]
    #[inline(always)]
    pub fn en32k(&self) -> En32kR {
        En32kR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1kHz Output Enable"]
    #[inline(always)]
    pub fn en1k(&self) -> En1kR {
        En1kR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> StartupR {
        StartupR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline(always)]
    pub fn wrtlock(&self) -> WrtlockR {
        WrtlockR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Oscillator Calibration"]
    #[inline(always)]
    pub fn calib(&self) -> CalibR {
        CalibR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<Osc32kSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - 32kHz Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en32k(&mut self) -> En32kW<Osc32kSpec> {
        En32kW::new(self, 2)
    }
    #[doc = "Bit 3 - 1kHz Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1k(&mut self) -> En1kW<Osc32kSpec> {
        En1kW::new(self, 3)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<Osc32kSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> OndemandW<Osc32kSpec> {
        OndemandW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Oscillator Start-Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> StartupW<Osc32kSpec> {
        StartupW::new(self, 8)
    }
    #[doc = "Bit 12 - Write Lock"]
    #[inline(always)]
    #[must_use]
    pub fn wrtlock(&mut self) -> WrtlockW<Osc32kSpec> {
        WrtlockW::new(self, 12)
    }
    #[doc = "Bits 16:22 - Oscillator Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn calib(&mut self) -> CalibW<Osc32kSpec> {
        CalibW::new(self, 16)
    }
}
#[doc = "32kHz Internal Oscillator (OSC32K) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`osc32k::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc32k::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osc32kSpec;
impl crate::RegisterSpec for Osc32kSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osc32k::R`](R) reader structure"]
impl crate::Readable for Osc32kSpec {}
#[doc = "`write(|w| ..)` method takes [`osc32k::W`](W) writer structure"]
impl crate::Writable for Osc32kSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OSC32K to value 0x003f_0080"]
impl crate::Resettable for Osc32kSpec {
    const RESET_VALUE: u32 = 0x003f_0080;
}
