#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    en: En,
    cfg0: Cfg0,
    cfg1: Cfg1,
    cfg2: Cfg2,
    framecfg: Framecfg,
    dtxdatcfg: Dtxdatcfg,
    irhfcfg: Irhfcfg,
    irlfcfg: Irlfcfg,
    timingcfg: Timingcfg,
    startframecfg: Startframecfg,
    sigframecfg: Sigframecfg,
    clkdiv: Clkdiv,
    trigctrl: Trigctrl,
    cmd: Cmd,
    rxdata: Rxdata,
    rxdatap: Rxdatap,
    txdata: Txdata,
    status: Status,
    if_: If,
    ien: Ien,
    syncbusy: Syncbusy,
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
    pub const fn cfg0(&self) -> &Cfg0 {
        &self.cfg0
    }
    #[doc = "0x0c - No Description"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn cfg2(&self) -> &Cfg2 {
        &self.cfg2
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn framecfg(&self) -> &Framecfg {
        &self.framecfg
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn dtxdatcfg(&self) -> &Dtxdatcfg {
        &self.dtxdatcfg
    }
    #[doc = "0x1c - No Description"]
    #[inline(always)]
    pub const fn irhfcfg(&self) -> &Irhfcfg {
        &self.irhfcfg
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn irlfcfg(&self) -> &Irlfcfg {
        &self.irlfcfg
    }
    #[doc = "0x24 - No Description"]
    #[inline(always)]
    pub const fn timingcfg(&self) -> &Timingcfg {
        &self.timingcfg
    }
    #[doc = "0x28 - No Description"]
    #[inline(always)]
    pub const fn startframecfg(&self) -> &Startframecfg {
        &self.startframecfg
    }
    #[doc = "0x2c - No Description"]
    #[inline(always)]
    pub const fn sigframecfg(&self) -> &Sigframecfg {
        &self.sigframecfg
    }
    #[doc = "0x30 - No Description"]
    #[inline(always)]
    pub const fn clkdiv(&self) -> &Clkdiv {
        &self.clkdiv
    }
    #[doc = "0x34 - No Description"]
    #[inline(always)]
    pub const fn trigctrl(&self) -> &Trigctrl {
        &self.trigctrl
    }
    #[doc = "0x38 - No Description"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x3c - No Description"]
    #[inline(always)]
    pub const fn rxdata(&self) -> &Rxdata {
        &self.rxdata
    }
    #[doc = "0x40 - No Description"]
    #[inline(always)]
    pub const fn rxdatap(&self) -> &Rxdatap {
        &self.rxdatap
    }
    #[doc = "0x44 - No Description"]
    #[inline(always)]
    pub const fn txdata(&self) -> &Txdata {
        &self.txdata
    }
    #[doc = "0x48 - No Description"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x4c - No Description"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x50 - No Description"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x54 - No Description"]
    #[inline(always)]
    pub const fn syncbusy(&self) -> &Syncbusy {
        &self.syncbusy
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
#[doc = "CFG0 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`] module"]
#[doc(alias = "CFG0")]
pub type Cfg0 = crate::Reg<cfg0::Cfg0Spec>;
#[doc = "No Description"]
pub mod cfg0;
#[doc = "CFG1 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`] module"]
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "No Description"]
pub mod cfg1;
#[doc = "CFG2 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`] module"]
#[doc(alias = "CFG2")]
pub type Cfg2 = crate::Reg<cfg2::Cfg2Spec>;
#[doc = "No Description"]
pub mod cfg2;
#[doc = "FRAMECFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`framecfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`framecfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@framecfg`] module"]
#[doc(alias = "FRAMECFG")]
pub type Framecfg = crate::Reg<framecfg::FramecfgSpec>;
#[doc = "No Description"]
pub mod framecfg;
#[doc = "DTXDATCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxdatcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtxdatcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxdatcfg`] module"]
#[doc(alias = "DTXDATCFG")]
pub type Dtxdatcfg = crate::Reg<dtxdatcfg::DtxdatcfgSpec>;
#[doc = "No Description"]
pub mod dtxdatcfg;
#[doc = "IRHFCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`irhfcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irhfcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irhfcfg`] module"]
#[doc(alias = "IRHFCFG")]
pub type Irhfcfg = crate::Reg<irhfcfg::IrhfcfgSpec>;
#[doc = "No Description"]
pub mod irhfcfg;
#[doc = "IRLFCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`irlfcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irlfcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irlfcfg`] module"]
#[doc(alias = "IRLFCFG")]
pub type Irlfcfg = crate::Reg<irlfcfg::IrlfcfgSpec>;
#[doc = "No Description"]
pub mod irlfcfg;
#[doc = "TIMINGCFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`timingcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timingcfg`] module"]
#[doc(alias = "TIMINGCFG")]
pub type Timingcfg = crate::Reg<timingcfg::TimingcfgSpec>;
#[doc = "No Description"]
pub mod timingcfg;
#[doc = "STARTFRAMECFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`startframecfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`startframecfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@startframecfg`] module"]
#[doc(alias = "STARTFRAMECFG")]
pub type Startframecfg = crate::Reg<startframecfg::StartframecfgSpec>;
#[doc = "No Description"]
pub mod startframecfg;
#[doc = "SIGFRAMECFG (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`sigframecfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sigframecfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sigframecfg`] module"]
#[doc(alias = "SIGFRAMECFG")]
pub type Sigframecfg = crate::Reg<sigframecfg::SigframecfgSpec>;
#[doc = "No Description"]
pub mod sigframecfg;
#[doc = "CLKDIV (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`clkdiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkdiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkdiv`] module"]
#[doc(alias = "CLKDIV")]
pub type Clkdiv = crate::Reg<clkdiv::ClkdivSpec>;
#[doc = "No Description"]
pub mod clkdiv;
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
#[doc = "RXDATA (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdata`] module"]
#[doc(alias = "RXDATA")]
pub type Rxdata = crate::Reg<rxdata::RxdataSpec>;
#[doc = "No Description"]
pub mod rxdata;
#[doc = "RXDATAP (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdatap::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdatap`] module"]
#[doc(alias = "RXDATAP")]
pub type Rxdatap = crate::Reg<rxdatap::RxdatapSpec>;
#[doc = "No Description"]
pub mod rxdatap;
#[doc = "TXDATA (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdata`] module"]
#[doc(alias = "TXDATA")]
pub type Txdata = crate::Reg<txdata::TxdataSpec>;
#[doc = "No Description"]
pub mod txdata;
#[doc = "STATUS (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "No Description"]
pub mod status;
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
#[doc = "SYNCBUSY (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncbusy`] module"]
#[doc(alias = "SYNCBUSY")]
pub type Syncbusy = crate::Reg<syncbusy::SyncbusySpec>;
#[doc = "No Description"]
pub mod syncbusy;
