#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fifo: Fifo,
    _reserved1: [u8; 0x3c],
    tx_status: TxStatus,
    rx_status: RxStatus,
    tx_prot: TxProt,
    rx_prot: RxProt,
    tx_header: TxHeader,
    rx_header: RxHeader,
    configuration: Configuration,
}
impl RegisterBlock {
    #[doc = "0x00 - A write access to any address in this area will be mapped to the TX FIFO (only for the payload). A read access to any address in this area will be mapped to the RX FIFO (only for the payload). Using an address range (16 x 32-bit) rather than one single address mapped to the FIFO allows using incremental bursts."]
    #[inline(always)]
    pub const fn fifo(&self) -> &Fifo {
        &self.fifo
    }
    #[doc = "0x40 - TX Status register."]
    #[inline(always)]
    pub const fn tx_status(&self) -> &TxStatus {
        &self.tx_status
    }
    #[doc = "0x44 - RX Status register."]
    #[inline(always)]
    pub const fn rx_status(&self) -> &RxStatus {
        &self.rx_status
    }
    #[doc = "0x48 - TX Protection register."]
    #[inline(always)]
    pub const fn tx_prot(&self) -> &TxProt {
        &self.tx_prot
    }
    #[doc = "0x4c - RX Protection register."]
    #[inline(always)]
    pub const fn rx_prot(&self) -> &RxProt {
        &self.rx_prot
    }
    #[doc = "0x50 - A write access to this register will be mapped to the TX FIFO (only for header)."]
    #[inline(always)]
    pub const fn tx_header(&self) -> &TxHeader {
        &self.tx_header
    }
    #[doc = "0x54 - A read access to this register will be mapped to the RX FIFO (only for the header)."]
    #[inline(always)]
    pub const fn rx_header(&self) -> &RxHeader {
        &self.rx_header
    }
    #[doc = "0x58 - Configuration register."]
    #[inline(always)]
    pub const fn configuration(&self) -> &Configuration {
        &self.configuration
    }
}
#[doc = "FIFO (rw) register accessor: A write access to any address in this area will be mapped to the TX FIFO (only for the payload). A read access to any address in this area will be mapped to the RX FIFO (only for the payload). Using an address range (16 x 32-bit) rather than one single address mapped to the FIFO allows using incremental bursts.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`] module"]
#[doc(alias = "FIFO")]
pub type Fifo = crate::Reg<fifo::FifoSpec>;
#[doc = "A write access to any address in this area will be mapped to the TX FIFO (only for the payload). A read access to any address in this area will be mapped to the RX FIFO (only for the payload). Using an address range (16 x 32-bit) rather than one single address mapped to the FIFO allows using incremental bursts."]
pub mod fifo;
#[doc = "TX_STATUS (r) register accessor: TX Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_status`] module"]
#[doc(alias = "TX_STATUS")]
pub type TxStatus = crate::Reg<tx_status::TxStatusSpec>;
#[doc = "TX Status register."]
pub mod tx_status;
#[doc = "RX_STATUS (r) register accessor: RX Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_status`] module"]
#[doc(alias = "RX_STATUS")]
pub type RxStatus = crate::Reg<rx_status::RxStatusSpec>;
#[doc = "RX Status register."]
pub mod rx_status;
#[doc = "TX_PROT (r) register accessor: TX Protection register.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_prot::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_prot`] module"]
#[doc(alias = "TX_PROT")]
pub type TxProt = crate::Reg<tx_prot::TxProtSpec>;
#[doc = "TX Protection register."]
pub mod tx_prot;
#[doc = "RX_PROT (r) register accessor: RX Protection register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_prot::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_prot`] module"]
#[doc(alias = "RX_PROT")]
pub type RxProt = crate::Reg<rx_prot::RxProtSpec>;
#[doc = "RX Protection register."]
pub mod rx_prot;
#[doc = "TX_HEADER (w) register accessor: A write access to this register will be mapped to the TX FIFO (only for header).\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_header::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_header`] module"]
#[doc(alias = "TX_HEADER")]
pub type TxHeader = crate::Reg<tx_header::TxHeaderSpec>;
#[doc = "A write access to this register will be mapped to the TX FIFO (only for header)."]
pub mod tx_header;
#[doc = "RX_HEADER (r) register accessor: A read access to this register will be mapped to the RX FIFO (only for the header).\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_header::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_header`] module"]
#[doc(alias = "RX_HEADER")]
pub type RxHeader = crate::Reg<rx_header::RxHeaderSpec>;
#[doc = "A read access to this register will be mapped to the RX FIFO (only for the header)."]
pub mod rx_header;
#[doc = "CONFIGURATION (rw) register accessor: Configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`configuration::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`configuration::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@configuration`] module"]
#[doc(alias = "CONFIGURATION")]
pub type Configuration = crate::Reg<configuration::ConfigurationSpec>;
#[doc = "Configuration register."]
pub mod configuration;
