#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `XOSCRDY` reader - XOSC Ready"]
pub type XoscrdyR = crate::BitReader;
#[doc = "Field `XOSCFAIL` reader - XOSC Clock Failure Detector"]
pub type XoscfailR = crate::BitReader;
#[doc = "Field `XOSCCKSW` reader - XOSC Clock Switch"]
pub type XoscckswR = crate::BitReader;
#[doc = "Field `OSC48MRDY` reader - OSC48M Ready"]
pub type Osc48mrdyR = crate::BitReader;
#[doc = "Field `DPLLLCKR` reader - DPLL Lock Rise"]
pub type DplllckrR = crate::BitReader;
#[doc = "Field `DPLLLCKF` reader - DPLL Lock Fall"]
pub type DplllckfR = crate::BitReader;
#[doc = "Field `DPLLTO` reader - DPLL Timeout"]
pub type DplltoR = crate::BitReader;
#[doc = "Field `DPLLLDRTO` reader - DPLL Ratio Ready"]
pub type DpllldrtoR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - XOSC Ready"]
    #[inline(always)]
    pub fn xoscrdy(&self) -> XoscrdyR {
        XoscrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - XOSC Clock Failure Detector"]
    #[inline(always)]
    pub fn xoscfail(&self) -> XoscfailR {
        XoscfailR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - XOSC Clock Switch"]
    #[inline(always)]
    pub fn xosccksw(&self) -> XoscckswR {
        XoscckswR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - OSC48M Ready"]
    #[inline(always)]
    pub fn osc48mrdy(&self) -> Osc48mrdyR {
        Osc48mrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - DPLL Lock Rise"]
    #[inline(always)]
    pub fn dplllckr(&self) -> DplllckrR {
        DplllckrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DPLL Lock Fall"]
    #[inline(always)]
    pub fn dplllckf(&self) -> DplllckfR {
        DplllckfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DPLL Timeout"]
    #[inline(always)]
    pub fn dpllto(&self) -> DplltoR {
        DplltoR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DPLL Ratio Ready"]
    #[inline(always)]
    pub fn dpllldrto(&self) -> DpllldrtoR {
        DpllldrtoR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Power and Clocks Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0;
}
