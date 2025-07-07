#[doc = "Register `BODCORE` reader"]
pub type R = crate::R<BodcoreSpec>;
#[doc = "Register `BODCORE` writer"]
pub type W = crate::W<BodcoreSpec>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - Hysteresis Enable"]
pub type HystR = crate::BitReader;
#[doc = "Field `HYST` writer - Hysteresis Enable"]
pub type HystW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Action when Threshold Crossed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Actionselect {
    #[doc = "0: No action"]
    None = 0,
    #[doc = "1: The BODCORE generates a reset"]
    Reset = 1,
    #[doc = "2: The BODCORE generates an interrupt"]
    Int = 2,
}
impl From<Actionselect> for u8 {
    #[inline(always)]
    fn from(variant: Actionselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Actionselect {
    type Ux = u8;
}
impl crate::IsEnum for Actionselect {}
#[doc = "Field `ACTION` reader - Action when Threshold Crossed"]
pub type ActionR = crate::FieldReader<Actionselect>;
impl ActionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Actionselect> {
        match self.bits {
            0 => Some(Actionselect::None),
            1 => Some(Actionselect::Reset),
            2 => Some(Actionselect::Int),
            _ => None,
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Actionselect::None
    }
    #[doc = "The BODCORE generates a reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Actionselect::Reset
    }
    #[doc = "The BODCORE generates an interrupt"]
    #[inline(always)]
    pub fn is_int(&self) -> bool {
        *self == Actionselect::Int
    }
}
#[doc = "Field `ACTION` writer - Action when Threshold Crossed"]
pub type ActionW<'a, REG> = crate::FieldWriter<'a, REG, 2, Actionselect>;
impl<'a, REG> ActionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Actionselect::None)
    }
    #[doc = "The BODCORE generates a reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Actionselect::Reset)
    }
    #[doc = "The BODCORE generates an interrupt"]
    #[inline(always)]
    pub fn int(self) -> &'a mut crate::W<REG> {
        self.variant(Actionselect::Int)
    }
}
#[doc = "Field `STDBYCFG` reader - Configuration in Standby mode"]
pub type StdbycfgR = crate::BitReader;
#[doc = "Field `STDBYCFG` writer - Configuration in Standby mode"]
pub type StdbycfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTCFG` reader - Configuration in Active mode"]
pub type ActcfgR = crate::BitReader;
#[doc = "Field `ACTCFG` writer - Configuration in Active mode"]
pub type ActcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Prescaler Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pselselect {
    #[doc = "0: Divide clock by 2"]
    Div2 = 0,
    #[doc = "1: Divide clock by 4"]
    Div4 = 1,
    #[doc = "2: Divide clock by 8"]
    Div8 = 2,
    #[doc = "3: Divide clock by 16"]
    Div16 = 3,
    #[doc = "4: Divide clock by 32"]
    Div32 = 4,
    #[doc = "5: Divide clock by 64"]
    Div64 = 5,
    #[doc = "6: Divide clock by 128"]
    Div128 = 6,
    #[doc = "7: Divide clock by 256"]
    Div256 = 7,
    #[doc = "8: Divide clock by 512"]
    Div512 = 8,
    #[doc = "9: Divide clock by 1024"]
    Div1024 = 9,
    #[doc = "10: Divide clock by 2048"]
    Div2048 = 10,
    #[doc = "11: Divide clock by 4096"]
    Div4096 = 11,
    #[doc = "12: Divide clock by 8192"]
    Div8192 = 12,
    #[doc = "13: Divide clock by 16384"]
    Div16384 = 13,
    #[doc = "14: Divide clock by 32768"]
    Div32768 = 14,
    #[doc = "15: Divide clock by 65536"]
    Div65536 = 15,
}
impl From<Pselselect> for u8 {
    #[inline(always)]
    fn from(variant: Pselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pselselect {
    type Ux = u8;
}
impl crate::IsEnum for Pselselect {}
#[doc = "Field `PSEL` reader - Prescaler Select"]
pub type PselR = crate::FieldReader<Pselselect>;
impl PselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pselselect {
        match self.bits {
            0 => Pselselect::Div2,
            1 => Pselselect::Div4,
            2 => Pselselect::Div8,
            3 => Pselselect::Div16,
            4 => Pselselect::Div32,
            5 => Pselselect::Div64,
            6 => Pselselect::Div128,
            7 => Pselselect::Div256,
            8 => Pselselect::Div512,
            9 => Pselselect::Div1024,
            10 => Pselselect::Div2048,
            11 => Pselselect::Div4096,
            12 => Pselselect::Div8192,
            13 => Pselselect::Div16384,
            14 => Pselselect::Div32768,
            15 => Pselselect::Div65536,
            _ => unreachable!(),
        }
    }
    #[doc = "Divide clock by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Pselselect::Div2
    }
    #[doc = "Divide clock by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Pselselect::Div4
    }
    #[doc = "Divide clock by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Pselselect::Div8
    }
    #[doc = "Divide clock by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Pselselect::Div16
    }
    #[doc = "Divide clock by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Pselselect::Div32
    }
    #[doc = "Divide clock by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Pselselect::Div64
    }
    #[doc = "Divide clock by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Pselselect::Div128
    }
    #[doc = "Divide clock by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Pselselect::Div256
    }
    #[doc = "Divide clock by 512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Pselselect::Div512
    }
    #[doc = "Divide clock by 1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == Pselselect::Div1024
    }
    #[doc = "Divide clock by 2048"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == Pselselect::Div2048
    }
    #[doc = "Divide clock by 4096"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == Pselselect::Div4096
    }
    #[doc = "Divide clock by 8192"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == Pselselect::Div8192
    }
    #[doc = "Divide clock by 16384"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == Pselselect::Div16384
    }
    #[doc = "Divide clock by 32768"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == Pselselect::Div32768
    }
    #[doc = "Divide clock by 65536"]
    #[inline(always)]
    pub fn is_div65536(&self) -> bool {
        *self == Pselselect::Div65536
    }
}
#[doc = "Field `PSEL` writer - Prescaler Select"]
pub type PselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Pselselect, crate::Safe>;
impl<'a, REG> PselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide clock by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div2)
    }
    #[doc = "Divide clock by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div4)
    }
    #[doc = "Divide clock by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div8)
    }
    #[doc = "Divide clock by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div16)
    }
    #[doc = "Divide clock by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div32)
    }
    #[doc = "Divide clock by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div64)
    }
    #[doc = "Divide clock by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div128)
    }
    #[doc = "Divide clock by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div256)
    }
    #[doc = "Divide clock by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div512)
    }
    #[doc = "Divide clock by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div1024)
    }
    #[doc = "Divide clock by 2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div2048)
    }
    #[doc = "Divide clock by 4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div4096)
    }
    #[doc = "Divide clock by 8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div8192)
    }
    #[doc = "Divide clock by 16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div16384)
    }
    #[doc = "Divide clock by 32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div32768)
    }
    #[doc = "Divide clock by 65536"]
    #[inline(always)]
    pub fn div65536(self) -> &'a mut crate::W<REG> {
        self.variant(Pselselect::Div65536)
    }
}
#[doc = "Field `LEVEL` reader - Threshold Level"]
pub type LevelR = crate::FieldReader;
#[doc = "Field `LEVEL` writer - Threshold Level"]
pub type LevelW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Hysteresis Enable"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Action when Threshold Crossed"]
    #[inline(always)]
    pub fn action(&self) -> ActionR {
        ActionR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Configuration in Standby mode"]
    #[inline(always)]
    pub fn stdbycfg(&self) -> StdbycfgR {
        StdbycfgR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Configuration in Active mode"]
    #[inline(always)]
    pub fn actcfg(&self) -> ActcfgR {
        ActcfgR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Prescaler Select"]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Threshold Level"]
    #[inline(always)]
    pub fn level(&self) -> LevelR {
        LevelR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<BodcoreSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bit 2 - Hysteresis Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hyst(&mut self) -> HystW<BodcoreSpec> {
        HystW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Action when Threshold Crossed"]
    #[inline(always)]
    #[must_use]
    pub fn action(&mut self) -> ActionW<BodcoreSpec> {
        ActionW::new(self, 3)
    }
    #[doc = "Bit 5 - Configuration in Standby mode"]
    #[inline(always)]
    #[must_use]
    pub fn stdbycfg(&mut self) -> StdbycfgW<BodcoreSpec> {
        StdbycfgW::new(self, 5)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<BodcoreSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 8 - Configuration in Active mode"]
    #[inline(always)]
    #[must_use]
    pub fn actcfg(&mut self) -> ActcfgW<BodcoreSpec> {
        ActcfgW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Prescaler Select"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PselW<BodcoreSpec> {
        PselW::new(self, 12)
    }
    #[doc = "Bits 16:21 - Threshold Level"]
    #[inline(always)]
    #[must_use]
    pub fn level(&mut self) -> LevelW<BodcoreSpec> {
        LevelW::new(self, 16)
    }
}
#[doc = "BODCORE Control\n\nYou can [`read`](crate::Reg::read) this register and get [`bodcore::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bodcore::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BodcoreSpec;
impl crate::RegisterSpec for BodcoreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bodcore::R`](R) reader structure"]
impl crate::Readable for BodcoreSpec {}
#[doc = "`write(|w| ..)` method takes [`bodcore::W`](W) writer structure"]
impl crate::Writable for BodcoreSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BODCORE to value 0"]
impl crate::Resettable for BodcoreSpec {
    const RESET_VALUE: u32 = 0;
}
