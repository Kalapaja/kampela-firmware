#[doc = "Register `EUSART2_ROUTEEN` reader"]
pub type R = crate::R<Eusart2RouteenSpec>;
#[doc = "Register `EUSART2_ROUTEEN` writer"]
pub type W = crate::W<Eusart2RouteenSpec>;
#[doc = "Field `CSPEN` reader - CS pin enable control bit"]
pub type CspenR = crate::BitReader;
#[doc = "Field `CSPEN` writer - CS pin enable control bit"]
pub type CspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSPEN` reader - RTS pin enable control bit"]
pub type RtspenR = crate::BitReader;
#[doc = "Field `RTSPEN` writer - RTS pin enable control bit"]
pub type RtspenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXPEN` reader - RX pin enable control bit"]
pub type RxpenR = crate::BitReader;
#[doc = "Field `RXPEN` writer - RX pin enable control bit"]
pub type RxpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCLKPEN` reader - SCLK pin enable control bit"]
pub type SclkpenR = crate::BitReader;
#[doc = "Field `SCLKPEN` writer - SCLK pin enable control bit"]
pub type SclkpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPEN` reader - TX pin enable control bit"]
pub type TxpenR = crate::BitReader;
#[doc = "Field `TXPEN` writer - TX pin enable control bit"]
pub type TxpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CS pin enable control bit"]
    #[inline(always)]
    pub fn cspen(&self) -> CspenR {
        CspenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTS pin enable control bit"]
    #[inline(always)]
    pub fn rtspen(&self) -> RtspenR {
        RtspenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX pin enable control bit"]
    #[inline(always)]
    pub fn rxpen(&self) -> RxpenR {
        RxpenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCLK pin enable control bit"]
    #[inline(always)]
    pub fn sclkpen(&self) -> SclkpenR {
        SclkpenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TX pin enable control bit"]
    #[inline(always)]
    pub fn txpen(&self) -> TxpenR {
        TxpenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CS pin enable control bit"]
    #[inline(always)]
    pub fn cspen(&mut self) -> CspenW<Eusart2RouteenSpec> {
        CspenW::new(self, 0)
    }
    #[doc = "Bit 1 - RTS pin enable control bit"]
    #[inline(always)]
    pub fn rtspen(&mut self) -> RtspenW<Eusart2RouteenSpec> {
        RtspenW::new(self, 1)
    }
    #[doc = "Bit 2 - RX pin enable control bit"]
    #[inline(always)]
    pub fn rxpen(&mut self) -> RxpenW<Eusart2RouteenSpec> {
        RxpenW::new(self, 2)
    }
    #[doc = "Bit 3 - SCLK pin enable control bit"]
    #[inline(always)]
    pub fn sclkpen(&mut self) -> SclkpenW<Eusart2RouteenSpec> {
        SclkpenW::new(self, 3)
    }
    #[doc = "Bit 4 - TX pin enable control bit"]
    #[inline(always)]
    pub fn txpen(&mut self) -> TxpenW<Eusart2RouteenSpec> {
        TxpenW::new(self, 4)
    }
}
#[doc = "EUSART2 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart2_routeen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart2_routeen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eusart2RouteenSpec;
impl crate::RegisterSpec for Eusart2RouteenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eusart2_routeen::R`](R) reader structure"]
impl crate::Readable for Eusart2RouteenSpec {}
#[doc = "`write(|w| ..)` method takes [`eusart2_routeen::W`](W) writer structure"]
impl crate::Writable for Eusart2RouteenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EUSART2_ROUTEEN to value 0"]
impl crate::Resettable for Eusart2RouteenSpec {}
