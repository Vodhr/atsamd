#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `XOSCRDY` reader - XOSC Ready Interrupt Enable"]
pub type XoscrdyR = crate::BitReader;
#[doc = "Field `XOSCRDY` writer - XOSC Ready Interrupt Enable"]
pub type XoscrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XOSCFAIL` reader - XOSC Clock Failure Detector Interrupt Enable"]
pub type XoscfailR = crate::BitReader;
#[doc = "Field `XOSCFAIL` writer - XOSC Clock Failure Detector Interrupt Enable"]
pub type XoscfailW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSC48MRDY` reader - OSC48M Ready Interrupt Enable"]
pub type Osc48mrdyR = crate::BitReader;
#[doc = "Field `OSC48MRDY` writer - OSC48M Ready Interrupt Enable"]
pub type Osc48mrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLCKR` reader - DPLL Lock Rise Interrupt Enable"]
pub type DplllckrR = crate::BitReader;
#[doc = "Field `DPLLLCKR` writer - DPLL Lock Rise Interrupt Enable"]
pub type DplllckrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLCKF` reader - DPLL Lock Fall Interrupt Enable"]
pub type DplllckfR = crate::BitReader;
#[doc = "Field `DPLLLCKF` writer - DPLL Lock Fall Interrupt Enable"]
pub type DplllckfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLTO` reader - DPLL Time Out Interrupt Enable"]
pub type DpllltoR = crate::BitReader;
#[doc = "Field `DPLLLTO` writer - DPLL Time Out Interrupt Enable"]
pub type DpllltoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLLLDRTO` reader - DPLL Ratio Ready Interrupt Enable"]
pub type DpllldrtoR = crate::BitReader;
#[doc = "Field `DPLLLDRTO` writer - DPLL Ratio Ready Interrupt Enable"]
pub type DpllldrtoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XoscrdyR {
        XoscrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XOSC Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    pub fn xoscfail(&self) -> XoscfailR {
        XoscfailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - OSC48M Ready Interrupt Enable"]
    #[inline(always)]
    pub fn osc48mrdy(&self) -> Osc48mrdyR {
        Osc48mrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - DPLL Lock Rise Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckr(&self) -> DplllckrR {
        DplllckrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DPLL Lock Fall Interrupt Enable"]
    #[inline(always)]
    pub fn dplllckf(&self) -> DplllckfR {
        DplllckfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DPLL Time Out Interrupt Enable"]
    #[inline(always)]
    pub fn dplllto(&self) -> DpllltoR {
        DpllltoR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DPLL Ratio Ready Interrupt Enable"]
    #[inline(always)]
    pub fn dpllldrto(&self) -> DpllldrtoR {
        DpllldrtoR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - XOSC Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xoscrdy(&mut self) -> XoscrdyW<IntenclrSpec> {
        XoscrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - XOSC Clock Failure Detector Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xoscfail(&mut self) -> XoscfailW<IntenclrSpec> {
        XoscfailW::new(self, 1)
    }
    #[doc = "Bit 4 - OSC48M Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn osc48mrdy(&mut self) -> Osc48mrdyW<IntenclrSpec> {
        Osc48mrdyW::new(self, 4)
    }
    #[doc = "Bit 8 - DPLL Lock Rise Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dplllckr(&mut self) -> DplllckrW<IntenclrSpec> {
        DplllckrW::new(self, 8)
    }
    #[doc = "Bit 9 - DPLL Lock Fall Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dplllckf(&mut self) -> DplllckfW<IntenclrSpec> {
        DplllckfW::new(self, 9)
    }
    #[doc = "Bit 10 - DPLL Time Out Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dplllto(&mut self) -> DpllltoW<IntenclrSpec> {
        DpllltoW::new(self, 10)
    }
    #[doc = "Bit 11 - DPLL Ratio Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpllldrto(&mut self) -> DpllldrtoW<IntenclrSpec> {
        DpllldrtoW::new(self, 11)
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {
    const RESET_VALUE: u32 = 0;
}
