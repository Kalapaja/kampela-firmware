#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    en: En,
    swrst: Swrst,
    ctrl: Ctrl,
    cmd: Cmd,
    dispctrl: Dispctrl,
    bacfg: Bacfg,
    bactrl: Bactrl,
    status: Status,
    arega: Arega,
    aregb: Aregb,
    if_: If,
    ien: Ien,
    biasctrl: Biasctrl,
    dispctrlx: Dispctrlx,
    _reserved15: [u8; 0x04],
    segd0: Segd0,
    _reserved16: [u8; 0x04],
    segd1: Segd1,
    _reserved17: [u8; 0x04],
    segd2: Segd2,
    _reserved18: [u8; 0x04],
    segd3: Segd3,
    _reserved19: [u8; 0x64],
    updatectrl: Updatectrl,
    _reserved20: [u8; 0x2c],
    framerate: Framerate,
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
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn dispctrl(&self) -> &Dispctrl {
        &self.dispctrl
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn bacfg(&self) -> &Bacfg {
        &self.bacfg
    }
    #[doc = "0x1c - No Description"]
    #[inline(always)]
    pub const fn bactrl(&self) -> &Bactrl {
        &self.bactrl
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x24 - No Description"]
    #[inline(always)]
    pub const fn arega(&self) -> &Arega {
        &self.arega
    }
    #[doc = "0x28 - No Description"]
    #[inline(always)]
    pub const fn aregb(&self) -> &Aregb {
        &self.aregb
    }
    #[doc = "0x2c - No Description"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x30 - No Description"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x34 - No Description"]
    #[inline(always)]
    pub const fn biasctrl(&self) -> &Biasctrl {
        &self.biasctrl
    }
    #[doc = "0x38 - No Description"]
    #[inline(always)]
    pub const fn dispctrlx(&self) -> &Dispctrlx {
        &self.dispctrlx
    }
    #[doc = "0x40 - No Description"]
    #[inline(always)]
    pub const fn segd0(&self) -> &Segd0 {
        &self.segd0
    }
    #[doc = "0x48 - No Description"]
    #[inline(always)]
    pub const fn segd1(&self) -> &Segd1 {
        &self.segd1
    }
    #[doc = "0x50 - No Description"]
    #[inline(always)]
    pub const fn segd2(&self) -> &Segd2 {
        &self.segd2
    }
    #[doc = "0x58 - No Description"]
    #[inline(always)]
    pub const fn segd3(&self) -> &Segd3 {
        &self.segd3
    }
    #[doc = "0xc0 - No Description"]
    #[inline(always)]
    pub const fn updatectrl(&self) -> &Updatectrl {
        &self.updatectrl
    }
    #[doc = "0xf0 - No Description"]
    #[inline(always)]
    pub const fn framerate(&self) -> &Framerate {
        &self.framerate
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
#[doc = "DISPCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dispctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dispctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dispctrl`] module"]
#[doc(alias = "DISPCTRL")]
pub type Dispctrl = crate::Reg<dispctrl::DispctrlSpec>;
#[doc = "No Description"]
pub mod dispctrl;
#[doc = "BACFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`bacfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bacfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bacfg`] module"]
#[doc(alias = "BACFG")]
pub type Bacfg = crate::Reg<bacfg::BacfgSpec>;
#[doc = "No Description"]
pub mod bacfg;
#[doc = "BACTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`bactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bactrl`] module"]
#[doc(alias = "BACTRL")]
pub type Bactrl = crate::Reg<bactrl::BactrlSpec>;
#[doc = "No Description"]
pub mod bactrl;
#[doc = "STATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "No Description"]
pub mod status;
#[doc = "AREGA (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`arega::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arega::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arega`] module"]
#[doc(alias = "AREGA")]
pub type Arega = crate::Reg<arega::AregaSpec>;
#[doc = "No Description"]
pub mod arega;
#[doc = "AREGB (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`aregb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aregb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aregb`] module"]
#[doc(alias = "AREGB")]
pub type Aregb = crate::Reg<aregb::AregbSpec>;
#[doc = "No Description"]
pub mod aregb;
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
#[doc = "BIASCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`biasctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`biasctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@biasctrl`] module"]
#[doc(alias = "BIASCTRL")]
pub type Biasctrl = crate::Reg<biasctrl::BiasctrlSpec>;
#[doc = "No Description"]
pub mod biasctrl;
#[doc = "DISPCTRLX (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dispctrlx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dispctrlx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dispctrlx`] module"]
#[doc(alias = "DISPCTRLX")]
pub type Dispctrlx = crate::Reg<dispctrlx::DispctrlxSpec>;
#[doc = "No Description"]
pub mod dispctrlx;
#[doc = "SEGD0 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`segd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd0`] module"]
#[doc(alias = "SEGD0")]
pub type Segd0 = crate::Reg<segd0::Segd0Spec>;
#[doc = "No Description"]
pub mod segd0;
#[doc = "SEGD1 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`segd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd1`] module"]
#[doc(alias = "SEGD1")]
pub type Segd1 = crate::Reg<segd1::Segd1Spec>;
#[doc = "No Description"]
pub mod segd1;
#[doc = "SEGD2 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`segd2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd2`] module"]
#[doc(alias = "SEGD2")]
pub type Segd2 = crate::Reg<segd2::Segd2Spec>;
#[doc = "No Description"]
pub mod segd2;
#[doc = "SEGD3 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`segd3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@segd3`] module"]
#[doc(alias = "SEGD3")]
pub type Segd3 = crate::Reg<segd3::Segd3Spec>;
#[doc = "No Description"]
pub mod segd3;
#[doc = "UPDATECTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`updatectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`updatectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@updatectrl`] module"]
#[doc(alias = "UPDATECTRL")]
pub type Updatectrl = crate::Reg<updatectrl::UpdatectrlSpec>;
#[doc = "No Description"]
pub mod updatectrl;
#[doc = "FRAMERATE (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`framerate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framerate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framerate`] module"]
#[doc(alias = "FRAMERATE")]
pub type Framerate = crate::Reg<framerate::FramerateSpec>;
#[doc = "No Description"]
pub mod framerate;
