#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    en: En,
    ctrl: Ctrl,
    cmd: Cmd,
    timer: Timer,
    status: Status,
    maskreq: Maskreq,
    stmask: Stmask,
    cmpthr: Cmpthr,
    if_: If,
    ien: Ien,
    trigger: Trigger,
    _reserved12: [u8; 0x18],
    cfg0: Cfg0,
    _reserved13: [u8; 0x04],
    scale0: Scale0,
    sched0: Sched0,
    cfg1: Cfg1,
    _reserved16: [u8; 0x04],
    scale1: Scale1,
    sched1: Sched1,
    _reserved18: [u8; 0x08],
    singlefifocfg: Singlefifocfg,
    singlefifodata: Singlefifodata,
    singlefifostat: Singlefifostat,
    singledata: Singledata,
    scanfifocfg: Scanfifocfg,
    scanfifodata: Scanfifodata,
    scanfifostat: Scanfifostat,
    scandata: Scandata,
    _reserved26: [u8; 0x08],
    single: Single,
    _reserved27: [u8; 0x04],
    scan0: Scan0,
    scan1: Scan1,
    scan2: Scan2,
    scan3: Scan3,
    scan4: Scan4,
    scan5: Scan5,
    scan6: Scan6,
    scan7: Scan7,
    scan8: Scan8,
    scan9: Scan9,
    scan10: Scan10,
    scan11: Scan11,
    scan12: Scan12,
    scan13: Scan13,
    scan14: Scan14,
    scan15: Scan15,
}
impl RegisterBlock {
    #[doc = "0x00 - IPVERSION"]
    #[inline(always)]
    pub const fn ipversion(&self) -> &Ipversion {
        &self.ipversion
    }
    #[doc = "0x04 - Enable"]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
    #[doc = "0x08 - Control"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - Command"]
    #[inline(always)]
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x10 - Timer"]
    #[inline(always)]
    pub const fn timer(&self) -> &Timer {
        &self.timer
    }
    #[doc = "0x14 - Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x18 - Mask Request"]
    #[inline(always)]
    pub const fn maskreq(&self) -> &Maskreq {
        &self.maskreq
    }
    #[doc = "0x1c - Scan Table Mask"]
    #[inline(always)]
    pub const fn stmask(&self) -> &Stmask {
        &self.stmask
    }
    #[doc = "0x20 - Comparator Threshold"]
    #[inline(always)]
    pub const fn cmpthr(&self) -> &Cmpthr {
        &self.cmpthr
    }
    #[doc = "0x24 - Interrupt Flag"]
    #[inline(always)]
    pub const fn if_(&self) -> &If {
        &self.if_
    }
    #[doc = "0x28 - Interrupt Enable"]
    #[inline(always)]
    pub const fn ien(&self) -> &Ien {
        &self.ien
    }
    #[doc = "0x2c - Trigger"]
    #[inline(always)]
    pub const fn trigger(&self) -> &Trigger {
        &self.trigger
    }
    #[doc = "0x48 - Configration"]
    #[inline(always)]
    pub const fn cfg0(&self) -> &Cfg0 {
        &self.cfg0
    }
    #[doc = "0x50 - Scale"]
    #[inline(always)]
    pub const fn scale0(&self) -> &Scale0 {
        &self.scale0
    }
    #[doc = "0x54 - Scheduling"]
    #[inline(always)]
    pub const fn sched0(&self) -> &Sched0 {
        &self.sched0
    }
    #[doc = "0x58 - Configration"]
    #[inline(always)]
    pub const fn cfg1(&self) -> &Cfg1 {
        &self.cfg1
    }
    #[doc = "0x60 - Scale"]
    #[inline(always)]
    pub const fn scale1(&self) -> &Scale1 {
        &self.scale1
    }
    #[doc = "0x64 - Scheduling"]
    #[inline(always)]
    pub const fn sched1(&self) -> &Sched1 {
        &self.sched1
    }
    #[doc = "0x70 - Single FIFO Configuration"]
    #[inline(always)]
    pub const fn singlefifocfg(&self) -> &Singlefifocfg {
        &self.singlefifocfg
    }
    #[doc = "0x74 - Read the oldest valid data from the single FIFO and pop the FIFO"]
    #[inline(always)]
    pub const fn singlefifodata(&self) -> &Singlefifodata {
        &self.singlefifodata
    }
    #[doc = "0x78 - Single FIFO status"]
    #[inline(always)]
    pub const fn singlefifostat(&self) -> &Singlefifostat {
        &self.singlefifostat
    }
    #[doc = "0x7c - latest single queue conversion data"]
    #[inline(always)]
    pub const fn singledata(&self) -> &Singledata {
        &self.singledata
    }
    #[doc = "0x80 - SCAN FIFO configuration"]
    #[inline(always)]
    pub const fn scanfifocfg(&self) -> &Scanfifocfg {
        &self.scanfifocfg
    }
    #[doc = "0x84 - Read the oldest valid data from the scan FIFO and pop the FIFO"]
    #[inline(always)]
    pub const fn scanfifodata(&self) -> &Scanfifodata {
        &self.scanfifodata
    }
    #[doc = "0x88 - Scan FIFO status"]
    #[inline(always)]
    pub const fn scanfifostat(&self) -> &Scanfifostat {
        &self.scanfifostat
    }
    #[doc = "0x8c - Most recent data data from scan queue conversion"]
    #[inline(always)]
    pub const fn scandata(&self) -> &Scandata {
        &self.scandata
    }
    #[doc = "0x98 - No Description"]
    #[inline(always)]
    pub const fn single(&self) -> &Single {
        &self.single
    }
    #[doc = "0xa0 - No Description"]
    #[inline(always)]
    pub const fn scan0(&self) -> &Scan0 {
        &self.scan0
    }
    #[doc = "0xa4 - No Description"]
    #[inline(always)]
    pub const fn scan1(&self) -> &Scan1 {
        &self.scan1
    }
    #[doc = "0xa8 - No Description"]
    #[inline(always)]
    pub const fn scan2(&self) -> &Scan2 {
        &self.scan2
    }
    #[doc = "0xac - No Description"]
    #[inline(always)]
    pub const fn scan3(&self) -> &Scan3 {
        &self.scan3
    }
    #[doc = "0xb0 - No Description"]
    #[inline(always)]
    pub const fn scan4(&self) -> &Scan4 {
        &self.scan4
    }
    #[doc = "0xb4 - No Description"]
    #[inline(always)]
    pub const fn scan5(&self) -> &Scan5 {
        &self.scan5
    }
    #[doc = "0xb8 - No Description"]
    #[inline(always)]
    pub const fn scan6(&self) -> &Scan6 {
        &self.scan6
    }
    #[doc = "0xbc - No Description"]
    #[inline(always)]
    pub const fn scan7(&self) -> &Scan7 {
        &self.scan7
    }
    #[doc = "0xc0 - No Description"]
    #[inline(always)]
    pub const fn scan8(&self) -> &Scan8 {
        &self.scan8
    }
    #[doc = "0xc4 - No Description"]
    #[inline(always)]
    pub const fn scan9(&self) -> &Scan9 {
        &self.scan9
    }
    #[doc = "0xc8 - No Description"]
    #[inline(always)]
    pub const fn scan10(&self) -> &Scan10 {
        &self.scan10
    }
    #[doc = "0xcc - No Description"]
    #[inline(always)]
    pub const fn scan11(&self) -> &Scan11 {
        &self.scan11
    }
    #[doc = "0xd0 - No Description"]
    #[inline(always)]
    pub const fn scan12(&self) -> &Scan12 {
        &self.scan12
    }
    #[doc = "0xd4 - No Description"]
    #[inline(always)]
    pub const fn scan13(&self) -> &Scan13 {
        &self.scan13
    }
    #[doc = "0xd8 - No Description"]
    #[inline(always)]
    pub const fn scan14(&self) -> &Scan14 {
        &self.scan14
    }
    #[doc = "0xdc - No Description"]
    #[inline(always)]
    pub const fn scan15(&self) -> &Scan15 {
        &self.scan15
    }
}
#[doc = "IPVERSION (r) register accessor: IPVERSION\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipversion`] module"]
#[doc(alias = "IPVERSION")]
pub type Ipversion = crate::Reg<ipversion::IpversionSpec>;
#[doc = "IPVERSION"]
pub mod ipversion;
#[doc = "EN (rw) register accessor: Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`] module"]
#[doc(alias = "EN")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = "Enable"]
pub mod en;
#[doc = "CTRL (rw) register accessor: Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Control"]
pub mod ctrl;
#[doc = "CMD (w) register accessor: Command\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "Command"]
pub mod cmd;
#[doc = "TIMER (rw) register accessor: Timer\n\nYou can [`read`](crate::Reg::read) this register and get [`timer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer`] module"]
#[doc(alias = "TIMER")]
pub type Timer = crate::Reg<timer::TimerSpec>;
#[doc = "Timer"]
pub mod timer;
#[doc = "STATUS (r) register accessor: Status\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status"]
pub mod status;
#[doc = "MASKREQ (rw) register accessor: Mask Request\n\nYou can [`read`](crate::Reg::read) this register and get [`maskreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maskreq`] module"]
#[doc(alias = "MASKREQ")]
pub type Maskreq = crate::Reg<maskreq::MaskreqSpec>;
#[doc = "Mask Request"]
pub mod maskreq;
#[doc = "STMASK (r) register accessor: Scan Table Mask\n\nYou can [`read`](crate::Reg::read) this register and get [`stmask::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stmask`] module"]
#[doc(alias = "STMASK")]
pub type Stmask = crate::Reg<stmask::StmaskSpec>;
#[doc = "Scan Table Mask"]
pub mod stmask;
#[doc = "CMPTHR (rw) register accessor: Comparator Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpthr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpthr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmpthr`] module"]
#[doc(alias = "CMPTHR")]
pub type Cmpthr = crate::Reg<cmpthr::CmpthrSpec>;
#[doc = "Comparator Threshold"]
pub mod cmpthr;
#[doc = "IF (rw) register accessor: Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@if_`] module"]
#[doc(alias = "IF")]
pub type If = crate::Reg<if_::IfSpec>;
#[doc = "Interrupt Flag"]
pub mod if_;
#[doc = "IEN (rw) register accessor: Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ien`] module"]
#[doc(alias = "IEN")]
pub type Ien = crate::Reg<ien::IenSpec>;
#[doc = "Interrupt Enable"]
pub mod ien;
#[doc = "TRIGGER (rw) register accessor: Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trigger`] module"]
#[doc(alias = "TRIGGER")]
pub type Trigger = crate::Reg<trigger::TriggerSpec>;
#[doc = "Trigger"]
pub mod trigger;
#[doc = "CFG0 (rw) register accessor: Configration\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg0`] module"]
#[doc(alias = "CFG0")]
pub type Cfg0 = crate::Reg<cfg0::Cfg0Spec>;
#[doc = "Configration"]
pub mod cfg0;
#[doc = "SCALE0 (rw) register accessor: Scale\n\nYou can [`read`](crate::Reg::read) this register and get [`scale0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scale0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scale0`] module"]
#[doc(alias = "SCALE0")]
pub type Scale0 = crate::Reg<scale0::Scale0Spec>;
#[doc = "Scale"]
pub mod scale0;
#[doc = "SCHED0 (rw) register accessor: Scheduling\n\nYou can [`read`](crate::Reg::read) this register and get [`sched0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sched0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sched0`] module"]
#[doc(alias = "SCHED0")]
pub type Sched0 = crate::Reg<sched0::Sched0Spec>;
#[doc = "Scheduling"]
pub mod sched0;
#[doc = "CFG1 (rw) register accessor: Configration\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg1`] module"]
#[doc(alias = "CFG1")]
pub type Cfg1 = crate::Reg<cfg1::Cfg1Spec>;
#[doc = "Configration"]
pub mod cfg1;
#[doc = "SCALE1 (rw) register accessor: Scale\n\nYou can [`read`](crate::Reg::read) this register and get [`scale1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scale1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scale1`] module"]
#[doc(alias = "SCALE1")]
pub type Scale1 = crate::Reg<scale1::Scale1Spec>;
#[doc = "Scale"]
pub mod scale1;
#[doc = "SCHED1 (rw) register accessor: Scheduling\n\nYou can [`read`](crate::Reg::read) this register and get [`sched1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sched1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sched1`] module"]
#[doc(alias = "SCHED1")]
pub type Sched1 = crate::Reg<sched1::Sched1Spec>;
#[doc = "Scheduling"]
pub mod sched1;
#[doc = "SINGLEFIFOCFG (rw) register accessor: Single FIFO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`singlefifocfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlefifocfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singlefifocfg`] module"]
#[doc(alias = "SINGLEFIFOCFG")]
pub type Singlefifocfg = crate::Reg<singlefifocfg::SinglefifocfgSpec>;
#[doc = "Single FIFO Configuration"]
pub mod singlefifocfg;
#[doc = "SINGLEFIFODATA (r) register accessor: Read the oldest valid data from the single FIFO and pop the FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`singlefifodata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singlefifodata`] module"]
#[doc(alias = "SINGLEFIFODATA")]
pub type Singlefifodata = crate::Reg<singlefifodata::SinglefifodataSpec>;
#[doc = "Read the oldest valid data from the single FIFO and pop the FIFO"]
pub mod singlefifodata;
#[doc = "SINGLEFIFOSTAT (r) register accessor: Single FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`singlefifostat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singlefifostat`] module"]
#[doc(alias = "SINGLEFIFOSTAT")]
pub type Singlefifostat = crate::Reg<singlefifostat::SinglefifostatSpec>;
#[doc = "Single FIFO status"]
pub mod singlefifostat;
#[doc = "SINGLEDATA (r) register accessor: latest single queue conversion data\n\nYou can [`read`](crate::Reg::read) this register and get [`singledata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@singledata`] module"]
#[doc(alias = "SINGLEDATA")]
pub type Singledata = crate::Reg<singledata::SingledataSpec>;
#[doc = "latest single queue conversion data"]
pub mod singledata;
#[doc = "SCANFIFOCFG (rw) register accessor: SCAN FIFO configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`scanfifocfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanfifocfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scanfifocfg`] module"]
#[doc(alias = "SCANFIFOCFG")]
pub type Scanfifocfg = crate::Reg<scanfifocfg::ScanfifocfgSpec>;
#[doc = "SCAN FIFO configuration"]
pub mod scanfifocfg;
#[doc = "SCANFIFODATA (r) register accessor: Read the oldest valid data from the scan FIFO and pop the FIFO\n\nYou can [`read`](crate::Reg::read) this register and get [`scanfifodata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scanfifodata`] module"]
#[doc(alias = "SCANFIFODATA")]
pub type Scanfifodata = crate::Reg<scanfifodata::ScanfifodataSpec>;
#[doc = "Read the oldest valid data from the scan FIFO and pop the FIFO"]
pub mod scanfifodata;
#[doc = "SCANFIFOSTAT (r) register accessor: Scan FIFO status\n\nYou can [`read`](crate::Reg::read) this register and get [`scanfifostat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scanfifostat`] module"]
#[doc(alias = "SCANFIFOSTAT")]
pub type Scanfifostat = crate::Reg<scanfifostat::ScanfifostatSpec>;
#[doc = "Scan FIFO status"]
pub mod scanfifostat;
#[doc = "SCANDATA (r) register accessor: Most recent data data from scan queue conversion\n\nYou can [`read`](crate::Reg::read) this register and get [`scandata::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scandata`] module"]
#[doc(alias = "SCANDATA")]
pub type Scandata = crate::Reg<scandata::ScandataSpec>;
#[doc = "Most recent data data from scan queue conversion"]
pub mod scandata;
#[doc = "SINGLE (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`single::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`single::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@single`] module"]
#[doc(alias = "SINGLE")]
pub type Single = crate::Reg<single::SingleSpec>;
#[doc = "No Description"]
pub mod single;
#[doc = "SCAN0 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`scan0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan0`] module"]
#[doc(alias = "SCAN0")]
pub type Scan0 = crate::Reg<scan0::Scan0Spec>;
#[doc = "No Description"]
pub mod scan0;
#[doc = "SCAN1 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`scan1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan1`] module"]
#[doc(alias = "SCAN1")]
pub type Scan1 = crate::Reg<scan1::Scan1Spec>;
#[doc = "No Description"]
pub mod scan1;
#[doc = "SCAN2 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`scan2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan2`] module"]
#[doc(alias = "SCAN2")]
pub type Scan2 = crate::Reg<scan2::Scan2Spec>;
#[doc = "No Description"]
pub mod scan2;
#[doc = "SCAN3 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`scan3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan3`] module"]
#[doc(alias = "SCAN3")]
pub type Scan3 = crate::Reg<scan3::Scan3Spec>;
#[doc = "No Description"]
pub mod scan3;
#[doc = "SCAN4 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`scan4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan4`] module"]
#[doc(alias = "SCAN4")]
pub type Scan4 = crate::Reg<scan4::Scan4Spec>;
#[doc = "No Description"]
pub mod scan4;
#[doc = "SCAN5 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`scan5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan5`] module"]
#[doc(alias = "SCAN5")]
pub type Scan5 = crate::Reg<scan5::Scan5Spec>;
#[doc = "No Description"]
pub mod scan5;
#[doc = "SCAN6 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`scan6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan6`] module"]
#[doc(alias = "SCAN6")]
pub type Scan6 = crate::Reg<scan6::Scan6Spec>;
#[doc = "No Description"]
pub mod scan6;
#[doc = "SCAN7 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`scan7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan7`] module"]
#[doc(alias = "SCAN7")]
pub type Scan7 = crate::Reg<scan7::Scan7Spec>;
#[doc = "No Description"]
pub mod scan7;
#[doc = "SCAN8 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`scan8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan8`] module"]
#[doc(alias = "SCAN8")]
pub type Scan8 = crate::Reg<scan8::Scan8Spec>;
#[doc = "No Description"]
pub mod scan8;
#[doc = "SCAN9 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`scan9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan9`] module"]
#[doc(alias = "SCAN9")]
pub type Scan9 = crate::Reg<scan9::Scan9Spec>;
#[doc = "No Description"]
pub mod scan9;
#[doc = "SCAN10 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`scan10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan10`] module"]
#[doc(alias = "SCAN10")]
pub type Scan10 = crate::Reg<scan10::Scan10Spec>;
#[doc = "No Description"]
pub mod scan10;
#[doc = "SCAN11 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`scan11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan11`] module"]
#[doc(alias = "SCAN11")]
pub type Scan11 = crate::Reg<scan11::Scan11Spec>;
#[doc = "No Description"]
pub mod scan11;
#[doc = "SCAN12 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`scan12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan12`] module"]
#[doc(alias = "SCAN12")]
pub type Scan12 = crate::Reg<scan12::Scan12Spec>;
#[doc = "No Description"]
pub mod scan12;
#[doc = "SCAN13 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`scan13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan13`] module"]
#[doc(alias = "SCAN13")]
pub type Scan13 = crate::Reg<scan13::Scan13Spec>;
#[doc = "No Description"]
pub mod scan13;
#[doc = "SCAN14 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`scan14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan14`] module"]
#[doc(alias = "SCAN14")]
pub type Scan14 = crate::Reg<scan14::Scan14Spec>;
#[doc = "No Description"]
pub mod scan14;
#[doc = "SCAN15 (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`scan15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scan15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scan15`] module"]
#[doc(alias = "SCAN15")]
pub type Scan15 = crate::Reg<scan15::Scan15Spec>;
#[doc = "No Description"]
pub mod scan15;
