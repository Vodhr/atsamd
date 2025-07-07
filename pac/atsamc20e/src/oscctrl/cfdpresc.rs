#[doc = "Register `CFDPRESC` reader"]
pub type R = crate::R<CfdprescSpec>;
#[doc = "Register `CFDPRESC` writer"]
pub type W = crate::W<CfdprescSpec>;
#[doc = "Clock Failure Detector Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfdprescselect {
    #[doc = "0: 48 MHz"]
    Div1 = 0,
    #[doc = "1: 24 MHz"]
    Div2 = 1,
    #[doc = "2: 12 MHz"]
    Div4 = 2,
    #[doc = "3: 6 MHz"]
    Div8 = 3,
    #[doc = "4: 3 MHz"]
    Div16 = 4,
    #[doc = "5: 1.5 MHz"]
    Div32 = 5,
    #[doc = "6: 0.75 MHz"]
    Div64 = 6,
    #[doc = "7: 0.3125 MHz"]
    Div128 = 7,
}
impl From<Cfdprescselect> for u8 {
    #[inline(always)]
    fn from(variant: Cfdprescselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfdprescselect {
    type Ux = u8;
}
impl crate::IsEnum for Cfdprescselect {}
#[doc = "Field `CFDPRESC` reader - Clock Failure Detector Prescaler"]
pub type CfdprescR = crate::FieldReader<Cfdprescselect>;
impl CfdprescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfdprescselect {
        match self.bits {
            0 => Cfdprescselect::Div1,
            1 => Cfdprescselect::Div2,
            2 => Cfdprescselect::Div4,
            3 => Cfdprescselect::Div8,
            4 => Cfdprescselect::Div16,
            5 => Cfdprescselect::Div32,
            6 => Cfdprescselect::Div64,
            7 => Cfdprescselect::Div128,
            _ => unreachable!(),
        }
    }
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Cfdprescselect::Div1
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Cfdprescselect::Div2
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Cfdprescselect::Div4
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Cfdprescselect::Div8
    }
    #[doc = "3 MHz"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Cfdprescselect::Div16
    }
    #[doc = "1.5 MHz"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Cfdprescselect::Div32
    }
    #[doc = "0.75 MHz"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Cfdprescselect::Div64
    }
    #[doc = "0.3125 MHz"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Cfdprescselect::Div128
    }
}
#[doc = "Field `CFDPRESC` writer - Clock Failure Detector Prescaler"]
pub type CfdprescW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cfdprescselect, crate::Safe>;
impl<'a, REG> CfdprescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdprescselect::Div1)
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdprescselect::Div2)
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdprescselect::Div4)
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdprescselect::Div8)
    }
    #[doc = "3 MHz"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdprescselect::Div16)
    }
    #[doc = "1.5 MHz"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdprescselect::Div32)
    }
    #[doc = "0.75 MHz"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdprescselect::Div64)
    }
    #[doc = "0.3125 MHz"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Cfdprescselect::Div128)
    }
}
impl R {
    #[doc = "Bits 0:2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub fn cfdpresc(&self) -> CfdprescR {
        CfdprescR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn cfdpresc(&mut self) -> CfdprescW<CfdprescSpec> {
        CfdprescW::new(self, 0)
    }
}
#[doc = "Clock Failure Detector Prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`cfdpresc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdpresc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfdprescSpec;
impl crate::RegisterSpec for CfdprescSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cfdpresc::R`](R) reader structure"]
impl crate::Readable for CfdprescSpec {}
#[doc = "`write(|w| ..)` method takes [`cfdpresc::W`](W) writer structure"]
impl crate::Writable for CfdprescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CFDPRESC to value 0"]
impl crate::Resettable for CfdprescSpec {
    const RESET_VALUE: u8 = 0;
}
