#[doc = "Register `PCNT0_S0INROUTE` reader"]
pub type R = crate::R<Pcnt0S0inrouteSpec>;
#[doc = "Register `PCNT0_S0INROUTE` writer"]
pub type W = crate::W<Pcnt0S0inrouteSpec>;
#[doc = "Field `PORT` reader - S0IN port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - S0IN port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - S0IN pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - S0IN pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - S0IN port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - S0IN pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - S0IN port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<Pcnt0S0inrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - S0IN pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<Pcnt0S0inrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "S0IN port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`pcnt0_s0inroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcnt0_s0inroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pcnt0S0inrouteSpec;
impl crate::RegisterSpec for Pcnt0S0inrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcnt0_s0inroute::R`](R) reader structure"]
impl crate::Readable for Pcnt0S0inrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`pcnt0_s0inroute::W`](W) writer structure"]
impl crate::Writable for Pcnt0S0inrouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PCNT0_S0INROUTE to value 0"]
impl crate::Resettable for Pcnt0S0inrouteSpec {}
