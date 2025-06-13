#[doc = "Register `EUSART2_TXROUTE` reader"]
pub type R = crate::R<Eusart2TxrouteSpec>;
#[doc = "Register `EUSART2_TXROUTE` writer"]
pub type W = crate::W<Eusart2TxrouteSpec>;
#[doc = "Field `PORT` reader - TX port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - TX port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - TX pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - TX pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - TX port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - TX pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TX port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<Eusart2TxrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - TX pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<Eusart2TxrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "TX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart2_txroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart2_txroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eusart2TxrouteSpec;
impl crate::RegisterSpec for Eusart2TxrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eusart2_txroute::R`](R) reader structure"]
impl crate::Readable for Eusart2TxrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`eusart2_txroute::W`](W) writer structure"]
impl crate::Writable for Eusart2TxrouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EUSART2_TXROUTE to value 0"]
impl crate::Resettable for Eusart2TxrouteSpec {}
