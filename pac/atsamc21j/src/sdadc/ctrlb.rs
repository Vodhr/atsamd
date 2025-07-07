#[doc = "Register `CTRLB` reader"]
pub type R = crate::R<CtrlbSpec>;
#[doc = "Register `CTRLB` writer"]
pub type W = crate::W<CtrlbSpec>;
#[doc = "Prescaler Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescalerselect {
    #[doc = "0: Peripheral clock divided by 2"]
    Div2 = 0,
    #[doc = "1: Peripheral clock divided by 4"]
    Div4 = 1,
    #[doc = "2: Peripheral clock divided by 8"]
    Div8 = 2,
    #[doc = "3: Peripheral clock divided by 16"]
    Div16 = 3,
    #[doc = "4: Peripheral clock divided by 32"]
    Div32 = 4,
    #[doc = "5: Peripheral clock divided by 64"]
    Div64 = 5,
    #[doc = "6: Peripheral clock divided by 128"]
    Div128 = 6,
    #[doc = "7: Peripheral clock divided by 256"]
    Div256 = 7,
}
impl From<Prescalerselect> for u8 {
    #[inline(always)]
    fn from(variant: Prescalerselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescalerselect {
    type Ux = u8;
}
impl crate::IsEnum for Prescalerselect {}
#[doc = "Field `PRESCALER` reader - Prescaler Configuration"]
pub type PrescalerR = crate::FieldReader<Prescalerselect>;
impl PrescalerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prescalerselect> {
        match self.bits {
            0 => Some(Prescalerselect::Div2),
            1 => Some(Prescalerselect::Div4),
            2 => Some(Prescalerselect::Div8),
            3 => Some(Prescalerselect::Div16),
            4 => Some(Prescalerselect::Div32),
            5 => Some(Prescalerselect::Div64),
            6 => Some(Prescalerselect::Div128),
            7 => Some(Prescalerselect::Div256),
            _ => None,
        }
    }
    #[doc = "Peripheral clock divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Prescalerselect::Div2
    }
    #[doc = "Peripheral clock divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Prescalerselect::Div4
    }
    #[doc = "Peripheral clock divided by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Prescalerselect::Div8
    }
    #[doc = "Peripheral clock divided by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Prescalerselect::Div16
    }
    #[doc = "Peripheral clock divided by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Prescalerselect::Div32
    }
    #[doc = "Peripheral clock divided by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Prescalerselect::Div64
    }
    #[doc = "Peripheral clock divided by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Prescalerselect::Div128
    }
    #[doc = "Peripheral clock divided by 256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Prescalerselect::Div256
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler Configuration"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 8, Prescalerselect>;
impl<'a, REG> PrescalerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div2)
    }
    #[doc = "Peripheral clock divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div4)
    }
    #[doc = "Peripheral clock divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div8)
    }
    #[doc = "Peripheral clock divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div16)
    }
    #[doc = "Peripheral clock divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div32)
    }
    #[doc = "Peripheral clock divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div64)
    }
    #[doc = "Peripheral clock divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div128)
    }
    #[doc = "Peripheral clock divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div256)
    }
}
#[doc = "Over Sampling Ratio\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Osrselect {
    #[doc = "0: Over Sampling Ratio is 64"]
    Osr64 = 0,
    #[doc = "1: Over Sampling Ratio is 128"]
    Osr128 = 1,
    #[doc = "2: Over Sampling Ratio is 256"]
    Osr256 = 2,
    #[doc = "3: Over Sampling Ratio is 512"]
    Osr512 = 3,
    #[doc = "4: Over Sampling Ratio is 1024"]
    Osr1024 = 4,
}
impl From<Osrselect> for u8 {
    #[inline(always)]
    fn from(variant: Osrselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Osrselect {
    type Ux = u8;
}
impl crate::IsEnum for Osrselect {}
#[doc = "Field `OSR` reader - Over Sampling Ratio"]
pub type OsrR = crate::FieldReader<Osrselect>;
impl OsrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Osrselect> {
        match self.bits {
            0 => Some(Osrselect::Osr64),
            1 => Some(Osrselect::Osr128),
            2 => Some(Osrselect::Osr256),
            3 => Some(Osrselect::Osr512),
            4 => Some(Osrselect::Osr1024),
            _ => None,
        }
    }
    #[doc = "Over Sampling Ratio is 64"]
    #[inline(always)]
    pub fn is_osr64(&self) -> bool {
        *self == Osrselect::Osr64
    }
    #[doc = "Over Sampling Ratio is 128"]
    #[inline(always)]
    pub fn is_osr128(&self) -> bool {
        *self == Osrselect::Osr128
    }
    #[doc = "Over Sampling Ratio is 256"]
    #[inline(always)]
    pub fn is_osr256(&self) -> bool {
        *self == Osrselect::Osr256
    }
    #[doc = "Over Sampling Ratio is 512"]
    #[inline(always)]
    pub fn is_osr512(&self) -> bool {
        *self == Osrselect::Osr512
    }
    #[doc = "Over Sampling Ratio is 1024"]
    #[inline(always)]
    pub fn is_osr1024(&self) -> bool {
        *self == Osrselect::Osr1024
    }
}
#[doc = "Field `OSR` writer - Over Sampling Ratio"]
pub type OsrW<'a, REG> = crate::FieldWriter<'a, REG, 3, Osrselect>;
impl<'a, REG> OsrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Over Sampling Ratio is 64"]
    #[inline(always)]
    pub fn osr64(self) -> &'a mut crate::W<REG> {
        self.variant(Osrselect::Osr64)
    }
    #[doc = "Over Sampling Ratio is 128"]
    #[inline(always)]
    pub fn osr128(self) -> &'a mut crate::W<REG> {
        self.variant(Osrselect::Osr128)
    }
    #[doc = "Over Sampling Ratio is 256"]
    #[inline(always)]
    pub fn osr256(self) -> &'a mut crate::W<REG> {
        self.variant(Osrselect::Osr256)
    }
    #[doc = "Over Sampling Ratio is 512"]
    #[inline(always)]
    pub fn osr512(self) -> &'a mut crate::W<REG> {
        self.variant(Osrselect::Osr512)
    }
    #[doc = "Over Sampling Ratio is 1024"]
    #[inline(always)]
    pub fn osr1024(self) -> &'a mut crate::W<REG> {
        self.variant(Osrselect::Osr1024)
    }
}
#[doc = "Field `SKPCNT` reader - Skip Sample Count"]
pub type SkpcntR = crate::FieldReader;
#[doc = "Field `SKPCNT` writer - Skip Sample Count"]
pub type SkpcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Prescaler Configuration"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Over Sampling Ratio"]
    #[inline(always)]
    pub fn osr(&self) -> OsrR {
        OsrR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:15 - Skip Sample Count"]
    #[inline(always)]
    pub fn skpcnt(&self) -> SkpcntR {
        SkpcntR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Prescaler Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PrescalerW<CtrlbSpec> {
        PrescalerW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Over Sampling Ratio"]
    #[inline(always)]
    #[must_use]
    pub fn osr(&mut self) -> OsrW<CtrlbSpec> {
        OsrW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Skip Sample Count"]
    #[inline(always)]
    #[must_use]
    pub fn skpcnt(&mut self) -> SkpcntW<CtrlbSpec> {
        SkpcntW::new(self, 12)
    }
}
#[doc = "Control B\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlbSpec;
impl crate::RegisterSpec for CtrlbSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrlb::R`](R) reader structure"]
impl crate::Readable for CtrlbSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlb::W`](W) writer structure"]
impl crate::Writable for CtrlbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0x2000"]
impl crate::Resettable for CtrlbSpec {
    const RESET_VALUE: u16 = 0x2000;
}
