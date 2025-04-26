#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    en: En,
    ctrl: Ctrl,
    cmd: Cmd,
    state: State,
    status: Status,
    clkdiv: Clkdiv,
    saddr: Saddr,
    saddrmask: Saddrmask,
    rxdata: Rxdata,
    rxdouble: Rxdouble,
    rxdatap: Rxdatap,
    rxdoublep: Rxdoublep,
    txdata: Txdata,
    txdouble: Txdouble,
    if_: If,
    ien: Ien,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x04 - No Description"]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
    #[doc = "0x08 - No Description"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x1c - No Description"]
    #[inline(always)]
    pub const fn saddr(&self) -> &Saddr {
        &self.saddr
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn saddrmask(&self) -> &Saddrmask {
        &self.saddrmask
    }
    #[doc = "0x24 - No Description"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x28 - No Description"]
    #[inline(always)]
    pub const fn rxdouble(&self) -> &Rxdouble {
        &self.rxdouble
    }
    #[doc = "0x2c - No Description"]
    #[inline(always)]
    pub const fn rxdatap(&self) -> &Rxdatap {
        &self.rxdatap
    }
    #[doc = "0x30 - No Description"]
    #[inline(always)]
    pub const fn rxdoublep(&self) -> &Rxdoublep {
        &self.rxdoublep
    }
    #[doc = "0x34 - No Description"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x38 - No Description"]
    #[inline(always)]
    pub const fn txdouble(&self) -> &Txdouble {
        &self.txdouble
    }
    #[doc = "0x3c - No Description"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x40 - No Description"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
}
#[doc = "IPVERSION (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "EN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`] module"]
#[doc(alias = "EN")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = "No Description"]
pub mod en;
#[doc = "CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "STATE (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "No Description"]
pub mod state;
#[doc = "STATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "No Description"]
pub mod status;
#[doc = "CLKDIV (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`] module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "No Description"]
pub mod clkdiv;
#[doc = "SADDR (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`saddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddr`] module"]
#[doc(alias = "SADDR")]
pub type Saddr = crate::Reg<saddr::SaddrSpec>;
#[doc = "No Description"]
pub mod saddr;
#[doc = "SADDRMASK (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`saddrmask::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddrmask::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@saddrmask`] module"]
#[doc(alias = "SADDRMASK")]
pub type Saddrmask = crate::Reg<saddrmask::SaddrmaskSpec>;
#[doc = "No Description"]
pub mod saddrmask;
#[doc = "RXDATA (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`] module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "No Description"]
pub mod rxdata;
#[doc = "RXDOUBLE (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdouble::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdouble`] module"]
#[doc(alias = "RXDOUBLE")]
pub type Rxdouble = crate::Reg<rxdouble::RxdoubleSpec>;
#[doc = "No Description"]
pub mod rxdouble;
#[doc = "RXDATAP (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdatap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdatap`] module"]
#[doc(alias = "RXDATAP")]
pub type Rxdatap = crate::Reg<rxdatap::RxdatapSpec>;
#[doc = "No Description"]
pub mod rxdatap;
#[doc = "RXDOUBLEP (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdoublep::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdoublep`] module"]
#[doc(alias = "RXDOUBLEP")]
pub type Rxdoublep = crate::Reg<rxdoublep::RxdoublepSpec>;
#[doc = "No Description"]
pub mod rxdoublep;
#[doc = "TXDATA (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`] module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "No Description"]
pub mod txdata;
#[doc = "TXDOUBLE (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdouble::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdouble`] module"]
#[doc(alias = "TXDOUBLE")]
pub type Txdouble = crate::Reg<txdouble::TxdoubleSpec>;
#[doc = "No Description"]
pub mod txdouble;
#[doc = "IF (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "No Description"]
pub mod if_;
#[doc = "IEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "No Description"]
pub mod ien;
