#[doc = "Register `ROOTSESWVERSION` reader"]
pub type R = crate::R<RootseswversionSpec>;
#[doc = "Register `ROOTSESWVERSION` writer"]
pub type W = crate::W<RootseswversionSpec>;
#[doc = "Field `SWVERSION` reader - SW Version"]
pub type SwversionR = crate::FieldReader<u32>;
#[doc = "Field `SWVERSION` writer - SW Version"]
pub type SwversionW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SW Version"]
    #[inline(always)]
    pub fn swversion(&self) -> SwversionR {
        SwversionR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SW Version"]
    #[inline(always)]
    pub fn swversion(&mut self) -> SwversionW<RootseswversionSpec> {
        SwversionW::new(self, 0)
    }
}
#[doc = "SE Software version\n\nYou can [`read`](crate::Reg::read) this register and get [`rootseswversion::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rootseswversion::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RootseswversionSpec;
impl crate::RegisterSpec for RootseswversionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rootseswversion::R`](R) reader structure"]
impl crate::Readable for RootseswversionSpec {}
#[doc = "`write(|w| ..)` method takes [`rootseswversion::W`](W) writer structure"]
impl crate::Writable for RootseswversionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROOTSESWVERSION to value 0"]
impl crate::Resettable for RootseswversionSpec {}
