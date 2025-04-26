#[doc = "Register `RXDATAXP` reader"]
pub type R = crate::R<RxdataxpSpec>;
#[doc = "Field `RXDATAP` reader - RX Data Peek"]
pub type RxdatapR = crate::FieldReader<u16>;
#[doc = "Field `PERRP` reader - Data Parity Error Peek"]
pub type PerrpR = crate::BitReader;
#[doc = "Field `FERRP` reader - Data Framing Error Peek"]
pub type FerrpR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - RX Data Peek"]
    #[inline(always)]
    pub fn rxdatap(&self) -> RxdatapR {
        RxdatapR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 14 - Data Parity Error Peek"]
    #[inline(always)]
    pub fn perrp(&self) -> PerrpR {
        PerrpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data Framing Error Peek"]
    #[inline(always)]
    pub fn ferrp(&self) -> FerrpR {
        FerrpR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdataxp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdataxpSpec;
impl crate::RegisterSpec for RxdataxpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdataxp::R`](R) reader structure"]
impl crate::Readable for RxdataxpSpec {}
#[doc = "`reset()` method sets RXDATAXP to value 0"]
impl crate::Resettable for RxdataxpSpec {}
