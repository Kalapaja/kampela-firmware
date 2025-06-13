#[doc = "Register `ACMP0_ROUTEEN` reader"]
pub type R = crate::R<Acmp0RouteenSpec>;
#[doc = "Register `ACMP0_ROUTEEN` writer"]
pub type W = crate::W<Acmp0RouteenSpec>;
#[doc = "Field `ACMPOUTPEN` reader - ACMPOUT pin enable control bit"]
pub type AcmpoutpenR = crate::BitReader;
#[doc = "Field `ACMPOUTPEN` writer - ACMPOUT pin enable control bit"]
pub type AcmpoutpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ACMPOUT pin enable control bit"]
    #[inline(always)]
    pub fn acmpoutpen(&self) -> AcmpoutpenR {
        AcmpoutpenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACMPOUT pin enable control bit"]
    #[inline(always)]
    pub fn acmpoutpen(&mut self) -> AcmpoutpenW<Acmp0RouteenSpec> {
        AcmpoutpenW::new(self, 0)
    }
}
#[doc = "ACMP0 pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`acmp0_routeen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acmp0_routeen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Acmp0RouteenSpec;
impl crate::RegisterSpec for Acmp0RouteenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acmp0_routeen::R`](R) reader structure"]
impl crate::Readable for Acmp0RouteenSpec {}
#[doc = "`write(|w| ..)` method takes [`acmp0_routeen::W`](W) writer structure"]
impl crate::Writable for Acmp0RouteenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACMP0_ROUTEEN to value 0"]
impl crate::Resettable for Acmp0RouteenSpec {}
