#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    en: En,
    ctrl: Ctrl,
    frame: Frame,
    trigctrl: Trigctrl,
    cmd: Cmd,
    status: Status,
    clkdiv: Clkdiv,
    rxdatax: Rxdatax,
    rxdata: Rxdata,
    rxdoublex: Rxdoublex,
    rxdouble: Rxdouble,
    rxdataxp: Rxdataxp,
    rxdoublexp: Rxdoublexp,
    txdatax: Txdatax,
    txdata: Txdata,
    txdoublex: Txdoublex,
    txdouble: Txdouble,
    if_: If,
    ien: Ien,
    irctrl: Irctrl,
    i2sctrl: I2sctrl,
    timing: Timing,
    ctrlx: Ctrlx,
    timecmp0: Timecmp0,
    timecmp1: Timecmp1,
    timecmp2: Timecmp2,
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
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn frame(&self) -> &Frame {
        &self.frame
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn trigctrl(&self) -> &Trigctrl {
        &self.trigctrl
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x1c - No Description"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn rxdatax(&self) -> &Rxdatax {
        &self.rxdatax
    }
    #[doc = "0x24 - No Description"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x28 - No Description"]
    #[inline(always)]
    pub const fn rxdoublex(&self) -> &Rxdoublex {
        &self.rxdoublex
    }
    #[doc = "0x2c - No Description"]
    #[inline(always)]
    pub const fn rxdouble(&self) -> &Rxdouble {
        &self.rxdouble
    }
    #[doc = "0x30 - No Description"]
    #[inline(always)]
    pub const fn rxdataxp(&self) -> &Rxdataxp {
        &self.rxdataxp
    }
    #[doc = "0x34 - No Description"]
    #[inline(always)]
    pub const fn rxdoublexp(&self) -> &Rxdoublexp {
        &self.rxdoublexp
    }
    #[doc = "0x38 - No Description"]
    #[inline(always)]
    pub const fn txdatax(&self) -> &Txdatax {
        &self.txdatax
    }
    #[doc = "0x3c - No Description"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x40 - No Description"]
    #[inline(always)]
    pub const fn txdoublex(&self) -> &Txdoublex {
        &self.txdoublex
    }
    #[doc = "0x44 - No Description"]
    #[inline(always)]
    pub const fn txdouble(&self) -> &Txdouble {
        &self.txdouble
    }
    #[doc = "0x48 - No Description"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x4c - No Description"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x50 - No Description"]
    #[inline(always)]
    pub const fn irctrl(&self) -> &Irctrl {
        &self.irctrl
    }
    #[doc = "0x54 - No Description"]
    #[inline(always)]
    pub const fn i2sctrl(&self) -> &I2sctrl {
        &self.i2sctrl
    }
    #[doc = "0x58 - No Description"]
    #[inline(always)]
    pub const fn timing(&self) -> &Timing {
        &self.timing
    }
    #[doc = "0x5c - No Description"]
    #[inline(always)]
    pub const fn ctrlx(&self) -> &Ctrlx {
        &self.ctrlx
    }
    #[doc = "0x60 - No Description"]
    #[inline(always)]
    pub const fn timecmp0(&self) -> &Timecmp0 {
        &self.timecmp0
    }
    #[doc = "0x64 - No Description"]
    #[inline(always)]
    pub const fn timecmp1(&self) -> &Timecmp1 {
        &self.timecmp1
    }
    #[doc = "0x68 - No Description"]
    #[inline(always)]
    pub const fn timecmp2(&self) -> &Timecmp2 {
        &self.timecmp2
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
#[doc = "CTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "No Description"]
pub mod ctrl;
#[doc = "FRAME (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`frame::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frame`] module"]
#[doc(alias = "FRAME")]
pub type Frame = crate::Reg<frame::FrameSpec>;
#[doc = "No Description"]
pub mod frame;
#[doc = "TRIGCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`trigctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigctrl`] module"]
#[doc(alias = "TRIGCTRL")]
pub type Trigctrl = crate::Reg<trigctrl::TrigctrlSpec>;
#[doc = "No Description"]
pub mod trigctrl;
#[doc = "CMD (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "STATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "No Description"]
pub mod status;
#[doc = "CLKDIV (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`] module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "No Description"]
pub mod clkdiv;
#[doc = "RXDATAX (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdatax::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdatax`] module"]
#[doc(alias = "RXDATAX")]
pub type Rxdatax = crate::Reg<rxdatax::RxdataxSpec>;
#[doc = "No Description"]
pub mod rxdatax;
#[doc = "RXDATA (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`] module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "No Description"]
pub mod rxdata;
#[doc = "RXDOUBLEX (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdoublex::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdoublex`] module"]
#[doc(alias = "RXDOUBLEX")]
pub type Rxdoublex = crate::Reg<rxdoublex::RxdoublexSpec>;
#[doc = "No Description"]
pub mod rxdoublex;
#[doc = "RXDOUBLE (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdouble::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdouble`] module"]
#[doc(alias = "RXDOUBLE")]
pub type Rxdouble = crate::Reg<rxdouble::RxdoubleSpec>;
#[doc = "No Description"]
pub mod rxdouble;
#[doc = "RXDATAXP (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdataxp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdataxp`] module"]
#[doc(alias = "RXDATAXP")]
pub type Rxdataxp = crate::Reg<rxdataxp::RxdataxpSpec>;
#[doc = "No Description"]
pub mod rxdataxp;
#[doc = "RXDOUBLEXP (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdoublexp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdoublexp`] module"]
#[doc(alias = "RXDOUBLEXP")]
pub type Rxdoublexp = crate::Reg<rxdoublexp::RxdoublexpSpec>;
#[doc = "No Description"]
pub mod rxdoublexp;
#[doc = "TXDATAX (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdatax::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdatax`] module"]
#[doc(alias = "TXDATAX")]
pub type Txdatax = crate::Reg<txdatax::TxdataxSpec>;
#[doc = "No Description"]
pub mod txdatax;
#[doc = "TXDATA (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`] module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "No Description"]
pub mod txdata;
#[doc = "TXDOUBLEX (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdoublex::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdoublex`] module"]
#[doc(alias = "TXDOUBLEX")]
pub type Txdoublex = crate::Reg<txdoublex::TxdoublexSpec>;
#[doc = "No Description"]
pub mod txdoublex;
#[doc = "TXDOUBLE (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdouble::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdouble`] module"]
#[doc(alias = "TXDOUBLE")]
pub type Txdouble = crate::Reg<txdouble::TxdoubleSpec>;
#[doc = "No Description"]
pub mod txdouble;
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
#[doc = "IRCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`irctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irctrl`] module"]
#[doc(alias = "IRCTRL")]
pub type Irctrl = crate::Reg<irctrl::IrctrlSpec>;
#[doc = "No Description"]
pub mod irctrl;
#[doc = "I2SCTRL (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`i2sctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2sctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2sctrl`] module"]
#[doc(alias = "I2SCTRL")]
pub type I2sctrl = crate::Reg<i2sctrl::I2sctrlSpec>;
#[doc = "No Description"]
pub mod i2sctrl;
#[doc = "TIMING (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`timing::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timing`] module"]
#[doc(alias = "TIMING")]
pub type Timing = crate::Reg<timing::TimingSpec>;
#[doc = "No Description"]
pub mod timing;
#[doc = "CTRLX (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrlx`] module"]
#[doc(alias = "CTRLX")]
pub type Ctrlx = crate::Reg<ctrlx::CtrlxSpec>;
#[doc = "No Description"]
pub mod ctrlx;
#[doc = "TIMECMP0 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`timecmp0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timecmp0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timecmp0`] module"]
#[doc(alias = "TIMECMP0")]
pub type Timecmp0 = crate::Reg<timecmp0::Timecmp0Spec>;
#[doc = "No Description"]
pub mod timecmp0;
#[doc = "TIMECMP1 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`timecmp1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timecmp1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timecmp1`] module"]
#[doc(alias = "TIMECMP1")]
pub type Timecmp1 = crate::Reg<timecmp1::Timecmp1Spec>;
#[doc = "No Description"]
pub mod timecmp1;
#[doc = "TIMECMP2 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`timecmp2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timecmp2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timecmp2`] module"]
#[doc(alias = "TIMECMP2")]
pub type Timecmp2 = crate::Reg<timecmp2::Timecmp2Spec>;
#[doc = "No Description"]
pub mod timecmp2;
