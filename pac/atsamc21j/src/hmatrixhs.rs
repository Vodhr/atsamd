#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mcfg: [Mcfg; 16],
    scfg: [Scfg; 16],
    prs: [Prs; 4],
    _reserved3: [u8; 0x60],
    mrcr: Mrcr,
    _reserved4: [u8; 0x0c],
    sfr: [Sfr; 16],
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - Master Configuration"]
    #[inline(always)]
    pub const fn mcfg(&self, n: usize) -> &Mcfg {
        &self.mcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - Master Configuration"]
    #[inline(always)]
    pub fn mcfg_iter(&self) -> impl Iterator<Item = &Mcfg> {
        self.mcfg.iter()
    }
    #[doc = "0x40..0x80 - Slave Configuration"]
    #[inline(always)]
    pub const fn scfg(&self, n: usize) -> &Scfg {
        &self.scfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x80 - Slave Configuration"]
    #[inline(always)]
    pub fn scfg_iter(&self) -> impl Iterator<Item = &Scfg> {
        self.scfg.iter()
    }
    #[doc = "0x80..0xa0 - PRS\\[%s\\]"]
    #[inline(always)]
    pub const fn prs(&self, n: usize) -> &Prs {
        &self.prs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xa0 - PRS\\[%s\\]"]
    #[inline(always)]
    pub fn prs_iter(&self) -> impl Iterator<Item = &Prs> {
        self.prs.iter()
    }
    #[doc = "0x100 - Master Remap Control"]
    #[inline(always)]
    pub const fn mrcr(&self) -> &Mrcr {
        &self.mrcr
    }
    #[doc = "0x110..0x150 - Special Function"]
    #[inline(always)]
    pub const fn sfr(&self, n: usize) -> &Sfr {
        &self.sfr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x110..0x150 - Special Function"]
    #[inline(always)]
    pub fn sfr_iter(&self) -> impl Iterator<Item = &Sfr> {
        self.sfr.iter()
    }
}
#[doc = "MCFG (rw) register accessor: Master Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`mcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg`]
module"]
#[doc(alias = "MCFG")]
pub type Mcfg = crate::Reg<mcfg::McfgSpec>;
#[doc = "Master Configuration"]
pub mod mcfg;
#[doc = "SCFG (rw) register accessor: Slave Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`scfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scfg`]
module"]
#[doc(alias = "SCFG")]
pub type Scfg = crate::Reg<scfg::ScfgSpec>;
#[doc = "Slave Configuration"]
pub mod scfg;
#[doc = "PRS\\[%s\\]"]
pub use self::prs::Prs;
#[doc = r"Cluster"]
#[doc = "PRS\\[%s\\]"]
pub mod prs;
#[doc = "MRCR (rw) register accessor: Master Remap Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrcr`]
module"]
#[doc(alias = "MRCR")]
pub type Mrcr = crate::Reg<mrcr::MrcrSpec>;
#[doc = "Master Remap Control"]
pub mod mrcr;
#[doc = "SFR (rw) register accessor: Special Function\n\nYou can [`read`](crate::Reg::read) this register and get [`sfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfr`]
module"]
#[doc(alias = "SFR")]
pub type Sfr = crate::Reg<sfr::SfrSpec>;
#[doc = "Special Function"]
pub mod sfr;
