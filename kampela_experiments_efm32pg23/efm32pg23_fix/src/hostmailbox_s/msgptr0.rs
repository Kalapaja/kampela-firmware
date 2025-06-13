#[doc = "Register `MSGPTR0` reader"]
pub type R = crate::R<Msgptr0Spec>;
#[doc = "Register `MSGPTR0` writer"]
pub type W = crate::W<Msgptr0Spec>;
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
    pub fn ptr(&mut self) -> PtrW<Msgptr0Spec> {
        PtrW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`msgptr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`msgptr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Msgptr0Spec;
impl crate::RegisterSpec for Msgptr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msgptr0::R`](R) reader structure"]
impl crate::Readable for Msgptr0Spec {}
#[doc = "`write(|w| ..)` method takes [`msgptr0::W`](W) writer structure"]
impl crate::Writable for Msgptr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MSGPTR0 to value 0"]
impl crate::Resettable for Msgptr0Spec {}
