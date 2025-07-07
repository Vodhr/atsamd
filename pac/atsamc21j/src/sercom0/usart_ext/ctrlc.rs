#[doc = "Register `CTRLC` reader"]
pub type R = crate::R<CtrlcSpec>;
#[doc = "Register `CTRLC` writer"]
pub type W = crate::W<CtrlcSpec>;
#[doc = "Field `GTIME` reader - RS485 Guard Time"]
pub type GtimeR = crate::FieldReader;
#[doc = "Field `GTIME` writer - RS485 Guard Time"]
pub type GtimeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BRKLEN` reader - LIN Master Break Length"]
pub type BrklenR = crate::FieldReader;
#[doc = "Field `BRKLEN` writer - LIN Master Break Length"]
pub type BrklenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HDRDLY` reader - LIN Master Header Delay"]
pub type HdrdlyR = crate::FieldReader;
#[doc = "Field `HDRDLY` writer - LIN Master Header Delay"]
pub type HdrdlyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - RS485 Guard Time"]
    #[inline(always)]
    pub fn gtime(&self) -> GtimeR {
        GtimeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - LIN Master Break Length"]
    #[inline(always)]
    pub fn brklen(&self) -> BrklenR {
        BrklenR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - LIN Master Header Delay"]
    #[inline(always)]
    pub fn hdrdly(&self) -> HdrdlyR {
        HdrdlyR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - RS485 Guard Time"]
    #[inline(always)]
    #[must_use]
    pub fn gtime(&mut self) -> GtimeW<CtrlcSpec> {
        GtimeW::new(self, 0)
    }
    #[doc = "Bits 8:9 - LIN Master Break Length"]
    #[inline(always)]
    #[must_use]
    pub fn brklen(&mut self) -> BrklenW<CtrlcSpec> {
        BrklenW::new(self, 8)
    }
    #[doc = "Bits 10:11 - LIN Master Header Delay"]
    #[inline(always)]
    #[must_use]
    pub fn hdrdly(&mut self) -> HdrdlyW<CtrlcSpec> {
        HdrdlyW::new(self, 10)
    }
}
#[doc = "USART_EXT Control C\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlcSpec;
impl crate::RegisterSpec for CtrlcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlc::R`](R) reader structure"]
impl crate::Readable for CtrlcSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlc::W`](W) writer structure"]
impl crate::Writable for CtrlcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRLC to value 0"]
impl crate::Resettable for CtrlcSpec {
    const RESET_VALUE: u32 = 0;
}
