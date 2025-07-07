#[doc = "Register `CPUDIV` reader"]
pub type R = crate::R<CpudivSpec>;
#[doc = "Register `CPUDIV` writer"]
pub type W = crate::W<CpudivSpec>;
#[doc = "CPU Clock Division Factor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpudivselect {
    #[doc = "1: Divide by 1"]
    Div1 = 1,
    #[doc = "2: Divide by 2"]
    Div2 = 2,
    #[doc = "4: Divide by 4"]
    Div4 = 4,
    #[doc = "8: Divide by 8"]
    Div8 = 8,
    #[doc = "16: Divide by 16"]
    Div16 = 16,
    #[doc = "32: Divide by 32"]
    Div32 = 32,
    #[doc = "64: Divide by 64"]
    Div64 = 64,
    #[doc = "128: Divide by 128"]
    Div128 = 128,
}
impl From<Cpudivselect> for u8 {
    #[inline(always)]
    fn from(variant: Cpudivselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpudivselect {
    type Ux = u8;
}
impl crate::IsEnum for Cpudivselect {}
#[doc = "Field `CPUDIV` reader - CPU Clock Division Factor"]
pub type CpudivR = crate::FieldReader<Cpudivselect>;
impl CpudivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpudivselect> {
        match self.bits {
            1 => Some(Cpudivselect::Div1),
            2 => Some(Cpudivselect::Div2),
            4 => Some(Cpudivselect::Div4),
            8 => Some(Cpudivselect::Div8),
            16 => Some(Cpudivselect::Div16),
            32 => Some(Cpudivselect::Div32),
            64 => Some(Cpudivselect::Div64),
            128 => Some(Cpudivselect::Div128),
            _ => None,
        }
    }
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Cpudivselect::Div1
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Cpudivselect::Div2
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Cpudivselect::Div4
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Cpudivselect::Div8
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Cpudivselect::Div16
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Cpudivselect::Div32
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Cpudivselect::Div64
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Cpudivselect::Div128
    }
}
#[doc = "Field `CPUDIV` writer - CPU Clock Division Factor"]
pub type CpudivW<'a, REG> = crate::FieldWriter<'a, REG, 8, Cpudivselect>;
impl<'a, REG> CpudivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudivselect::Div1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudivselect::Div2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudivselect::Div4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudivselect::Div8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudivselect::Div16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudivselect::Div32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudivselect::Div64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Cpudivselect::Div128)
    }
}
impl R {
    #[doc = "Bits 0:7 - CPU Clock Division Factor"]
    #[inline(always)]
    pub fn cpudiv(&self) -> CpudivR {
        CpudivR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CPU Clock Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn cpudiv(&mut self) -> CpudivW<CpudivSpec> {
        CpudivW::new(self, 0)
    }
}
#[doc = "CPU Clock Division\n\nYou can [`read`](crate::Reg::read) this register and get [`cpudiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpudiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpudivSpec;
impl crate::RegisterSpec for CpudivSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`cpudiv::R`](R) reader structure"]
impl crate::Readable for CpudivSpec {}
#[doc = "`write(|w| ..)` method takes [`cpudiv::W`](W) writer structure"]
impl crate::Writable for CpudivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets CPUDIV to value 0x01"]
impl crate::Resettable for CpudivSpec {
    const RESET_VALUE: u8 = 0x01;
}
