#[doc = "Register `INPUTCTRL` reader"]
pub type R = crate::R<InputctrlSpec>;
#[doc = "Register `INPUTCTRL` writer"]
pub type W = crate::W<InputctrlSpec>;
#[doc = "SDADC Input Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Muxselselect {
    #[doc = "0: SDADC AIN0 Pin"]
    Ain0 = 0,
    #[doc = "1: SDADC AIN1 Pin"]
    Ain1 = 1,
    #[doc = "2: SDADC AIN2 Pin"]
    Ain2 = 2,
}
impl From<Muxselselect> for u8 {
    #[inline(always)]
    fn from(variant: Muxselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Muxselselect {
    type Ux = u8;
}
impl crate::IsEnum for Muxselselect {}
#[doc = "Field `MUXSEL` reader - SDADC Input Selection"]
pub type MuxselR = crate::FieldReader<Muxselselect>;
impl MuxselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Muxselselect> {
        match self.bits {
            0 => Some(Muxselselect::Ain0),
            1 => Some(Muxselselect::Ain1),
            2 => Some(Muxselselect::Ain2),
            _ => None,
        }
    }
    #[doc = "SDADC AIN0 Pin"]
    #[inline(always)]
    pub fn is_ain0(&self) -> bool {
        *self == Muxselselect::Ain0
    }
    #[doc = "SDADC AIN1 Pin"]
    #[inline(always)]
    pub fn is_ain1(&self) -> bool {
        *self == Muxselselect::Ain1
    }
    #[doc = "SDADC AIN2 Pin"]
    #[inline(always)]
    pub fn is_ain2(&self) -> bool {
        *self == Muxselselect::Ain2
    }
}
#[doc = "Field `MUXSEL` writer - SDADC Input Selection"]
pub type MuxselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Muxselselect>;
impl<'a, REG> MuxselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDADC AIN0 Pin"]
    #[inline(always)]
    pub fn ain0(self) -> &'a mut crate::W<REG> {
        self.variant(Muxselselect::Ain0)
    }
    #[doc = "SDADC AIN1 Pin"]
    #[inline(always)]
    pub fn ain1(self) -> &'a mut crate::W<REG> {
        self.variant(Muxselselect::Ain1)
    }
    #[doc = "SDADC AIN2 Pin"]
    #[inline(always)]
    pub fn ain2(self) -> &'a mut crate::W<REG> {
        self.variant(Muxselselect::Ain2)
    }
}
impl R {
    #[doc = "Bits 0:3 - SDADC Input Selection"]
    #[inline(always)]
    pub fn muxsel(&self) -> MuxselR {
        MuxselR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - SDADC Input Selection"]
    #[inline(always)]
    #[must_use]
    pub fn muxsel(&mut self) -> MuxselW<InputctrlSpec> {
        MuxselW::new(self, 0)
    }
}
#[doc = "Input Control\n\nYou can [`read`](crate::Reg::read) this register and get [`inputctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputctrlSpec;
impl crate::RegisterSpec for InputctrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`inputctrl::R`](R) reader structure"]
impl crate::Readable for InputctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`inputctrl::W`](W) writer structure"]
impl crate::Writable for InputctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INPUTCTRL to value 0"]
impl crate::Resettable for InputctrlSpec {
    const RESET_VALUE: u8 = 0;
}
