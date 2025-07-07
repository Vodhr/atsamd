#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `BODVDDRDY` reader - BODVDD Ready"]
pub type BodvddrdyR = crate::BitReader;
#[doc = "Field `BODVDDRDY` writer - BODVDD Ready"]
pub type BodvddrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODVDDDET` reader - BODVDD Detection"]
pub type BodvdddetR = crate::BitReader;
#[doc = "Field `BODVDDDET` writer - BODVDD Detection"]
pub type BodvdddetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BVDDSRDY` reader - BODVDD Synchronization Ready"]
pub type BvddsrdyR = crate::BitReader;
#[doc = "Field `BVDDSRDY` writer - BODVDD Synchronization Ready"]
pub type BvddsrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODCORERDY` reader - BODCORE Ready"]
pub type BodcorerdyR = crate::BitReader;
#[doc = "Field `BODCORERDY` writer - BODCORE Ready"]
pub type BodcorerdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BODCOREDET` reader - BODCORE Detection"]
pub type BodcoredetR = crate::BitReader;
#[doc = "Field `BODCOREDET` writer - BODCORE Detection"]
pub type BodcoredetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCORESRDY` reader - BODCORE Synchronization Ready"]
pub type BcoresrdyR = crate::BitReader;
#[doc = "Field `BCORESRDY` writer - BODCORE Synchronization Ready"]
pub type BcoresrdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BODVDD Ready"]
    #[inline(always)]
    pub fn bodvddrdy(&self) -> BodvddrdyR {
        BodvddrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BODVDD Detection"]
    #[inline(always)]
    pub fn bodvdddet(&self) -> BodvdddetR {
        BodvdddetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BODVDD Synchronization Ready"]
    #[inline(always)]
    pub fn bvddsrdy(&self) -> BvddsrdyR {
        BvddsrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BODCORE Ready"]
    #[inline(always)]
    pub fn bodcorerdy(&self) -> BodcorerdyR {
        BodcorerdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BODCORE Detection"]
    #[inline(always)]
    pub fn bodcoredet(&self) -> BodcoredetR {
        BodcoredetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - BODCORE Synchronization Ready"]
    #[inline(always)]
    pub fn bcoresrdy(&self) -> BcoresrdyR {
        BcoresrdyR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BODVDD Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bodvddrdy(&mut self) -> BodvddrdyW<IntensetSpec> {
        BodvddrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - BODVDD Detection"]
    #[inline(always)]
    #[must_use]
    pub fn bodvdddet(&mut self) -> BodvdddetW<IntensetSpec> {
        BodvdddetW::new(self, 1)
    }
    #[doc = "Bit 2 - BODVDD Synchronization Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bvddsrdy(&mut self) -> BvddsrdyW<IntensetSpec> {
        BvddsrdyW::new(self, 2)
    }
    #[doc = "Bit 3 - BODCORE Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bodcorerdy(&mut self) -> BodcorerdyW<IntensetSpec> {
        BodcorerdyW::new(self, 3)
    }
    #[doc = "Bit 4 - BODCORE Detection"]
    #[inline(always)]
    #[must_use]
    pub fn bodcoredet(&mut self) -> BodcoredetW<IntensetSpec> {
        BodcoredetW::new(self, 4)
    }
    #[doc = "Bit 5 - BODCORE Synchronization Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bcoresrdy(&mut self) -> BcoresrdyW<IntensetSpec> {
        BcoresrdyW::new(self, 5)
    }
}
#[doc = "Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u32 = 0;
}
