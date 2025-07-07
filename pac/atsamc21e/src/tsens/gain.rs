#[doc = "Register `GAIN` reader"]
pub type R = crate::R<GainSpec>;
#[doc = "Register `GAIN` writer"]
pub type W = crate::W<GainSpec>;
#[doc = "Field `GAIN` reader - Time Amplifier Gain"]
pub type GainR = crate::FieldReader<u32>;
#[doc = "Field `GAIN` writer - Time Amplifier Gain"]
pub type GainW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Time Amplifier Gain"]
    #[inline(always)]
    pub fn gain(&self) -> GainR {
        GainR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Time Amplifier Gain"]
    #[inline(always)]
    #[must_use]
    pub fn gain(&mut self) -> GainW<GainSpec> {
        GainW::new(self, 0)
    }
}
#[doc = "Gain Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gain::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gain::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GainSpec;
impl crate::RegisterSpec for GainSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gain::R`](R) reader structure"]
impl crate::Readable for GainSpec {}
#[doc = "`write(|w| ..)` method takes [`gain::W`](W) writer structure"]
impl crate::Writable for GainSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GAIN to value 0"]
impl crate::Resettable for GainSpec {
    const RESET_VALUE: u32 = 0;
}
