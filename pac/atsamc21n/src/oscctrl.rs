#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intenclr: Intenclr,
    intenset: Intenset,
    intflag: Intflag,
    status: Status,
    xoscctrl: Xoscctrl,
    cfdpresc: Cfdpresc,
    evctrl: Evctrl,
    osc48mctrl: Osc48mctrl,
    osc48mdiv: Osc48mdiv,
    osc48mstup: Osc48mstup,
    _reserved10: [u8; 0x01],
    osc48msyncbusy: Osc48msyncbusy,
    dpllctrla: Dpllctrla,
    _reserved12: [u8; 0x03],
    dpllratio: Dpllratio,
    dpllctrlb: Dpllctrlb,
    dpllpresc: Dpllpresc,
    _reserved15: [u8; 0x03],
    dpllsyncbusy: Dpllsyncbusy,
    _reserved16: [u8; 0x03],
    dpllstatus: Dpllstatus,
    _reserved17: [u8; 0x07],
    cal48m: Cal48m,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Enable Clear"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x04 - Interrupt Enable Set"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x08 - Interrupt Flag Status and Clear"]
    #[inline(always)]
    pub const fn intflag(&self) -> &Intflag {
        &self.intflag
    }
    #[doc = "0x0c - Power and Clocks Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - External Multipurpose Crystal Oscillator (XOSC) Control"]
    #[inline(always)]
    pub const fn xoscctrl(&self) -> &Xoscctrl {
        &self.xoscctrl
    }
    #[doc = "0x12 - Clock Failure Detector Prescaler"]
    #[inline(always)]
    pub const fn cfdpresc(&self) -> &Cfdpresc {
        &self.cfdpresc
    }
    #[doc = "0x13 - Event Control"]
    #[inline(always)]
    pub const fn evctrl(&self) -> &Evctrl {
        &self.evctrl
    }
    #[doc = "0x14 - 48MHz Internal Oscillator (OSC48M) Control"]
    #[inline(always)]
    pub const fn osc48mctrl(&self) -> &Osc48mctrl {
        &self.osc48mctrl
    }
    #[doc = "0x15 - OSC48M Divider"]
    #[inline(always)]
    pub const fn osc48mdiv(&self) -> &Osc48mdiv {
        &self.osc48mdiv
    }
    #[doc = "0x16 - OSC48M Startup Time"]
    #[inline(always)]
    pub const fn osc48mstup(&self) -> &Osc48mstup {
        &self.osc48mstup
    }
    #[doc = "0x18 - OSC48M Synchronization Busy"]
    #[inline(always)]
    pub const fn osc48msyncbusy(&self) -> &Osc48msyncbusy {
        &self.osc48msyncbusy
    }
    #[doc = "0x1c - DPLL Control"]
    #[inline(always)]
    pub const fn dpllctrla(&self) -> &Dpllctrla {
        &self.dpllctrla
    }
    #[doc = "0x20 - DPLL Ratio Control"]
    #[inline(always)]
    pub const fn dpllratio(&self) -> &Dpllratio {
        &self.dpllratio
    }
    #[doc = "0x24 - Digital Core Configuration"]
    #[inline(always)]
    pub const fn dpllctrlb(&self) -> &Dpllctrlb {
        &self.dpllctrlb
    }
    #[doc = "0x28 - DPLL Prescaler"]
    #[inline(always)]
    pub const fn dpllpresc(&self) -> &Dpllpresc {
        &self.dpllpresc
    }
    #[doc = "0x2c - DPLL Synchronization Busy"]
    #[inline(always)]
    pub const fn dpllsyncbusy(&self) -> &Dpllsyncbusy {
        &self.dpllsyncbusy
    }
    #[doc = "0x30 - DPLL Status"]
    #[inline(always)]
    pub const fn dpllstatus(&self) -> &Dpllstatus {
        &self.dpllstatus
    }
    #[doc = "0x38 - 48MHz Oscillator Calibration"]
    #[inline(always)]
    pub const fn cal48m(&self) -> &Cal48m {
        &self.cal48m
    }
}
#[doc = "INTENCLR (rw) register accessor: Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Interrupt Enable Clear"]
pub mod intenclr;
#[doc = "INTENSET (rw) register accessor: Interrupt Enable Set\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Interrupt Enable Set"]
pub mod intenset;
#[doc = "INTFLAG (rw) register accessor: Interrupt Flag Status and Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intflag`]
module"]
#[doc(alias = "INTFLAG")]
pub type Intflag = crate::Reg<intflag::IntflagSpec>;
#[doc = "Interrupt Flag Status and Clear"]
pub mod intflag;
#[doc = "STATUS (r) register accessor: Power and Clocks Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Power and Clocks Status"]
pub mod status;
#[doc = "XOSCCTRL (rw) register accessor: External Multipurpose Crystal Oscillator (XOSC) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`xoscctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xoscctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xoscctrl`]
module"]
#[doc(alias = "XOSCCTRL")]
pub type Xoscctrl = crate::Reg<xoscctrl::XoscctrlSpec>;
#[doc = "External Multipurpose Crystal Oscillator (XOSC) Control"]
pub mod xoscctrl;
#[doc = "CFDPRESC (rw) register accessor: Clock Failure Detector Prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`cfdpresc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfdpresc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfdpresc`]
module"]
#[doc(alias = "CFDPRESC")]
pub type Cfdpresc = crate::Reg<cfdpresc::CfdprescSpec>;
#[doc = "Clock Failure Detector Prescaler"]
pub mod cfdpresc;
#[doc = "EVCTRL (rw) register accessor: Event Control\n\nYou can [`read`](crate::Reg::read) this register and get [`evctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@evctrl`]
module"]
#[doc(alias = "EVCTRL")]
pub type Evctrl = crate::Reg<evctrl::EvctrlSpec>;
#[doc = "Event Control"]
pub mod evctrl;
#[doc = "OSC48MCTRL (rw) register accessor: 48MHz Internal Oscillator (OSC48M) Control\n\nYou can [`read`](crate::Reg::read) this register and get [`osc48mctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc48mctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc48mctrl`]
module"]
#[doc(alias = "OSC48MCTRL")]
pub type Osc48mctrl = crate::Reg<osc48mctrl::Osc48mctrlSpec>;
#[doc = "48MHz Internal Oscillator (OSC48M) Control"]
pub mod osc48mctrl;
#[doc = "OSC48MDIV (rw) register accessor: OSC48M Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`osc48mdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc48mdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc48mdiv`]
module"]
#[doc(alias = "OSC48MDIV")]
pub type Osc48mdiv = crate::Reg<osc48mdiv::Osc48mdivSpec>;
#[doc = "OSC48M Divider"]
pub mod osc48mdiv;
#[doc = "OSC48MSTUP (rw) register accessor: OSC48M Startup Time\n\nYou can [`read`](crate::Reg::read) this register and get [`osc48mstup::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc48mstup::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc48mstup`]
module"]
#[doc(alias = "OSC48MSTUP")]
pub type Osc48mstup = crate::Reg<osc48mstup::Osc48mstupSpec>;
#[doc = "OSC48M Startup Time"]
pub mod osc48mstup;
#[doc = "OSC48MSYNCBUSY (r) register accessor: OSC48M Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`osc48msyncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc48msyncbusy`]
module"]
#[doc(alias = "OSC48MSYNCBUSY")]
pub type Osc48msyncbusy = crate::Reg<osc48msyncbusy::Osc48msyncbusySpec>;
#[doc = "OSC48M Synchronization Busy"]
pub mod osc48msyncbusy;
#[doc = "DPLLCTRLA (rw) register accessor: DPLL Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllctrla::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllctrla::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllctrla`]
module"]
#[doc(alias = "DPLLCTRLA")]
pub type Dpllctrla = crate::Reg<dpllctrla::DpllctrlaSpec>;
#[doc = "DPLL Control"]
pub mod dpllctrla;
#[doc = "DPLLRATIO (rw) register accessor: DPLL Ratio Control\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllratio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllratio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllratio`]
module"]
#[doc(alias = "DPLLRATIO")]
pub type Dpllratio = crate::Reg<dpllratio::DpllratioSpec>;
#[doc = "DPLL Ratio Control"]
pub mod dpllratio;
#[doc = "DPLLCTRLB (rw) register accessor: Digital Core Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllctrlb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllctrlb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllctrlb`]
module"]
#[doc(alias = "DPLLCTRLB")]
pub type Dpllctrlb = crate::Reg<dpllctrlb::DpllctrlbSpec>;
#[doc = "Digital Core Configuration"]
pub mod dpllctrlb;
#[doc = "DPLLPRESC (rw) register accessor: DPLL Prescaler\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllpresc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllpresc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllpresc`]
module"]
#[doc(alias = "DPLLPRESC")]
pub type Dpllpresc = crate::Reg<dpllpresc::DpllprescSpec>;
#[doc = "DPLL Prescaler"]
pub mod dpllpresc;
#[doc = "DPLLSYNCBUSY (r) register accessor: DPLL Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllsyncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllsyncbusy`]
module"]
#[doc(alias = "DPLLSYNCBUSY")]
pub type Dpllsyncbusy = crate::Reg<dpllsyncbusy::DpllsyncbusySpec>;
#[doc = "DPLL Synchronization Busy"]
pub mod dpllsyncbusy;
#[doc = "DPLLSTATUS (r) register accessor: DPLL Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllstatus`]
module"]
#[doc(alias = "DPLLSTATUS")]
pub type Dpllstatus = crate::Reg<dpllstatus::DpllstatusSpec>;
#[doc = "DPLL Status"]
pub mod dpllstatus;
#[doc = "CAL48M (rw) register accessor: 48MHz Oscillator Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`cal48m::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal48m::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cal48m`]
module"]
#[doc(alias = "CAL48M")]
pub type Cal48m = crate::Reg<cal48m::Cal48mSpec>;
#[doc = "48MHz Oscillator Calibration"]
pub mod cal48m;
