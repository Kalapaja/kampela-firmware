#[doc = "Register `CMU_CLKOUT2ROUTE` reader"]
pub type R = crate::R<CmuClkout2routeSpec>;
#[doc = "Register `CMU_CLKOUT2ROUTE` writer"]
pub type W = crate::W<CmuClkout2routeSpec>;
#[doc = "Field `PORT` reader - CLKOUT2 port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - CLKOUT2 port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - CLKOUT2 pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - CLKOUT2 pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CLKOUT2 port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - CLKOUT2 pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CLKOUT2 port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<CmuClkout2routeSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - CLKOUT2 pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<CmuClkout2routeSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "CLKOUT2 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`cmu_clkout2route::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmu_clkout2route::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmuClkout2routeSpec;
impl crate::RegisterSpec for CmuClkout2routeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmu_clkout2route::R`](R) reader structure"]
impl crate::Readable for CmuClkout2routeSpec {}
#[doc = "`write(|w| ..)` method takes [`cmu_clkout2route::W`](W) writer structure"]
impl crate::Writable for CmuClkout2routeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMU_CLKOUT2ROUTE to value 0"]
impl crate::Resettable for CmuClkout2routeSpec {}
