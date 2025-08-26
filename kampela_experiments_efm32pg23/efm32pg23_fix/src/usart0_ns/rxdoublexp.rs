#[doc = "Register `RXDOUBLEXP` reader"]
pub type R = crate::R<RxdoublexpSpec>;
#[doc = "Field `RXDATAP0` reader - RX Data 0 Peek"]
pub type Rxdatap0R = crate::FieldReader<u16>;
#[doc = "Field `PERRP0` reader - Data Parity Error 0 Peek"]
pub type Perrp0R = crate::BitReader;
#[doc = "Field `FERRP0` reader - Data Framing Error 0 Peek"]
pub type Ferrp0R = crate::BitReader;
#[doc = "Field `RXDATAP1` reader - RX Data 1 Peek"]
pub type Rxdatap1R = crate::FieldReader<u16>;
#[doc = "Field `PERRP1` reader - Data Parity Error 1 Peek"]
pub type Perrp1R = crate::BitReader;
#[doc = "Field `FERRP1` reader - Data Framing Error 1 Peek"]
pub type Ferrp1R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - RX Data 0 Peek"]
    #[inline(always)]
    pub fn rxdatap0(&self) -> Rxdatap0R {
        Rxdatap0R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 14 - Data Parity Error 0 Peek"]
    #[inline(always)]
    pub fn perrp0(&self) -> Perrp0R {
        Perrp0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Data Framing Error 0 Peek"]
    #[inline(always)]
    pub fn ferrp0(&self) -> Ferrp0R {
        Ferrp0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:24 - RX Data 1 Peek"]
    #[inline(always)]
    pub fn rxdatap1(&self) -> Rxdatap1R {
        Rxdatap1R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 30 - Data Parity Error 1 Peek"]
    #[inline(always)]
    pub fn perrp1(&self) -> Perrp1R {
        Perrp1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Data Framing Error 1 Peek"]
    #[inline(always)]
    pub fn ferrp1(&self) -> Ferrp1R {
        Ferrp1R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdoublexp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdoublexpSpec;
impl crate::RegisterSpec for RxdoublexpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdoublexp::R`](R) reader structure"]
impl crate::Readable for RxdoublexpSpec {}
#[doc = "`reset()` method sets RXDOUBLEXP to value 0"]
impl crate::Resettable for RxdoublexpSpec {}
