#[doc = "Register `STATUSB` reader"]
pub type R = crate::R<StatusbSpec>;
#[doc = "Field `READY0` reader - Comparator 0 Ready"]
pub type Ready0R = crate::BitReader;
#[doc = "Field `READY1` reader - Comparator 1 Ready"]
pub type Ready1R = crate::BitReader;
#[doc = "Field `READY2` reader - Comparator 2 Ready"]
pub type Ready2R = crate::BitReader;
#[doc = "Field `READY3` reader - Comparator 3 Ready"]
pub type Ready3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Comparator 0 Ready"]
    #[inline(always)]
    pub fn ready0(&self) -> Ready0R {
        Ready0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Ready"]
    #[inline(always)]
    pub fn ready1(&self) -> Ready1R {
        Ready1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator 2 Ready"]
    #[inline(always)]
    pub fn ready2(&self) -> Ready2R {
        Ready2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator 3 Ready"]
    #[inline(always)]
    pub fn ready3(&self) -> Ready3R {
        Ready3R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Status B\n\nYou can [`read`](crate::Reg::read) this register and get [`statusb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusbSpec;
impl crate::RegisterSpec for StatusbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`statusb::R`](R) reader structure"]
impl crate::Readable for StatusbSpec {}
#[doc = "`reset()` method sets STATUSB to value 0"]
impl crate::Resettable for StatusbSpec {
    const RESET_VALUE: u8 = 0;
}
