#[doc = "Register `TX_STATUS` reader"]
pub type R = crate::R<TxStatusSpec>;
#[doc = "Field `REMBYTES` reader - REMBYTES"]
pub type RembytesR = crate::FieldReader<u16>;
#[doc = "Field `MSGINFO` reader - MSGINFO"]
pub type MsginfoR = crate::FieldReader;
#[doc = "Field `TXINT` reader - TXINT"]
pub type TxintR = crate::BitReader;
#[doc = "Field `TXFULL` reader - TXFULL"]
pub type TxfullR = crate::BitReader;
#[doc = "Field `TXERROR` reader - TXERROR"]
pub type TxerrorR = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - REMBYTES"]
    #[inline(always)]
    pub fn rembytes(&self) -> RembytesR {
        RembytesR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - MSGINFO"]
    #[inline(always)]
    pub fn msginfo(&self) -> MsginfoR {
        MsginfoR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TXINT"]
    #[inline(always)]
    pub fn txint(&self) -> TxintR {
        TxintR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TXFULL"]
    #[inline(always)]
    pub fn txfull(&self) -> TxfullR {
        TxfullR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - TXERROR"]
    #[inline(always)]
    pub fn txerror(&self) -> TxerrorR {
        TxerrorR::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "TX Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxStatusSpec;
impl crate::RegisterSpec for TxStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_status::R`](R) reader structure"]
impl crate::Readable for TxStatusSpec {}
#[doc = "`reset()` method sets TX_STATUS to value 0"]
impl crate::Resettable for TxStatusSpec {}
