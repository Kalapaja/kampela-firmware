#[doc = "Register `TXDOUBLEX` writer"]
pub type W = crate::W<TxdoublexSpec>;
#[doc = "Field `TXDATA0` writer - TX Data"]
pub type Txdata0W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `UBRXAT0` writer - Unblock RX After Transmission"]
pub type Ubrxat0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRIAT0` writer - Set TXTRI After Transmission"]
pub type Txtriat0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBREAK0` writer - Transmit Data As Break"]
pub type Txbreak0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDISAT0` writer - Clear TXEN After Transmission"]
pub type Txdisat0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXENAT0` writer - Enable RX After Transmission"]
pub type Rxenat0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDATA1` writer - TX Data"]
pub type Txdata1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `UBRXAT1` writer - Unblock RX After Transmission"]
pub type Ubrxat1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRIAT1` writer - Set TXTRI After Transmission"]
pub type Txtriat1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBREAK1` writer - Transmit Data As Break"]
pub type Txbreak1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDISAT1` writer - Clear TXEN After Transmission"]
pub type Txdisat1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXENAT1` writer - Enable RX After Transmission"]
pub type Rxenat1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    pub fn txdata0(&mut self) -> Txdata0W<TxdoublexSpec> {
        Txdata0W::new(self, 0)
    }
    #[doc = "Bit 11 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat0(&mut self) -> Ubrxat0W<TxdoublexSpec> {
        Ubrxat0W::new(self, 11)
    }
    #[doc = "Bit 12 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat0(&mut self) -> Txtriat0W<TxdoublexSpec> {
        Txtriat0W::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit Data As Break"]
    #[inline(always)]
    pub fn txbreak0(&mut self) -> Txbreak0W<TxdoublexSpec> {
        Txbreak0W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear TXEN After Transmission"]
    #[inline(always)]
    pub fn txdisat0(&mut self) -> Txdisat0W<TxdoublexSpec> {
        Txdisat0W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat0(&mut self) -> Rxenat0W<TxdoublexSpec> {
        Rxenat0W::new(self, 15)
    }
    #[doc = "Bits 16:24 - TX Data"]
    #[inline(always)]
    pub fn txdata1(&mut self) -> Txdata1W<TxdoublexSpec> {
        Txdata1W::new(self, 16)
    }
    #[doc = "Bit 27 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat1(&mut self) -> Ubrxat1W<TxdoublexSpec> {
        Ubrxat1W::new(self, 27)
    }
    #[doc = "Bit 28 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat1(&mut self) -> Txtriat1W<TxdoublexSpec> {
        Txtriat1W::new(self, 28)
    }
    #[doc = "Bit 29 - Transmit Data As Break"]
    #[inline(always)]
    pub fn txbreak1(&mut self) -> Txbreak1W<TxdoublexSpec> {
        Txbreak1W::new(self, 29)
    }
    #[doc = "Bit 30 - Clear TXEN After Transmission"]
    #[inline(always)]
    pub fn txdisat1(&mut self) -> Txdisat1W<TxdoublexSpec> {
        Txdisat1W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat1(&mut self) -> Rxenat1W<TxdoublexSpec> {
        Rxenat1W::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdoublex::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdoublexSpec;
impl crate::RegisterSpec for TxdoublexSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txdoublex::W`](W) writer structure"]
impl crate::Writable for TxdoublexSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDOUBLEX to value 0"]
impl crate::Resettable for TxdoublexSpec {}
