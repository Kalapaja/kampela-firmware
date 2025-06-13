#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    msgptr0: Msgptr0,
    msgptr1: Msgptr1,
    msgptr2: Msgptr2,
    msgptr3: Msgptr3,
    _reserved4: [u8; 0x30],
    if_: If,
    ien: Ien,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn msgptr0(&self) -> &Msgptr0 {
        &self.msgptr0
    }
    #[doc = "0x04 - No Description"]
    #[inline(always)]
    pub const fn msgptr1(&self) -> &Msgptr1 {
        &self.msgptr1
    }
    #[doc = "0x08 - No Description"]
    #[inline(always)]
    pub const fn msgptr2(&self) -> &Msgptr2 {
        &self.msgptr2
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn msgptr3(&self) -> &Msgptr3 {
        &self.msgptr3
    }
    #[doc = "0x40 - No Description"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x44 - No Description"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
}
#[doc = "MSGPTR0 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`msgptr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msgptr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgptr0`] module"]
#[doc(alias = "MSGPTR0")]
pub type Msgptr0 = crate::Reg<msgptr0::Msgptr0Spec>;
#[doc = "No Description"]
pub mod msgptr0;
#[doc = "MSGPTR1 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`msgptr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msgptr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgptr1`] module"]
#[doc(alias = "MSGPTR1")]
pub type Msgptr1 = crate::Reg<msgptr1::Msgptr1Spec>;
#[doc = "No Description"]
pub mod msgptr1;
#[doc = "MSGPTR2 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`msgptr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msgptr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgptr2`] module"]
#[doc(alias = "MSGPTR2")]
pub type Msgptr2 = crate::Reg<msgptr2::Msgptr2Spec>;
#[doc = "No Description"]
pub mod msgptr2;
#[doc = "MSGPTR3 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`msgptr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msgptr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msgptr3`] module"]
#[doc(alias = "MSGPTR3")]
pub type Msgptr3 = crate::Reg<msgptr3::Msgptr3Spec>;
#[doc = "No Description"]
pub mod msgptr3;
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
