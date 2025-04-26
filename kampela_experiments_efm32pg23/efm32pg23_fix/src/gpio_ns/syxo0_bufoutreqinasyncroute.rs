#[doc = "Register `SYXO0_BUFOUTREQINASYNCROUTE` reader"]
pub type R = crate::R<Syxo0BufoutreqinasyncrouteSpec>;
#[doc = "Register `SYXO0_BUFOUTREQINASYNCROUTE` writer"]
pub type W = crate::W<Syxo0BufoutreqinasyncrouteSpec>;
#[doc = "Field `PORT` reader - BUFOUTREQINASYNC port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - BUFOUTREQINASYNC port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - BUFOUTREQINASYNC pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - BUFOUTREQINASYNC pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - BUFOUTREQINASYNC port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - BUFOUTREQINASYNC pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - BUFOUTREQINASYNC port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<Syxo0BufoutreqinasyncrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - BUFOUTREQINASYNC pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<Syxo0BufoutreqinasyncrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "BUFOUTREQINASYNC port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`syxo0_bufoutreqinasyncroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syxo0_bufoutreqinasyncroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syxo0BufoutreqinasyncrouteSpec;
impl crate::RegisterSpec for Syxo0BufoutreqinasyncrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syxo0_bufoutreqinasyncroute::R`](R) reader structure"]
impl crate::Readable for Syxo0BufoutreqinasyncrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`syxo0_bufoutreqinasyncroute::W`](W) writer structure"]
impl crate::Writable for Syxo0BufoutreqinasyncrouteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYXO0_BUFOUTREQINASYNCROUTE to value 0"]
impl crate::Resettable for Syxo0BufoutreqinasyncrouteSpec {}
