#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
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
#[doc = "Field `VREG33RDY` reader - VREG33 Ready"]
pub type Vreg33rdyR = crate::BitReader;
#[doc = "Field `VREG33RDY` writer - VREG33 Ready"]
pub type Vreg33rdyW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 6 - VREG33 Ready"]
    #[inline(always)]
    pub fn vreg33rdy(&self) -> Vreg33rdyR {
        Vreg33rdyR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BODVDD Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bodvddrdy(&mut self) -> BodvddrdyW<IntflagSpec> {
        BodvddrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - BODVDD Detection"]
    #[inline(always)]
    #[must_use]
    pub fn bodvdddet(&mut self) -> BodvdddetW<IntflagSpec> {
        BodvdddetW::new(self, 1)
    }
    #[doc = "Bit 2 - BODVDD Synchronization Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bvddsrdy(&mut self) -> BvddsrdyW<IntflagSpec> {
        BvddsrdyW::new(self, 2)
    }
    #[doc = "Bit 6 - VREG33 Ready"]
    #[inline(always)]
    #[must_use]
    pub fn vreg33rdy(&mut self) -> Vreg33rdyW<IntflagSpec> {
        Vreg33rdyW::new(self, 6)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {
    const RESET_VALUE: u32 = 0;
}
