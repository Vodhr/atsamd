#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x01],
    sleepcfg: Sleepcfg,
    _reserved1: [u8; 0x06],
    stdbycfg: Stdbycfg,
}
impl RegisterBlock {
    #[doc = "0x01 - Sleep Configuration"]
    #[inline(always)]
    pub const fn sleepcfg(&self) -> &Sleepcfg {
        &self.sleepcfg
    }
    #[doc = "0x08 - Standby Configuration"]
    #[inline(always)]
    pub const fn stdbycfg(&self) -> &Stdbycfg {
        &self.stdbycfg
    }
}
#[doc = "SLEEPCFG (rw) register accessor: Sleep Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sleepcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleepcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sleepcfg`]
module"]
#[doc(alias = "SLEEPCFG")]
pub type Sleepcfg = crate::Reg<sleepcfg::SleepcfgSpec>;
#[doc = "Sleep Configuration"]
pub mod sleepcfg;
#[doc = "STDBYCFG (rw) register accessor: Standby Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`stdbycfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stdbycfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stdbycfg`]
module"]
#[doc(alias = "STDBYCFG")]
pub type Stdbycfg = crate::Reg<stdbycfg::StdbycfgSpec>;
#[doc = "Standby Configuration"]
pub mod stdbycfg;
