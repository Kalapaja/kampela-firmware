#[doc = "Register `LETIMER_OUT0ROUTE` reader"]
pub type R = crate::R<LetimerOut0routeSpec>;
#[doc = "Register `LETIMER_OUT0ROUTE` writer"]
pub type W = crate::W<LetimerOut0routeSpec>;
#[doc = "Field `PORT` reader - OUT0 port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - OUT0 port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - OUT0 pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - OUT0 pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - OUT0 port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - OUT0 pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - OUT0 port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<LetimerOut0routeSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - OUT0 pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<LetimerOut0routeSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "OUT0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`letimer_out0route::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`letimer_out0route::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LetimerOut0routeSpec;
impl crate::RegisterSpec for LetimerOut0routeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`letimer_out0route::R`](R) reader structure"]
impl crate::Readable for LetimerOut0routeSpec {}
#[doc = "`write(|w| ..)` method takes [`letimer_out0route::W`](W) writer structure"]
impl crate::Writable for LetimerOut0routeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LETIMER_OUT0ROUTE to value 0"]
impl crate::Resettable for LetimerOut0routeSpec {}
