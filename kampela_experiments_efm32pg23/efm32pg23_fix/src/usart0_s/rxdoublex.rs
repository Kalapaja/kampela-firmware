#[doc = "Register `RXDOUBLEX` reader"]
pub type R = crate::R<RxdoublexSpec>;
#[doc = "Field `RXDATA0` reader - RX Data 0"]
pub type Rxdata0R = crate::FieldReader<u16>;
#[doc = "Field `PERR0` reader - Data Parity Error 0"]
pub type Perr0R = crate::BitReader;
#[doc = "Field `FERR0` reader - Data Framing Error 0"]
pub type Ferr0R = crate::BitReader;
#[doc = "Field `RXDATA1` reader - RX Data 1"]
pub type Rxdata1R = crate::FieldReader<u16>;
#[doc = "Field `PERR1` reader - Data Parity Error 1"]
pub type Perr1R = crate::BitReader;
#[doc = "Field `FERR1` reader - Data Framing Error 1"]
pub type Ferr1R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - RX Data 0"]
    #[inline(always)]
    pub fn rxdata0(&self) -> Rxdata0R {
        Rxdata0R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 14 - Data Parity Error 0"]
    #[inline(always)]
    pub fn perr0(&self) -> Perr0R {
        Perr0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data Framing Error 0"]
    #[inline(always)]
    pub fn ferr0(&self) -> Ferr0R {
        Ferr0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:24 - RX Data 1"]
    #[inline(always)]
    pub fn rxdata1(&self) -> Rxdata1R {
        Rxdata1R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 30 - Data Parity Error 1"]
    #[inline(always)]
    pub fn perr1(&self) -> Perr1R {
        Perr1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Data Framing Error 1"]
    #[inline(always)]
    pub fn ferr1(&self) -> Ferr1R {
        Ferr1R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdoublex::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdoublexSpec;
impl crate::RegisterSpec for RxdoublexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdoublex::R`](R) reader structure"]
impl crate::Readable for RxdoublexSpec {}
#[doc = "`reset()` method sets RXDOUBLEX to value 0"]
impl crate::Resettable for RxdoublexSpec {}
