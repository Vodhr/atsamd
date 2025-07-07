#[doc = "Register `EVCTRL` reader"]
pub type R = crate::R<EvctrlSpec>;
#[doc = "Register `EVCTRL` writer"]
pub type W = crate::W<EvctrlSpec>;
#[doc = "Field `COMPEO0` reader - Comparator 0 Event Output Enable"]
pub type Compeo0R = crate::BitReader;
#[doc = "Field `COMPEO0` writer - Comparator 0 Event Output Enable"]
pub type Compeo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEO1` reader - Comparator 1 Event Output Enable"]
pub type Compeo1R = crate::BitReader;
#[doc = "Field `COMPEO1` writer - Comparator 1 Event Output Enable"]
pub type Compeo1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEO2` reader - Comparator 2 Event Output Enable"]
pub type Compeo2R = crate::BitReader;
#[doc = "Field `COMPEO2` writer - Comparator 2 Event Output Enable"]
pub type Compeo2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEO3` reader - Comparator 3 Event Output Enable"]
pub type Compeo3R = crate::BitReader;
#[doc = "Field `COMPEO3` writer - Comparator 3 Event Output Enable"]
pub type Compeo3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINEO0` reader - Window 0 Event Output Enable"]
pub type Wineo0R = crate::BitReader;
#[doc = "Field `WINEO0` writer - Window 0 Event Output Enable"]
pub type Wineo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WINEO1` reader - Window 1 Event Output Enable"]
pub type Wineo1R = crate::BitReader;
#[doc = "Field `WINEO1` writer - Window 1 Event Output Enable"]
pub type Wineo1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEI0` reader - Comparator 0 Event Input Enable"]
pub type Compei0R = crate::BitReader;
#[doc = "Field `COMPEI0` writer - Comparator 0 Event Input Enable"]
pub type Compei0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEI1` reader - Comparator 1 Event Input Enable"]
pub type Compei1R = crate::BitReader;
#[doc = "Field `COMPEI1` writer - Comparator 1 Event Input Enable"]
pub type Compei1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEI2` reader - Comparator 2 Event Input Enable"]
pub type Compei2R = crate::BitReader;
#[doc = "Field `COMPEI2` writer - Comparator 2 Event Input Enable"]
pub type Compei2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEI3` reader - Comparator 3 Event Input Enable"]
pub type Compei3R = crate::BitReader;
#[doc = "Field `COMPEI3` writer - Comparator 3 Event Input Enable"]
pub type Compei3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVEI0` reader - Comparator 0 Input Event Invert Enable"]
pub type Invei0R = crate::BitReader;
#[doc = "Field `INVEI0` writer - Comparator 0 Input Event Invert Enable"]
pub type Invei0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVEI1` reader - Comparator 1 Input Event Invert Enable"]
pub type Invei1R = crate::BitReader;
#[doc = "Field `INVEI1` writer - Comparator 1 Input Event Invert Enable"]
pub type Invei1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVEI2` reader - Comparator 2 Input Event Invert Enable"]
pub type Invei2R = crate::BitReader;
#[doc = "Field `INVEI2` writer - Comparator 2 Input Event Invert Enable"]
pub type Invei2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVEI3` reader - Comparator 3 Input Event Invert Enable"]
pub type Invei3R = crate::BitReader;
#[doc = "Field `INVEI3` writer - Comparator 3 Input Event Invert Enable"]
pub type Invei3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 0 Event Output Enable"]
    #[inline(always)]
    pub fn compeo0(&self) -> Compeo0R {
        Compeo0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Event Output Enable"]
    #[inline(always)]
    pub fn compeo1(&self) -> Compeo1R {
        Compeo1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator 2 Event Output Enable"]
    #[inline(always)]
    pub fn compeo2(&self) -> Compeo2R {
        Compeo2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator 3 Event Output Enable"]
    #[inline(always)]
    pub fn compeo3(&self) -> Compeo3R {
        Compeo3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Window 0 Event Output Enable"]
    #[inline(always)]
    pub fn wineo0(&self) -> Wineo0R {
        Wineo0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Window 1 Event Output Enable"]
    #[inline(always)]
    pub fn wineo1(&self) -> Wineo1R {
        Wineo1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Comparator 0 Event Input Enable"]
    #[inline(always)]
    pub fn compei0(&self) -> Compei0R {
        Compei0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator 1 Event Input Enable"]
    #[inline(always)]
    pub fn compei1(&self) -> Compei1R {
        Compei1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Comparator 2 Event Input Enable"]
    #[inline(always)]
    pub fn compei2(&self) -> Compei2R {
        Compei2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Comparator 3 Event Input Enable"]
    #[inline(always)]
    pub fn compei3(&self) -> Compei3R {
        Compei3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Comparator 0 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei0(&self) -> Invei0R {
        Invei0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Comparator 1 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei1(&self) -> Invei1R {
        Invei1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Comparator 2 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei2(&self) -> Invei2R {
        Invei2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Comparator 3 Input Event Invert Enable"]
    #[inline(always)]
    pub fn invei3(&self) -> Invei3R {
        Invei3R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compeo0(&mut self) -> Compeo0W<EvctrlSpec> {
        Compeo0W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 1 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compeo1(&mut self) -> Compeo1W<EvctrlSpec> {
        Compeo1W::new(self, 1)
    }
    #[doc = "Bit 2 - Comparator 2 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compeo2(&mut self) -> Compeo2W<EvctrlSpec> {
        Compeo2W::new(self, 2)
    }
    #[doc = "Bit 3 - Comparator 3 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compeo3(&mut self) -> Compeo3W<EvctrlSpec> {
        Compeo3W::new(self, 3)
    }
    #[doc = "Bit 4 - Window 0 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wineo0(&mut self) -> Wineo0W<EvctrlSpec> {
        Wineo0W::new(self, 4)
    }
    #[doc = "Bit 5 - Window 1 Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wineo1(&mut self) -> Wineo1W<EvctrlSpec> {
        Wineo1W::new(self, 5)
    }
    #[doc = "Bit 8 - Comparator 0 Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compei0(&mut self) -> Compei0W<EvctrlSpec> {
        Compei0W::new(self, 8)
    }
    #[doc = "Bit 9 - Comparator 1 Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compei1(&mut self) -> Compei1W<EvctrlSpec> {
        Compei1W::new(self, 9)
    }
    #[doc = "Bit 10 - Comparator 2 Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compei2(&mut self) -> Compei2W<EvctrlSpec> {
        Compei2W::new(self, 10)
    }
    #[doc = "Bit 11 - Comparator 3 Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn compei3(&mut self) -> Compei3W<EvctrlSpec> {
        Compei3W::new(self, 11)
    }
    #[doc = "Bit 12 - Comparator 0 Input Event Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn invei0(&mut self) -> Invei0W<EvctrlSpec> {
        Invei0W::new(self, 12)
    }
    #[doc = "Bit 13 - Comparator 1 Input Event Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn invei1(&mut self) -> Invei1W<EvctrlSpec> {
        Invei1W::new(self, 13)
    }
    #[doc = "Bit 14 - Comparator 2 Input Event Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn invei2(&mut self) -> Invei2W<EvctrlSpec> {
        Invei2W::new(self, 14)
    }
    #[doc = "Bit 15 - Comparator 3 Input Event Invert Enable"]
    #[inline(always)]
    #[must_use]
    pub fn invei3(&mut self) -> Invei3W<EvctrlSpec> {
        Invei3W::new(self, 15)
    }
}
#[doc = "Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EvctrlSpec;
impl crate::RegisterSpec for EvctrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`evctrl::R`](R) reader structure"]
impl crate::Readable for EvctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`evctrl::W`](W) writer structure"]
impl crate::Writable for EvctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets EVCTRL to value 0"]
impl crate::Resettable for EvctrlSpec {
    const RESET_VALUE: u16 = 0;
}
