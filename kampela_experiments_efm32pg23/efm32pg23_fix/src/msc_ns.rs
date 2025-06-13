#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    readctrl: Readctrl,
    rdatactrl: Rdatactrl,
    writectrl: Writectrl,
    writecmd: Writecmd,
    addrb: Addrb,
    wdata: Wdata,
    status: Status,
    if_: If,
    ien: Ien,
    _reserved10: [u8; 0x0c],
    userdatasize: Userdatasize,
    cmd: Cmd,
    lock: Lock,
    misclockword: Misclockword,
    _reserved14: [u8; 0x0c],
    pwrctrl: Pwrctrl,
    _reserved15: [u8; 0xcc],
    pagelock0: Pagelock0,
    pagelock1: Pagelock1,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x04 - No Description"]
    #[inline(always)]
    pub const fn readctrl(&self) -> &Readctrl {
        &self.readctrl
    }
    #[doc = "0x08 - No Description"]
    #[inline(always)]
    pub const fn rdatactrl(&self) -> &Rdatactrl {
        &self.rdatactrl
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn writectrl(&self) -> &Writectrl {
        &self.writectrl
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn writecmd(&self) -> &Writecmd {
        &self.writecmd
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn addrb(&self) -> &Addrb {
        &self.addrb
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn wdata(&self) -> &Wdata {
        &self.wdata
    }
    #[doc = "0x1c - No Description"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
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
    #[doc = "0x34 - No Description"]
    #[inline(always)]
    pub const fn userdatasize(&self) -> &Userdatasize {
        &self.userdatasize
    }
    #[doc = "0x38 - No Description"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x3c - No Description"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x40 - No Description"]
    #[inline(always)]
    pub const fn misclockword(&self) -> &Misclockword {
        &self.misclockword
    }
    #[doc = "0x50 - No Description"]
    #[inline(always)]
    pub const fn pwrctrl(&self) -> &Pwrctrl {
        &self.pwrctrl
    }
    #[doc = "0x120 - No Description"]
    #[inline(always)]
    pub const fn pagelock0(&self) -> &Pagelock0 {
        &self.pagelock0
    }
    #[doc = "0x124 - No Description"]
    #[inline(always)]
    pub const fn pagelock1(&self) -> &Pagelock1 {
        &self.pagelock1
    }
}
#[doc = "IPVERSION (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "READCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`readctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@readctrl`] module"]
#[doc(alias = "READCTRL")]
pub type Readctrl = crate::Reg<readctrl::ReadctrlSpec>;
#[doc = "No Description"]
pub mod readctrl;
#[doc = "RDATACTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rdatactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdatactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdatactrl`] module"]
#[doc(alias = "RDATACTRL")]
pub type Rdatactrl = crate::Reg<rdatactrl::RdatactrlSpec>;
#[doc = "No Description"]
pub mod rdatactrl;
#[doc = "WRITECTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`writectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writectrl`] module"]
#[doc(alias = "WRITECTRL")]
pub type Writectrl = crate::Reg<writectrl::WritectrlSpec>;
#[doc = "No Description"]
pub mod writectrl;
#[doc = "WRITECMD (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`writecmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writecmd`] module"]
#[doc(alias = "WRITECMD")]
pub type Writecmd = crate::Reg<writecmd::WritecmdSpec>;
#[doc = "No Description"]
pub mod writecmd;
#[doc = "ADDRB (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`addrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addrb`] module"]
#[doc(alias = "ADDRB")]
pub type Addrb = crate::Reg<addrb::AddrbSpec>;
#[doc = "No Description"]
pub mod addrb;
#[doc = "WDATA (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`wdata::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdata`] module"]
#[doc(alias = "WDATA")]
pub type Wdata = crate::Reg<wdata::WdataSpec>;
#[doc = "No Description"]
pub mod wdata;
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
#[doc = "USERDATASIZE (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`userdatasize::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@userdatasize`] module"]
#[doc(alias = "USERDATASIZE")]
pub type Userdatasize = crate::Reg<userdatasize::UserdatasizeSpec>;
#[doc = "No Description"]
pub mod userdatasize;
#[doc = "CMD (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "LOCK (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "No Description"]
pub mod lock;
#[doc = "MISCLOCKWORD (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`misclockword::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misclockword::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misclockword`] module"]
#[doc(alias = "MISCLOCKWORD")]
pub type Misclockword = crate::Reg<misclockword::MisclockwordSpec>;
#[doc = "No Description"]
pub mod misclockword;
#[doc = "PWRCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrctrl`] module"]
#[doc(alias = "PWRCTRL")]
pub type Pwrctrl = crate::Reg<pwrctrl::PwrctrlSpec>;
#[doc = "No Description"]
pub mod pwrctrl;
#[doc = "PAGELOCK0 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pagelock0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pagelock0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pagelock0`] module"]
#[doc(alias = "PAGELOCK0")]
pub type Pagelock0 = crate::Reg<pagelock0::Pagelock0Spec>;
#[doc = "No Description"]
pub mod pagelock0;
#[doc = "PAGELOCK1 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pagelock1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pagelock1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pagelock1`] module"]
#[doc(alias = "PAGELOCK1")]
pub type Pagelock1 = crate::Reg<pagelock1::Pagelock1Spec>;
#[doc = "No Description"]
pub mod pagelock1;
