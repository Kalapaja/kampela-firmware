#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    ch0_reqsel: Ch0Reqsel,
    ch1_reqsel: Ch1Reqsel,
    ch2_reqsel: Ch2Reqsel,
    ch3_reqsel: Ch3Reqsel,
    ch4_reqsel: Ch4Reqsel,
    ch5_reqsel: Ch5Reqsel,
    ch6_reqsel: Ch6Reqsel,
    ch7_reqsel: Ch7Reqsel,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x04 - No Description"]
    #[inline(always)]
    pub const fn ch0_reqsel(&self) -> &Ch0Reqsel {
        &self.ch0_reqsel
    }
    #[doc = "0x08 - No Description"]
    #[inline(always)]
    pub const fn ch1_reqsel(&self) -> &Ch1Reqsel {
        &self.ch1_reqsel
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn ch2_reqsel(&self) -> &Ch2Reqsel {
        &self.ch2_reqsel
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn ch3_reqsel(&self) -> &Ch3Reqsel {
        &self.ch3_reqsel
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn ch4_reqsel(&self) -> &Ch4Reqsel {
        &self.ch4_reqsel
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn ch5_reqsel(&self) -> &Ch5Reqsel {
        &self.ch5_reqsel
    }
    #[doc = "0x1c - No Description"]
    #[inline(always)]
    pub const fn ch6_reqsel(&self) -> &Ch6Reqsel {
        &self.ch6_reqsel
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn ch7_reqsel(&self) -> &Ch7Reqsel {
        &self.ch7_reqsel
    }
}
#[doc = "IPVERSION (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "No Description"]
pub mod ipversion;
#[doc = "CH0_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch0_reqsel`] module"]
#[doc(alias = "CH0_REQSEL")]
pub type Ch0Reqsel = crate::Reg<ch0_reqsel::Ch0ReqselSpec>;
#[doc = "No Description"]
pub mod ch0_reqsel;
#[doc = "CH1_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch1_reqsel`] module"]
#[doc(alias = "CH1_REQSEL")]
pub type Ch1Reqsel = crate::Reg<ch1_reqsel::Ch1ReqselSpec>;
#[doc = "No Description"]
pub mod ch1_reqsel;
#[doc = "CH2_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch2_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch2_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch2_reqsel`] module"]
#[doc(alias = "CH2_REQSEL")]
pub type Ch2Reqsel = crate::Reg<ch2_reqsel::Ch2ReqselSpec>;
#[doc = "No Description"]
pub mod ch2_reqsel;
#[doc = "CH3_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch3_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch3_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch3_reqsel`] module"]
#[doc(alias = "CH3_REQSEL")]
pub type Ch3Reqsel = crate::Reg<ch3_reqsel::Ch3ReqselSpec>;
#[doc = "No Description"]
pub mod ch3_reqsel;
#[doc = "CH4_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch4_reqsel`] module"]
#[doc(alias = "CH4_REQSEL")]
pub type Ch4Reqsel = crate::Reg<ch4_reqsel::Ch4ReqselSpec>;
#[doc = "No Description"]
pub mod ch4_reqsel;
#[doc = "CH5_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch5_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch5_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch5_reqsel`] module"]
#[doc(alias = "CH5_REQSEL")]
pub type Ch5Reqsel = crate::Reg<ch5_reqsel::Ch5ReqselSpec>;
#[doc = "No Description"]
pub mod ch5_reqsel;
#[doc = "CH6_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch6_reqsel`] module"]
#[doc(alias = "CH6_REQSEL")]
pub type Ch6Reqsel = crate::Reg<ch6_reqsel::Ch6ReqselSpec>;
#[doc = "No Description"]
pub mod ch6_reqsel;
#[doc = "CH7_REQSEL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_reqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_reqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ch7_reqsel`] module"]
#[doc(alias = "CH7_REQSEL")]
pub type Ch7Reqsel = crate::Reg<ch7_reqsel::Ch7ReqselSpec>;
#[doc = "No Description"]
pub mod ch7_reqsel;
