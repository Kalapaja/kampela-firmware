#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rfimlcdctrl: Rfimlcdctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn rfimlcdctrl(&self) -> &Rfimlcdctrl {
        &self.rfimlcdctrl
    }
}
#[doc = "RFIMLCDCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rfimlcdctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfimlcdctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfimlcdctrl`] module"]
#[doc(alias = "RFIMLCDCTRL")]
pub type Rfimlcdctrl = crate::Reg<rfimlcdctrl::RfimlcdctrlSpec>;
#[doc = "No Description"]
pub mod rfimlcdctrl;
