#[doc = "Register `CONSUMER_VDAC0_ASYNCTRIGCH0` reader"]
pub type R = crate::R<ConsumerVdac0Asynctrigch0Spec>;
#[doc = "Register `CONSUMER_VDAC0_ASYNCTRIGCH0` writer"]
pub type W = crate::W<ConsumerVdac0Asynctrigch0Spec>;
#[doc = "Field `PRSSEL` reader - ASYNCTRIG async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - ASYNCTRIG async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ASYNCTRIG async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ASYNCTRIG async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<ConsumerVdac0Asynctrigch0Spec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "ASYNCTRIG consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_vdac0_asynctrigch0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_vdac0_asynctrigch0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerVdac0Asynctrigch0Spec;
impl crate::RegisterSpec for ConsumerVdac0Asynctrigch0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_vdac0_asynctrigch0::R`](R) reader structure"]
impl crate::Readable for ConsumerVdac0Asynctrigch0Spec {}
#[doc = "`write(|w| ..)` method takes [`consumer_vdac0_asynctrigch0::W`](W) writer structure"]
impl crate::Writable for ConsumerVdac0Asynctrigch0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_VDAC0_ASYNCTRIGCH0 to value 0"]
impl crate::Resettable for ConsumerVdac0Asynctrigch0Spec {}
