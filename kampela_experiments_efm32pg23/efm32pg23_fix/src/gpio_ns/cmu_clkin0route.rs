#[doc = "Register `CMU_CLKIN0ROUTE` reader"]
pub type R = crate::R<CmuClkin0routeSpec>;
#[doc = "Register `CMU_CLKIN0ROUTE` writer"]
pub type W = crate::W<CmuClkin0routeSpec>;
#[doc = "Field `PORT` reader - CLKIN0 port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - CLKIN0 port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - CLKIN0 pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - CLKIN0 pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CLKIN0 port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - CLKIN0 pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CLKIN0 port select register"]
    #[inline(always)]
    pub fn port(&mut self) -> PortW<CmuClkin0routeSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - CLKIN0 pin select register"]
    #[inline(always)]
    pub fn pin(&mut self) -> PinW<CmuClkin0routeSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "CLKIN0 port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`cmu_clkin0route::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmu_clkin0route::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmuClkin0routeSpec;
impl crate::RegisterSpec for CmuClkin0routeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmu_clkin0route::R`](R) reader structure"]
impl crate::Readable for CmuClkin0routeSpec {}
#[doc = "`write(|w| ..)` method takes [`cmu_clkin0route::W`](W) writer structure"]
impl crate::Writable for CmuClkin0routeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMU_CLKIN0ROUTE to value 0"]
impl crate::Resettable for CmuClkin0routeSpec {}
