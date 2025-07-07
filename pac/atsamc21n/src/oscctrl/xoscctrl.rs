#[doc = "Register `XOSCCTRL` reader"]
pub type R = crate::R<XoscctrlSpec>;
#[doc = "Register `XOSCCTRL` writer"]
pub type W = crate::W<XoscctrlSpec>;
#[doc = "Field `ENABLE` reader - Oscillator Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Oscillator Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XTALEN` reader - Crystal Oscillator Enable"]
pub type XtalenR = crate::BitReader;
#[doc = "Field `XTALEN` writer - Crystal Oscillator Enable"]
pub type XtalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFDEN` reader - Xosc Clock Failure Detector Enable"]
pub type CfdenR = crate::BitReader;
#[doc = "Field `CFDEN` writer - Xosc Clock Failure Detector Enable"]
pub type CfdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWBEN` reader - Xosc Clock Switch Enable"]
pub type SwbenR = crate::BitReader;
#[doc = "Field `SWBEN` writer - Xosc Clock Switch Enable"]
pub type SwbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run in Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run in Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - On Demand Control"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Control"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Oscillator Gain\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gainselect {
    #[doc = "0: 2 MHz"]
    Gain2 = 0,
    #[doc = "1: 4 MHz"]
    Gain4 = 1,
    #[doc = "2: 8 MHz"]
    Gain8 = 2,
    #[doc = "3: 16 MHz"]
    Gain16 = 3,
    #[doc = "4: 30 MHz"]
    Gain30 = 4,
}
impl From<Gainselect> for u8 {
    #[inline(always)]
    fn from(variant: Gainselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gainselect {
    type Ux = u8;
}
impl crate::IsEnum for Gainselect {}
#[doc = "Field `GAIN` reader - Oscillator Gain"]
pub type GainR = crate::FieldReader<Gainselect>;
impl GainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Gainselect> {
        match self.bits {
            0 => Some(Gainselect::Gain2),
            1 => Some(Gainselect::Gain4),
            2 => Some(Gainselect::Gain8),
            3 => Some(Gainselect::Gain16),
            4 => Some(Gainselect::Gain30),
            _ => None,
        }
    }
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn is_gain2(&self) -> bool {
        *self == Gainselect::Gain2
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn is_gain4(&self) -> bool {
        *self == Gainselect::Gain4
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn is_gain8(&self) -> bool {
        *self == Gainselect::Gain8
    }
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn is_gain16(&self) -> bool {
        *self == Gainselect::Gain16
    }
    #[doc = "30 MHz"]
    #[inline(always)]
    pub fn is_gain30(&self) -> bool {
        *self == Gainselect::Gain30
    }
}
#[doc = "Field `GAIN` writer - Oscillator Gain"]
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 3, Gainselect>;
impl<'a, REG> GainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2 MHz"]
    #[inline(always)]
    pub fn gain2(self) -> &'a mut crate::W<REG> {
        self.variant(Gainselect::Gain2)
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn gain4(self) -> &'a mut crate::W<REG> {
        self.variant(Gainselect::Gain4)
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn gain8(self) -> &'a mut crate::W<REG> {
        self.variant(Gainselect::Gain8)
    }
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn gain16(self) -> &'a mut crate::W<REG> {
        self.variant(Gainselect::Gain16)
    }
    #[doc = "30 MHz"]
    #[inline(always)]
    pub fn gain30(self) -> &'a mut crate::W<REG> {
        self.variant(Gainselect::Gain30)
    }
}
#[doc = "Field `AMPGC` reader - Automatic Amplitude Gain Control"]
pub type AmpgcR = crate::BitReader;
#[doc = "Field `AMPGC` writer - Automatic Amplitude Gain Control"]
pub type AmpgcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Start-Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Startupselect {
    #[doc = "0: 31 us"]
    Cycle1 = 0,
    #[doc = "1: 61 us"]
    Cycle2 = 1,
    #[doc = "2: 122 us"]
    Cycle4 = 2,
    #[doc = "3: 244 us"]
    Cycle8 = 3,
    #[doc = "4: 488 us"]
    Cycle16 = 4,
    #[doc = "5: 977 us"]
    Cycle32 = 5,
    #[doc = "6: 1953 us"]
    Cycle64 = 6,
    #[doc = "7: 3906 us"]
    Cycle128 = 7,
    #[doc = "8: 7813 us"]
    Cycle256 = 8,
    #[doc = "9: 15625 us"]
    Cycle512 = 9,
    #[doc = "10: 31250 us"]
    Cycle1024 = 10,
    #[doc = "11: 62500 us"]
    Cycle2048 = 11,
    #[doc = "12: 125000 us"]
    Cycle4096 = 12,
    #[doc = "13: 250000 us"]
    Cycle8192 = 13,
    #[doc = "14: 500000 us"]
    Cycle16384 = 14,
    #[doc = "15: 1000000 us"]
    Cycle32768 = 15,
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
#[doc = "Field `STARTUP` reader - Start-Up Time"]
pub type StartupR = crate::FieldReader<Startupselect>;
impl StartupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Startupselect {
        match self.bits {
            0 => Startupselect::Cycle1,
            1 => Startupselect::Cycle2,
            2 => Startupselect::Cycle4,
            3 => Startupselect::Cycle8,
            4 => Startupselect::Cycle16,
            5 => Startupselect::Cycle32,
            6 => Startupselect::Cycle64,
            7 => Startupselect::Cycle128,
            8 => Startupselect::Cycle256,
            9 => Startupselect::Cycle512,
            10 => Startupselect::Cycle1024,
            11 => Startupselect::Cycle2048,
            12 => Startupselect::Cycle4096,
            13 => Startupselect::Cycle8192,
            14 => Startupselect::Cycle16384,
            15 => Startupselect::Cycle32768,
            _ => unreachable!(),
        }
    }
    #[doc = "31 us"]
    #[inline(always)]
    pub fn is_cycle1(&self) -> bool {
        *self == Startupselect::Cycle1
    }
    #[doc = "61 us"]
    #[inline(always)]
    pub fn is_cycle2(&self) -> bool {
        *self == Startupselect::Cycle2
    }
    #[doc = "122 us"]
    #[inline(always)]
    pub fn is_cycle4(&self) -> bool {
        *self == Startupselect::Cycle4
    }
    #[doc = "244 us"]
    #[inline(always)]
    pub fn is_cycle8(&self) -> bool {
        *self == Startupselect::Cycle8
    }
    #[doc = "488 us"]
    #[inline(always)]
    pub fn is_cycle16(&self) -> bool {
        *self == Startupselect::Cycle16
    }
    #[doc = "977 us"]
    #[inline(always)]
    pub fn is_cycle32(&self) -> bool {
        *self == Startupselect::Cycle32
    }
    #[doc = "1953 us"]
    #[inline(always)]
    pub fn is_cycle64(&self) -> bool {
        *self == Startupselect::Cycle64
    }
    #[doc = "3906 us"]
    #[inline(always)]
    pub fn is_cycle128(&self) -> bool {
        *self == Startupselect::Cycle128
    }
    #[doc = "7813 us"]
    #[inline(always)]
    pub fn is_cycle256(&self) -> bool {
        *self == Startupselect::Cycle256
    }
    #[doc = "15625 us"]
    #[inline(always)]
    pub fn is_cycle512(&self) -> bool {
        *self == Startupselect::Cycle512
    }
    #[doc = "31250 us"]
    #[inline(always)]
    pub fn is_cycle1024(&self) -> bool {
        *self == Startupselect::Cycle1024
    }
    #[doc = "62500 us"]
    #[inline(always)]
    pub fn is_cycle2048(&self) -> bool {
        *self == Startupselect::Cycle2048
    }
    #[doc = "125000 us"]
    #[inline(always)]
    pub fn is_cycle4096(&self) -> bool {
        *self == Startupselect::Cycle4096
    }
    #[doc = "250000 us"]
    #[inline(always)]
    pub fn is_cycle8192(&self) -> bool {
        *self == Startupselect::Cycle8192
    }
    #[doc = "500000 us"]
    #[inline(always)]
    pub fn is_cycle16384(&self) -> bool {
        *self == Startupselect::Cycle16384
    }
    #[doc = "1000000 us"]
    #[inline(always)]
    pub fn is_cycle32768(&self) -> bool {
        *self == Startupselect::Cycle32768
    }
}
#[doc = "Field `STARTUP` writer - Start-Up Time"]
pub type StartupW<'a, REG> = crate::FieldWriter<'a, REG, 4, Startupselect, crate::Safe>;
impl<'a, REG> StartupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "31 us"]
    #[inline(always)]
    pub fn cycle1(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle1)
    }
    #[doc = "61 us"]
    #[inline(always)]
    pub fn cycle2(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle2)
    }
    #[doc = "122 us"]
    #[inline(always)]
    pub fn cycle4(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle4)
    }
    #[doc = "244 us"]
    #[inline(always)]
    pub fn cycle8(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle8)
    }
    #[doc = "488 us"]
    #[inline(always)]
    pub fn cycle16(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle16)
    }
    #[doc = "977 us"]
    #[inline(always)]
    pub fn cycle32(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle32)
    }
    #[doc = "1953 us"]
    #[inline(always)]
    pub fn cycle64(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle64)
    }
    #[doc = "3906 us"]
    #[inline(always)]
    pub fn cycle128(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle128)
    }
    #[doc = "7813 us"]
    #[inline(always)]
    pub fn cycle256(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle256)
    }
    #[doc = "15625 us"]
    #[inline(always)]
    pub fn cycle512(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle512)
    }
    #[doc = "31250 us"]
    #[inline(always)]
    pub fn cycle1024(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle1024)
    }
    #[doc = "62500 us"]
    #[inline(always)]
    pub fn cycle2048(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle2048)
    }
    #[doc = "125000 us"]
    #[inline(always)]
    pub fn cycle4096(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle4096)
    }
    #[doc = "250000 us"]
    #[inline(always)]
    pub fn cycle8192(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle8192)
    }
    #[doc = "500000 us"]
    #[inline(always)]
    pub fn cycle16384(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle16384)
    }
    #[doc = "1000000 us"]
    #[inline(always)]
    pub fn cycle32768(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle32768)
    }
}
impl R {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn xtalen(&self) -> XtalenR {
        XtalenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Xosc Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CfdenR {
        CfdenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Xosc Clock Switch Enable"]
    #[inline(always)]
    pub fn swben(&self) -> SwbenR {
        SwbenR::new(((self.bits >> 4) & 1) != 0)
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
    #[doc = "Bits 8:10 - Oscillator Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
    #[inline(always)]
    pub fn ampgc(&self) -> AmpgcR {
        AmpgcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Start-Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> StartupR {
        StartupR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<XoscctrlSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Crystal Oscillator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xtalen(&mut self) -> XtalenW<XoscctrlSpec> {
        XtalenW::new(self, 2)
    }
    #[doc = "Bit 3 - Xosc Clock Failure Detector Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cfden(&mut self) -> CfdenW<XoscctrlSpec> {
        CfdenW::new(self, 3)
    }
    #[doc = "Bit 4 - Xosc Clock Switch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swben(&mut self) -> SwbenW<XoscctrlSpec> {
        SwbenW::new(self, 4)
    }
    #[doc = "Bit 6 - Run in Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<XoscctrlSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - On Demand Control"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> OndemandW<XoscctrlSpec> {
        OndemandW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Oscillator Gain"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GainW<XoscctrlSpec> {
        GainW::new(self, 8)
    }
    #[doc = "Bit 11 - Automatic Amplitude Gain Control"]
    #[inline(always)]
    #[must_use]
    pub fn ampgc(&mut self) -> AmpgcW<XoscctrlSpec> {
        AmpgcW::new(self, 11)
    }
    #[doc = "Bits 12:15 - Start-Up Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> StartupW<XoscctrlSpec> {
        StartupW::new(self, 12)
    }
}
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xoscctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xoscctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XoscctrlSpec;
impl crate::RegisterSpec for XoscctrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`xoscctrl::R`](R) reader structure"]
impl crate::Readable for XoscctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`xoscctrl::W`](W) writer structure"]
impl crate::Writable for XoscctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets XOSCCTRL to value 0x80"]
impl crate::Resettable for XoscctrlSpec {
    const RESET_VALUE: u16 = 0x80;
}
