#[doc = "Register `RX_STATUS` reader"]
pub type R = crate::R<RxStatusSpec>;
#[doc = "Field `REMBYTES` reader - REMBYTES"]
pub type RembytesR = crate::FieldReader<u16>;
#[doc = "Field `MSGINFO` reader - MSGINFO"]
pub type MsginfoR = crate::FieldReader;
#[doc = "Field `RXINT` reader - RXINT"]
pub type RxintR = crate::BitReader;
#[doc = "Field `RXEMPTY` reader - RXEMPTY"]
pub type RxemptyR = crate::BitReader;
#[doc = "Field `RXHDR` reader - RXHDR"]
pub type RxhdrR = crate::BitReader;
#[doc = "Field `RXERROR` reader - RXERROR"]
pub type RxerrorR = crate::BitReader;
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
    #[doc = "Bit 20 - RXINT"]
    #[inline(always)]
    pub fn rxint(&self) -> RxintR {
        RxintR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RXEMPTY"]
    #[inline(always)]
    pub fn rxempty(&self) -> RxemptyR {
        RxemptyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - RXHDR"]
    #[inline(always)]
    pub fn rxhdr(&self) -> RxhdrR {
        RxhdrR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - RXERROR"]
    #[inline(always)]
    pub fn rxerror(&self) -> RxerrorR {
        RxerrorR::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "RX Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxStatusSpec;
impl crate::RegisterSpec for RxStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_status::R`](R) reader structure"]
impl crate::Readable for RxStatusSpec {}
#[doc = "`reset()` method sets RX_STATUS to value 0"]
impl crate::Resettable for RxStatusSpec {}
