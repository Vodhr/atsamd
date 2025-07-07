#[doc = "Register `STATUSA` reader"]
pub type R = crate::R<StatusaSpec>;
#[doc = "Field `STATE0` reader - Comparator 0 Current State"]
pub type State0R = crate::BitReader;
#[doc = "Field `STATE1` reader - Comparator 1 Current State"]
pub type State1R = crate::BitReader;
#[doc = "Field `STATE2` reader - Comparator 2 Current State"]
pub type State2R = crate::BitReader;
#[doc = "Field `STATE3` reader - Comparator 3 Current State"]
pub type State3R = crate::BitReader;
#[doc = "Window 0 Current State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wstate0select {
    #[doc = "0: Signal is above window"]
    Above = 0,
    #[doc = "1: Signal is inside window"]
    Inside = 1,
    #[doc = "2: Signal is below window"]
    Below = 2,
}
impl From<Wstate0select> for u8 {
    #[inline(always)]
    fn from(variant: Wstate0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wstate0select {
    type Ux = u8;
}
impl crate::IsEnum for Wstate0select {}
#[doc = "Field `WSTATE0` reader - Window 0 Current State"]
pub type Wstate0R = crate::FieldReader<Wstate0select>;
impl Wstate0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wstate0select> {
        match self.bits {
            0 => Some(Wstate0select::Above),
            1 => Some(Wstate0select::Inside),
            2 => Some(Wstate0select::Below),
            _ => None,
        }
    }
    #[doc = "Signal is above window"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == Wstate0select::Above
    }
    #[doc = "Signal is inside window"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == Wstate0select::Inside
    }
    #[doc = "Signal is below window"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == Wstate0select::Below
    }
}
#[doc = "Window 1 Current State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wstate1select {
    #[doc = "0: Signal is above window"]
    Above = 0,
    #[doc = "1: Signal is inside window"]
    Inside = 1,
    #[doc = "2: Signal is below window"]
    Below = 2,
}
impl From<Wstate1select> for u8 {
    #[inline(always)]
    fn from(variant: Wstate1select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wstate1select {
    type Ux = u8;
}
impl crate::IsEnum for Wstate1select {}
#[doc = "Field `WSTATE1` reader - Window 1 Current State"]
pub type Wstate1R = crate::FieldReader<Wstate1select>;
impl Wstate1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wstate1select> {
        match self.bits {
            0 => Some(Wstate1select::Above),
            1 => Some(Wstate1select::Inside),
            2 => Some(Wstate1select::Below),
            _ => None,
        }
    }
    #[doc = "Signal is above window"]
    #[inline(always)]
    pub fn is_above(&self) -> bool {
        *self == Wstate1select::Above
    }
    #[doc = "Signal is inside window"]
    #[inline(always)]
    pub fn is_inside(&self) -> bool {
        *self == Wstate1select::Inside
    }
    #[doc = "Signal is below window"]
    #[inline(always)]
    pub fn is_below(&self) -> bool {
        *self == Wstate1select::Below
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 0 Current State"]
    #[inline(always)]
    pub fn state0(&self) -> State0R {
        State0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator 1 Current State"]
    #[inline(always)]
    pub fn state1(&self) -> State1R {
        State1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator 2 Current State"]
    #[inline(always)]
    pub fn state2(&self) -> State2R {
        State2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator 3 Current State"]
    #[inline(always)]
    pub fn state3(&self) -> State3R {
        State3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Window 0 Current State"]
    #[inline(always)]
    pub fn wstate0(&self) -> Wstate0R {
        Wstate0R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Window 1 Current State"]
    #[inline(always)]
    pub fn wstate1(&self) -> Wstate1R {
        Wstate1R::new((self.bits >> 6) & 3)
    }
}
#[doc = "Status A\n\nYou can [`read`](crate::Reg::read) this register and get [`statusa::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusaSpec;
impl crate::RegisterSpec for StatusaSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`statusa::R`](R) reader structure"]
impl crate::Readable for StatusaSpec {}
#[doc = "`reset()` method sets STATUSA to value 0"]
impl crate::Resettable for StatusaSpec {
    const RESET_VALUE: u8 = 0;
}
