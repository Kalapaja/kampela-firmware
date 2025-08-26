#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    _reserved1: [u8; 0x04],
    status: Status,
    _reserved2: [u8; 0x04],
    lock: Lock,
    wdoglock: Wdoglock,
    _reserved4: [u8; 0x08],
    if_: If,
    ien: Ien,
    _reserved6: [u8; 0x28],
    calcmd: Calcmd,
    calctrl: Calctrl,
    calcnt: Calcnt,
    _reserved9: [u8; 0x08],
    clken0: Clken0,
    clken1: Clken1,
    _reserved11: [u8; 0x04],
    sysclkctrl: Sysclkctrl,
    _reserved12: [u8; 0x0c],
    traceclkctrl: Traceclkctrl,
    _reserved13: [u8; 0x0c],
    exportclkctrl: Exportclkctrl,
    _reserved14: [u8; 0x6c],
    dpllrefclkctrl: Dpllrefclkctrl,
    _reserved15: [u8; 0x1c],
    em01grpaclkctrl: Em01grpaclkctrl,
    _reserved16: [u8; 0x04],
    em01grpcclkctrl: Em01grpcclkctrl,
    _reserved17: [u8; 0x14],
    em23grpaclkctrl: Em23grpaclkctrl,
    _reserved18: [u8; 0x1c],
    em4grpaclkctrl: Em4grpaclkctrl,
    _reserved19: [u8; 0x1c],
    iadcclkctrl: Iadcclkctrl,
    _reserved20: [u8; 0x7c],
    wdog0clkctrl: Wdog0clkctrl,
    _reserved21: [u8; 0x04],
    wdog1clkctrl: Wdog1clkctrl,
    _reserved22: [u8; 0x14],
    eusart0clkctrl: Eusart0clkctrl,
    _reserved23: [u8; 0x1c],
    sysrtc0clkctrl: Sysrtc0clkctrl,
    _reserved24: [u8; 0x0c],
    lcdclkctrl: Lcdclkctrl,
    _reserved25: [u8; 0x0c],
    vdac0clkctrl: Vdac0clkctrl,
    _reserved26: [u8; 0x0c],
    pcnt0clkctrl: Pcnt0clkctrl,
    _reserved27: [u8; 0x1c],
    lesensehfclkctrl: Lesensehfclkctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - No Description"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x08 - No Description"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn wdoglock(&self) -> &Wdoglock {
        &self.wdoglock
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
    #[doc = "0x50 - No Description"]
    #[inline(always)]
    pub const fn calcmd(&self) -> &Calcmd {
        &self.calcmd
    }
    #[doc = "0x54 - No Description"]
    #[inline(always)]
    pub const fn calctrl(&self) -> &Calctrl {
        &self.calctrl
    }
    #[doc = "0x58 - No Description"]
    #[inline(always)]
    pub const fn calcnt(&self) -> &Calcnt {
        &self.calcnt
    }
    #[doc = "0x64 - No Description"]
    #[inline(always)]
    pub const fn clken0(&self) -> &Clken0 {
        &self.clken0
    }
    #[doc = "0x68 - No Description"]
    #[inline(always)]
    pub const fn clken1(&self) -> &Clken1 {
        &self.clken1
    }
    #[doc = "0x70 - No Description"]
    #[inline(always)]
    pub const fn sysclkctrl(&self) -> &Sysclkctrl {
        &self.sysclkctrl
    }
    #[doc = "0x80 - No Description"]
    #[inline(always)]
    pub const fn traceclkctrl(&self) -> &Traceclkctrl {
        &self.traceclkctrl
    }
    #[doc = "0x90 - No Description"]
    #[inline(always)]
    pub const fn exportclkctrl(&self) -> &Exportclkctrl {
        &self.exportclkctrl
    }
    #[doc = "0x100 - No Description"]
    #[inline(always)]
    pub const fn dpllrefclkctrl(&self) -> &Dpllrefclkctrl {
        &self.dpllrefclkctrl
    }
    #[doc = "0x120 - No Description"]
    #[inline(always)]
    pub const fn em01grpaclkctrl(&self) -> &Em01grpaclkctrl {
        &self.em01grpaclkctrl
    }
    #[doc = "0x128 - No Description"]
    #[inline(always)]
    pub const fn em01grpcclkctrl(&self) -> &Em01grpcclkctrl {
        &self.em01grpcclkctrl
    }
    #[doc = "0x140 - No Description"]
    #[inline(always)]
    pub const fn em23grpaclkctrl(&self) -> &Em23grpaclkctrl {
        &self.em23grpaclkctrl
    }
    #[doc = "0x160 - No Description"]
    #[inline(always)]
    pub const fn em4grpaclkctrl(&self) -> &Em4grpaclkctrl {
        &self.em4grpaclkctrl
    }
    #[doc = "0x180 - No Description"]
    #[inline(always)]
    pub const fn iadcclkctrl(&self) -> &Iadcclkctrl {
        &self.iadcclkctrl
    }
    #[doc = "0x200 - No Description"]
    #[inline(always)]
    pub const fn wdog0clkctrl(&self) -> &Wdog0clkctrl {
        &self.wdog0clkctrl
    }
    #[doc = "0x208 - No Description"]
    #[inline(always)]
    pub const fn wdog1clkctrl(&self) -> &Wdog1clkctrl {
        &self.wdog1clkctrl
    }
    #[doc = "0x220 - No Description"]
    #[inline(always)]
    pub const fn eusart0clkctrl(&self) -> &Eusart0clkctrl {
        &self.eusart0clkctrl
    }
    #[doc = "0x240 - No Description"]
    #[inline(always)]
    pub const fn sysrtc0clkctrl(&self) -> &Sysrtc0clkctrl {
        &self.sysrtc0clkctrl
    }
    #[doc = "0x250 - No Description"]
    #[inline(always)]
    pub const fn lcdclkctrl(&self) -> &Lcdclkctrl {
        &self.lcdclkctrl
    }
    #[doc = "0x260 - No Description"]
    #[inline(always)]
    pub const fn vdac0clkctrl(&self) -> &Vdac0clkctrl {
        &self.vdac0clkctrl
    }
    #[doc = "0x270 - No Description"]
    #[inline(always)]
    pub const fn pcnt0clkctrl(&self) -> &Pcnt0clkctrl {
        &self.pcnt0clkctrl
    }
    #[doc = "0x290 - No Description"]
    #[inline(always)]
    pub const fn lesensehfclkctrl(&self) -> &Lesensehfclkctrl {
        &self.lesensehfclkctrl
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
#[doc = "WDOGLOCK (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdoglock::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdoglock`] module"]
#[doc(alias = "WDOGLOCK")]
pub type Wdoglock = crate::Reg<wdoglock::WdoglockSpec>;
#[doc = "No Description"]
pub mod wdoglock;
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
#[doc = "CALCMD (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calcmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calcmd`] module"]
#[doc(alias = "CALCMD")]
pub type Calcmd = crate::Reg<calcmd::CalcmdSpec>;
#[doc = "No Description"]
pub mod calcmd;
#[doc = "CALCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`calctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calctrl`] module"]
#[doc(alias = "CALCTRL")]
pub type Calctrl = crate::Reg<calctrl::CalctrlSpec>;
#[doc = "No Description"]
pub mod calctrl;
#[doc = "CALCNT (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`calcnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calcnt`] module"]
#[doc(alias = "CALCNT")]
pub type Calcnt = crate::Reg<calcnt::CalcntSpec>;
#[doc = "No Description"]
pub mod calcnt;
#[doc = "CLKEN0 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`clken0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clken0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clken0`] module"]
#[doc(alias = "CLKEN0")]
pub type Clken0 = crate::Reg<clken0::Clken0Spec>;
#[doc = "No Description"]
pub mod clken0;
#[doc = "CLKEN1 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`clken1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clken1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clken1`] module"]
#[doc(alias = "CLKEN1")]
pub type Clken1 = crate::Reg<clken1::Clken1Spec>;
#[doc = "No Description"]
pub mod clken1;
#[doc = "SYSCLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sysclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysclkctrl`] module"]
#[doc(alias = "SYSCLKCTRL")]
pub type Sysclkctrl = crate::Reg<sysclkctrl::SysclkctrlSpec>;
#[doc = "No Description"]
pub mod sysclkctrl;
#[doc = "TRACECLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`traceclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`traceclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@traceclkctrl`] module"]
#[doc(alias = "TRACECLKCTRL")]
pub type Traceclkctrl = crate::Reg<traceclkctrl::TraceclkctrlSpec>;
#[doc = "No Description"]
pub mod traceclkctrl;
#[doc = "EXPORTCLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`exportclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exportclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exportclkctrl`] module"]
#[doc(alias = "EXPORTCLKCTRL")]
pub type Exportclkctrl = crate::Reg<exportclkctrl::ExportclkctrlSpec>;
#[doc = "No Description"]
pub mod exportclkctrl;
#[doc = "DPLLREFCLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dpllrefclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpllrefclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpllrefclkctrl`] module"]
#[doc(alias = "DPLLREFCLKCTRL")]
pub type Dpllrefclkctrl = crate::Reg<dpllrefclkctrl::DpllrefclkctrlSpec>;
#[doc = "No Description"]
pub mod dpllrefclkctrl;
#[doc = "EM01GRPACLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em01grpaclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em01grpaclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em01grpaclkctrl`] module"]
#[doc(alias = "EM01GRPACLKCTRL")]
pub type Em01grpaclkctrl = crate::Reg<em01grpaclkctrl::Em01grpaclkctrlSpec>;
#[doc = "No Description"]
pub mod em01grpaclkctrl;
#[doc = "EM01GRPCCLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em01grpcclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em01grpcclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em01grpcclkctrl`] module"]
#[doc(alias = "EM01GRPCCLKCTRL")]
pub type Em01grpcclkctrl = crate::Reg<em01grpcclkctrl::Em01grpcclkctrlSpec>;
#[doc = "No Description"]
pub mod em01grpcclkctrl;
#[doc = "EM23GRPACLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em23grpaclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em23grpaclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em23grpaclkctrl`] module"]
#[doc(alias = "EM23GRPACLKCTRL")]
pub type Em23grpaclkctrl = crate::Reg<em23grpaclkctrl::Em23grpaclkctrlSpec>;
#[doc = "No Description"]
pub mod em23grpaclkctrl;
#[doc = "EM4GRPACLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em4grpaclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4grpaclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@em4grpaclkctrl`] module"]
#[doc(alias = "EM4GRPACLKCTRL")]
pub type Em4grpaclkctrl = crate::Reg<em4grpaclkctrl::Em4grpaclkctrlSpec>;
#[doc = "No Description"]
pub mod em4grpaclkctrl;
#[doc = "IADCCLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`iadcclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iadcclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iadcclkctrl`] module"]
#[doc(alias = "IADCCLKCTRL")]
pub type Iadcclkctrl = crate::Reg<iadcclkctrl::IadcclkctrlSpec>;
#[doc = "No Description"]
pub mod iadcclkctrl;
#[doc = "WDOG0CLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`wdog0clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdog0clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdog0clkctrl`] module"]
#[doc(alias = "WDOG0CLKCTRL")]
pub type Wdog0clkctrl = crate::Reg<wdog0clkctrl::Wdog0clkctrlSpec>;
#[doc = "No Description"]
pub mod wdog0clkctrl;
#[doc = "WDOG1CLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`wdog1clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdog1clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdog1clkctrl`] module"]
#[doc(alias = "WDOG1CLKCTRL")]
pub type Wdog1clkctrl = crate::Reg<wdog1clkctrl::Wdog1clkctrlSpec>;
#[doc = "No Description"]
pub mod wdog1clkctrl;
#[doc = "EUSART0CLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart0clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart0clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eusart0clkctrl`] module"]
#[doc(alias = "EUSART0CLKCTRL")]
pub type Eusart0clkctrl = crate::Reg<eusart0clkctrl::Eusart0clkctrlSpec>;
#[doc = "No Description"]
pub mod eusart0clkctrl;
#[doc = "SYSRTC0CLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sysrtc0clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sysrtc0clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sysrtc0clkctrl`] module"]
#[doc(alias = "SYSRTC0CLKCTRL")]
pub type Sysrtc0clkctrl = crate::Reg<sysrtc0clkctrl::Sysrtc0clkctrlSpec>;
#[doc = "No Description"]
pub mod sysrtc0clkctrl;
#[doc = "LCDCLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcdclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcdclkctrl`] module"]
#[doc(alias = "LCDCLKCTRL")]
pub type Lcdclkctrl = crate::Reg<lcdclkctrl::LcdclkctrlSpec>;
#[doc = "No Description"]
pub mod lcdclkctrl;
#[doc = "VDAC0CLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`vdac0clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vdac0clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vdac0clkctrl`] module"]
#[doc(alias = "VDAC0CLKCTRL")]
pub type Vdac0clkctrl = crate::Reg<vdac0clkctrl::Vdac0clkctrlSpec>;
#[doc = "No Description"]
pub mod vdac0clkctrl;
#[doc = "PCNT0CLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnt0clkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnt0clkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcnt0clkctrl`] module"]
#[doc(alias = "PCNT0CLKCTRL")]
pub type Pcnt0clkctrl = crate::Reg<pcnt0clkctrl::Pcnt0clkctrlSpec>;
#[doc = "No Description"]
pub mod pcnt0clkctrl;
#[doc = "LESENSEHFCLKCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`lesensehfclkctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesensehfclkctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lesensehfclkctrl`] module"]
#[doc(alias = "LESENSEHFCLKCTRL")]
pub type Lesensehfclkctrl = crate::Reg<lesensehfclkctrl::LesensehfclkctrlSpec>;
#[doc = "No Description"]
pub mod lesensehfclkctrl;
