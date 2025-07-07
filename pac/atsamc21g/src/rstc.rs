#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rcause: Rcause,
}
impl RegisterBlock {
    #[doc = "0x00 - Reset Cause"]
    #[inline(always)]
    pub const fn rcause(&self) -> &Rcause {
        &self.rcause
    }
}
#[doc = "RCAUSE (r) register accessor: Reset Cause\n\nYou can [`read`](crate::Reg::read) this register and get [`rcause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rcause`]
module"]
#[doc(alias = "RCAUSE")]
pub type Rcause = crate::Reg<rcause::RcauseSpec>;
#[doc = "Reset Cause"]
pub mod rcause;
