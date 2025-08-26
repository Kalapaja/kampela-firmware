#[doc = "Register `TIMER1_CDTI1ROUTE` reader"]
pub type R = crate::R<Timer1Cdti1routeSpec>;
#[doc = "Register `TIMER1_CDTI1ROUTE` writer"]
pub type W = crate::W<Timer1Cdti1routeSpec>;
#[doc = "Field `PORT` reader - CDTI1 port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - CDTI1 port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - CDTI1 pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - CDTI1 pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CDTI1 port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - CDTI1 pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CDTI1 port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<Timer1Cdti1routeSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - CDTI1 pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<Timer1Cdti1routeSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "CDTI1 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`timer1_cdti1route::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer1_cdti1route::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timer1Cdti1routeSpec;
impl crate::RegisterSpec for Timer1Cdti1routeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1_cdti1route::R`](R) reader structure"]
impl crate::Readable for Timer1Cdti1routeSpec {}
#[doc = "`write(|w| ..)` method takes [`timer1_cdti1route::W`](W) writer structure"]
impl crate::Writable for Timer1Cdti1routeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER1_CDTI1ROUTE to value 0"]
impl crate::Resettable for Timer1Cdti1routeSpec {}
