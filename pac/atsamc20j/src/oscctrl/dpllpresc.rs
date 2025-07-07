#[doc = "Register `DPLLPRESC` reader"]
pub type R = crate::R<DpllprescSpec>;
#[doc = "Register `DPLLPRESC` writer"]
pub type W = crate::W<DpllprescSpec>;
#[doc = "Output Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescselect {
    #[doc = "0: DPLL output is divided by 1"]
    Div1 = 0,
    #[doc = "1: DPLL output is divided by 2"]
    Div2 = 1,
    #[doc = "2: DPLL output is divided by 4"]
    Div4 = 2,
}
impl From<Prescselect> for u8 {
    #[inline(always)]
    fn from(variant: Prescselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescselect {
    type Ux = u8;
}
impl crate::IsEnum for Prescselect {}
#[doc = "Field `PRESC` reader - Output Clock Prescaler"]
pub type PrescR = crate::FieldReader<Prescselect>;
impl PrescR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prescselect> {
        match self.bits {
            0 => Some(Prescselect::Div1),
            1 => Some(Prescselect::Div2),
            2 => Some(Prescselect::Div4),
            _ => None,
        }
    }
    #[doc = "DPLL output is divided by 1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Prescselect::Div1
    }
    #[doc = "DPLL output is divided by 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Prescselect::Div2
    }
    #[doc = "DPLL output is divided by 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Prescselect::Div4
    }
}
#[doc = "Field `PRESC` writer - Output Clock Prescaler"]
pub type PrescW<'a, REG> = crate::FieldWriter<'a, REG, 2, Prescselect>;
impl<'a, REG> PrescW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DPLL output is divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Prescselect::Div1)
    }
    #[doc = "DPLL output is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescselect::Div2)
    }
    #[doc = "DPLL output is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Prescselect::Div4)
    }
}
impl R {
    #[doc = "Bits 0:1 - Output Clock Prescaler"]
    #[inline(always)]
    pub fn presc(&self) -> PrescR {
        PrescR::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Output Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn presc(&mut self) -> PrescW<DpllprescSpec> {
        PrescW::new(self, 0)
    }
}
#[doc = "DPLL Prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllpresc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllpresc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpllprescSpec;
impl crate::RegisterSpec for DpllprescSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dpllpresc::R`](R) reader structure"]
impl crate::Readable for DpllprescSpec {}
#[doc = "`write(|w| ..)` method takes [`dpllpresc::W`](W) writer structure"]
impl crate::Writable for DpllprescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets DPLLPRESC to value 0"]
impl crate::Resettable for DpllprescSpec {
    const RESET_VALUE: u8 = 0;
}
