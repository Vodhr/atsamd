#[doc = "Register `CAL48M` reader"]
pub type R = crate::R<Cal48mSpec>;
#[doc = "Register `CAL48M` writer"]
pub type W = crate::W<Cal48mSpec>;
#[doc = "Field `FCAL` reader - Frequency Calibration (48MHz)"]
pub type FcalR = crate::FieldReader;
#[doc = "Field `FCAL` writer - Frequency Calibration (48MHz)"]
pub type FcalW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FRANGE` reader - Frequency Range (48MHz)"]
pub type FrangeR = crate::FieldReader;
#[doc = "Field `FRANGE` writer - Frequency Range (48MHz)"]
pub type FrangeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TCAL` reader - Temperature Calibration (48MHz)"]
pub type TcalR = crate::FieldReader;
#[doc = "Field `TCAL` writer - Temperature Calibration (48MHz)"]
pub type TcalW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Frequency Calibration (48MHz)"]
    #[inline(always)]
    pub fn fcal(&self) -> FcalR {
        FcalR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - Frequency Range (48MHz)"]
    #[inline(always)]
    pub fn frange(&self) -> FrangeR {
        FrangeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:21 - Temperature Calibration (48MHz)"]
    #[inline(always)]
    pub fn tcal(&self) -> TcalR {
        TcalR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Frequency Calibration (48MHz)"]
    #[inline(always)]
    #[must_use]
    pub fn fcal(&mut self) -> FcalW<Cal48mSpec> {
        FcalW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Frequency Range (48MHz)"]
    #[inline(always)]
    #[must_use]
    pub fn frange(&mut self) -> FrangeW<Cal48mSpec> {
        FrangeW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Temperature Calibration (48MHz)"]
    #[inline(always)]
    #[must_use]
    pub fn tcal(&mut self) -> TcalW<Cal48mSpec> {
        TcalW::new(self, 16)
    }
}
#[doc = "48MHz Oscillator Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`cal48m::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal48m::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cal48mSpec;
impl crate::RegisterSpec for Cal48mSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal48m::R`](R) reader structure"]
impl crate::Readable for Cal48mSpec {}
#[doc = "`write(|w| ..)` method takes [`cal48m::W`](W) writer structure"]
impl crate::Writable for Cal48mSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAL48M to value 0"]
impl crate::Resettable for Cal48mSpec {
    const RESET_VALUE: u32 = 0;
}
