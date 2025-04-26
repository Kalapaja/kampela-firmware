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
impl R {
    #[doc = "Bit 0 - Receive Trigger Enable"]
    #[inline(always)]
    pub fn rxten(&self) -> RxtenR {
        RxtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Trigger Enable"]
    #[inline(always)]
    pub fn txten(&self) -> TxtenR {
        TxtenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AUTOTX Trigger Enable"]
    #[inline(always)]
    pub fn autotxten(&self) -> AutotxtenR {
        AutotxtenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Trigger Enable"]
    #[inline(always)]
    pub fn rxten(&mut self) -> RxtenW<TrigctrlSpec> {
        RxtenW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Trigger Enable"]
    #[inline(always)]
    pub fn txten(&mut self) -> TxtenW<TrigctrlSpec> {
        TxtenW::new(self, 1)
    }
    #[doc = "Bit 2 - AUTOTX Trigger Enable"]
    #[inline(always)]
    pub fn autotxten(&mut self) -> AutotxtenW<TrigctrlSpec> {
        AutotxtenW::new(self, 2)
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
