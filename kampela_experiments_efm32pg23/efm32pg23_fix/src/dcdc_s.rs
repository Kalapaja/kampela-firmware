#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    ctrl: Ctrl,
    em01ctrl0: Em01ctrl0,
    _reserved3: [u8; 0x04],
    em23ctrl0: Em23ctrl0,
    _reserved4: [u8; 0x0c],
    pfmxctrl: Pfmxctrl,
    _reserved5: [u8; 0x04],
    if_: If,
    ien: Ien,
    status: Status,
    syncbusy: Syncbusy,
    _reserved9: [u8; 0x08],
    lock: Lock,
    lockstatus: Lockstatus,
}
impl RegisterBlock {
    #[doc = "0x00 - IPVERSION"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x04 - Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x08 - EM01 Configurations"]
    #[inline(always)]
    pub const fn em01ctrl0(&self) -> &Em01ctrl0 {
        &self.em01ctrl0
    }
    #[doc = "0x10 - EM23 Configurations"]
    #[inline(always)]
    pub const fn em23ctrl0(&self) -> &Em23ctrl0 {
        &self.em23ctrl0
    }
    #[doc = "0x20 - PFMX Control Register"]
    #[inline(always)]
    pub const fn pfmxctrl(&self) -> &Pfmxctrl {
        &self.pfmxctrl
    }
    #[doc = "0x28 - Interrupt Flags"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x2c - Interrupt Enable"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x30 - DCDC Status Register"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x34 - Syncbusy Status Register"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
    }
    #[doc = "0x40 - No Description"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x44 - No Description"]
    #[inline(always)]
    pub const fn lockstatus(&self) -> &Lockstatus {
        &self.lockstatus
    }
}
#[doc = "IPVERSION (r) register accessor: IPVERSION\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "IPVERSION"]
pub mod ipversion;
#[doc = "CTRL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "EM01CTRL0 (rw) register accessor: EM01 Configurations\n\nYou can [`read`](crate::Reg::read) this register and get [`em01ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em01ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em01ctrl0`] module"]
#[doc(alias = "EM01CTRL0")]
pub type Em01ctrl0 = crate::Reg<em01ctrl0::Em01ctrl0Spec>;
#[doc = "EM01 Configurations"]
pub mod em01ctrl0;
#[doc = "EM23CTRL0 (rw) register accessor: EM23 Configurations\n\nYou can [`read`](crate::Reg::read) this register and get [`em23ctrl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em23ctrl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em23ctrl0`] module"]
#[doc(alias = "EM23CTRL0")]
pub type Em23ctrl0 = crate::Reg<em23ctrl0::Em23ctrl0Spec>;
#[doc = "EM23 Configurations"]
pub mod em23ctrl0;
#[doc = "PFMXCTRL (rw) register accessor: PFMX Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pfmxctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfmxctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfmxctrl`] module"]
#[doc(alias = "PFMXCTRL")]
pub type Pfmxctrl = crate::Reg<pfmxctrl::PfmxctrlSpec>;
#[doc = "PFMX Control Register"]
pub mod pfmxctrl;
#[doc = "IF (rw) register accessor: Interrupt Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "Interrupt Flags"]
pub mod if_;
#[doc = "IEN (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "Interrupt Enable"]
pub mod ien;
#[doc = "STATUS (r) register accessor: DCDC Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "DCDC Status Register"]
pub mod status;
#[doc = "SYNCBUSY (r) register accessor: Syncbusy Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "Syncbusy Status Register"]
pub mod syncbusy;
#[doc = "LOCK (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "No Description"]
pub mod lock;
#[doc = "LOCKSTATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`lockstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lockstatus`] module"]
#[doc(alias = "LOCKSTATUS")]
pub type Lockstatus = crate::Reg<lockstatus::LockstatusSpec>;
#[doc = "No Description"]
pub mod lockstatus;
