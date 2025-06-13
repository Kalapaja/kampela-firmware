#[doc = "Register `TXDATAX` writer"]
pub type W = crate::W<TxdataxSpec>;
#[doc = "Field `TXDATAX` writer - TX Data"]
pub type TxdataxW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `UBRXAT` writer - Unblock RX After Transmission"]
pub type UbrxatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTRIAT` writer - Set TXTRI After Transmission"]
pub type TxtriatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBREAK` writer - Transmit Data As Break"]
pub type TxbreakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDISAT` writer - Clear TXEN After Transmission"]
pub type TxdisatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXENAT` writer - Enable RX After Transmission"]
pub type RxenatW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:8 - TX Data"]
    #[inline(always)]
    pub fn txdatax(&mut self) -> TxdataxW<TxdataxSpec> {
        TxdataxW::new(self, 0)
    }
    #[doc = "Bit 11 - Unblock RX After Transmission"]
    #[inline(always)]
    pub fn ubrxat(&mut self) -> UbrxatW<TxdataxSpec> {
        UbrxatW::new(self, 11)
    }
    #[doc = "Bit 12 - Set TXTRI After Transmission"]
    #[inline(always)]
    pub fn txtriat(&mut self) -> TxtriatW<TxdataxSpec> {
        TxtriatW::new(self, 12)
    }
    #[doc = "Bit 13 - Transmit Data As Break"]
    #[inline(always)]
    pub fn txbreak(&mut self) -> TxbreakW<TxdataxSpec> {
        TxbreakW::new(self, 13)
    }
    #[doc = "Bit 14 - Clear TXEN After Transmission"]
    #[inline(always)]
    pub fn txdisat(&mut self) -> TxdisatW<TxdataxSpec> {
        TxdisatW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable RX After Transmission"]
    #[inline(always)]
    pub fn rxenat(&mut self) -> RxenatW<TxdataxSpec> {
        RxenatW::new(self, 15)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdatax::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdataxSpec;
impl crate::RegisterSpec for TxdataxSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txdatax::W`](W) writer structure"]
impl crate::Writable for TxdataxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDATAX to value 0"]
impl crate::Resettable for TxdataxSpec {}
