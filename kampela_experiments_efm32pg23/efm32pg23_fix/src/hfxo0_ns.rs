#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    _reserved1: [u8; 0x0c],
    xtalcfg: Xtalcfg,
    _reserved2: [u8; 0x04],
    xtalctrl: Xtalctrl,
    xtalctrl1: Xtalctrl1,
    cfg: Cfg,
    _reserved5: [u8; 0x04],
    ctrl: Ctrl,
    _reserved6: [u8; 0x14],
    bufouttrim: Bufouttrim,
    bufoutctrl: Bufoutctrl,
    _reserved8: [u8; 0x08],
    cmd: Cmd,
    _reserved9: [u8; 0x04],
    status: Status,
    _reserved10: [u8; 0x14],
    if_: If,
    ien: Ien,
    _reserved12: [u8; 0x08],
    lock: Lock,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn xtalcfg(&self) -> &Xtalcfg {
        &self.xtalcfg
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn xtalctrl(&self) -> &Xtalctrl {
        &self.xtalctrl
    }
    #[doc = "0x1c - No Description"]
    #[inline(always)]
    pub const fn xtalctrl1(&self) -> &Xtalctrl1 {
        &self.xtalctrl1
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x28 - No Description"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x40 - No Description"]
    #[inline(always)]
    pub const fn bufouttrim(&self) -> &Bufouttrim {
        &self.bufouttrim
    }
    #[doc = "0x44 - No Description"]
    #[inline(always)]
    pub const fn bufoutctrl(&self) -> &Bufoutctrl {
        &self.bufoutctrl
    }
    #[doc = "0x50 - No Description"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x58 - No Description"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x70 - No Description"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x74 - No Description"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x80 - No Description"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
}
#[doc = "IPVERSION (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "XTALCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalcfg`] module"]
#[doc(alias = "XTALCFG")]
pub type Xtalcfg = crate::Reg<xtalcfg::XtalcfgSpec>;
#[doc = "No Description"]
pub mod xtalcfg;
#[doc = "XTALCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalctrl`] module"]
#[doc(alias = "XTALCTRL")]
pub type Xtalctrl = crate::Reg<xtalctrl::XtalctrlSpec>;
#[doc = "No Description"]
pub mod xtalctrl;
#[doc = "XTALCTRL1 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@xtalctrl1`] module"]
#[doc(alias = "XTALCTRL1")]
pub type Xtalctrl1 = crate::Reg<xtalctrl1::Xtalctrl1Spec>;
#[doc = "No Description"]
pub mod xtalctrl1;
#[doc = "CFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "No Description"]
pub mod cfg;
#[doc = "CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "BUFOUTTRIM (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`bufouttrim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bufouttrim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bufouttrim`] module"]
#[doc(alias = "BUFOUTTRIM")]
pub type Bufouttrim = crate::Reg<bufouttrim::BufouttrimSpec>;
#[doc = "No Description"]
pub mod bufouttrim;
#[doc = "BUFOUTCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`bufoutctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bufoutctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bufoutctrl`] module"]
#[doc(alias = "BUFOUTCTRL")]
pub type Bufoutctrl = crate::Reg<bufoutctrl::BufoutctrlSpec>;
#[doc = "No Description"]
pub mod bufoutctrl;
#[doc = "CMD (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "No Description"]
pub mod status;
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
#[doc = "LOCK (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "No Description"]
pub mod lock;
