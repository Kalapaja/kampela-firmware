#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sreg0: Sreg0,
    sreg1: Sreg1,
}
impl RegisterBlock {
    #[doc = "0x00 - Used for SIMCTRL Pointer in Verification Environment"]
    #[inline(always)]
    pub const fn sreg0(&self) -> &Sreg0 {
        &self.sreg0
    }
    #[doc = "0x04 - Used for SIMCTRL Data Access in Verification Environment"]
    #[inline(always)]
    pub const fn sreg1(&self) -> &Sreg1 {
        &self.sreg1
    }
}
#[doc = "SREG0 (rw) register accessor: Used for SIMCTRL Pointer in Verification Environment\n\nYou can [`read`](crate::Reg::read) this register and get [`sreg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sreg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sreg0`] module"]
#[doc(alias = "SREG0")]
pub type Sreg0 = crate::Reg<sreg0::Sreg0Spec>;
#[doc = "Used for SIMCTRL Pointer in Verification Environment"]
pub mod sreg0;
#[doc = "SREG1 (rw) register accessor: Used for SIMCTRL Data Access in Verification Environment\n\nYou can [`read`](crate::Reg::read) this register and get [`sreg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sreg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sreg1`] module"]
#[doc(alias = "SREG1")]
pub type Sreg1 = crate::Reg<sreg1::Sreg1Spec>;
#[doc = "Used for SIMCTRL Data Access in Verification Environment"]
pub mod sreg1;
