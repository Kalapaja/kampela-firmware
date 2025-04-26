#[doc = "Register `EFPIF` reader"]
pub type R = crate::R<EfpifSpec>;
#[doc = "Register `EFPIF` writer"]
pub type W = crate::W<EfpifSpec>;
#[doc = "Field `EFPIF` reader - EFP Interrupt Flag"]
pub type EfpifR = crate::BitReader;
#[doc = "Field `EFPIF` writer - EFP Interrupt Flag"]
pub type EfpifW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EFP Interrupt Flag"]
    #[inline(always)]
    pub fn efpif(&self) -> EfpifR {
        EfpifR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EFP Interrupt Flag"]
    #[inline(always)]
    pub fn efpif(&mut self) -> EfpifW<EfpifSpec> {
        EfpifW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`efpif::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efpif::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EfpifSpec;
impl crate::RegisterSpec for EfpifSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efpif::R`](R) reader structure"]
impl crate::Readable for EfpifSpec {}
#[doc = "`write(|w| ..)` method takes [`efpif::W`](W) writer structure"]
impl crate::Writable for EfpifSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EFPIF to value 0"]
impl crate::Resettable for EfpifSpec {}
