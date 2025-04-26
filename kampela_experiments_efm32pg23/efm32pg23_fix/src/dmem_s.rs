#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    cmd: Cmd,
    ctrl: Ctrl,
    eccerraddr0: Eccerraddr0,
    eccerraddr1: Eccerraddr1,
    _reserved5: [u8; 0x08],
    eccmerrind: Eccmerrind,
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
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x08 - No Description"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn eccerraddr0(&self) -> &Eccerraddr0 {
        &self.eccerraddr0
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn eccerraddr1(&self) -> &Eccerraddr1 {
        &self.eccerraddr1
    }
    #[doc = "0x1c - No Description"]
    #[inline(always)]
    pub const fn eccmerrind(&self) -> &Eccmerrind {
        &self.eccmerrind
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x24 - No Description"]
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
#[doc = "CMD (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "ECCERRADDR0 (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`eccerraddr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccerraddr0`] module"]
#[doc(alias = "ECCERRADDR0")]
pub type Eccerraddr0 = crate::Reg<eccerraddr0::Eccerraddr0Spec>;
#[doc = "No Description"]
pub mod eccerraddr0;
#[doc = "ECCERRADDR1 (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`eccerraddr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccerraddr1`] module"]
#[doc(alias = "ECCERRADDR1")]
pub type Eccerraddr1 = crate::Reg<eccerraddr1::Eccerraddr1Spec>;
#[doc = "No Description"]
pub mod eccerraddr1;
#[doc = "ECCMERRIND (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`eccmerrind::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccmerrind`] module"]
#[doc(alias = "ECCMERRIND")]
pub type Eccmerrind = crate::Reg<eccmerrind::EccmerrindSpec>;
#[doc = "No Description"]
pub mod eccmerrind;
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
