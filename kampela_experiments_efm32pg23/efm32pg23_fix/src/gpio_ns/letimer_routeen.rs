#[doc = "Register `LETIMER_ROUTEEN` reader"]
pub type R = crate::R<LetimerRouteenSpec>;
#[doc = "Register `LETIMER_ROUTEEN` writer"]
pub type W = crate::W<LetimerRouteenSpec>;
#[doc = "Field `OUT0PEN` reader - OUT0 pin enable control bit"]
pub type Out0penR = crate::BitReader;
#[doc = "Field `OUT0PEN` writer - OUT0 pin enable control bit"]
pub type Out0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT1PEN` reader - OUT1 pin enable control bit"]
pub type Out1penR = crate::BitReader;
#[doc = "Field `OUT1PEN` writer - OUT1 pin enable control bit"]
pub type Out1penW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - OUT0 pin enable control bit"]
    #[inline(always)]
    pub fn out0pen(&self) -> Out0penR {
        Out0penR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OUT1 pin enable control bit"]
    #[inline(always)]
    pub fn out1pen(&self) -> Out1penR {
        Out1penR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OUT0 pin enable control bit"]
    #[inline(always)]
    pub fn out0pen(&mut self) -> Out0penW<LetimerRouteenSpec> {
        Out0penW::new(self, 0)
    }
    #[doc = "Bit 1 - OUT1 pin enable control bit"]
    #[inline(always)]
    pub fn out1pen(&mut self) -> Out1penW<LetimerRouteenSpec> {
        Out1penW::new(self, 1)
    }
}
#[doc = "LETIMER pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`letimer_routeen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`letimer_routeen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LetimerRouteenSpec;
impl crate::RegisterSpec for LetimerRouteenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`letimer_routeen::R`](R) reader structure"]
impl crate::Readable for LetimerRouteenSpec {}
#[doc = "`write(|w| ..)` method takes [`letimer_routeen::W`](W) writer structure"]
impl crate::Writable for LetimerRouteenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LETIMER_ROUTEEN to value 0"]
impl crate::Resettable for LetimerRouteenSpec {}
