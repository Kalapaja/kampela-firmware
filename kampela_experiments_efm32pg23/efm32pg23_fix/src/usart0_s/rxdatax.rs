#[doc = "Register `RXDATAX` reader"]
pub type R = crate::R<RxdataxSpec>;
#[doc = "Field `RXDATA` reader - RX Data"]
pub type RxdataR = crate::FieldReader<u16>;
#[doc = "Field `PERR` reader - Data Parity Error"]
pub type PerrR = crate::BitReader;
#[doc = "Field `FERR` reader - Data Framing Error"]
pub type FerrR = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - RX Data"]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 14 - Data Parity Error"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data Framing Error"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdatax::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdataxSpec;
impl crate::RegisterSpec for RxdataxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdatax::R`](R) reader structure"]
impl crate::Readable for RxdataxSpec {}
#[doc = "`reset()` method sets RXDATAX to value 0"]
impl crate::Resettable for RxdataxSpec {}
