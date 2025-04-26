#[doc = "Register `TRIGCTRL` reader"]
pub type R = crate::R<TrigctrlSpec>;
#[doc = "Register `TRIGCTRL` writer"]
pub type W = crate::W<TrigctrlSpec>;
#[doc = "Field `RXTEN` reader - Receive Trigger Enable"]
pub type RxtenR = crate::BitReader;
#[doc = "Field `RXTEN` writer - Receive Trigger Enable"]
pub type RxtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTEN` reader - Transmit Trigger Enable"]
pub type TxtenR = crate::BitReader;
#[doc = "Field `TXTEN` writer - Transmit Trigger Enable"]
pub type TxtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOTXTEN` reader - AUTOTX Trigger Enable"]
pub type AutotxtenR = crate::BitReader;
#[doc = "Field `AUTOTXTEN` writer - AUTOTX Trigger Enable"]
pub type AutotxtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXARX0EN` reader - Enable Transmit Trigger after RX End of"]
pub type Txarx0enR = crate::BitReader;
#[doc = "Field `TXARX0EN` writer - Enable Transmit Trigger after RX End of"]
pub type Txarx0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXARX1EN` reader - Enable Transmit Trigger after RX End of"]
pub type Txarx1enR = crate::BitReader;
#[doc = "Field `TXARX1EN` writer - Enable Transmit Trigger after RX End of"]
pub type Txarx1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXARX2EN` reader - Enable Transmit Trigger after RX End of"]
pub type Txarx2enR = crate::BitReader;
#[doc = "Field `TXARX2EN` writer - Enable Transmit Trigger after RX End of"]
pub type Txarx2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXATX0EN` reader - Enable Receive Trigger after TX end of f"]
pub type Rxatx0enR = crate::BitReader;
#[doc = "Field `RXATX0EN` writer - Enable Receive Trigger after TX end of f"]
pub type Rxatx0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXATX1EN` reader - Enable Receive Trigger after TX end of f"]
pub type Rxatx1enR = crate::BitReader;
#[doc = "Field `RXATX1EN` writer - Enable Receive Trigger after TX end of f"]
pub type Rxatx1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXATX2EN` reader - Enable Receive Trigger after TX end of f"]
pub type Rxatx2enR = crate::BitReader;
#[doc = "Field `RXATX2EN` writer - Enable Receive Trigger after TX end of f"]
pub type Rxatx2enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Receive Trigger Enable"]
    #[inline(always)]
    pub fn rxten(&self) -> RxtenR {
        RxtenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Trigger Enable"]
    #[inline(always)]
    pub fn txten(&self) -> TxtenR {
        TxtenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AUTOTX Trigger Enable"]
    #[inline(always)]
    pub fn autotxten(&self) -> AutotxtenR {
        AutotxtenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Transmit Trigger after RX End of"]
    #[inline(always)]
    pub fn txarx0en(&self) -> Txarx0enR {
        Txarx0enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Transmit Trigger after RX End of"]
    #[inline(always)]
    pub fn txarx1en(&self) -> Txarx1enR {
        Txarx1enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Transmit Trigger after RX End of"]
    #[inline(always)]
    pub fn txarx2en(&self) -> Txarx2enR {
        Txarx2enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Receive Trigger after TX end of f"]
    #[inline(always)]
    pub fn rxatx0en(&self) -> Rxatx0enR {
        Rxatx0enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Receive Trigger after TX end of f"]
    #[inline(always)]
    pub fn rxatx1en(&self) -> Rxatx1enR {
        Rxatx1enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Receive Trigger after TX end of f"]
    #[inline(always)]
    pub fn rxatx2en(&self) -> Rxatx2enR {
        Rxatx2enR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Receive Trigger Enable"]
    #[inline(always)]
    pub fn rxten(&mut self) -> RxtenW<TrigctrlSpec> {
        RxtenW::new(self, 4)
    }
    #[doc = "Bit 5 - Transmit Trigger Enable"]
    #[inline(always)]
    pub fn txten(&mut self) -> TxtenW<TrigctrlSpec> {
        TxtenW::new(self, 5)
    }
    #[doc = "Bit 6 - AUTOTX Trigger Enable"]
    #[inline(always)]
    pub fn autotxten(&mut self) -> AutotxtenW<TrigctrlSpec> {
        AutotxtenW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Transmit Trigger after RX End of"]
    #[inline(always)]
    pub fn txarx0en(&mut self) -> Txarx0enW<TrigctrlSpec> {
        Txarx0enW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Transmit Trigger after RX End of"]
    #[inline(always)]
    pub fn txarx1en(&mut self) -> Txarx1enW<TrigctrlSpec> {
        Txarx1enW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Transmit Trigger after RX End of"]
    #[inline(always)]
    pub fn txarx2en(&mut self) -> Txarx2enW<TrigctrlSpec> {
        Txarx2enW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable Receive Trigger after TX end of f"]
    #[inline(always)]
    pub fn rxatx0en(&mut self) -> Rxatx0enW<TrigctrlSpec> {
        Rxatx0enW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable Receive Trigger after TX end of f"]
    #[inline(always)]
    pub fn rxatx1en(&mut self) -> Rxatx1enW<TrigctrlSpec> {
        Rxatx1enW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable Receive Trigger after TX end of f"]
    #[inline(always)]
    pub fn rxatx2en(&mut self) -> Rxatx2enW<TrigctrlSpec> {
        Rxatx2enW::new(self, 12)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`trigctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrigctrlSpec;
impl crate::RegisterSpec for TrigctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigctrl::R`](R) reader structure"]
impl crate::Readable for TrigctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`trigctrl::W`](W) writer structure"]
impl crate::Writable for TrigctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRIGCTRL to value 0"]
impl crate::Resettable for TrigctrlSpec {}
