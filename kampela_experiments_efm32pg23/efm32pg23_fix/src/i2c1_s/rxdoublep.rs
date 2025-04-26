#[doc = "Register `RXDOUBLEP` reader"]
pub type R = crate::R<RxdoublepSpec>;
#[doc = "Field `RXDATAP0` reader - RX Data 0 Peek"]
pub type Rxdatap0R = crate::FieldReader;
#[doc = "Field `RXDATAP1` reader - RX Data 1 Peek"]
pub type Rxdatap1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RX Data 0 Peek"]
    #[inline(always)]
    pub fn rxdatap0(&self) -> Rxdatap0R {
        Rxdatap0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX Data 1 Peek"]
    #[inline(always)]
    pub fn rxdatap1(&self) -> Rxdatap1R {
        Rxdatap1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdoublep::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdoublepSpec;
impl crate::RegisterSpec for RxdoublepSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdoublep::R`](R) reader structure"]
impl crate::Readable for RxdoublepSpec {}
#[doc = "`reset()` method sets RXDOUBLEP to value 0"]
impl crate::Resettable for RxdoublepSpec {}
