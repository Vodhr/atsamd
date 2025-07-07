#[doc = "Register `STATUSC` reader"]
pub type R = crate::R<StatuscSpec>;
#[doc = "Field `EVSYS_` reader - EVSYS APB Protect Enable"]
pub type Evsys_R = crate::BitReader;
#[doc = "Field `SERCOM0_` reader - SERCOM0 APB Protect Enable"]
pub type Sercom0_R = crate::BitReader;
#[doc = "Field `SERCOM1_` reader - SERCOM1 APB Protect Enable"]
pub type Sercom1_R = crate::BitReader;
#[doc = "Field `SERCOM2_` reader - SERCOM2 APB Protect Enable"]
pub type Sercom2_R = crate::BitReader;
#[doc = "Field `SERCOM3_` reader - SERCOM3 APB Protect Enable"]
pub type Sercom3_R = crate::BitReader;
#[doc = "Field `SERCOM4_` reader - SERCOM4 APB Protect Enable"]
pub type Sercom4_R = crate::BitReader;
#[doc = "Field `SERCOM5_` reader - SERCOM5 APB Protect Enable"]
pub type Sercom5_R = crate::BitReader;
#[doc = "Field `CAN0_` reader - CAN0 APB Protect Enable"]
pub type Can0_R = crate::BitReader;
#[doc = "Field `CAN1_` reader - CAN1 APB Protect Enable"]
pub type Can1_R = crate::BitReader;
#[doc = "Field `TCC0_` reader - TCC0 APB Protect Enable"]
pub type Tcc0_R = crate::BitReader;
#[doc = "Field `TCC1_` reader - TCC1 APB Protect Enable"]
pub type Tcc1_R = crate::BitReader;
#[doc = "Field `TCC2_` reader - TCC2 APB Protect Enable"]
pub type Tcc2_R = crate::BitReader;
#[doc = "Field `TC0_` reader - TC0 APB Protect Enable"]
pub type Tc0_R = crate::BitReader;
#[doc = "Field `TC1_` reader - TC1 APB Protect Enable"]
pub type Tc1_R = crate::BitReader;
#[doc = "Field `TC2_` reader - TC2 APB Protect Enable"]
pub type Tc2_R = crate::BitReader;
#[doc = "Field `TC3_` reader - TC3 APB Protect Enable"]
pub type Tc3_R = crate::BitReader;
#[doc = "Field `TC4_` reader - TC4 APB Protect Enable"]
pub type Tc4_R = crate::BitReader;
#[doc = "Field `ADC0_` reader - ADC0 APB Protect Enable"]
pub type Adc0_R = crate::BitReader;
#[doc = "Field `ADC1_` reader - ADC1 APB Protect Enable"]
pub type Adc1_R = crate::BitReader;
#[doc = "Field `SDADC_` reader - SDADC APB Protect Enable"]
pub type Sdadc_R = crate::BitReader;
#[doc = "Field `AC_` reader - AC APB Protect Enable"]
pub type Ac_R = crate::BitReader;
#[doc = "Field `DAC_` reader - DAC APB Protect Enable"]
pub type Dac_R = crate::BitReader;
#[doc = "Field `PTC_` reader - PTC APB Protect Enable"]
pub type Ptc_R = crate::BitReader;
#[doc = "Field `CCL_` reader - CCL APB Protect Enable"]
pub type Ccl_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - EVSYS APB Protect Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> Evsys_R {
        Evsys_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM0 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> Sercom0_R {
        Sercom0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM1 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> Sercom1_R {
        Sercom1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SERCOM2 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> Sercom2_R {
        Sercom2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SERCOM3 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom3_(&self) -> Sercom3_R {
        Sercom3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SERCOM4 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom4_(&self) -> Sercom4_R {
        Sercom4_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SERCOM5 APB Protect Enable"]
    #[inline(always)]
    pub fn sercom5_(&self) -> Sercom5_R {
        Sercom5_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CAN0 APB Protect Enable"]
    #[inline(always)]
    pub fn can0_(&self) -> Can0_R {
        Can0_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CAN1 APB Protect Enable"]
    #[inline(always)]
    pub fn can1_(&self) -> Can1_R {
        Can1_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TCC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc0_(&self) -> Tcc0_R {
        Tcc0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TCC1 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc1_(&self) -> Tcc1_R {
        Tcc1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TCC2 APB Protect Enable"]
    #[inline(always)]
    pub fn tcc2_(&self) -> Tcc2_R {
        Tcc2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TC0 APB Protect Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> Tc0_R {
        Tc0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TC1 APB Protect Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> Tc1_R {
        Tc1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TC2 APB Protect Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> Tc2_R {
        Tc2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TC3 APB Protect Enable"]
    #[inline(always)]
    pub fn tc3_(&self) -> Tc3_R {
        Tc3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TC4 APB Protect Enable"]
    #[inline(always)]
    pub fn tc4_(&self) -> Tc4_R {
        Tc4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC0 APB Protect Enable"]
    #[inline(always)]
    pub fn adc0_(&self) -> Adc0_R {
        Adc0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC1 APB Protect Enable"]
    #[inline(always)]
    pub fn adc1_(&self) -> Adc1_R {
        Adc1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SDADC APB Protect Enable"]
    #[inline(always)]
    pub fn sdadc_(&self) -> Sdadc_R {
        Sdadc_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - AC APB Protect Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> Ac_R {
        Ac_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DAC APB Protect Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> Dac_R {
        Dac_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PTC APB Protect Enable"]
    #[inline(always)]
    pub fn ptc_(&self) -> Ptc_R {
        Ptc_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CCL APB Protect Enable"]
    #[inline(always)]
    pub fn ccl_(&self) -> Ccl_R {
        Ccl_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge C\n\nYou can [`read`](crate::Reg::read) this register and get [`statusc::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatuscSpec;
impl crate::RegisterSpec for StatuscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusc::R`](R) reader structure"]
impl crate::Readable for StatuscSpec {}
#[doc = "`reset()` method sets STATUSC to value 0x0200_0000"]
impl crate::Resettable for StatuscSpec {
    const RESET_VALUE: u32 = 0x0200_0000;
}
