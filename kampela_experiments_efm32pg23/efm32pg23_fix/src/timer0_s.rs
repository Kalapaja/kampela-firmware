#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    cfg: Cfg,
    ctrl: Ctrl,
    cmd: Cmd,
    status: Status,
    if_: If,
    ien: Ien,
    top: Top,
    topb: Topb,
    cnt: Cnt,
    _reserved10: [u8; 0x04],
    lock: Lock,
    en: En,
    _reserved12: [u8; 0x2c],
    cc0_cfg: Cc0Cfg,
    cc0_ctrl: Cc0Ctrl,
    cc0_oc: Cc0Oc,
    _reserved15: [u8; 0x04],
    cc0_ocb: Cc0Ocb,
    cc0_icf: Cc0Icf,
    cc0_icof: Cc0Icof,
    _reserved18: [u8; 0x04],
    cc1_cfg: Cc1Cfg,
    cc1_ctrl: Cc1Ctrl,
    cc1_oc: Cc1Oc,
    _reserved21: [u8; 0x04],
    cc1_ocb: Cc1Ocb,
    cc1_icf: Cc1Icf,
    cc1_icof: Cc1Icof,
    _reserved24: [u8; 0x04],
    cc2_cfg: Cc2Cfg,
    cc2_ctrl: Cc2Ctrl,
    cc2_oc: Cc2Oc,
    _reserved27: [u8; 0x04],
    cc2_ocb: Cc2Ocb,
    cc2_icf: Cc2Icf,
    cc2_icof: Cc2Icof,
    _reserved30: [u8; 0x24],
    dtcfg: Dtcfg,
    dttimecfg: Dttimecfg,
    dtfcfg: Dtfcfg,
    dtctrl: Dtctrl,
    dtogen: Dtogen,
    dtfault: Dtfault,
    dtfaultc: Dtfaultc,
    dtlock: Dtlock,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x04 - No Description"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
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
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x1c - No Description"]
    #[inline(always)]
    pub const fn top(&self) -> &Top {
        &self.top
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn topb(&self) -> &Topb {
        &self.topb
    }
    #[doc = "0x24 - No Description"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x2c - No Description"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x30 - No Description"]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
    #[doc = "0x60 - No Description"]
    #[inline(always)]
    pub const fn cc0_cfg(&self) -> &Cc0Cfg {
        &self.cc0_cfg
    }
    #[doc = "0x64 - No Description"]
    #[inline(always)]
    pub const fn cc0_ctrl(&self) -> &Cc0Ctrl {
        &self.cc0_ctrl
    }
    #[doc = "0x68 - No Description"]
    #[inline(always)]
    pub const fn cc0_oc(&self) -> &Cc0Oc {
        &self.cc0_oc
    }
    #[doc = "0x70 - No Description"]
    #[inline(always)]
    pub const fn cc0_ocb(&self) -> &Cc0Ocb {
        &self.cc0_ocb
    }
    #[doc = "0x74 - No Description"]
    #[inline(always)]
    pub const fn cc0_icf(&self) -> &Cc0Icf {
        &self.cc0_icf
    }
    #[doc = "0x78 - No Description"]
    #[inline(always)]
    pub const fn cc0_icof(&self) -> &Cc0Icof {
        &self.cc0_icof
    }
    #[doc = "0x80 - No Description"]
    #[inline(always)]
    pub const fn cc1_cfg(&self) -> &Cc1Cfg {
        &self.cc1_cfg
    }
    #[doc = "0x84 - No Description"]
    #[inline(always)]
    pub const fn cc1_ctrl(&self) -> &Cc1Ctrl {
        &self.cc1_ctrl
    }
    #[doc = "0x88 - No Description"]
    #[inline(always)]
    pub const fn cc1_oc(&self) -> &Cc1Oc {
        &self.cc1_oc
    }
    #[doc = "0x90 - No Description"]
    #[inline(always)]
    pub const fn cc1_ocb(&self) -> &Cc1Ocb {
        &self.cc1_ocb
    }
    #[doc = "0x94 - No Description"]
    #[inline(always)]
    pub const fn cc1_icf(&self) -> &Cc1Icf {
        &self.cc1_icf
    }
    #[doc = "0x98 - No Description"]
    #[inline(always)]
    pub const fn cc1_icof(&self) -> &Cc1Icof {
        &self.cc1_icof
    }
    #[doc = "0xa0 - No Description"]
    #[inline(always)]
    pub const fn cc2_cfg(&self) -> &Cc2Cfg {
        &self.cc2_cfg
    }
    #[doc = "0xa4 - No Description"]
    #[inline(always)]
    pub const fn cc2_ctrl(&self) -> &Cc2Ctrl {
        &self.cc2_ctrl
    }
    #[doc = "0xa8 - No Description"]
    #[inline(always)]
    pub const fn cc2_oc(&self) -> &Cc2Oc {
        &self.cc2_oc
    }
    #[doc = "0xb0 - No Description"]
    #[inline(always)]
    pub const fn cc2_ocb(&self) -> &Cc2Ocb {
        &self.cc2_ocb
    }
    #[doc = "0xb4 - No Description"]
    #[inline(always)]
    pub const fn cc2_icf(&self) -> &Cc2Icf {
        &self.cc2_icf
    }
    #[doc = "0xb8 - No Description"]
    #[inline(always)]
    pub const fn cc2_icof(&self) -> &Cc2Icof {
        &self.cc2_icof
    }
    #[doc = "0xe0 - No Description"]
    #[inline(always)]
    pub const fn dtcfg(&self) -> &Dtcfg {
        &self.dtcfg
    }
    #[doc = "0xe4 - No Description"]
    #[inline(always)]
    pub const fn dttimecfg(&self) -> &Dttimecfg {
        &self.dttimecfg
    }
    #[doc = "0xe8 - No Description"]
    #[inline(always)]
    pub const fn dtfcfg(&self) -> &Dtfcfg {
        &self.dtfcfg
    }
    #[doc = "0xec - No Description"]
    #[inline(always)]
    pub const fn dtctrl(&self) -> &Dtctrl {
        &self.dtctrl
    }
    #[doc = "0xf0 - No Description"]
    #[inline(always)]
    pub const fn dtogen(&self) -> &Dtogen {
        &self.dtogen
    }
    #[doc = "0xf4 - No Description"]
    #[inline(always)]
    pub const fn dtfault(&self) -> &Dtfault {
        &self.dtfault
    }
    #[doc = "0xf8 - No Description"]
    #[inline(always)]
    pub const fn dtfaultc(&self) -> &Dtfaultc {
        &self.dtfaultc
    }
    #[doc = "0xfc - No Description"]
    #[inline(always)]
    pub const fn dtlock(&self) -> &Dtlock {
        &self.dtlock
    }
}
#[doc = "IPVERSION (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "No Description"]
pub mod ipversion;
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
#[doc = "TOP (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`top::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`top::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@top`] module"]
#[doc(alias = "TOP")]
pub type Top = crate::Reg<top::TopSpec>;
#[doc = "No Description"]
pub mod top;
#[doc = "TOPB (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`topb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`topb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@topb`] module"]
#[doc(alias = "TOPB")]
pub type Topb = crate::Reg<topb::TopbSpec>;
#[doc = "No Description"]
pub mod topb;
#[doc = "CNT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "No Description"]
pub mod cnt;
#[doc = "LOCK (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "No Description"]
pub mod lock;
#[doc = "EN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`] module"]
#[doc(alias = "EN")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = "No Description"]
pub mod en;
#[doc = "CC0_CFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_cfg`] module"]
#[doc(alias = "CC0_CFG")]
pub type Cc0Cfg = crate::Reg<cc0_cfg::Cc0CfgSpec>;
#[doc = "No Description"]
pub mod cc0_cfg;
#[doc = "CC0_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_ctrl`] module"]
#[doc(alias = "CC0_CTRL")]
pub type Cc0Ctrl = crate::Reg<cc0_ctrl::Cc0CtrlSpec>;
#[doc = "No Description"]
pub mod cc0_ctrl;
#[doc = "CC0_OC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_oc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_oc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_oc`] module"]
#[doc(alias = "CC0_OC")]
pub type Cc0Oc = crate::Reg<cc0_oc::Cc0OcSpec>;
#[doc = "No Description"]
pub mod cc0_oc;
#[doc = "CC0_OCB (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_ocb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_ocb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_ocb`] module"]
#[doc(alias = "CC0_OCB")]
pub type Cc0Ocb = crate::Reg<cc0_ocb::Cc0OcbSpec>;
#[doc = "No Description"]
pub mod cc0_ocb;
#[doc = "CC0_ICF (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_icf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_icf`] module"]
#[doc(alias = "CC0_ICF")]
pub type Cc0Icf = crate::Reg<cc0_icf::Cc0IcfSpec>;
#[doc = "No Description"]
pub mod cc0_icf;
#[doc = "CC0_ICOF (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_icof::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc0_icof`] module"]
#[doc(alias = "CC0_ICOF")]
pub type Cc0Icof = crate::Reg<cc0_icof::Cc0IcofSpec>;
#[doc = "No Description"]
pub mod cc0_icof;
#[doc = "CC1_CFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_cfg`] module"]
#[doc(alias = "CC1_CFG")]
pub type Cc1Cfg = crate::Reg<cc1_cfg::Cc1CfgSpec>;
#[doc = "No Description"]
pub mod cc1_cfg;
#[doc = "CC1_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_ctrl`] module"]
#[doc(alias = "CC1_CTRL")]
pub type Cc1Ctrl = crate::Reg<cc1_ctrl::Cc1CtrlSpec>;
#[doc = "No Description"]
pub mod cc1_ctrl;
#[doc = "CC1_OC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_oc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_oc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_oc`] module"]
#[doc(alias = "CC1_OC")]
pub type Cc1Oc = crate::Reg<cc1_oc::Cc1OcSpec>;
#[doc = "No Description"]
pub mod cc1_oc;
#[doc = "CC1_OCB (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_ocb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_ocb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_ocb`] module"]
#[doc(alias = "CC1_OCB")]
pub type Cc1Ocb = crate::Reg<cc1_ocb::Cc1OcbSpec>;
#[doc = "No Description"]
pub mod cc1_ocb;
#[doc = "CC1_ICF (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_icf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_icf`] module"]
#[doc(alias = "CC1_ICF")]
pub type Cc1Icf = crate::Reg<cc1_icf::Cc1IcfSpec>;
#[doc = "No Description"]
pub mod cc1_icf;
#[doc = "CC1_ICOF (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_icof::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc1_icof`] module"]
#[doc(alias = "CC1_ICOF")]
pub type Cc1Icof = crate::Reg<cc1_icof::Cc1IcofSpec>;
#[doc = "No Description"]
pub mod cc1_icof;
#[doc = "CC2_CFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_cfg`] module"]
#[doc(alias = "CC2_CFG")]
pub type Cc2Cfg = crate::Reg<cc2_cfg::Cc2CfgSpec>;
#[doc = "No Description"]
pub mod cc2_cfg;
#[doc = "CC2_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_ctrl`] module"]
#[doc(alias = "CC2_CTRL")]
pub type Cc2Ctrl = crate::Reg<cc2_ctrl::Cc2CtrlSpec>;
#[doc = "No Description"]
pub mod cc2_ctrl;
#[doc = "CC2_OC (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_oc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_oc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_oc`] module"]
#[doc(alias = "CC2_OC")]
pub type Cc2Oc = crate::Reg<cc2_oc::Cc2OcSpec>;
#[doc = "No Description"]
pub mod cc2_oc;
#[doc = "CC2_OCB (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_ocb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_ocb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_ocb`] module"]
#[doc(alias = "CC2_OCB")]
pub type Cc2Ocb = crate::Reg<cc2_ocb::Cc2OcbSpec>;
#[doc = "No Description"]
pub mod cc2_ocb;
#[doc = "CC2_ICF (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_icf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_icf`] module"]
#[doc(alias = "CC2_ICF")]
pub type Cc2Icf = crate::Reg<cc2_icf::Cc2IcfSpec>;
#[doc = "No Description"]
pub mod cc2_icf;
#[doc = "CC2_ICOF (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_icof::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cc2_icof`] module"]
#[doc(alias = "CC2_ICOF")]
pub type Cc2Icof = crate::Reg<cc2_icof::Cc2IcofSpec>;
#[doc = "No Description"]
pub mod cc2_icof;
#[doc = "DTCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dtcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtcfg`] module"]
#[doc(alias = "DTCFG")]
pub type Dtcfg = crate::Reg<dtcfg::DtcfgSpec>;
#[doc = "No Description"]
pub mod dtcfg;
#[doc = "DTTIMECFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dttimecfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dttimecfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dttimecfg`] module"]
#[doc(alias = "DTTIMECFG")]
pub type Dttimecfg = crate::Reg<dttimecfg::DttimecfgSpec>;
#[doc = "No Description"]
pub mod dttimecfg;
#[doc = "DTFCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dtfcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtfcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtfcfg`] module"]
#[doc(alias = "DTFCFG")]
pub type Dtfcfg = crate::Reg<dtfcfg::DtfcfgSpec>;
#[doc = "No Description"]
pub mod dtfcfg;
#[doc = "DTCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dtctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtctrl`] module"]
#[doc(alias = "DTCTRL")]
pub type Dtctrl = crate::Reg<dtctrl::DtctrlSpec>;
#[doc = "No Description"]
pub mod dtctrl;
#[doc = "DTOGEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dtogen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtogen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtogen`] module"]
#[doc(alias = "DTOGEN")]
pub type Dtogen = crate::Reg<dtogen::DtogenSpec>;
#[doc = "No Description"]
pub mod dtogen;
#[doc = "DTFAULT (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dtfault::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtfault`] module"]
#[doc(alias = "DTFAULT")]
pub type Dtfault = crate::Reg<dtfault::DtfaultSpec>;
#[doc = "No Description"]
pub mod dtfault;
#[doc = "DTFAULTC (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtfaultc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtfaultc`] module"]
#[doc(alias = "DTFAULTC")]
pub type Dtfaultc = crate::Reg<dtfaultc::DtfaultcSpec>;
#[doc = "No Description"]
pub mod dtfaultc;
#[doc = "DTLOCK (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtlock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtlock`] module"]
#[doc(alias = "DTLOCK")]
pub type Dtlock = crate::Reg<dtlock::DtlockSpec>;
#[doc = "No Description"]
pub mod dtlock;
