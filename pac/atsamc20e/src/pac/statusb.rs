#[doc = "Register `STATUSB` reader"]
pub type R = crate::R<StatusbSpec>;
#[doc = "Field `PORT_` reader - PORT APB Protect Enable"]
pub type Port_R = crate::BitReader;
#[doc = "Field `DSU_` reader - DSU APB Protect Enable"]
pub type Dsu_R = crate::BitReader;
#[doc = "Field `NVMCTRL_` reader - NVMCTRL APB Protect Enable"]
pub type Nvmctrl_R = crate::BitReader;
#[doc = "Field `DMAC_` reader - DMAC APB Protect Enable"]
pub type Dmac_R = crate::BitReader;
#[doc = "Field `MTB_` reader - MTB APB Protect Enable"]
pub type Mtb_R = crate::BitReader;
#[doc = "Field `HMATRIXHS_` reader - HMATRIXHS APB Protect Enable"]
pub type Hmatrixhs_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - PORT APB Protect Enable"]
    #[inline(always)]
    pub fn port_(&self) -> Port_R {
        Port_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DSU APB Protect Enable"]
    #[inline(always)]
    pub fn dsu_(&self) -> Dsu_R {
        Dsu_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NVMCTRL APB Protect Enable"]
    #[inline(always)]
    pub fn nvmctrl_(&self) -> Nvmctrl_R {
        Nvmctrl_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAC APB Protect Enable"]
    #[inline(always)]
    pub fn dmac_(&self) -> Dmac_R {
        Dmac_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MTB APB Protect Enable"]
    #[inline(always)]
    pub fn mtb_(&self) -> Mtb_R {
        Mtb_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HMATRIXHS APB Protect Enable"]
    #[inline(always)]
    pub fn hmatrixhs_(&self) -> Hmatrixhs_R {
        Hmatrixhs_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Peripheral write protection status - Bridge B\n\nYou can [`read`](crate::Reg::read) this register and get [`statusb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusbSpec;
impl crate::RegisterSpec for StatusbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`statusb::R`](R) reader structure"]
impl crate::Readable for StatusbSpec {}
#[doc = "`reset()` method sets STATUSB to value 0x02"]
impl crate::Resettable for StatusbSpec {
    const RESET_VALUE: u32 = 0x02;
}
