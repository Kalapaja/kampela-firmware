#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    info: Info,
    part: Part,
    meminfo: Meminfo,
    msize: Msize,
    pkginfo: Pkginfo,
    custominfo: Custominfo,
    swfix: Swfix,
    swcapa0: Swcapa0,
    swcapa1: Swcapa1,
    _reserved9: [u8; 0x04],
    extinfo: Extinfo,
    _reserved10: [u8; 0x14],
    eui48l: Eui48l,
    eui48h: Eui48h,
    eui64l: Eui64l,
    eui64h: Eui64h,
    caltemp: Caltemp,
    emutemp: Emutemp,
    hfrcodpllcal0: Hfrcodpllcal0,
    hfrcodpllcal1: Hfrcodpllcal1,
    hfrcodpllcal2: Hfrcodpllcal2,
    hfrcodpllcal3: Hfrcodpllcal3,
    hfrcodpllcal4: Hfrcodpllcal4,
    hfrcodpllcal5: Hfrcodpllcal5,
    hfrcodpllcal6: Hfrcodpllcal6,
    hfrcodpllcal7: Hfrcodpllcal7,
    hfrcodpllcal8: Hfrcodpllcal8,
    hfrcodpllcal9: Hfrcodpllcal9,
    hfrcodpllcal10: Hfrcodpllcal10,
    hfrcodpllcal11: Hfrcodpllcal11,
    hfrcodpllcal12: Hfrcodpllcal12,
    hfrcodpllcal13: Hfrcodpllcal13,
    hfrcodpllcal14: Hfrcodpllcal14,
    hfrcodpllcal15: Hfrcodpllcal15,
    hfrcodpllcal16: Hfrcodpllcal16,
    hfrcodpllcal17: Hfrcodpllcal17,
    hfrcoem23cal0: Hfrcoem23cal0,
    hfrcoem23cal1: Hfrcoem23cal1,
    hfrcoem23cal2: Hfrcoem23cal2,
    hfrcoem23cal3: Hfrcoem23cal3,
    hfrcoem23cal4: Hfrcoem23cal4,
    hfrcoem23cal5: Hfrcoem23cal5,
    hfrcoem23cal6: Hfrcoem23cal6,
    hfrcoem23cal7: Hfrcoem23cal7,
    hfrcoem23cal8: Hfrcoem23cal8,
    hfrcoem23cal9: Hfrcoem23cal9,
    hfrcoem23cal10: Hfrcoem23cal10,
    hfrcoem23cal11: Hfrcoem23cal11,
    hfrcoem23cal12: Hfrcoem23cal12,
    hfrcoem23cal13: Hfrcoem23cal13,
    hfrcoem23cal14: Hfrcoem23cal14,
    hfrcoem23cal15: Hfrcoem23cal15,
    hfrcoem23cal16: Hfrcoem23cal16,
    hfrcoem23cal17: Hfrcoem23cal17,
    _reserved52: [u8; 0x48],
    modulename0: Modulename0,
    modulename1: Modulename1,
    modulename2: Modulename2,
    modulename3: Modulename3,
    modulename4: Modulename4,
    modulename5: Modulename5,
    modulename6: Modulename6,
    moduleinfo: Moduleinfo,
    modxocal: Modxocal,
    _reserved61: [u8; 0x28],
    hfxocal: Hfxocal,
    iadc0gain0: Iadc0gain0,
    iadc0gain1: Iadc0gain1,
    iadc0offsetcal0: Iadc0offsetcal0,
    iadc0normaloffsetcal0: Iadc0normaloffsetcal0,
    iadc0normaloffsetcal1: Iadc0normaloffsetcal1,
    iadc0hispdoffsetcal0: Iadc0hispdoffsetcal0,
    iadc0hispdoffsetcal1: Iadc0hispdoffsetcal1,
    _reserved69: [u8; 0x60],
    legacy: Legacy,
    _reserved70: [u8; 0x5c],
    rtherm: Rtherm,
}
impl RegisterBlock {
    #[doc = "0x00 - Version of the device info structure being used"]
    #[inline(always)]
    pub const fn info(&self) -> &Info {
        &self.info
    }
    #[doc = "0x04 - Part description"]
    #[inline(always)]
    pub const fn part(&self) -> &Part {
        &self.part
    }
    #[doc = "0x08 - Flash page size and misc. chip information"]
    #[inline(always)]
    pub const fn meminfo(&self) -> &Meminfo {
        &self.meminfo
    }
    #[doc = "0x0c - Flash and SRAM Memory size in kB"]
    #[inline(always)]
    pub const fn msize(&self) -> &Msize {
        &self.msize
    }
    #[doc = "0x10 - Miscellaneous device information"]
    #[inline(always)]
    pub const fn pkginfo(&self) -> &Pkginfo {
        &self.pkginfo
    }
    #[doc = "0x14 - Custom information"]
    #[inline(always)]
    pub const fn custominfo(&self) -> &Custominfo {
        &self.custominfo
    }
    #[doc = "0x18 - Used to track s/w workaround info"]
    #[inline(always)]
    pub const fn swfix(&self) -> &Swfix {
        &self.swfix
    }
    #[doc = "0x1c - Software Capability Vector 0"]
    #[inline(always)]
    pub const fn swcapa0(&self) -> &Swcapa0 {
        &self.swcapa0
    }
    #[doc = "0x20 - Software Capability Vector 1"]
    #[inline(always)]
    pub const fn swcapa1(&self) -> &Swcapa1 {
        &self.swcapa1
    }
    #[doc = "0x28 - External component description"]
    #[inline(always)]
    pub const fn extinfo(&self) -> &Extinfo {
        &self.extinfo
    }
    #[doc = "0x40 - MA-L compliant EUI48 OUI (low bits) and Unique Identifier (24-bit)"]
    #[inline(always)]
    pub const fn eui48l(&self) -> &Eui48l {
        &self.eui48l
    }
    #[doc = "0x44 - MA-L compliant EUI48 OUI (high bits)"]
    #[inline(always)]
    pub const fn eui48h(&self) -> &Eui48h {
        &self.eui48h
    }
    #[doc = "0x48 - MA-L compliant EUI64 Unique Identifier (low bits)"]
    #[inline(always)]
    pub const fn eui64l(&self) -> &Eui64l {
        &self.eui64l
    }
    #[doc = "0x4c - MA-L compliant EUI64 OUI and Unique Identifier (high bits)"]
    #[inline(always)]
    pub const fn eui64h(&self) -> &Eui64h {
        &self.eui64h
    }
    #[doc = "0x50 - Calibration Temperature Information"]
    #[inline(always)]
    pub const fn caltemp(&self) -> &Caltemp {
        &self.caltemp
    }
    #[doc = "0x54 - EMU Temperature Sensor Calibration"]
    #[inline(always)]
    pub const fn emutemp(&self) -> &Emutemp {
        &self.emutemp
    }
    #[doc = "0x58 - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal0(&self) -> &Hfrcodpllcal0 {
        &self.hfrcodpllcal0
    }
    #[doc = "0x5c - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal1(&self) -> &Hfrcodpllcal1 {
        &self.hfrcodpllcal1
    }
    #[doc = "0x60 - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal2(&self) -> &Hfrcodpllcal2 {
        &self.hfrcodpllcal2
    }
    #[doc = "0x64 - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal3(&self) -> &Hfrcodpllcal3 {
        &self.hfrcodpllcal3
    }
    #[doc = "0x68 - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal4(&self) -> &Hfrcodpllcal4 {
        &self.hfrcodpllcal4
    }
    #[doc = "0x6c - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal5(&self) -> &Hfrcodpllcal5 {
        &self.hfrcodpllcal5
    }
    #[doc = "0x70 - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal6(&self) -> &Hfrcodpllcal6 {
        &self.hfrcodpllcal6
    }
    #[doc = "0x74 - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal7(&self) -> &Hfrcodpllcal7 {
        &self.hfrcodpllcal7
    }
    #[doc = "0x78 - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal8(&self) -> &Hfrcodpllcal8 {
        &self.hfrcodpllcal8
    }
    #[doc = "0x7c - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal9(&self) -> &Hfrcodpllcal9 {
        &self.hfrcodpllcal9
    }
    #[doc = "0x80 - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal10(&self) -> &Hfrcodpllcal10 {
        &self.hfrcodpllcal10
    }
    #[doc = "0x84 - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal11(&self) -> &Hfrcodpllcal11 {
        &self.hfrcodpllcal11
    }
    #[doc = "0x88 - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal12(&self) -> &Hfrcodpllcal12 {
        &self.hfrcodpllcal12
    }
    #[doc = "0x8c - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal13(&self) -> &Hfrcodpllcal13 {
        &self.hfrcodpllcal13
    }
    #[doc = "0x90 - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal14(&self) -> &Hfrcodpllcal14 {
        &self.hfrcodpllcal14
    }
    #[doc = "0x94 - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal15(&self) -> &Hfrcodpllcal15 {
        &self.hfrcodpllcal15
    }
    #[doc = "0x98 - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal16(&self) -> &Hfrcodpllcal16 {
        &self.hfrcodpllcal16
    }
    #[doc = "0x9c - HFRCODPLL Calibration"]
    #[inline(always)]
    pub const fn hfrcodpllcal17(&self) -> &Hfrcodpllcal17 {
        &self.hfrcodpllcal17
    }
    #[doc = "0xa0 - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal0(&self) -> &Hfrcoem23cal0 {
        &self.hfrcoem23cal0
    }
    #[doc = "0xa4 - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal1(&self) -> &Hfrcoem23cal1 {
        &self.hfrcoem23cal1
    }
    #[doc = "0xa8 - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal2(&self) -> &Hfrcoem23cal2 {
        &self.hfrcoem23cal2
    }
    #[doc = "0xac - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal3(&self) -> &Hfrcoem23cal3 {
        &self.hfrcoem23cal3
    }
    #[doc = "0xb0 - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal4(&self) -> &Hfrcoem23cal4 {
        &self.hfrcoem23cal4
    }
    #[doc = "0xb4 - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal5(&self) -> &Hfrcoem23cal5 {
        &self.hfrcoem23cal5
    }
    #[doc = "0xb8 - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal6(&self) -> &Hfrcoem23cal6 {
        &self.hfrcoem23cal6
    }
    #[doc = "0xbc - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal7(&self) -> &Hfrcoem23cal7 {
        &self.hfrcoem23cal7
    }
    #[doc = "0xc0 - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal8(&self) -> &Hfrcoem23cal8 {
        &self.hfrcoem23cal8
    }
    #[doc = "0xc4 - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal9(&self) -> &Hfrcoem23cal9 {
        &self.hfrcoem23cal9
    }
    #[doc = "0xc8 - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal10(&self) -> &Hfrcoem23cal10 {
        &self.hfrcoem23cal10
    }
    #[doc = "0xcc - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal11(&self) -> &Hfrcoem23cal11 {
        &self.hfrcoem23cal11
    }
    #[doc = "0xd0 - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal12(&self) -> &Hfrcoem23cal12 {
        &self.hfrcoem23cal12
    }
    #[doc = "0xd4 - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal13(&self) -> &Hfrcoem23cal13 {
        &self.hfrcoem23cal13
    }
    #[doc = "0xd8 - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal14(&self) -> &Hfrcoem23cal14 {
        &self.hfrcoem23cal14
    }
    #[doc = "0xdc - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal15(&self) -> &Hfrcoem23cal15 {
        &self.hfrcoem23cal15
    }
    #[doc = "0xe0 - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal16(&self) -> &Hfrcoem23cal16 {
        &self.hfrcoem23cal16
    }
    #[doc = "0xe4 - HFRCOEM23 Calibration"]
    #[inline(always)]
    pub const fn hfrcoem23cal17(&self) -> &Hfrcoem23cal17 {
        &self.hfrcoem23cal17
    }
    #[doc = "0x130 - Characters 1-4 of Module Name stored as a null terminated string"]
    #[inline(always)]
    pub const fn modulename0(&self) -> &Modulename0 {
        &self.modulename0
    }
    #[doc = "0x134 - Characters 5-8 of Module Name stored as a null terminated string"]
    #[inline(always)]
    pub const fn modulename1(&self) -> &Modulename1 {
        &self.modulename1
    }
    #[doc = "0x138 - Characters 9-12 of Module Name stored as a null terminated string"]
    #[inline(always)]
    pub const fn modulename2(&self) -> &Modulename2 {
        &self.modulename2
    }
    #[doc = "0x13c - Characters 13-16 of Module Name stored as a null terminated string"]
    #[inline(always)]
    pub const fn modulename3(&self) -> &Modulename3 {
        &self.modulename3
    }
    #[doc = "0x140 - Characters 17-20 of Module Name stored as a null terminated string"]
    #[inline(always)]
    pub const fn modulename4(&self) -> &Modulename4 {
        &self.modulename4
    }
    #[doc = "0x144 - Characters 21-24 of Module Name stored as a null terminated string"]
    #[inline(always)]
    pub const fn modulename5(&self) -> &Modulename5 {
        &self.modulename5
    }
    #[doc = "0x148 - Characters 25-26 of Module Name stored as a null terminated string"]
    #[inline(always)]
    pub const fn modulename6(&self) -> &Modulename6 {
        &self.modulename6
    }
    #[doc = "0x14c - Module Information"]
    #[inline(always)]
    pub const fn moduleinfo(&self) -> &Moduleinfo {
        &self.moduleinfo
    }
    #[doc = "0x150 - Module Crystal Oscillator Calibration"]
    #[inline(always)]
    pub const fn modxocal(&self) -> &Modxocal {
        &self.modxocal
    }
    #[doc = "0x17c - High Frequency Crystal Oscillator Calibration data"]
    #[inline(always)]
    pub const fn hfxocal(&self) -> &Hfxocal {
        &self.hfxocal
    }
    #[doc = "0x180 - IADC0 Gain Calibration Info"]
    #[inline(always)]
    pub const fn iadc0gain0(&self) -> &Iadc0gain0 {
        &self.iadc0gain0
    }
    #[doc = "0x184 - IADC0 Gain Calibration Info"]
    #[inline(always)]
    pub const fn iadc0gain1(&self) -> &Iadc0gain1 {
        &self.iadc0gain1
    }
    #[doc = "0x188 - IADC0 Offset Calibration Info"]
    #[inline(always)]
    pub const fn iadc0offsetcal0(&self) -> &Iadc0offsetcal0 {
        &self.iadc0offsetcal0
    }
    #[doc = "0x18c - IADC0 Normal Offset Calibration Info"]
    #[inline(always)]
    pub const fn iadc0normaloffsetcal0(&self) -> &Iadc0normaloffsetcal0 {
        &self.iadc0normaloffsetcal0
    }
    #[doc = "0x190 - IADC0 Normal Offset Calibration Info"]
    #[inline(always)]
    pub const fn iadc0normaloffsetcal1(&self) -> &Iadc0normaloffsetcal1 {
        &self.iadc0normaloffsetcal1
    }
    #[doc = "0x194 - IADC High Speed Offset Calibration Info"]
    #[inline(always)]
    pub const fn iadc0hispdoffsetcal0(&self) -> &Iadc0hispdoffsetcal0 {
        &self.iadc0hispdoffsetcal0
    }
    #[doc = "0x198 - IADC High Speed Offset Calibration Info"]
    #[inline(always)]
    pub const fn iadc0hispdoffsetcal1(&self) -> &Iadc0hispdoffsetcal1 {
        &self.iadc0hispdoffsetcal1
    }
    #[doc = "0x1fc - This is the legacy device detection information for tools compatability"]
    #[inline(always)]
    pub const fn legacy(&self) -> &Legacy {
        &self.legacy
    }
    #[doc = "0x25c - RTHERM"]
    #[inline(always)]
    pub const fn rtherm(&self) -> &Rtherm {
        &self.rtherm
    }
}
#[doc = "INFO (r) register accessor: Version of the device info structure being used\n\nYou can [`read`](crate::Reg::read) this register and get [`info::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@info`] module"]
#[doc(alias = "INFO")]
pub type Info = crate::Reg<info::InfoSpec>;
#[doc = "Version of the device info structure being used"]
pub mod info;
#[doc = "PART (r) register accessor: Part description\n\nYou can [`read`](crate::Reg::read) this register and get [`part::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@part`] module"]
#[doc(alias = "PART")]
pub type Part = crate::Reg<part::PartSpec>;
#[doc = "Part description"]
pub mod part;
#[doc = "MEMINFO (r) register accessor: Flash page size and misc. chip information\n\nYou can [`read`](crate::Reg::read) this register and get [`meminfo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@meminfo`] module"]
#[doc(alias = "MEMINFO")]
pub type Meminfo = crate::Reg<meminfo::MeminfoSpec>;
#[doc = "Flash page size and misc. chip information"]
pub mod meminfo;
#[doc = "MSIZE (r) register accessor: Flash and SRAM Memory size in kB\n\nYou can [`read`](crate::Reg::read) this register and get [`msize::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msize`] module"]
#[doc(alias = "MSIZE")]
pub type Msize = crate::Reg<msize::MsizeSpec>;
#[doc = "Flash and SRAM Memory size in kB"]
pub mod msize;
#[doc = "PKGINFO (r) register accessor: Miscellaneous device information\n\nYou can [`read`](crate::Reg::read) this register and get [`pkginfo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkginfo`] module"]
#[doc(alias = "PKGINFO")]
pub type Pkginfo = crate::Reg<pkginfo::PkginfoSpec>;
#[doc = "Miscellaneous device information"]
pub mod pkginfo;
#[doc = "CUSTOMINFO (r) register accessor: Custom information\n\nYou can [`read`](crate::Reg::read) this register and get [`custominfo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@custominfo`] module"]
#[doc(alias = "CUSTOMINFO")]
pub type Custominfo = crate::Reg<custominfo::CustominfoSpec>;
#[doc = "Custom information"]
pub mod custominfo;
#[doc = "SWFIX (r) register accessor: Used to track s/w workaround info\n\nYou can [`read`](crate::Reg::read) this register and get [`swfix::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swfix`] module"]
#[doc(alias = "SWFIX")]
pub type Swfix = crate::Reg<swfix::SwfixSpec>;
#[doc = "Used to track s/w workaround info"]
pub mod swfix;
#[doc = "SWCAPA0 (r) register accessor: Software Capability Vector 0\n\nYou can [`read`](crate::Reg::read) this register and get [`swcapa0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swcapa0`] module"]
#[doc(alias = "SWCAPA0")]
pub type Swcapa0 = crate::Reg<swcapa0::Swcapa0Spec>;
#[doc = "Software Capability Vector 0"]
pub mod swcapa0;
#[doc = "SWCAPA1 (r) register accessor: Software Capability Vector 1\n\nYou can [`read`](crate::Reg::read) this register and get [`swcapa1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swcapa1`] module"]
#[doc(alias = "SWCAPA1")]
pub type Swcapa1 = crate::Reg<swcapa1::Swcapa1Spec>;
#[doc = "Software Capability Vector 1"]
pub mod swcapa1;
#[doc = "EXTINFO (r) register accessor: External component description\n\nYou can [`read`](crate::Reg::read) this register and get [`extinfo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extinfo`] module"]
#[doc(alias = "EXTINFO")]
pub type Extinfo = crate::Reg<extinfo::ExtinfoSpec>;
#[doc = "External component description"]
pub mod extinfo;
#[doc = "EUI48L (r) register accessor: MA-L compliant EUI48 OUI (low bits) and Unique Identifier (24-bit)\n\nYou can [`read`](crate::Reg::read) this register and get [`eui48l::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eui48l`] module"]
#[doc(alias = "EUI48L")]
pub type Eui48l = crate::Reg<eui48l::Eui48lSpec>;
#[doc = "MA-L compliant EUI48 OUI (low bits) and Unique Identifier (24-bit)"]
pub mod eui48l;
#[doc = "EUI48H (r) register accessor: MA-L compliant EUI48 OUI (high bits)\n\nYou can [`read`](crate::Reg::read) this register and get [`eui48h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eui48h`] module"]
#[doc(alias = "EUI48H")]
pub type Eui48h = crate::Reg<eui48h::Eui48hSpec>;
#[doc = "MA-L compliant EUI48 OUI (high bits)"]
pub mod eui48h;
#[doc = "EUI64L (r) register accessor: MA-L compliant EUI64 Unique Identifier (low bits)\n\nYou can [`read`](crate::Reg::read) this register and get [`eui64l::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eui64l`] module"]
#[doc(alias = "EUI64L")]
pub type Eui64l = crate::Reg<eui64l::Eui64lSpec>;
#[doc = "MA-L compliant EUI64 Unique Identifier (low bits)"]
pub mod eui64l;
#[doc = "EUI64H (r) register accessor: MA-L compliant EUI64 OUI and Unique Identifier (high bits)\n\nYou can [`read`](crate::Reg::read) this register and get [`eui64h::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eui64h`] module"]
#[doc(alias = "EUI64H")]
pub type Eui64h = crate::Reg<eui64h::Eui64hSpec>;
#[doc = "MA-L compliant EUI64 OUI and Unique Identifier (high bits)"]
pub mod eui64h;
#[doc = "CALTEMP (r) register accessor: Calibration Temperature Information\n\nYou can [`read`](crate::Reg::read) this register and get [`caltemp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caltemp`] module"]
#[doc(alias = "CALTEMP")]
pub type Caltemp = crate::Reg<caltemp::CaltempSpec>;
#[doc = "Calibration Temperature Information"]
pub mod caltemp;
#[doc = "EMUTEMP (r) register accessor: EMU Temperature Sensor Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`emutemp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emutemp`] module"]
#[doc(alias = "EMUTEMP")]
pub type Emutemp = crate::Reg<emutemp::EmutempSpec>;
#[doc = "EMU Temperature Sensor Calibration"]
pub mod emutemp;
#[doc = "HFRCODPLLCAL0 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal0`] module"]
#[doc(alias = "HFRCODPLLCAL0")]
pub type Hfrcodpllcal0 = crate::Reg<hfrcodpllcal0::Hfrcodpllcal0Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal0;
#[doc = "HFRCODPLLCAL1 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal1`] module"]
#[doc(alias = "HFRCODPLLCAL1")]
pub type Hfrcodpllcal1 = crate::Reg<hfrcodpllcal1::Hfrcodpllcal1Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal1;
#[doc = "HFRCODPLLCAL2 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal2`] module"]
#[doc(alias = "HFRCODPLLCAL2")]
pub type Hfrcodpllcal2 = crate::Reg<hfrcodpllcal2::Hfrcodpllcal2Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal2;
#[doc = "HFRCODPLLCAL3 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal3`] module"]
#[doc(alias = "HFRCODPLLCAL3")]
pub type Hfrcodpllcal3 = crate::Reg<hfrcodpllcal3::Hfrcodpllcal3Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal3;
#[doc = "HFRCODPLLCAL4 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal4`] module"]
#[doc(alias = "HFRCODPLLCAL4")]
pub type Hfrcodpllcal4 = crate::Reg<hfrcodpllcal4::Hfrcodpllcal4Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal4;
#[doc = "HFRCODPLLCAL5 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal5`] module"]
#[doc(alias = "HFRCODPLLCAL5")]
pub type Hfrcodpllcal5 = crate::Reg<hfrcodpllcal5::Hfrcodpllcal5Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal5;
#[doc = "HFRCODPLLCAL6 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal6`] module"]
#[doc(alias = "HFRCODPLLCAL6")]
pub type Hfrcodpllcal6 = crate::Reg<hfrcodpllcal6::Hfrcodpllcal6Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal6;
#[doc = "HFRCODPLLCAL7 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal7`] module"]
#[doc(alias = "HFRCODPLLCAL7")]
pub type Hfrcodpllcal7 = crate::Reg<hfrcodpllcal7::Hfrcodpllcal7Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal7;
#[doc = "HFRCODPLLCAL8 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal8`] module"]
#[doc(alias = "HFRCODPLLCAL8")]
pub type Hfrcodpllcal8 = crate::Reg<hfrcodpllcal8::Hfrcodpllcal8Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal8;
#[doc = "HFRCODPLLCAL9 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal9`] module"]
#[doc(alias = "HFRCODPLLCAL9")]
pub type Hfrcodpllcal9 = crate::Reg<hfrcodpllcal9::Hfrcodpllcal9Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal9;
#[doc = "HFRCODPLLCAL10 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal10`] module"]
#[doc(alias = "HFRCODPLLCAL10")]
pub type Hfrcodpllcal10 = crate::Reg<hfrcodpllcal10::Hfrcodpllcal10Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal10;
#[doc = "HFRCODPLLCAL11 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal11`] module"]
#[doc(alias = "HFRCODPLLCAL11")]
pub type Hfrcodpllcal11 = crate::Reg<hfrcodpllcal11::Hfrcodpllcal11Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal11;
#[doc = "HFRCODPLLCAL12 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal12`] module"]
#[doc(alias = "HFRCODPLLCAL12")]
pub type Hfrcodpllcal12 = crate::Reg<hfrcodpllcal12::Hfrcodpllcal12Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal12;
#[doc = "HFRCODPLLCAL13 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal13`] module"]
#[doc(alias = "HFRCODPLLCAL13")]
pub type Hfrcodpllcal13 = crate::Reg<hfrcodpllcal13::Hfrcodpllcal13Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal13;
#[doc = "HFRCODPLLCAL14 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal14`] module"]
#[doc(alias = "HFRCODPLLCAL14")]
pub type Hfrcodpllcal14 = crate::Reg<hfrcodpllcal14::Hfrcodpllcal14Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal14;
#[doc = "HFRCODPLLCAL15 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal15`] module"]
#[doc(alias = "HFRCODPLLCAL15")]
pub type Hfrcodpllcal15 = crate::Reg<hfrcodpllcal15::Hfrcodpllcal15Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal15;
#[doc = "HFRCODPLLCAL16 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal16`] module"]
#[doc(alias = "HFRCODPLLCAL16")]
pub type Hfrcodpllcal16 = crate::Reg<hfrcodpllcal16::Hfrcodpllcal16Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal16;
#[doc = "HFRCODPLLCAL17 (r) register accessor: HFRCODPLL Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcodpllcal17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcodpllcal17`] module"]
#[doc(alias = "HFRCODPLLCAL17")]
pub type Hfrcodpllcal17 = crate::Reg<hfrcodpllcal17::Hfrcodpllcal17Spec>;
#[doc = "HFRCODPLL Calibration"]
pub mod hfrcodpllcal17;
#[doc = "HFRCOEM23CAL0 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal0`] module"]
#[doc(alias = "HFRCOEM23CAL0")]
pub type Hfrcoem23cal0 = crate::Reg<hfrcoem23cal0::Hfrcoem23cal0Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal0;
#[doc = "HFRCOEM23CAL1 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal1`] module"]
#[doc(alias = "HFRCOEM23CAL1")]
pub type Hfrcoem23cal1 = crate::Reg<hfrcoem23cal1::Hfrcoem23cal1Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal1;
#[doc = "HFRCOEM23CAL2 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal2`] module"]
#[doc(alias = "HFRCOEM23CAL2")]
pub type Hfrcoem23cal2 = crate::Reg<hfrcoem23cal2::Hfrcoem23cal2Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal2;
#[doc = "HFRCOEM23CAL3 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal3`] module"]
#[doc(alias = "HFRCOEM23CAL3")]
pub type Hfrcoem23cal3 = crate::Reg<hfrcoem23cal3::Hfrcoem23cal3Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal3;
#[doc = "HFRCOEM23CAL4 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal4`] module"]
#[doc(alias = "HFRCOEM23CAL4")]
pub type Hfrcoem23cal4 = crate::Reg<hfrcoem23cal4::Hfrcoem23cal4Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal4;
#[doc = "HFRCOEM23CAL5 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal5`] module"]
#[doc(alias = "HFRCOEM23CAL5")]
pub type Hfrcoem23cal5 = crate::Reg<hfrcoem23cal5::Hfrcoem23cal5Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal5;
#[doc = "HFRCOEM23CAL6 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal6`] module"]
#[doc(alias = "HFRCOEM23CAL6")]
pub type Hfrcoem23cal6 = crate::Reg<hfrcoem23cal6::Hfrcoem23cal6Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal6;
#[doc = "HFRCOEM23CAL7 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal7`] module"]
#[doc(alias = "HFRCOEM23CAL7")]
pub type Hfrcoem23cal7 = crate::Reg<hfrcoem23cal7::Hfrcoem23cal7Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal7;
#[doc = "HFRCOEM23CAL8 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal8`] module"]
#[doc(alias = "HFRCOEM23CAL8")]
pub type Hfrcoem23cal8 = crate::Reg<hfrcoem23cal8::Hfrcoem23cal8Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal8;
#[doc = "HFRCOEM23CAL9 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal9`] module"]
#[doc(alias = "HFRCOEM23CAL9")]
pub type Hfrcoem23cal9 = crate::Reg<hfrcoem23cal9::Hfrcoem23cal9Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal9;
#[doc = "HFRCOEM23CAL10 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal10`] module"]
#[doc(alias = "HFRCOEM23CAL10")]
pub type Hfrcoem23cal10 = crate::Reg<hfrcoem23cal10::Hfrcoem23cal10Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal10;
#[doc = "HFRCOEM23CAL11 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal11`] module"]
#[doc(alias = "HFRCOEM23CAL11")]
pub type Hfrcoem23cal11 = crate::Reg<hfrcoem23cal11::Hfrcoem23cal11Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal11;
#[doc = "HFRCOEM23CAL12 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal12`] module"]
#[doc(alias = "HFRCOEM23CAL12")]
pub type Hfrcoem23cal12 = crate::Reg<hfrcoem23cal12::Hfrcoem23cal12Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal12;
#[doc = "HFRCOEM23CAL13 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal13`] module"]
#[doc(alias = "HFRCOEM23CAL13")]
pub type Hfrcoem23cal13 = crate::Reg<hfrcoem23cal13::Hfrcoem23cal13Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal13;
#[doc = "HFRCOEM23CAL14 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal14`] module"]
#[doc(alias = "HFRCOEM23CAL14")]
pub type Hfrcoem23cal14 = crate::Reg<hfrcoem23cal14::Hfrcoem23cal14Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal14;
#[doc = "HFRCOEM23CAL15 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal15`] module"]
#[doc(alias = "HFRCOEM23CAL15")]
pub type Hfrcoem23cal15 = crate::Reg<hfrcoem23cal15::Hfrcoem23cal15Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal15;
#[doc = "HFRCOEM23CAL16 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal16`] module"]
#[doc(alias = "HFRCOEM23CAL16")]
pub type Hfrcoem23cal16 = crate::Reg<hfrcoem23cal16::Hfrcoem23cal16Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal16;
#[doc = "HFRCOEM23CAL17 (r) register accessor: HFRCOEM23 Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`hfrcoem23cal17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfrcoem23cal17`] module"]
#[doc(alias = "HFRCOEM23CAL17")]
pub type Hfrcoem23cal17 = crate::Reg<hfrcoem23cal17::Hfrcoem23cal17Spec>;
#[doc = "HFRCOEM23 Calibration"]
pub mod hfrcoem23cal17;
#[doc = "MODULENAME0 (r) register accessor: Characters 1-4 of Module Name stored as a null terminated string\n\nYou can [`read`](crate::Reg::read) this register and get [`modulename0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modulename0`] module"]
#[doc(alias = "MODULENAME0")]
pub type Modulename0 = crate::Reg<modulename0::Modulename0Spec>;
#[doc = "Characters 1-4 of Module Name stored as a null terminated string"]
pub mod modulename0;
#[doc = "MODULENAME1 (r) register accessor: Characters 5-8 of Module Name stored as a null terminated string\n\nYou can [`read`](crate::Reg::read) this register and get [`modulename1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modulename1`] module"]
#[doc(alias = "MODULENAME1")]
pub type Modulename1 = crate::Reg<modulename1::Modulename1Spec>;
#[doc = "Characters 5-8 of Module Name stored as a null terminated string"]
pub mod modulename1;
#[doc = "MODULENAME2 (r) register accessor: Characters 9-12 of Module Name stored as a null terminated string\n\nYou can [`read`](crate::Reg::read) this register and get [`modulename2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modulename2`] module"]
#[doc(alias = "MODULENAME2")]
pub type Modulename2 = crate::Reg<modulename2::Modulename2Spec>;
#[doc = "Characters 9-12 of Module Name stored as a null terminated string"]
pub mod modulename2;
#[doc = "MODULENAME3 (r) register accessor: Characters 13-16 of Module Name stored as a null terminated string\n\nYou can [`read`](crate::Reg::read) this register and get [`modulename3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modulename3`] module"]
#[doc(alias = "MODULENAME3")]
pub type Modulename3 = crate::Reg<modulename3::Modulename3Spec>;
#[doc = "Characters 13-16 of Module Name stored as a null terminated string"]
pub mod modulename3;
#[doc = "MODULENAME4 (r) register accessor: Characters 17-20 of Module Name stored as a null terminated string\n\nYou can [`read`](crate::Reg::read) this register and get [`modulename4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modulename4`] module"]
#[doc(alias = "MODULENAME4")]
pub type Modulename4 = crate::Reg<modulename4::Modulename4Spec>;
#[doc = "Characters 17-20 of Module Name stored as a null terminated string"]
pub mod modulename4;
#[doc = "MODULENAME5 (r) register accessor: Characters 21-24 of Module Name stored as a null terminated string\n\nYou can [`read`](crate::Reg::read) this register and get [`modulename5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modulename5`] module"]
#[doc(alias = "MODULENAME5")]
pub type Modulename5 = crate::Reg<modulename5::Modulename5Spec>;
#[doc = "Characters 21-24 of Module Name stored as a null terminated string"]
pub mod modulename5;
#[doc = "MODULENAME6 (r) register accessor: Characters 25-26 of Module Name stored as a null terminated string\n\nYou can [`read`](crate::Reg::read) this register and get [`modulename6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modulename6`] module"]
#[doc(alias = "MODULENAME6")]
pub type Modulename6 = crate::Reg<modulename6::Modulename6Spec>;
#[doc = "Characters 25-26 of Module Name stored as a null terminated string"]
pub mod modulename6;
#[doc = "MODULEINFO (r) register accessor: Module Information\n\nYou can [`read`](crate::Reg::read) this register and get [`moduleinfo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@moduleinfo`] module"]
#[doc(alias = "MODULEINFO")]
pub type Moduleinfo = crate::Reg<moduleinfo::ModuleinfoSpec>;
#[doc = "Module Information"]
pub mod moduleinfo;
#[doc = "MODXOCAL (r) register accessor: Module Crystal Oscillator Calibration\n\nYou can [`read`](crate::Reg::read) this register and get [`modxocal::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@modxocal`] module"]
#[doc(alias = "MODXOCAL")]
pub type Modxocal = crate::Reg<modxocal::ModxocalSpec>;
#[doc = "Module Crystal Oscillator Calibration"]
pub mod modxocal;
#[doc = "HFXOCAL (r) register accessor: High Frequency Crystal Oscillator Calibration data\n\nYou can [`read`](crate::Reg::read) this register and get [`hfxocal::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfxocal`] module"]
#[doc(alias = "HFXOCAL")]
pub type Hfxocal = crate::Reg<hfxocal::HfxocalSpec>;
#[doc = "High Frequency Crystal Oscillator Calibration data"]
pub mod hfxocal;
#[doc = "IADC0GAIN0 (r) register accessor: IADC0 Gain Calibration Info\n\nYou can [`read`](crate::Reg::read) this register and get [`iadc0gain0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iadc0gain0`] module"]
#[doc(alias = "IADC0GAIN0")]
pub type Iadc0gain0 = crate::Reg<iadc0gain0::Iadc0gain0Spec>;
#[doc = "IADC0 Gain Calibration Info"]
pub mod iadc0gain0;
#[doc = "IADC0GAIN1 (r) register accessor: IADC0 Gain Calibration Info\n\nYou can [`read`](crate::Reg::read) this register and get [`iadc0gain1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iadc0gain1`] module"]
#[doc(alias = "IADC0GAIN1")]
pub type Iadc0gain1 = crate::Reg<iadc0gain1::Iadc0gain1Spec>;
#[doc = "IADC0 Gain Calibration Info"]
pub mod iadc0gain1;
#[doc = "IADC0OFFSETCAL0 (r) register accessor: IADC0 Offset Calibration Info\n\nYou can [`read`](crate::Reg::read) this register and get [`iadc0offsetcal0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iadc0offsetcal0`] module"]
#[doc(alias = "IADC0OFFSETCAL0")]
pub type Iadc0offsetcal0 = crate::Reg<iadc0offsetcal0::Iadc0offsetcal0Spec>;
#[doc = "IADC0 Offset Calibration Info"]
pub mod iadc0offsetcal0;
#[doc = "IADC0NORMALOFFSETCAL0 (r) register accessor: IADC0 Normal Offset Calibration Info\n\nYou can [`read`](crate::Reg::read) this register and get [`iadc0normaloffsetcal0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iadc0normaloffsetcal0`] module"]
#[doc(alias = "IADC0NORMALOFFSETCAL0")]
pub type Iadc0normaloffsetcal0 = crate::Reg<iadc0normaloffsetcal0::Iadc0normaloffsetcal0Spec>;
#[doc = "IADC0 Normal Offset Calibration Info"]
pub mod iadc0normaloffsetcal0;
#[doc = "IADC0NORMALOFFSETCAL1 (r) register accessor: IADC0 Normal Offset Calibration Info\n\nYou can [`read`](crate::Reg::read) this register and get [`iadc0normaloffsetcal1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iadc0normaloffsetcal1`] module"]
#[doc(alias = "IADC0NORMALOFFSETCAL1")]
pub type Iadc0normaloffsetcal1 = crate::Reg<iadc0normaloffsetcal1::Iadc0normaloffsetcal1Spec>;
#[doc = "IADC0 Normal Offset Calibration Info"]
pub mod iadc0normaloffsetcal1;
#[doc = "IADC0HISPDOFFSETCAL0 (r) register accessor: IADC High Speed Offset Calibration Info\n\nYou can [`read`](crate::Reg::read) this register and get [`iadc0hispdoffsetcal0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iadc0hispdoffsetcal0`] module"]
#[doc(alias = "IADC0HISPDOFFSETCAL0")]
pub type Iadc0hispdoffsetcal0 = crate::Reg<iadc0hispdoffsetcal0::Iadc0hispdoffsetcal0Spec>;
#[doc = "IADC High Speed Offset Calibration Info"]
pub mod iadc0hispdoffsetcal0;
#[doc = "IADC0HISPDOFFSETCAL1 (r) register accessor: IADC High Speed Offset Calibration Info\n\nYou can [`read`](crate::Reg::read) this register and get [`iadc0hispdoffsetcal1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iadc0hispdoffsetcal1`] module"]
#[doc(alias = "IADC0HISPDOFFSETCAL1")]
pub type Iadc0hispdoffsetcal1 = crate::Reg<iadc0hispdoffsetcal1::Iadc0hispdoffsetcal1Spec>;
#[doc = "IADC High Speed Offset Calibration Info"]
pub mod iadc0hispdoffsetcal1;
#[doc = "LEGACY (r) register accessor: This is the legacy device detection information for tools compatability\n\nYou can [`read`](crate::Reg::read) this register and get [`legacy::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@legacy`] module"]
#[doc(alias = "LEGACY")]
pub type Legacy = crate::Reg<legacy::LegacySpec>;
#[doc = "This is the legacy device detection information for tools compatability"]
pub mod legacy;
#[doc = "RTHERM (r) register accessor: RTHERM\n\nYou can [`read`](crate::Reg::read) this register and get [`rtherm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtherm`] module"]
#[doc(alias = "RTHERM")]
pub type Rtherm = crate::Reg<rtherm::RthermSpec>;
#[doc = "RTHERM"]
pub mod rtherm;
