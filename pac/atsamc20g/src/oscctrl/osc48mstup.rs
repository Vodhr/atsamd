#[doc = "Register `OSC48MSTUP` reader"]
pub type R = crate::R<Osc48mstupSpec>;
#[doc = "Register `OSC48MSTUP` writer"]
pub type W = crate::W<Osc48mstupSpec>;
#[doc = "Startup Time\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Startupselect {
    #[doc = "0: 166 ns"]
    Cycle8 = 0,
    #[doc = "1: 333 ns"]
    Cycle16 = 1,
    #[doc = "2: 667 ns"]
    Cycle32 = 2,
    #[doc = "3: 1.333 us"]
    Cycle64 = 3,
    #[doc = "4: 2.667 us"]
    Cycle128 = 4,
    #[doc = "5: 5.333 us"]
    Cycle256 = 5,
    #[doc = "6: 10.667 us"]
    Cycle512 = 6,
    #[doc = "7: 21.333 us"]
    Cycle1024 = 7,
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
#[doc = "Field `STARTUP` reader - Startup Time"]
pub type StartupR = crate::FieldReader<Startupselect>;
impl StartupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Startupselect {
        match self.bits {
            0 => Startupselect::Cycle8,
            1 => Startupselect::Cycle16,
            2 => Startupselect::Cycle32,
            3 => Startupselect::Cycle64,
            4 => Startupselect::Cycle128,
            5 => Startupselect::Cycle256,
            6 => Startupselect::Cycle512,
            7 => Startupselect::Cycle1024,
            _ => unreachable!(),
        }
    }
    #[doc = "166 ns"]
    #[inline(always)]
    pub fn is_cycle8(&self) -> bool {
        *self == Startupselect::Cycle8
    }
    #[doc = "333 ns"]
    #[inline(always)]
    pub fn is_cycle16(&self) -> bool {
        *self == Startupselect::Cycle16
    }
    #[doc = "667 ns"]
    #[inline(always)]
    pub fn is_cycle32(&self) -> bool {
        *self == Startupselect::Cycle32
    }
    #[doc = "1.333 us"]
    #[inline(always)]
    pub fn is_cycle64(&self) -> bool {
        *self == Startupselect::Cycle64
    }
    #[doc = "2.667 us"]
    #[inline(always)]
    pub fn is_cycle128(&self) -> bool {
        *self == Startupselect::Cycle128
    }
    #[doc = "5.333 us"]
    #[inline(always)]
    pub fn is_cycle256(&self) -> bool {
        *self == Startupselect::Cycle256
    }
    #[doc = "10.667 us"]
    #[inline(always)]
    pub fn is_cycle512(&self) -> bool {
        *self == Startupselect::Cycle512
    }
    #[doc = "21.333 us"]
    #[inline(always)]
    pub fn is_cycle1024(&self) -> bool {
        *self == Startupselect::Cycle1024
    }
}
#[doc = "Field `STARTUP` writer - Startup Time"]
pub type StartupW<'a, REG> = crate::FieldWriter<'a, REG, 3, Startupselect, crate::Safe>;
impl<'a, REG> StartupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "166 ns"]
    #[inline(always)]
    pub fn cycle8(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle8)
    }
    #[doc = "333 ns"]
    #[inline(always)]
    pub fn cycle16(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle16)
    }
    #[doc = "667 ns"]
    #[inline(always)]
    pub fn cycle32(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle32)
    }
    #[doc = "1.333 us"]
    #[inline(always)]
    pub fn cycle64(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle64)
    }
    #[doc = "2.667 us"]
    #[inline(always)]
    pub fn cycle128(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle128)
    }
    #[doc = "5.333 us"]
    #[inline(always)]
    pub fn cycle256(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle256)
    }
    #[doc = "10.667 us"]
    #[inline(always)]
    pub fn cycle512(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle512)
    }
    #[doc = "21.333 us"]
    #[inline(always)]
    pub fn cycle1024(self) -> &'a mut crate::W<REG> {
        self.variant(Startupselect::Cycle1024)
    }
}
impl R {
    #[doc = "Bits 0:2 - Startup Time"]
    #[inline(always)]
    pub fn startup(&self) -> StartupR {
        StartupR::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Startup Time"]
    #[inline(always)]
    #[must_use]
    pub fn startup(&mut self) -> StartupW<Osc48mstupSpec> {
        StartupW::new(self, 0)
    }
}
#[doc = "OSC48M Startup Time\n\nYou can [`read`](crate::Reg::read) this register and get [`osc48mstup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc48mstup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osc48mstupSpec;
impl crate::RegisterSpec for Osc48mstupSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`osc48mstup::R`](R) reader structure"]
impl crate::Readable for Osc48mstupSpec {}
#[doc = "`write(|w| ..)` method takes [`osc48mstup::W`](W) writer structure"]
impl crate::Writable for Osc48mstupSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OSC48MSTUP to value 0x07"]
impl crate::Resettable for Osc48mstupSpec {
    const RESET_VALUE: u8 = 0x07;
}
