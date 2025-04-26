#[doc = "Register `SYNCBUSY` reader"]
pub type R = crate::R<SyncbusySpec>;
#[doc = "Field `DIV` reader - SYNCBUSY for DIV in CLKDIV"]
pub type DivR = crate::BitReader;
#[doc = "Field `RXTEN` reader - SYNCBUSY for RXTEN in TRIGCTRL"]
pub type RxtenR = crate::BitReader;
#[doc = "Field `TXTEN` reader - SYNCBUSY for TXTEN in TRIGCTRL"]
pub type TxtenR = crate::BitReader;
#[doc = "Field `RXEN` reader - SYNCBUSY for RXEN in CMD"]
pub type RxenR = crate::BitReader;
#[doc = "Field `RXDIS` reader - SYNCBUSY for RXDIS in CMD"]
pub type RxdisR = crate::BitReader;
#[doc = "Field `TXEN` reader - SYNCBUSY for TXEN in CMD"]
pub type TxenR = crate::BitReader;
#[doc = "Field `TXDIS` reader - SYNCBUSY for TXDIS in CMD"]
pub type TxdisR = crate::BitReader;
#[doc = "Field `RXBLOCKEN` reader - SYNCBUSY for RXBLOCKEN in CMD"]
pub type RxblockenR = crate::BitReader;
#[doc = "Field `RXBLOCKDIS` reader - SYNCBUSY for RXBLOCKDIS in CMD"]
pub type RxblockdisR = crate::BitReader;
#[doc = "Field `TXTRIEN` reader - SYNCBUSY for TXTRIEN in CMD"]
pub type TxtrienR = crate::BitReader;
#[doc = "Field `TXTRIDIS` reader - SYNCBUSY in TXTRIDIS in CMD"]
pub type TxtridisR = crate::BitReader;
#[doc = "Field `AUTOTXTEN` reader - SYNCBUSY for AUTOTXTEN in TRIGCTRL"]
pub type AutotxtenR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SYNCBUSY for DIV in CLKDIV"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SYNCBUSY for RXTEN in TRIGCTRL"]
    #[inline(always)]
    pub fn rxten(&self) -> RxtenR {
        RxtenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SYNCBUSY for TXTEN in TRIGCTRL"]
    #[inline(always)]
    pub fn txten(&self) -> TxtenR {
        TxtenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SYNCBUSY for RXEN in CMD"]
    #[inline(always)]
    pub fn rxen(&self) -> RxenR {
        RxenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SYNCBUSY for RXDIS in CMD"]
    #[inline(always)]
    pub fn rxdis(&self) -> RxdisR {
        RxdisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SYNCBUSY for TXEN in CMD"]
    #[inline(always)]
    pub fn txen(&self) -> TxenR {
        TxenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SYNCBUSY for TXDIS in CMD"]
    #[inline(always)]
    pub fn txdis(&self) -> TxdisR {
        TxdisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SYNCBUSY for RXBLOCKEN in CMD"]
    #[inline(always)]
    pub fn rxblocken(&self) -> RxblockenR {
        RxblockenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SYNCBUSY for RXBLOCKDIS in CMD"]
    #[inline(always)]
    pub fn rxblockdis(&self) -> RxblockdisR {
        RxblockdisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SYNCBUSY for TXTRIEN in CMD"]
    #[inline(always)]
    pub fn txtrien(&self) -> TxtrienR {
        TxtrienR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SYNCBUSY in TXTRIDIS in CMD"]
    #[inline(always)]
    pub fn txtridis(&self) -> TxtridisR {
        TxtridisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SYNCBUSY for AUTOTXTEN in TRIGCTRL"]
    #[inline(always)]
    pub fn autotxten(&self) -> AutotxtenR {
        AutotxtenR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`syncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncbusySpec;
impl crate::RegisterSpec for SyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncbusy::R`](R) reader structure"]
impl crate::Readable for SyncbusySpec {}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SyncbusySpec {}
