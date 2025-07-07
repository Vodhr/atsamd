#[doc = "Register `APBCMASK` reader"]
pub type R = crate::R<ApbcmaskSpec>;
#[doc = "Register `APBCMASK` writer"]
pub type W = crate::W<ApbcmaskSpec>;
#[doc = "Field `EVSYS_` reader - EVSYS APB Clock Enable"]
pub type Evsys_R = crate::BitReader;
#[doc = "Field `EVSYS_` writer - EVSYS APB Clock Enable"]
pub type Evsys_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM0_` reader - SERCOM0 APB Clock Enable"]
pub type Sercom0_R = crate::BitReader;
#[doc = "Field `SERCOM0_` writer - SERCOM0 APB Clock Enable"]
pub type Sercom0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM1_` reader - SERCOM1 APB Clock Enable"]
pub type Sercom1_R = crate::BitReader;
#[doc = "Field `SERCOM1_` writer - SERCOM1 APB Clock Enable"]
pub type Sercom1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM2_` reader - SERCOM2 APB Clock Enable"]
pub type Sercom2_R = crate::BitReader;
#[doc = "Field `SERCOM2_` writer - SERCOM2 APB Clock Enable"]
pub type Sercom2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SERCOM3_` reader - SERCOM3 APB Clock Enable"]
pub type Sercom3_R = crate::BitReader;
#[doc = "Field `SERCOM3_` writer - SERCOM3 APB Clock Enable"]
pub type Sercom3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC0_` reader - TCC0 APB Clock Enable"]
pub type Tcc0_R = crate::BitReader;
#[doc = "Field `TCC0_` writer - TCC0 APB Clock Enable"]
pub type Tcc0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC1_` reader - TCC1 APB Clock Enable"]
pub type Tcc1_R = crate::BitReader;
#[doc = "Field `TCC1_` writer - TCC1 APB Clock Enable"]
pub type Tcc1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCC2_` reader - TCC2 APB Clock Enable"]
pub type Tcc2_R = crate::BitReader;
#[doc = "Field `TCC2_` writer - TCC2 APB Clock Enable"]
pub type Tcc2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC0_` reader - TC0 APB Clock Enable"]
pub type Tc0_R = crate::BitReader;
#[doc = "Field `TC0_` writer - TC0 APB Clock Enable"]
pub type Tc0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1_` reader - TC1 APB Clock Enable"]
pub type Tc1_R = crate::BitReader;
#[doc = "Field `TC1_` writer - TC1 APB Clock Enable"]
pub type Tc1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2_` reader - TC2 APB Clock Enable"]
pub type Tc2_R = crate::BitReader;
#[doc = "Field `TC2_` writer - TC2 APB Clock Enable"]
pub type Tc2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC3_` reader - TC3 APB Clock Enable"]
pub type Tc3_R = crate::BitReader;
#[doc = "Field `TC3_` writer - TC3 APB Clock Enable"]
pub type Tc3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC4_` reader - TC4 APB Clock Enable"]
pub type Tc4_R = crate::BitReader;
#[doc = "Field `TC4_` writer - TC4 APB Clock Enable"]
pub type Tc4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC0_` reader - ADC0 APB Clock Enable"]
pub type Adc0_R = crate::BitReader;
#[doc = "Field `ADC0_` writer - ADC0 APB Clock Enable"]
pub type Adc0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC1_` reader - ADC1 APB Clock Enable"]
pub type Adc1_R = crate::BitReader;
#[doc = "Field `ADC1_` writer - ADC1 APB Clock Enable"]
pub type Adc1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDADC_` reader - SDADC APB Clock Enable"]
pub type Sdadc_R = crate::BitReader;
#[doc = "Field `SDADC_` writer - SDADC APB Clock Enable"]
pub type Sdadc_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AC_` reader - AC APB Clock Enable"]
pub type Ac_R = crate::BitReader;
#[doc = "Field `AC_` writer - AC APB Clock Enable"]
pub type Ac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC_` reader - DAC APB Clock Enable"]
pub type Dac_R = crate::BitReader;
#[doc = "Field `DAC_` writer - DAC APB Clock Enable"]
pub type Dac_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTC_` reader - PTC APB Clock Enable"]
pub type Ptc_R = crate::BitReader;
#[doc = "Field `PTC_` writer - PTC APB Clock Enable"]
pub type Ptc_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCL_` reader - CCL APB Clock Enable"]
pub type Ccl_R = crate::BitReader;
#[doc = "Field `CCL_` writer - CCL APB Clock Enable"]
pub type Ccl_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EVSYS APB Clock Enable"]
    #[inline(always)]
    pub fn evsys_(&self) -> Evsys_R {
        Evsys_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom0_(&self) -> Sercom0_R {
        Sercom0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom1_(&self) -> Sercom1_R {
        Sercom1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom2_(&self) -> Sercom2_R {
        Sercom2_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SERCOM3 APB Clock Enable"]
    #[inline(always)]
    pub fn sercom3_(&self) -> Sercom3_R {
        Sercom3_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - TCC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc0_(&self) -> Tcc0_R {
        Tcc0_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TCC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc1_(&self) -> Tcc1_R {
        Tcc1_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TCC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tcc2_(&self) -> Tcc2_R {
        Tcc2_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TC0 APB Clock Enable"]
    #[inline(always)]
    pub fn tc0_(&self) -> Tc0_R {
        Tc0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TC1 APB Clock Enable"]
    #[inline(always)]
    pub fn tc1_(&self) -> Tc1_R {
        Tc1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TC2 APB Clock Enable"]
    #[inline(always)]
    pub fn tc2_(&self) -> Tc2_R {
        Tc2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TC3 APB Clock Enable"]
    #[inline(always)]
    pub fn tc3_(&self) -> Tc3_R {
        Tc3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TC4 APB Clock Enable"]
    #[inline(always)]
    pub fn tc4_(&self) -> Tc4_R {
        Tc4_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC0 APB Clock Enable"]
    #[inline(always)]
    pub fn adc0_(&self) -> Adc0_R {
        Adc0_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC1 APB Clock Enable"]
    #[inline(always)]
    pub fn adc1_(&self) -> Adc1_R {
        Adc1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SDADC APB Clock Enable"]
    #[inline(always)]
    pub fn sdadc_(&self) -> Sdadc_R {
        Sdadc_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - AC APB Clock Enable"]
    #[inline(always)]
    pub fn ac_(&self) -> Ac_R {
        Ac_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DAC APB Clock Enable"]
    #[inline(always)]
    pub fn dac_(&self) -> Dac_R {
        Dac_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PTC APB Clock Enable"]
    #[inline(always)]
    pub fn ptc_(&self) -> Ptc_R {
        Ptc_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CCL APB Clock Enable"]
    #[inline(always)]
    pub fn ccl_(&self) -> Ccl_R {
        Ccl_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EVSYS APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evsys_(&mut self) -> Evsys_W<ApbcmaskSpec> {
        Evsys_W::new(self, 0)
    }
    #[doc = "Bit 1 - SERCOM0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom0_(&mut self) -> Sercom0_W<ApbcmaskSpec> {
        Sercom0_W::new(self, 1)
    }
    #[doc = "Bit 2 - SERCOM1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom1_(&mut self) -> Sercom1_W<ApbcmaskSpec> {
        Sercom1_W::new(self, 2)
    }
    #[doc = "Bit 3 - SERCOM2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom2_(&mut self) -> Sercom2_W<ApbcmaskSpec> {
        Sercom2_W::new(self, 3)
    }
    #[doc = "Bit 4 - SERCOM3 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sercom3_(&mut self) -> Sercom3_W<ApbcmaskSpec> {
        Sercom3_W::new(self, 4)
    }
    #[doc = "Bit 9 - TCC0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc0_(&mut self) -> Tcc0_W<ApbcmaskSpec> {
        Tcc0_W::new(self, 9)
    }
    #[doc = "Bit 10 - TCC1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc1_(&mut self) -> Tcc1_W<ApbcmaskSpec> {
        Tcc1_W::new(self, 10)
    }
    #[doc = "Bit 11 - TCC2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcc2_(&mut self) -> Tcc2_W<ApbcmaskSpec> {
        Tcc2_W::new(self, 11)
    }
    #[doc = "Bit 12 - TC0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc0_(&mut self) -> Tc0_W<ApbcmaskSpec> {
        Tc0_W::new(self, 12)
    }
    #[doc = "Bit 13 - TC1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc1_(&mut self) -> Tc1_W<ApbcmaskSpec> {
        Tc1_W::new(self, 13)
    }
    #[doc = "Bit 14 - TC2 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc2_(&mut self) -> Tc2_W<ApbcmaskSpec> {
        Tc2_W::new(self, 14)
    }
    #[doc = "Bit 15 - TC3 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc3_(&mut self) -> Tc3_W<ApbcmaskSpec> {
        Tc3_W::new(self, 15)
    }
    #[doc = "Bit 16 - TC4 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tc4_(&mut self) -> Tc4_W<ApbcmaskSpec> {
        Tc4_W::new(self, 16)
    }
    #[doc = "Bit 17 - ADC0 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_(&mut self) -> Adc0_W<ApbcmaskSpec> {
        Adc0_W::new(self, 17)
    }
    #[doc = "Bit 18 - ADC1 APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1_(&mut self) -> Adc1_W<ApbcmaskSpec> {
        Adc1_W::new(self, 18)
    }
    #[doc = "Bit 19 - SDADC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdadc_(&mut self) -> Sdadc_W<ApbcmaskSpec> {
        Sdadc_W::new(self, 19)
    }
    #[doc = "Bit 20 - AC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ac_(&mut self) -> Ac_W<ApbcmaskSpec> {
        Ac_W::new(self, 20)
    }
    #[doc = "Bit 21 - DAC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dac_(&mut self) -> Dac_W<ApbcmaskSpec> {
        Dac_W::new(self, 21)
    }
    #[doc = "Bit 22 - PTC APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ptc_(&mut self) -> Ptc_W<ApbcmaskSpec> {
        Ptc_W::new(self, 22)
    }
    #[doc = "Bit 23 - CCL APB Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccl_(&mut self) -> Ccl_W<ApbcmaskSpec> {
        Ccl_W::new(self, 23)
    }
}
#[doc = "APBC Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`apbcmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbcmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApbcmaskSpec;
impl crate::RegisterSpec for ApbcmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbcmask::R`](R) reader structure"]
impl crate::Readable for ApbcmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`apbcmask::W`](W) writer structure"]
impl crate::Writable for ApbcmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APBCMASK to value 0"]
impl crate::Resettable for ApbcmaskSpec {
    const RESET_VALUE: u32 = 0;
}
