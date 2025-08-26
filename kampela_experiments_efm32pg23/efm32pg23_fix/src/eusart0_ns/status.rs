#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `RXENS` reader - Receiver Enable Status"]
pub type RxensR = crate::BitReader;
#[doc = "Field `TXENS` reader - Transmitter Enable Status"]
pub type TxensR = crate::BitReader;
#[doc = "Field `RXBLOCK` reader - Block Incoming Data"]
pub type RxblockR = crate::BitReader;
#[doc = "Field `TXTRI` reader - Transmitter Tristated"]
pub type TxtriR = crate::BitReader;
#[doc = "Field `TXC` reader - TX Complete"]
pub type TxcR = crate::BitReader;
#[doc = "Field `TXFL` reader - TX FIFO Level"]
pub type TxflR = crate::BitReader;
#[doc = "Field `RXFL` reader - RX FIFO Level"]
pub type RxflR = crate::BitReader;
#[doc = "Field `RXFULL` reader - RX FIFO Full"]
pub type RxfullR = crate::BitReader;
#[doc = "Field `RXIDLE` reader - RX Idle"]
pub type RxidleR = crate::BitReader;
#[doc = "Field `TXIDLE` reader - TX Idle"]
pub type TxidleR = crate::BitReader;
#[doc = "Field `TXFCNT` reader - Valid entries in TX FIFO"]
pub type TxfcntR = crate::FieldReader;
#[doc = "Field `AUTOBAUDDONE` reader - Auto Baud Rate Detection Completed"]
pub type AutobauddoneR = crate::BitReader;
#[doc = "Field `CLEARTXBUSY` reader - TX FIFO Clear Busy"]
pub type CleartxbusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receiver Enable Status"]
    #[inline(always)]
    pub fn rxens(&self) -> RxensR {
        RxensR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Enable Status"]
    #[inline(always)]
    pub fn txens(&self) -> TxensR {
        TxensR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Block Incoming Data"]
    #[inline(always)]
    pub fn rxblock(&self) -> RxblockR {
        RxblockR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmitter Tristated"]
    #[inline(always)]
    pub fn txtri(&self) -> TxtriR {
        TxtriR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TxcR {
        TxcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TX FIFO Level"]
    #[inline(always)]
    pub fn txfl(&self) -> TxflR {
        TxflR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX FIFO Level"]
    #[inline(always)]
    pub fn rxfl(&self) -> RxflR {
        RxflR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RX FIFO Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RxfullR {
        RxfullR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - RX Idle"]
    #[inline(always)]
    pub fn rxidle(&self) -> RxidleR {
        RxidleR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TX Idle"]
    #[inline(always)]
    pub fn txidle(&self) -> TxidleR {
        TxidleR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Valid entries in TX FIFO"]
    #[inline(always)]
    pub fn txfcnt(&self) -> TxfcntR {
        TxfcntR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Auto Baud Rate Detection Completed"]
    #[inline(always)]
    pub fn autobauddone(&self) -> AutobauddoneR {
        AutobauddoneR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - TX FIFO Clear Busy"]
    #[inline(always)]
    pub fn cleartxbusy(&self) -> CleartxbusyR {
        CleartxbusyR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x3040"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x3040;
}
