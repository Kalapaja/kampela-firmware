#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ipversion: Ipversion,
    en: En,
    ctrl: Ctrl,
    cmd: Cmd,
    init: Init,
    poly: Poly,
    inputdata: Inputdata,
    inputdatahword: Inputdatahword,
    inputdatabyte: Inputdatabyte,
    data: Data,
    datarev: Datarev,
    databyterev: Databyterev,
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
    pub const fn cmd(&self) -> &Cmd {
        &self.cmd
    }
    #[doc = "0x10 - No Description"]
    #[inline(always)]
    pub const fn init(&self) -> &Init {
        &self.init
    }
    #[doc = "0x14 - No Description"]
    #[inline(always)]
    pub const fn poly(&self) -> &Poly {
        &self.poly
    }
    #[doc = "0x18 - No Description"]
    #[inline(always)]
    pub const fn inputdata(&self) -> &Inputdata {
        &self.inputdata
    }
    #[doc = "0x1c - No Description"]
    #[inline(always)]
    pub const fn inputdatahword(&self) -> &Inputdatahword {
        &self.inputdatahword
    }
    #[doc = "0x20 - No Description"]
    #[inline(always)]
    pub const fn inputdatabyte(&self) -> &Inputdatabyte {
        &self.inputdatabyte
    }
    #[doc = "0x24 - No Description"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        &self.data
    }
    #[doc = "0x28 - No Description"]
    #[inline(always)]
    pub const fn datarev(&self) -> &Datarev {
        &self.datarev
    }
    #[doc = "0x2c - No Description"]
    #[inline(always)]
    pub const fn databyterev(&self) -> &Databyterev {
        &self.databyterev
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
#[doc = "CMD (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`] module"]
#[doc(alias = "CMD")]
pub type Cmd = crate::Reg<cmd::CmdSpec>;
#[doc = "No Description"]
pub mod cmd;
#[doc = "INIT (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`init::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`init::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@init`] module"]
#[doc(alias = "INIT")]
pub type Init = crate::Reg<init::InitSpec>;
#[doc = "No Description"]
pub mod init;
#[doc = "POLY (rw) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`poly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@poly`] module"]
#[doc(alias = "POLY")]
pub type Poly = crate::Reg<poly::PolySpec>;
#[doc = "No Description"]
pub mod poly;
#[doc = "INPUTDATA (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputdata::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inputdata`] module"]
#[doc(alias = "INPUTDATA")]
pub type Inputdata = crate::Reg<inputdata::InputdataSpec>;
#[doc = "No Description"]
pub mod inputdata;
#[doc = "INPUTDATAHWORD (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputdatahword::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inputdatahword`] module"]
#[doc(alias = "INPUTDATAHWORD")]
pub type Inputdatahword = crate::Reg<inputdatahword::InputdatahwordSpec>;
#[doc = "No Description"]
pub mod inputdatahword;
#[doc = "INPUTDATABYTE (w) register accessor: No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputdatabyte::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inputdatabyte`] module"]
#[doc(alias = "INPUTDATABYTE")]
pub type Inputdatabyte = crate::Reg<inputdatabyte::InputdatabyteSpec>;
#[doc = "No Description"]
pub mod inputdatabyte;
#[doc = "DATA (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "No Description"]
pub mod data;
#[doc = "DATAREV (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`datarev::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datarev`] module"]
#[doc(alias = "DATAREV")]
pub type Datarev = crate::Reg<datarev::DatarevSpec>;
#[doc = "No Description"]
pub mod datarev;
#[doc = "DATABYTEREV (r) register accessor: No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`databyterev::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@databyterev`] module"]
#[doc(alias = "DATABYTEREV")]
pub type Databyterev = crate::Reg<databyterev::DatabyterevSpec>;
#[doc = "No Description"]
pub mod databyterev;
