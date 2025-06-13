#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    ipversion: Ipversion,
    if_: If,
    ien: Ien,
    _reserved3: [u8; 0x04],
    chiprevhw: Chiprevhw,
    chiprev: Chiprev,
    _reserved5: [u8; 0x08],
    cfgsystic: Cfgsystic,
    _reserved6: [u8; 0x01d8],
    ctrl: Ctrl,
    _reserved7: [u8; 0x04],
    dmem0retnctrl: Dmem0retnctrl,
    _reserved8: [u8; 0x0100],
    rambiasconf: Rambiasconf,
    _reserved9: [u8; 0x0108],
    icacheramretnctrl: Icacheramretnctrl,
    dmem0portmapsel: Dmem0portmapsel,
    _reserved11: [u8; 0x01e0],
    rootdata0: Rootdata0,
    rootdata1: Rootdata1,
    rootlockstatus: Rootlockstatus,
    rootseswversion: Rootseswversion,
}
impl RegisterBlock {
    #[doc = "0x04 - No Description"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x08 - Read to get system status."]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x0c - Write to enable interrupts."]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x14 - Read to get the hard-wired chip revision."]
    #[inline(always)]
    pub const fn chiprevhw(&self) -> &Chiprevhw {
        &self.chiprevhw
    }
    #[doc = "0x18 - Read to get the chip revision programmed by feature configuration."]
    #[inline(always)]
    pub const fn chiprev(&self) -> &Chiprev {
        &self.chiprev
    }
    #[doc = "0x24 - Configure the source of the system tick for the M33."]
    #[inline(always)]
    pub const fn cfgsystic(&self) -> &Cfgsystic {
        &self.cfgsystic
    }
    #[doc = "0x200 - Configure to provide general RAM configuration."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x208 - Configure to provide general RAM retention configuration."]
    #[inline(always)]
    pub const fn dmem0retnctrl(&self) -> &Dmem0retnctrl {
        &self.dmem0retnctrl
    }
    #[doc = "0x30c - Configure RAM bias configure bits."]
    #[inline(always)]
    pub const fn rambiasconf(&self) -> &Rambiasconf {
        &self.rambiasconf
    }
    #[doc = "0x418 - Configure Host ICACHERAM retention configuration."]
    #[inline(always)]
    pub const fn icacheramretnctrl(&self) -> &Icacheramretnctrl {
        &self.icacheramretnctrl
    }
    #[doc = "0x41c - Configure DMEM0 port remap selection."]
    #[inline(always)]
    pub const fn dmem0portmapsel(&self) -> &Dmem0portmapsel {
        &self.dmem0portmapsel
    }
    #[doc = "0x600 - Generic data space for user to pass to root, e.g., address of struct in mem"]
    #[inline(always)]
    pub const fn rootdata0(&self) -> &Rootdata0 {
        &self.rootdata0
    }
    #[doc = "0x604 - Generic data space for user to pass to root, e.g., address of struct in mem"]
    #[inline(always)]
    pub const fn rootdata1(&self) -> &Rootdata1 {
        &self.rootdata1
    }
    #[doc = "0x608 - This register returns the status of the SE managed locks."]
    #[inline(always)]
    pub const fn rootlockstatus(&self) -> &Rootlockstatus {
        &self.rootlockstatus
    }
    #[doc = "0x60c - SE Software version"]
    #[inline(always)]
    pub const fn rootseswversion(&self) -> &Rootseswversion {
        &self.rootseswversion
    }
}
#[doc = "IPVERSION (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "IF (rw) register accessor: Read to get system status.\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "Read to get system status."]
pub mod if_;
#[doc = "IEN (rw) register accessor: Write to enable interrupts.\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "Write to enable interrupts."]
pub mod ien;
#[doc = "CHIPREVHW (rw) register accessor: Read to get the hard-wired chip revision.\n\nYou can [`read`](crate::Reg::read) this register and get [`chiprevhw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chiprevhw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chiprevhw`] module"]
#[doc(alias = "CHIPREVHW")]
pub type Chiprevhw = crate::Reg<chiprevhw::ChiprevhwSpec>;
#[doc = "Read to get the hard-wired chip revision."]
pub mod chiprevhw;
#[doc = "CHIPREV (rw) register accessor: Read to get the chip revision programmed by feature configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`chiprev::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chiprev::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chiprev`] module"]
#[doc(alias = "CHIPREV")]
pub type Chiprev = crate::Reg<chiprev::ChiprevSpec>;
#[doc = "Read to get the chip revision programmed by feature configuration."]
pub mod chiprev;
#[doc = "CFGSYSTIC (rw) register accessor: Configure the source of the system tick for the M33.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgsystic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgsystic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgsystic`] module"]
#[doc(alias = "CFGSYSTIC")]
pub type Cfgsystic = crate::Reg<cfgsystic::CfgsysticSpec>;
#[doc = "Configure the source of the system tick for the M33."]
pub mod cfgsystic;
#[doc = "CTRL (rw) register accessor: Configure to provide general RAM configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Configure to provide general RAM configuration."]
pub mod ctrl;
#[doc = "DMEM0RETNCTRL (rw) register accessor: Configure to provide general RAM retention configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmem0retnctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmem0retnctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmem0retnctrl`] module"]
#[doc(alias = "DMEM0RETNCTRL")]
pub type Dmem0retnctrl = crate::Reg<dmem0retnctrl::Dmem0retnctrlSpec>;
#[doc = "Configure to provide general RAM retention configuration."]
pub mod dmem0retnctrl;
#[doc = "RAMBIASCONF (rw) register accessor: Configure RAM bias configure bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`rambiasconf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rambiasconf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rambiasconf`] module"]
#[doc(alias = "RAMBIASCONF")]
pub type Rambiasconf = crate::Reg<rambiasconf::RambiasconfSpec>;
#[doc = "Configure RAM bias configure bits."]
pub mod rambiasconf;
#[doc = "ICACHERAMRETNCTRL (rw) register accessor: Configure Host ICACHERAM retention configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`icacheramretnctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icacheramretnctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icacheramretnctrl`] module"]
#[doc(alias = "ICACHERAMRETNCTRL")]
pub type Icacheramretnctrl = crate::Reg<icacheramretnctrl::IcacheramretnctrlSpec>;
#[doc = "Configure Host ICACHERAM retention configuration."]
pub mod icacheramretnctrl;
#[doc = "DMEM0PORTMAPSEL (rw) register accessor: Configure DMEM0 port remap selection.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmem0portmapsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmem0portmapsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmem0portmapsel`] module"]
#[doc(alias = "DMEM0PORTMAPSEL")]
pub type Dmem0portmapsel = crate::Reg<dmem0portmapsel::Dmem0portmapselSpec>;
#[doc = "Configure DMEM0 port remap selection."]
pub mod dmem0portmapsel;
#[doc = "ROOTDATA0 (rw) register accessor: Generic data space for user to pass to root, e.g., address of struct in mem\n\nYou can [`read`](crate::Reg::read) this register and get [`rootdata0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rootdata0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rootdata0`] module"]
#[doc(alias = "ROOTDATA0")]
pub type Rootdata0 = crate::Reg<rootdata0::Rootdata0Spec>;
#[doc = "Generic data space for user to pass to root, e.g., address of struct in mem"]
pub mod rootdata0;
#[doc = "ROOTDATA1 (rw) register accessor: Generic data space for user to pass to root, e.g., address of struct in mem\n\nYou can [`read`](crate::Reg::read) this register and get [`rootdata1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rootdata1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rootdata1`] module"]
#[doc(alias = "ROOTDATA1")]
pub type Rootdata1 = crate::Reg<rootdata1::Rootdata1Spec>;
#[doc = "Generic data space for user to pass to root, e.g., address of struct in mem"]
pub mod rootdata1;
#[doc = "ROOTLOCKSTATUS (r) register accessor: This register returns the status of the SE managed locks.\n\nYou can [`read`](crate::Reg::read) this register and get [`rootlockstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rootlockstatus`] module"]
#[doc(alias = "ROOTLOCKSTATUS")]
pub type Rootlockstatus = crate::Reg<rootlockstatus::RootlockstatusSpec>;
#[doc = "This register returns the status of the SE managed locks."]
pub mod rootlockstatus;
#[doc = "ROOTSESWVERSION (rw) register accessor: SE Software version\n\nYou can [`read`](crate::Reg::read) this register and get [`rootseswversion::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rootseswversion::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rootseswversion`] module"]
#[doc(alias = "ROOTSESWVERSION")]
pub type Rootseswversion = crate::Reg<rootseswversion::RootseswversionSpec>;
#[doc = "SE Software version"]
pub mod rootseswversion;
