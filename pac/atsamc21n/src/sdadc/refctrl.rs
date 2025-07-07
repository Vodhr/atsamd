#[doc = "Register `REFCTRL` reader"]
pub type R = crate::R<RefctrlSpec>;
#[doc = "Register `REFCTRL` writer"]
pub type W = crate::W<RefctrlSpec>;
#[doc = "Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refselselect {
    #[doc = "0: Internal Bandgap Reference"]
    Intref = 0,
    #[doc = "1: External Reference"]
    Arefb = 1,
    #[doc = "2: Internal DAC Output"]
    Dac = 2,
    #[doc = "3: VDDANA"]
    Intvcc = 3,
}
impl From<Refselselect> for u8 {
    #[inline(always)]
    fn from(variant: Refselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refselselect {
    type Ux = u8;
}
impl crate::IsEnum for Refselselect {}
#[doc = "Field `REFSEL` reader - Reference Selection"]
pub type RefselR = crate::FieldReader<Refselselect>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refselselect {
        match self.bits {
            0 => Refselselect::Intref,
            1 => Refselselect::Arefb,
            2 => Refselselect::Dac,
            3 => Refselselect::Intvcc,
            _ => unreachable!(),
        }
    }
    #[doc = "Internal Bandgap Reference"]
    #[inline(always)]
    pub fn is_intref(&self) -> bool {
        *self == Refselselect::Intref
    }
    #[doc = "External Reference"]
    #[inline(always)]
    pub fn is_arefb(&self) -> bool {
        *self == Refselselect::Arefb
    }
    #[doc = "Internal DAC Output"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == Refselselect::Dac
    }
    #[doc = "VDDANA"]
    #[inline(always)]
    pub fn is_intvcc(&self) -> bool {
        *self == Refselselect::Intvcc
    }
}
#[doc = "Field `REFSEL` writer - Reference Selection"]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refselselect, crate::Safe>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal Bandgap Reference"]
    #[inline(always)]
    pub fn intref(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Intref)
    }
    #[doc = "External Reference"]
    #[inline(always)]
    pub fn arefb(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Arefb)
    }
    #[doc = "Internal DAC Output"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Dac)
    }
    #[doc = "VDDANA"]
    #[inline(always)]
    pub fn intvcc(self) -> &'a mut crate::W<REG> {
        self.variant(Refselselect::Intvcc)
    }
}
#[doc = "Field `REFRANGE` reader - Reference Range"]
pub type RefrangeR = crate::FieldReader;
#[doc = "Field `REFRANGE` writer - Reference Range"]
pub type RefrangeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ONREFBUF` reader - Reference Buffer"]
pub type OnrefbufR = crate::BitReader;
#[doc = "Field `ONREFBUF` writer - Reference Buffer"]
pub type OnrefbufW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new(self.bits & 3)
    }
    #[doc = "Bits 4:5 - Reference Range"]
    #[inline(always)]
    pub fn refrange(&self) -> RefrangeR {
        RefrangeR::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 7 - Reference Buffer"]
    #[inline(always)]
    pub fn onrefbuf(&self) -> OnrefbufR {
        OnrefbufR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> RefselW<RefctrlSpec> {
        RefselW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Reference Range"]
    #[inline(always)]
    #[must_use]
    pub fn refrange(&mut self) -> RefrangeW<RefctrlSpec> {
        RefrangeW::new(self, 4)
    }
    #[doc = "Bit 7 - Reference Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn onrefbuf(&mut self) -> OnrefbufW<RefctrlSpec> {
        OnrefbufW::new(self, 7)
    }
}
#[doc = "Reference Control\n\nYou can [`read`](crate::Reg::read) this register and get [`refctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`refctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefctrlSpec;
impl crate::RegisterSpec for RefctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`refctrl::R`](R) reader structure"]
impl crate::Readable for RefctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`refctrl::W`](W) writer structure"]
impl crate::Writable for RefctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets REFCTRL to value 0"]
impl crate::Resettable for RefctrlSpec {
    const RESET_VALUE: u8 = 0;
}
