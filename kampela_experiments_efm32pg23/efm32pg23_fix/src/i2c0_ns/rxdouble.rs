#[doc = "Register `RXDOUBLE` reader"]
pub type R = crate::R<RxdoubleSpec>;
#[doc = "Field `RXDATA0` reader - RX Data 0"]
pub type Rxdata0R = crate::FieldReader;
#[doc = "Field `RXDATA1` reader - RX Data 1"]
pub type Rxdata1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RX Data 0"]
    #[inline(always)]
    pub fn rxdata0(&self) -> Rxdata0R {
        Rxdata0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX Data 1"]
    #[inline(always)]
    pub fn rxdata1(&self) -> Rxdata1R {
        Rxdata1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdouble::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdoubleSpec;
impl crate::RegisterSpec for RxdoubleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdouble::R`](R) reader structure"]
impl crate::Readable for RxdoubleSpec {}
#[doc = "`reset()` method sets RXDOUBLE to value 0"]
impl crate::Resettable for RxdoubleSpec {}
