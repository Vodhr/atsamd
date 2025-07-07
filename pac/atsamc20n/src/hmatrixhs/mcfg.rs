#[doc = "Register `MCFG[%s]` reader"]
pub type R = crate::R<McfgSpec>;
#[doc = "Register `MCFG[%s]` writer"]
pub type W = crate::W<McfgSpec>;
#[doc = "Undefined Length Burst Type\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ulbtselect {
    #[doc = "0: Infinite Length"]
    Infinite = 0,
    #[doc = "1: Single Access"]
    Single = 1,
    #[doc = "2: Four Beat Burst"]
    FourBeat = 2,
    #[doc = "3: Eight Beat Burst"]
    EightBeat = 3,
    #[doc = "4: Sixteen Beat Burst"]
    SixteenBeat = 4,
}
impl From<Ulbtselect> for u8 {
    #[inline(always)]
    fn from(variant: Ulbtselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ulbtselect {
    type Ux = u8;
}
impl crate::IsEnum for Ulbtselect {}
#[doc = "Field `ULBT` reader - Undefined Length Burst Type"]
pub type UlbtR = crate::FieldReader<Ulbtselect>;
impl UlbtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ulbtselect> {
        match self.bits {
            0 => Some(Ulbtselect::Infinite),
            1 => Some(Ulbtselect::Single),
            2 => Some(Ulbtselect::FourBeat),
            3 => Some(Ulbtselect::EightBeat),
            4 => Some(Ulbtselect::SixteenBeat),
            _ => None,
        }
    }
    #[doc = "Infinite Length"]
    #[inline(always)]
    pub fn is_infinite(&self) -> bool {
        *self == Ulbtselect::Infinite
    }
    #[doc = "Single Access"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Ulbtselect::Single
    }
    #[doc = "Four Beat Burst"]
    #[inline(always)]
    pub fn is_four_beat(&self) -> bool {
        *self == Ulbtselect::FourBeat
    }
    #[doc = "Eight Beat Burst"]
    #[inline(always)]
    pub fn is_eight_beat(&self) -> bool {
        *self == Ulbtselect::EightBeat
    }
    #[doc = "Sixteen Beat Burst"]
    #[inline(always)]
    pub fn is_sixteen_beat(&self) -> bool {
        *self == Ulbtselect::SixteenBeat
    }
}
#[doc = "Field `ULBT` writer - Undefined Length Burst Type"]
pub type UlbtW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ulbtselect>;
impl<'a, REG> UlbtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Infinite Length"]
    #[inline(always)]
    pub fn infinite(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbtselect::Infinite)
    }
    #[doc = "Single Access"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbtselect::Single)
    }
    #[doc = "Four Beat Burst"]
    #[inline(always)]
    pub fn four_beat(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbtselect::FourBeat)
    }
    #[doc = "Eight Beat Burst"]
    #[inline(always)]
    pub fn eight_beat(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbtselect::EightBeat)
    }
    #[doc = "Sixteen Beat Burst"]
    #[inline(always)]
    pub fn sixteen_beat(self) -> &'a mut crate::W<REG> {
        self.variant(Ulbtselect::SixteenBeat)
    }
}
impl R {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&self) -> UlbtR {
        UlbtR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    #[must_use]
    pub fn ulbt(&mut self) -> UlbtW<McfgSpec> {
        UlbtW::new(self, 0)
    }
}
#[doc = "Master Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McfgSpec;
impl crate::RegisterSpec for McfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcfg::R`](R) reader structure"]
impl crate::Readable for McfgSpec {}
#[doc = "`write(|w| ..)` method takes [`mcfg::W`](W) writer structure"]
impl crate::Writable for McfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MCFG[%s]
to value 0x02"]
impl crate::Resettable for McfgSpec {
    const RESET_VALUE: u32 = 0x02;
}
