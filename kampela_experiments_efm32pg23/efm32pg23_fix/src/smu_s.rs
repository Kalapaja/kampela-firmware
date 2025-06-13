#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    status: Status,
    lock: Lock,
    if_: If,
    ien: Ien,
    _reserved5: [u8; 0x0c],
    m33ctrl: M33ctrl,
    _reserved6: [u8; 0x1c],
    ppupatd0: Ppupatd0,
    ppupatd1: Ppupatd1,
    _reserved8: [u8; 0x18],
    ppusatd0: Ppusatd0,
    ppusatd1: Ppusatd1,
    _reserved10: [u8; 0xd8],
    ppufs: Ppufs,
    _reserved11: [u8; 0x0c],
    bmpupatd0: Bmpupatd0,
    _reserved12: [u8; 0x1c],
    bmpusatd0: Bmpusatd0,
    _reserved13: [u8; 0xdc],
    bmpufs: Bmpufs,
    bmpufsaddr: Bmpufsaddr,
    _reserved15: [u8; 0x08],
    esaurtypes0: Esaurtypes0,
    esaurtypes1: Esaurtypes1,
    _reserved17: [u8; 0x08],
    esaumrb01: Esaumrb01,
    esaumrb12: Esaumrb12,
    _reserved19: [u8; 0x08],
    esaumrb45: Esaumrb45,
    esaumrb56: Esaumrb56,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x04 - No Description"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x08 - No Description"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x20 - Holds the M33 control settings"]
    #[inline(always)]
    pub const fn m33ctrl(&self) -> &M33ctrl {
        &self.m33ctrl
    }
    #[doc = "0x40 - Set peripheral bits to 1 to mark as privileged access only"]
    #[inline(always)]
    pub const fn ppupatd0(&self) -> &Ppupatd0 {
        &self.ppupatd0
    }
    #[doc = "0x44 - Set peripheral bits to 1 to mark as privileged access only"]
    #[inline(always)]
    pub const fn ppupatd1(&self) -> &Ppupatd1 {
        &self.ppupatd1
    }
    #[doc = "0x60 - Set peripheral bits to 1 to mark as secure access only"]
    #[inline(always)]
    pub const fn ppusatd0(&self) -> &Ppusatd0 {
        &self.ppusatd0
    }
    #[doc = "0x64 - Set peripheral bits to 1 to mark as secure access only"]
    #[inline(always)]
    pub const fn ppusatd1(&self) -> &Ppusatd1 {
        &self.ppusatd1
    }
    #[doc = "0x140 - No Description"]
    #[inline(always)]
    pub const fn ppufs(&self) -> &Ppufs {
        &self.ppufs
    }
    #[doc = "0x150 - Set master bits to 1 to mark as a privileged master"]
    #[inline(always)]
    pub const fn bmpupatd0(&self) -> &Bmpupatd0 {
        &self.bmpupatd0
    }
    #[doc = "0x170 - Set master bits to 1 to mark as a secure master"]
    #[inline(always)]
    pub const fn bmpusatd0(&self) -> &Bmpusatd0 {
        &self.bmpusatd0
    }
    #[doc = "0x250 - No Description"]
    #[inline(always)]
    pub const fn bmpufs(&self) -> &Bmpufs {
        &self.bmpufs
    }
    #[doc = "0x254 - No Description"]
    #[inline(always)]
    pub const fn bmpufsaddr(&self) -> &Bmpufsaddr {
        &self.bmpufsaddr
    }
    #[doc = "0x260 - No Description"]
    #[inline(always)]
    pub const fn esaurtypes0(&self) -> &Esaurtypes0 {
        &self.esaurtypes0
    }
    #[doc = "0x264 - No Description"]
    #[inline(always)]
    pub const fn esaurtypes1(&self) -> &Esaurtypes1 {
        &self.esaurtypes1
    }
    #[doc = "0x270 - No Description"]
    #[inline(always)]
    pub const fn esaumrb01(&self) -> &Esaumrb01 {
        &self.esaumrb01
    }
    #[doc = "0x274 - No Description"]
    #[inline(always)]
    pub const fn esaumrb12(&self) -> &Esaumrb12 {
        &self.esaumrb12
    }
    #[doc = "0x280 - No Description"]
    #[inline(always)]
    pub const fn esaumrb45(&self) -> &Esaumrb45 {
        &self.esaumrb45
    }
    #[doc = "0x284 - No Description"]
    #[inline(always)]
    pub const fn esaumrb56(&self) -> &Esaumrb56 {
        &self.esaumrb56
    }
}
#[doc = "IPVERSION (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "STATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "No Description"]
pub mod status;
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
#[doc = "M33CTRL (rw) register accessor: Holds the M33 control settings\n\nYou can [`read`](crate::Reg::read) this register and get [`m33ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m33ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m33ctrl`] module"]
#[doc(alias = "M33CTRL")]
pub type M33ctrl = crate::Reg<m33ctrl::M33ctrlSpec>;
#[doc = "Holds the M33 control settings"]
pub mod m33ctrl;
#[doc = "PPUPATD0 (rw) register accessor: Set peripheral bits to 1 to mark as privileged access only\n\nYou can [`read`](crate::Reg::read) this register and get [`ppupatd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppupatd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppupatd0`] module"]
#[doc(alias = "PPUPATD0")]
pub type Ppupatd0 = crate::Reg<ppupatd0::Ppupatd0Spec>;
#[doc = "Set peripheral bits to 1 to mark as privileged access only"]
pub mod ppupatd0;
#[doc = "PPUPATD1 (rw) register accessor: Set peripheral bits to 1 to mark as privileged access only\n\nYou can [`read`](crate::Reg::read) this register and get [`ppupatd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppupatd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppupatd1`] module"]
#[doc(alias = "PPUPATD1")]
pub type Ppupatd1 = crate::Reg<ppupatd1::Ppupatd1Spec>;
#[doc = "Set peripheral bits to 1 to mark as privileged access only"]
pub mod ppupatd1;
#[doc = "PPUSATD0 (rw) register accessor: Set peripheral bits to 1 to mark as secure access only\n\nYou can [`read`](crate::Reg::read) this register and get [`ppusatd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppusatd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppusatd0`] module"]
#[doc(alias = "PPUSATD0")]
pub type Ppusatd0 = crate::Reg<ppusatd0::Ppusatd0Spec>;
#[doc = "Set peripheral bits to 1 to mark as secure access only"]
pub mod ppusatd0;
#[doc = "PPUSATD1 (rw) register accessor: Set peripheral bits to 1 to mark as secure access only\n\nYou can [`read`](crate::Reg::read) this register and get [`ppusatd1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppusatd1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppusatd1`] module"]
#[doc(alias = "PPUSATD1")]
pub type Ppusatd1 = crate::Reg<ppusatd1::Ppusatd1Spec>;
#[doc = "Set peripheral bits to 1 to mark as secure access only"]
pub mod ppusatd1;
#[doc = "PPUFS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ppufs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppufs`] module"]
#[doc(alias = "PPUFS")]
pub type Ppufs = crate::Reg<ppufs::PpufsSpec>;
#[doc = "No Description"]
pub mod ppufs;
#[doc = "BMPUPATD0 (rw) register accessor: Set master bits to 1 to mark as a privileged master\n\nYou can [`read`](crate::Reg::read) this register and get [`bmpupatd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmpupatd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmpupatd0`] module"]
#[doc(alias = "BMPUPATD0")]
pub type Bmpupatd0 = crate::Reg<bmpupatd0::Bmpupatd0Spec>;
#[doc = "Set master bits to 1 to mark as a privileged master"]
pub mod bmpupatd0;
#[doc = "BMPUSATD0 (rw) register accessor: Set master bits to 1 to mark as a secure master\n\nYou can [`read`](crate::Reg::read) this register and get [`bmpusatd0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmpusatd0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmpusatd0`] module"]
#[doc(alias = "BMPUSATD0")]
pub type Bmpusatd0 = crate::Reg<bmpusatd0::Bmpusatd0Spec>;
#[doc = "Set master bits to 1 to mark as a secure master"]
pub mod bmpusatd0;
#[doc = "BMPUFS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`bmpufs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmpufs`] module"]
#[doc(alias = "BMPUFS")]
pub type Bmpufs = crate::Reg<bmpufs::BmpufsSpec>;
#[doc = "No Description"]
pub mod bmpufs;
#[doc = "BMPUFSADDR (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`bmpufsaddr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmpufsaddr`] module"]
#[doc(alias = "BMPUFSADDR")]
pub type Bmpufsaddr = crate::Reg<bmpufsaddr::BmpufsaddrSpec>;
#[doc = "No Description"]
pub mod bmpufsaddr;
#[doc = "ESAURTYPES0 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`esaurtypes0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esaurtypes0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esaurtypes0`] module"]
#[doc(alias = "ESAURTYPES0")]
pub type Esaurtypes0 = crate::Reg<esaurtypes0::Esaurtypes0Spec>;
#[doc = "No Description"]
pub mod esaurtypes0;
#[doc = "ESAURTYPES1 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`esaurtypes1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esaurtypes1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esaurtypes1`] module"]
#[doc(alias = "ESAURTYPES1")]
pub type Esaurtypes1 = crate::Reg<esaurtypes1::Esaurtypes1Spec>;
#[doc = "No Description"]
pub mod esaurtypes1;
#[doc = "ESAUMRB01 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`esaumrb01::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esaumrb01::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esaumrb01`] module"]
#[doc(alias = "ESAUMRB01")]
pub type Esaumrb01 = crate::Reg<esaumrb01::Esaumrb01Spec>;
#[doc = "No Description"]
pub mod esaumrb01;
#[doc = "ESAUMRB12 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`esaumrb12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esaumrb12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esaumrb12`] module"]
#[doc(alias = "ESAUMRB12")]
pub type Esaumrb12 = crate::Reg<esaumrb12::Esaumrb12Spec>;
#[doc = "No Description"]
pub mod esaumrb12;
#[doc = "ESAUMRB45 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`esaumrb45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esaumrb45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esaumrb45`] module"]
#[doc(alias = "ESAUMRB45")]
pub type Esaumrb45 = crate::Reg<esaumrb45::Esaumrb45Spec>;
#[doc = "No Description"]
pub mod esaumrb45;
#[doc = "ESAUMRB56 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`esaumrb56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esaumrb56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@esaumrb56`] module"]
#[doc(alias = "ESAUMRB56")]
pub type Esaumrb56 = crate::Reg<esaumrb56::Esaumrb56Spec>;
#[doc = "No Description"]
pub mod esaumrb56;
