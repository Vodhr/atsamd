#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `COMP0` reader - Comparator 0"]
pub type Comp0R = crate::BitReader;
#[doc = "Field `COMP0` writer - Comparator 0"]
pub type Comp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP1` reader - Comparator 1"]
pub type Comp1R = crate::BitReader;
#[doc = "Field `COMP1` writer - Comparator 1"]
pub type Comp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP2` reader - Comparator 2"]
pub type Comp2R = crate::BitReader;
#[doc = "Field `COMP2` writer - Comparator 2"]
pub type Comp2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP3` reader - Comparator 3"]
pub type Comp3R = crate::BitReader;
#[doc = "Field `COMP3` writer - Comparator 3"]
pub type Comp3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN0` reader - Window 0"]
pub type Win0R = crate::BitReader;
#[doc = "Field `WIN0` writer - Window 0"]
pub type Win0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN1` reader - Window 1"]
pub type Win1R = crate::BitReader;
#[doc = "Field `WIN1` writer - Window 1"]
pub type Win1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Comparator 0"]
    #[inline(always)]
    pub fn comp0(&self) -> Comp0R {
        Comp0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1"]
    #[inline(always)]
    pub fn comp1(&self) -> Comp1R {
        Comp1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator 2"]
    #[inline(always)]
    pub fn comp2(&self) -> Comp2R {
        Comp2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator 3"]
    #[inline(always)]
    pub fn comp3(&self) -> Comp3R {
        Comp3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Window 0"]
    #[inline(always)]
    pub fn win0(&self) -> Win0R {
        Win0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Window 1"]
    #[inline(always)]
    pub fn win1(&self) -> Win1R {
        Win1R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 0"]
    #[inline(always)]
    #[must_use]
    pub fn comp0(&mut self) -> Comp0W<IntflagSpec> {
        Comp0W::new(self, 0)
    }
    #[doc = "Bit 1 - Comparator 1"]
    #[inline(always)]
    #[must_use]
    pub fn comp1(&mut self) -> Comp1W<IntflagSpec> {
        Comp1W::new(self, 1)
    }
    #[doc = "Bit 2 - Comparator 2"]
    #[inline(always)]
    #[must_use]
    pub fn comp2(&mut self) -> Comp2W<IntflagSpec> {
        Comp2W::new(self, 2)
    }
    #[doc = "Bit 3 - Comparator 3"]
    #[inline(always)]
    #[must_use]
    pub fn comp3(&mut self) -> Comp3W<IntflagSpec> {
        Comp3W::new(self, 3)
    }
    #[doc = "Bit 4 - Window 0"]
    #[inline(always)]
    #[must_use]
    pub fn win0(&mut self) -> Win0W<IntflagSpec> {
        Win0W::new(self, 4)
    }
    #[doc = "Bit 5 - Window 1"]
    #[inline(always)]
    #[must_use]
    pub fn win1(&mut self) -> Win1W<IntflagSpec> {
        Win1W::new(self, 5)
    }
}
#[doc = "Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {
    const RESET_VALUE: u8 = 0;
}
