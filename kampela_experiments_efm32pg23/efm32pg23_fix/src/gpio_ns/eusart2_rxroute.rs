#[doc = "Register `EUSART2_RXROUTE` reader"]
pub type R = crate::R<Eusart2RxrouteSpec>;
#[doc = "Register `EUSART2_RXROUTE` writer"]
pub type W = crate::W<Eusart2RxrouteSpec>;
#[doc = "Field `PORT` reader - RX port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - RX port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - RX pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - RX pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - RX port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - RX pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RX port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<Eusart2RxrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - RX pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<Eusart2RxrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "RX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart2_rxroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart2_rxroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eusart2RxrouteSpec;
impl crate::RegisterSpec for Eusart2RxrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eusart2_rxroute::R`](R) reader structure"]
impl crate::Readable for Eusart2RxrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`eusart2_rxroute::W`](W) writer structure"]
impl crate::Writable for Eusart2RxrouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EUSART2_RXROUTE to value 0"]
impl crate::Resettable for Eusart2RxrouteSpec {}
