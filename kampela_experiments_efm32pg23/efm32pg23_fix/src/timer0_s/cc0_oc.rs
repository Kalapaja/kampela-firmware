#[doc = "Register `CC0_OC` reader"]
pub type R = crate::R<Cc0OcSpec>;
#[doc = "Register `CC0_OC` writer"]
pub type W = crate::W<Cc0OcSpec>;
#[doc = "Field `OC` reader - Output Compare Value"]
pub type OcR = crate::FieldReader<u32>;
#[doc = "Field `OC` writer - Output Compare Value"]
pub type OcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Output Compare Value"]
    #[inline(always)]
    pub fn oc(&self) -> OcR {
        OcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Output Compare Value"]
    #[inline(always)]
    pub fn oc(&mut self) -> OcW<Cc0OcSpec> {
        OcW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_oc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc0_oc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc0OcSpec;
impl crate::RegisterSpec for Cc0OcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc0_oc::R`](R) reader structure"]
impl crate::Readable for Cc0OcSpec {}
#[doc = "`write(|w| ..)` method takes [`cc0_oc::W`](W) writer structure"]
impl crate::Writable for Cc0OcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC0_OC to value 0"]
impl crate::Resettable for Cc0OcSpec {}
