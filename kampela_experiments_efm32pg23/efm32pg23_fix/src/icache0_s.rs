#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    ctrl: Ctrl,
    pchits: Pchits,
    pcmisses: Pcmisses,
    pcahits: Pcahits,
    status: Status,
    cmd: Cmd,
    lpmode: Lpmode,
    if_: If,
    ien: Ien,
}
impl RegisterBlock {
    #[doc = "0x00 - The read only IPVERSION field gives the version for this module. There may be minor software changes required for modules with different values of IPVERSION."]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x04 - No Description"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x08 - No Description"]
    #[inline(always)]
    pub const fn pchits(&self) -> &Pchits {
        &self.pchits
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn pcmisses(&self) -> &Pcmisses {
        &self.pcmisses
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn pcahits(&self) -> &Pcahits {
        &self.pcahits
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x1c - No Description"]
    #[inline(always)]
    pub const fn lpmode(&self) -> &Lpmode {
        &self.lpmode
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
#[doc = "IPVERSION (r) register accessor: The read only IPVERSION field gives the version for this module. There may be minor software changes required for modules with different values of IPVERSION.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "The read only IPVERSION field gives the version for this module. There may be minor software changes required for modules with different values of IPVERSION."]
pub mod ipversion;
#[doc = "CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "PCHITS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pchits::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pchits`] module"]
#[doc(alias = "PCHITS")]
pub type Pchits = crate::Reg<pchits::PchitsSpec>;
#[doc = "No Description"]
pub mod pchits;
#[doc = "PCMISSES (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pcmisses::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmisses`] module"]
#[doc(alias = "PCMISSES")]
pub type Pcmisses = crate::Reg<pcmisses::PcmissesSpec>;
#[doc = "No Description"]
pub mod pcmisses;
#[doc = "PCAHITS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pcahits::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcahits`] module"]
#[doc(alias = "PCAHITS")]
pub type Pcahits = crate::Reg<pcahits::PcahitsSpec>;
#[doc = "No Description"]
pub mod pcahits;
#[doc = "STATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "No Description"]
pub mod status;
#[doc = "CMD (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "LPMODE (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpmode`] module"]
#[doc(alias = "LPMODE")]
pub type Lpmode = crate::Reg<lpmode::LpmodeSpec>;
#[doc = "No Description"]
pub mod lpmode;
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
