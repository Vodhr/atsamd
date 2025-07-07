#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Unaligned accesses generates a Hard Fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnalignTrpselect {
    #[doc = "0: Do not trap unaligned halfword and word accesses"]
    Value0 = 0,
    #[doc = "1: Trap unaligned halfword and word accesses"]
    Value1 = 1,
}
impl From<UnalignTrpselect> for bool {
    #[inline(always)]
    fn from(variant: UnalignTrpselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UNALIGN_TRP` reader - Unaligned accesses generates a Hard Fault"]
pub type UnalignTrpR = crate::BitReader<UnalignTrpselect>;
impl UnalignTrpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UnalignTrpselect {
        match self.bits {
            false => UnalignTrpselect::Value0,
            true => UnalignTrpselect::Value1,
        }
    }
    #[doc = "Do not trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == UnalignTrpselect::Value0
    }
    #[doc = "Trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == UnalignTrpselect::Value1
    }
}
#[doc = "Stack 8-byte aligned on exception entry\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stkalignselect {
    #[doc = "0: 4-byte aligned"]
    Value0 = 0,
    #[doc = "1: 8-byte aligned"]
    Value1 = 1,
}
impl From<Stkalignselect> for bool {
    #[inline(always)]
    fn from(variant: Stkalignselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STKALIGN` reader - Stack 8-byte aligned on exception entry"]
pub type StkalignR = crate::BitReader<Stkalignselect>;
impl StkalignR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stkalignselect {
        match self.bits {
            false => Stkalignselect::Value0,
            true => Stkalignselect::Value1,
        }
    }
    #[doc = "4-byte aligned"]
    #[inline(always)]
    pub fn is_value_0(&self) -> bool {
        *self == Stkalignselect::Value0
    }
    #[doc = "8-byte aligned"]
    #[inline(always)]
    pub fn is_value_1(&self) -> bool {
        *self == Stkalignselect::Value1
    }
}
impl R {
    #[doc = "Bit 3 - Unaligned accesses generates a Hard Fault"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UnalignTrpR {
        UnalignTrpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 9 - Stack 8-byte aligned on exception entry"]
    #[inline(always)]
    pub fn stkalign(&self) -> StkalignR {
        StkalignR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Configuration and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`reset()` method sets CCR to value 0x0204"]
impl crate::Resettable for CcrSpec {
    const RESET_VALUE: u32 = 0x0204;
}
