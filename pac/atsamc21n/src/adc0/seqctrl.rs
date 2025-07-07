#[doc = "Register `SEQCTRL` reader"]
pub type R = crate::R<SeqctrlSpec>;
#[doc = "Register `SEQCTRL` writer"]
pub type W = crate::W<SeqctrlSpec>;
#[doc = "Field `SEQEN` reader - Enable Positive Input in the Sequence"]
pub type SeqenR = crate::FieldReader<u32>;
#[doc = "Field `SEQEN` writer - Enable Positive Input in the Sequence"]
pub type SeqenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Enable Positive Input in the Sequence"]
    #[inline(always)]
    pub fn seqen(&self) -> SeqenR {
        SeqenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Enable Positive Input in the Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn seqen(&mut self) -> SeqenW<SeqctrlSpec> {
        SeqenW::new(self, 0)
    }
}
#[doc = "Sequence Control\n\nYou can [`read`](crate::Reg::read) this register and get [`seqctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seqctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqctrlSpec;
impl crate::RegisterSpec for SeqctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`seqctrl::R`](R) reader structure"]
impl crate::Readable for SeqctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`seqctrl::W`](W) writer structure"]
impl crate::Writable for SeqctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEQCTRL to value 0"]
impl crate::Resettable for SeqctrlSpec {
    const RESET_VALUE: u32 = 0;
}
