#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    en: En,
    swrst: Swrst,
    cfg: Cfg,
    cmd: Cmd,
    status: Status,
    cnt: Cnt,
    syncbusy: Syncbusy,
    lock: Lock,
    _reserved9: [u8; 0x1c],
    grp0_if: Grp0If,
    grp0_ien: Grp0Ien,
    grp0_ctrl: Grp0Ctrl,
    grp0_cmp0value: Grp0Cmp0value,
    grp0_cmp1value: Grp0Cmp1value,
    grp0_cap0value: Grp0Cap0value,
    grp0_syncbusy: Grp0Syncbusy,
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
    pub const fn swrst(&self) -> &Swrst {
        &self.swrst
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn cnt(&self) -> &Cnt {
        &self.cnt
    }
    #[doc = "0x1c - No Description"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x40 - No Description"]
    #[inline(always)]
    pub const fn grp0_if(&self) -> &Grp0If {
        &self.grp0_if
    }
    #[doc = "0x44 - No Description"]
    #[inline(always)]
    pub const fn grp0_ien(&self) -> &Grp0Ien {
        &self.grp0_ien
    }
    #[doc = "0x48 - No Description"]
    #[inline(always)]
    pub const fn grp0_ctrl(&self) -> &Grp0Ctrl {
        &self.grp0_ctrl
    }
    #[doc = "0x4c - No Description"]
    #[inline(always)]
    pub const fn grp0_cmp0value(&self) -> &Grp0Cmp0value {
        &self.grp0_cmp0value
    }
    #[doc = "0x50 - No Description"]
    #[inline(always)]
    pub const fn grp0_cmp1value(&self) -> &Grp0Cmp1value {
        &self.grp0_cmp1value
    }
    #[doc = "0x54 - No Description"]
    #[inline(always)]
    pub const fn grp0_cap0value(&self) -> &Grp0Cap0value {
        &self.grp0_cap0value
    }
    #[doc = "0x58 - No Description"]
    #[inline(always)]
    pub const fn grp0_syncbusy(&self) -> &Grp0Syncbusy {
        &self.grp0_syncbusy
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
#[doc = "SWRST (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`swrst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swrst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swrst`] module"]
#[doc(alias = "SWRST")]
pub type Swrst = crate::Reg<swrst::SwrstSpec>;
#[doc = "No Description"]
pub mod swrst;
#[doc = "CFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`] module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "No Description"]
pub mod cfg;
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
#[doc = "CNT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnt`] module"]
#[doc(alias = "CNT")]
pub type Cnt = crate::Reg<cnt::CntSpec>;
#[doc = "No Description"]
pub mod cnt;
#[doc = "SYNCBUSY (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "No Description"]
pub mod syncbusy;
#[doc = "LOCK (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "No Description"]
pub mod lock;
#[doc = "GRP0_IF (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`grp0_if::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp0_if::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grp0_if`] module"]
#[doc(alias = "GRP0_IF")]
pub type Grp0If = crate::Reg<grp0_if::Grp0IfSpec>;
#[doc = "No Description"]
pub mod grp0_if;
#[doc = "GRP0_IEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`grp0_ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp0_ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grp0_ien`] module"]
#[doc(alias = "GRP0_IEN")]
pub type Grp0Ien = crate::Reg<grp0_ien::Grp0IenSpec>;
#[doc = "No Description"]
pub mod grp0_ien;
#[doc = "GRP0_CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`grp0_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp0_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grp0_ctrl`] module"]
#[doc(alias = "GRP0_CTRL")]
pub type Grp0Ctrl = crate::Reg<grp0_ctrl::Grp0CtrlSpec>;
#[doc = "No Description"]
pub mod grp0_ctrl;
#[doc = "GRP0_CMP0VALUE (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`grp0_cmp0value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp0_cmp0value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grp0_cmp0value`] module"]
#[doc(alias = "GRP0_CMP0VALUE")]
pub type Grp0Cmp0value = crate::Reg<grp0_cmp0value::Grp0Cmp0valueSpec>;
#[doc = "No Description"]
pub mod grp0_cmp0value;
#[doc = "GRP0_CMP1VALUE (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`grp0_cmp1value::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp0_cmp1value::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grp0_cmp1value`] module"]
#[doc(alias = "GRP0_CMP1VALUE")]
pub type Grp0Cmp1value = crate::Reg<grp0_cmp1value::Grp0Cmp1valueSpec>;
#[doc = "No Description"]
pub mod grp0_cmp1value;
#[doc = "GRP0_CAP0VALUE (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`grp0_cap0value::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grp0_cap0value`] module"]
#[doc(alias = "GRP0_CAP0VALUE")]
pub type Grp0Cap0value = crate::Reg<grp0_cap0value::Grp0Cap0valueSpec>;
#[doc = "No Description"]
pub mod grp0_cap0value;
#[doc = "GRP0_SYNCBUSY (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`grp0_syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grp0_syncbusy`] module"]
#[doc(alias = "GRP0_SYNCBUSY")]
pub type Grp0Syncbusy = crate::Reg<grp0_syncbusy::Grp0SyncbusySpec>;
#[doc = "No Description"]
pub mod grp0_syncbusy;
