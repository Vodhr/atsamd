#[doc = "Register `SEQSTATUS` reader"]
pub type R = crate::R<SeqstatusSpec>;
#[doc = "Field `SEQSTATE` reader - Sequence State"]
pub type SeqstateR = crate::FieldReader;
#[doc = "Field `SEQBUSY` reader - Sequence Busy"]
pub type SeqbusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Sequence State"]
    #[inline(always)]
    pub fn seqstate(&self) -> SeqstateR {
        SeqstateR::new(self.bits & 0x1f)
    }
    #[doc = "Bit 7 - Sequence Busy"]
    #[inline(always)]
    pub fn seqbusy(&self) -> SeqbusyR {
        SeqbusyR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Sequence Status\n\nYou can [`read`](crate::Reg::read) this register and get [`seqstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SeqstatusSpec;
impl crate::RegisterSpec for SeqstatusSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`seqstatus::R`](R) reader structure"]
impl crate::Readable for SeqstatusSpec {}
#[doc = "`reset()` method sets SEQSTATUS to value 0"]
impl crate::Resettable for SeqstatusSpec {
    const RESET_VALUE: u8 = 0;
}
