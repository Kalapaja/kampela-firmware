#[doc = "Register `EFPIEN` reader"]
pub type R = crate::R<EfpienSpec>;
#[doc = "Register `EFPIEN` writer"]
pub type W = crate::W<EfpienSpec>;
#[doc = "Field `EFPIEN` reader - EFP Interrupt enable"]
pub type EfpienR = crate::BitReader;
#[doc = "Field `EFPIEN` writer - EFP Interrupt enable"]
pub type EfpienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EFP Interrupt enable"]
    #[inline(always)]
    pub fn efpien(&self) -> EfpienR {
        EfpienR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EFP Interrupt enable"]
    #[inline(always)]
    pub fn efpien(&mut self) -> EfpienW<EfpienSpec> {
        EfpienW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`efpien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efpien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfpienSpec;
impl crate::RegisterSpec for EfpienSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efpien::R`](R) reader structure"]
impl crate::Readable for EfpienSpec {}
#[doc = "`write(|w| ..)` method takes [`efpien::W`](W) writer structure"]
impl crate::Writable for EfpienSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EFPIEN to value 0"]
impl crate::Resettable for EfpienSpec {}
