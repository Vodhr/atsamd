#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ctrla: Ctrla,
    _reserved1: [u8; 0x03],
    status: Status,
    _reserved2: [u8; 0x03],
    dividend: Dividend,
    divisor: Divisor,
    result: Result,
    rem: Rem,
    sqrnum: Sqrnum,
}
impl RegisterBlock {
    #[doc = "0x00 - Control"]
    #[inline(always)]
    pub const fn ctrla(&self) -> &Ctrla {
        &self.ctrla
    }
    #[doc = "0x04 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - Dividend"]
    #[inline(always)]
    pub const fn dividend(&self) -> &Dividend {
        &self.dividend
    }
    #[doc = "0x0c - Divisor"]
    #[inline(always)]
    pub const fn divisor(&self) -> &Divisor {
        &self.divisor
    }
    #[doc = "0x10 - Result"]
    #[inline(always)]
    pub const fn result(&self) -> &Result {
        &self.result
    }
    #[doc = "0x14 - Remainder"]
    #[inline(always)]
    pub const fn rem(&self) -> &Rem {
        &self.rem
    }
    #[doc = "0x18 - Square Root Input"]
    #[inline(always)]
    pub const fn sqrnum(&self) -> &Sqrnum {
        &self.sqrnum
    }
}
#[doc = "CTRLA (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrla`]
module"]
#[doc(alias = "CTRLA")]
pub type Ctrla = crate::Reg<ctrla::CtrlaSpec>;
#[doc = "Control"]
pub mod ctrla;
#[doc = "STATUS (rw) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "DIVIDEND (rw) register accessor: Dividend\n\nYou can [`read`](crate::Reg::read) this register and get [`dividend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dividend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dividend`]
module"]
#[doc(alias = "DIVIDEND")]
pub type Dividend = crate::Reg<dividend::DividendSpec>;
#[doc = "Dividend"]
pub mod dividend;
#[doc = "DIVISOR (rw) register accessor: Divisor\n\nYou can [`read`](crate::Reg::read) this register and get [`divisor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`divisor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@divisor`]
module"]
#[doc(alias = "DIVISOR")]
pub type Divisor = crate::Reg<divisor::DivisorSpec>;
#[doc = "Divisor"]
pub mod divisor;
#[doc = "RESULT (r) register accessor: Result\n\nYou can [`read`](crate::Reg::read) this register and get [`result::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@result`]
module"]
#[doc(alias = "RESULT")]
pub type Result = crate::Reg<result::ResultSpec>;
#[doc = "Result"]
pub mod result;
#[doc = "REM (r) register accessor: Remainder\n\nYou can [`read`](crate::Reg::read) this register and get [`rem::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rem`]
module"]
#[doc(alias = "REM")]
pub type Rem = crate::Reg<rem::RemSpec>;
#[doc = "Remainder"]
pub mod rem;
#[doc = "SQRNUM (rw) register accessor: Square Root Input\n\nYou can [`read`](crate::Reg::read) this register and get [`sqrnum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqrnum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqrnum`]
module"]
#[doc(alias = "SQRNUM")]
pub type Sqrnum = crate::Reg<sqrnum::SqrnumSpec>;
#[doc = "Square Root Input"]
pub mod sqrnum;
