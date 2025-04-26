#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    decbod: Decbod,
    _reserved1: [u8; 0x0c],
    bod3sense: Bod3sense,
    _reserved2: [u8; 0x18],
    vregvddcmpctrl: Vregvddcmpctrl,
    pd1paretctrl: Pd1paretctrl,
    _reserved4: [u8; 0x18],
    ipversion: Ipversion,
    lock: Lock,
    if_: If,
    ien: Ien,
    em4ctrl: Em4ctrl,
    cmd: Cmd,
    ctrl: Ctrl,
    templimits: Templimits,
    _reserved12: [u8; 0x08],
    status: Status,
    temp: Temp,
    _reserved14: [u8; 0x04],
    rstctrl: Rstctrl,
    rstcause: Rstcause,
    tamperrstcause: Tamperrstcause,
    _reserved17: [u8; 0x04],
    dgif: Dgif,
    dgien: Dgien,
    _reserved19: [u8; 0x58],
    efpif: Efpif,
    efpien: Efpien,
}
impl RegisterBlock {
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn decbod(&self) -> &Decbod {
        &self.decbod
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn bod3sense(&self) -> &Bod3sense {
        &self.bod3sense
    }
    #[doc = "0x3c - No Description"]
    #[inline(always)]
    pub const fn vregvddcmpctrl(&self) -> &Vregvddcmpctrl {
        &self.vregvddcmpctrl
    }
    #[doc = "0x40 - No Description"]
    #[inline(always)]
    pub const fn pd1paretctrl(&self) -> &Pd1paretctrl {
        &self.pd1paretctrl
    }
    #[doc = "0x5c - IP Version"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x60 - No Description"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x64 - No Description"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x68 - No Description"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x6c - No Description"]
    #[inline(always)]
    pub const fn em4ctrl(&self) -> &Em4ctrl {
        &self.em4ctrl
    }
    #[doc = "0x70 - No Description"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x74 - No Description"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x78 - No Description"]
    #[inline(always)]
    pub const fn templimits(&self) -> &Templimits {
        &self.templimits
    }
    #[doc = "0x84 - No Description"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x88 - No Description"]
    #[inline(always)]
    pub const fn temp(&self) -> &Temp {
        &self.temp
    }
    #[doc = "0x90 - No Description"]
    #[inline(always)]
    pub const fn rstctrl(&self) -> &Rstctrl {
        &self.rstctrl
    }
    #[doc = "0x94 - No Description"]
    #[inline(always)]
    pub const fn rstcause(&self) -> &Rstcause {
        &self.rstcause
    }
    #[doc = "0x98 - No Description"]
    #[inline(always)]
    pub const fn tamperrstcause(&self) -> &Tamperrstcause {
        &self.tamperrstcause
    }
    #[doc = "0xa0 - No Description"]
    #[inline(always)]
    pub const fn dgif(&self) -> &Dgif {
        &self.dgif
    }
    #[doc = "0xa4 - No Description"]
    #[inline(always)]
    pub const fn dgien(&self) -> &Dgien {
        &self.dgien
    }
    #[doc = "0x100 - No Description"]
    #[inline(always)]
    pub const fn efpif(&self) -> &Efpif {
        &self.efpif
    }
    #[doc = "0x104 - No Description"]
    #[inline(always)]
    pub const fn efpien(&self) -> &Efpien {
        &self.efpien
    }
}
#[doc = "DECBOD (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`decbod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`decbod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@decbod`] module"]
#[doc(alias = "DECBOD")]
pub type Decbod = crate::Reg<decbod::DecbodSpec>;
#[doc = "No Description"]
pub mod decbod;
#[doc = "BOD3SENSE (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`bod3sense::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod3sense::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bod3sense`] module"]
#[doc(alias = "BOD3SENSE")]
pub type Bod3sense = crate::Reg<bod3sense::Bod3senseSpec>;
#[doc = "No Description"]
pub mod bod3sense;
#[doc = "VREGVDDCMPCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`vregvddcmpctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vregvddcmpctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vregvddcmpctrl`] module"]
#[doc(alias = "VREGVDDCMPCTRL")]
pub type Vregvddcmpctrl = crate::Reg<vregvddcmpctrl::VregvddcmpctrlSpec>;
#[doc = "No Description"]
pub mod vregvddcmpctrl;
#[doc = "PD1PARETCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pd1paretctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pd1paretctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pd1paretctrl`] module"]
#[doc(alias = "PD1PARETCTRL")]
pub type Pd1paretctrl = crate::Reg<pd1paretctrl::Pd1paretctrlSpec>;
#[doc = "No Description"]
pub mod pd1paretctrl;
#[doc = "IPVERSION (r) register accessor: IP Version\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "IP Version"]
pub mod ipversion;
#[doc = "LOCK (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "No Description"]
pub mod lock;
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
#[doc = "EM4CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em4ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4ctrl`] module"]
#[doc(alias = "EM4CTRL")]
pub type Em4ctrl = crate::Reg<em4ctrl::Em4ctrlSpec>;
#[doc = "No Description"]
pub mod em4ctrl;
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
#[doc = "TEMPLIMITS (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`templimits::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`templimits::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@templimits`] module"]
#[doc(alias = "TEMPLIMITS")]
pub type Templimits = crate::Reg<templimits::TemplimitsSpec>;
#[doc = "No Description"]
pub mod templimits;
#[doc = "STATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "No Description"]
pub mod status;
#[doc = "TEMP (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`temp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@temp`] module"]
#[doc(alias = "TEMP")]
pub type Temp = crate::Reg<temp::TempSpec>;
#[doc = "No Description"]
pub mod temp;
#[doc = "RSTCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rstctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstctrl`] module"]
#[doc(alias = "RSTCTRL")]
pub type Rstctrl = crate::Reg<rstctrl::RstctrlSpec>;
#[doc = "No Description"]
pub mod rstctrl;
#[doc = "RSTCAUSE (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rstcause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstcause`] module"]
#[doc(alias = "RSTCAUSE")]
pub type Rstcause = crate::Reg<rstcause::RstcauseSpec>;
#[doc = "No Description"]
pub mod rstcause;
#[doc = "TAMPERRSTCAUSE (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`tamperrstcause::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tamperrstcause`] module"]
#[doc(alias = "TAMPERRSTCAUSE")]
pub type Tamperrstcause = crate::Reg<tamperrstcause::TamperrstcauseSpec>;
#[doc = "No Description"]
pub mod tamperrstcause;
#[doc = "DGIF (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dgif::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgif::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dgif`] module"]
#[doc(alias = "DGIF")]
pub type Dgif = crate::Reg<dgif::DgifSpec>;
#[doc = "No Description"]
pub mod dgif;
#[doc = "DGIEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dgien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dgien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dgien`] module"]
#[doc(alias = "DGIEN")]
pub type Dgien = crate::Reg<dgien::DgienSpec>;
#[doc = "No Description"]
pub mod dgien;
#[doc = "EFPIF (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`efpif::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efpif::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efpif`] module"]
#[doc(alias = "EFPIF")]
pub type Efpif = crate::Reg<efpif::EfpifSpec>;
#[doc = "No Description"]
pub mod efpif;
#[doc = "EFPIEN (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`efpien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efpien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@efpien`] module"]
#[doc(alias = "EFPIEN")]
pub type Efpien = crate::Reg<efpien::EfpienSpec>;
#[doc = "No Description"]
pub mod efpien;
