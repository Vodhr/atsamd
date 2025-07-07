#[doc = "Register `CAL` reader"]
pub type R = crate::R<CalSpec>;
#[doc = "Register `CAL` writer"]
pub type W = crate::W<CalSpec>;
#[doc = "Field `FCAL` reader - Frequency Calibration"]
pub type FcalR = crate::FieldReader;
#[doc = "Field `FCAL` writer - Frequency Calibration"]
pub type FcalW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TCAL` reader - Temperature Calibration"]
pub type TcalR = crate::FieldReader;
#[doc = "Field `TCAL` writer - Temperature Calibration"]
pub type TcalW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Frequency Calibration"]
    #[inline(always)]
    pub fn fcal(&self) -> FcalR {
        FcalR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Temperature Calibration"]
    #[inline(always)]
    pub fn tcal(&self) -> TcalR {
        TcalR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Frequency Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn fcal(&mut self) -> FcalW<CalSpec> {
        FcalW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Temperature Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn tcal(&mut self) -> TcalW<CalSpec> {
        TcalW::new(self, 8)
    }
}
#[doc = "Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalSpec;
impl crate::RegisterSpec for CalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal::R`](R) reader structure"]
impl crate::Readable for CalSpec {}
#[doc = "`write(|w| ..)` method takes [`cal::W`](W) writer structure"]
impl crate::Writable for CalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAL to value 0"]
impl crate::Resettable for CalSpec {
    const RESET_VALUE: u32 = 0;
}
