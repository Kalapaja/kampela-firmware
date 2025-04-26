#[doc = "Register `LETIMER_OUT1ROUTE` reader"]
pub type R = crate::R<LetimerOut1routeSpec>;
#[doc = "Register `LETIMER_OUT1ROUTE` writer"]
pub type W = crate::W<LetimerOut1routeSpec>;
#[doc = "Field `PORT` reader - OUT1 port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - OUT1 port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - OUT1 pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - OUT1 pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - OUT1 port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - OUT1 pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - OUT1 port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<LetimerOut1routeSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - OUT1 pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<LetimerOut1routeSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "OUT1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`letimer_out1route::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`letimer_out1route::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LetimerOut1routeSpec;
impl crate::RegisterSpec for LetimerOut1routeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`letimer_out1route::R`](R) reader structure"]
impl crate::Readable for LetimerOut1routeSpec {}
#[doc = "`write(|w| ..)` method takes [`letimer_out1route::W`](W) writer structure"]
impl crate::Writable for LetimerOut1routeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LETIMER_OUT1ROUTE to value 0"]
impl crate::Resettable for LetimerOut1routeSpec {}
