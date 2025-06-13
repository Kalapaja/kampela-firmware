#[doc = "Register `CMU_ROUTEEN` reader"]
pub type R = crate::R<CmuRouteenSpec>;
#[doc = "Register `CMU_ROUTEEN` writer"]
pub type W = crate::W<CmuRouteenSpec>;
#[doc = "Field `CLKOUT0PEN` reader - CLKOUT0 pin enable control bit"]
pub type Clkout0penR = crate::BitReader;
#[doc = "Field `CLKOUT0PEN` writer - CLKOUT0 pin enable control bit"]
pub type Clkout0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKOUT1PEN` reader - CLKOUT1 pin enable control bit"]
pub type Clkout1penR = crate::BitReader;
#[doc = "Field `CLKOUT1PEN` writer - CLKOUT1 pin enable control bit"]
pub type Clkout1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKOUT2PEN` reader - CLKOUT2 pin enable control bit"]
pub type Clkout2penR = crate::BitReader;
#[doc = "Field `CLKOUT2PEN` writer - CLKOUT2 pin enable control bit"]
pub type Clkout2penW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CLKOUT0 pin enable control bit"]
    #[inline(always)]
    pub fn clkout0pen(&self) -> Clkout0penR {
        Clkout0penR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CLKOUT1 pin enable control bit"]
    #[inline(always)]
    pub fn clkout1pen(&self) -> Clkout1penR {
        Clkout1penR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CLKOUT2 pin enable control bit"]
    #[inline(always)]
    pub fn clkout2pen(&self) -> Clkout2penR {
        Clkout2penR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLKOUT0 pin enable control bit"]
    #[inline(always)]
    pub fn clkout0pen(&mut self) -> Clkout0penW<CmuRouteenSpec> {
        Clkout0penW::new(self, 0)
    }
    #[doc = "Bit 1 - CLKOUT1 pin enable control bit"]
    #[inline(always)]
    pub fn clkout1pen(&mut self) -> Clkout1penW<CmuRouteenSpec> {
        Clkout1penW::new(self, 1)
    }
    #[doc = "Bit 2 - CLKOUT2 pin enable control bit"]
    #[inline(always)]
    pub fn clkout2pen(&mut self) -> Clkout2penW<CmuRouteenSpec> {
        Clkout2penW::new(self, 2)
    }
}
#[doc = "CMU pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`cmu_routeen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmu_routeen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmuRouteenSpec;
impl crate::RegisterSpec for CmuRouteenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmu_routeen::R`](R) reader structure"]
impl crate::Readable for CmuRouteenSpec {}
#[doc = "`write(|w| ..)` method takes [`cmu_routeen::W`](W) writer structure"]
impl crate::Writable for CmuRouteenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMU_ROUTEEN to value 0"]
impl crate::Resettable for CmuRouteenSpec {}
