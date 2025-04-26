#[doc = "Register `MSGPTR3` reader"]
pub type R = crate::R<Msgptr3Spec>;
#[doc = "Register `MSGPTR3` writer"]
pub type W = crate::W<Msgptr3Spec>;
#[doc = "Field `PTR` reader - Pointer"]
pub type PtrR = crate::FieldReader<u32>;
#[doc = "Field `PTR` writer - Pointer"]
pub type PtrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Pointer"]
    #[inline(always)]
    pub fn ptr(&self) -> PtrR {
        PtrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Pointer"]
    #[inline(always)]
    pub fn ptr(&mut self) -> PtrW<Msgptr3Spec> {
        PtrW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`msgptr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msgptr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Msgptr3Spec;
impl crate::RegisterSpec for Msgptr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msgptr3::R`](R) reader structure"]
impl crate::Readable for Msgptr3Spec {}
#[doc = "`write(|w| ..)` method takes [`msgptr3::W`](W) writer structure"]
impl crate::Writable for Msgptr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSGPTR3 to value 0"]
impl crate::Resettable for Msgptr3Spec {}
