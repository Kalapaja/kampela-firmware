#[doc = "Register `EUSART0_SCLKROUTE` reader"]
pub type R = crate::R<Eusart0SclkrouteSpec>;
#[doc = "Register `EUSART0_SCLKROUTE` writer"]
pub type W = crate::W<Eusart0SclkrouteSpec>;
#[doc = "Field `PORT` reader - SCLK port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - SCLK port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - SCLK pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - SCLK pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - SCLK port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - SCLK pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SCLK port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<Eusart0SclkrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - SCLK pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<Eusart0SclkrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "SCLK port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart0_sclkroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart0_sclkroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eusart0SclkrouteSpec;
impl crate::RegisterSpec for Eusart0SclkrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eusart0_sclkroute::R`](R) reader structure"]
impl crate::Readable for Eusart0SclkrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`eusart0_sclkroute::W`](W) writer structure"]
impl crate::Writable for Eusart0SclkrouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EUSART0_SCLKROUTE to value 0"]
impl crate::Resettable for Eusart0SclkrouteSpec {}
