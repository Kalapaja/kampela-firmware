#[doc = "Register `CONSUMER_SYSRTC0_IN0` reader"]
pub type R = crate::R<ConsumerSysrtc0In0Spec>;
#[doc = "Register `CONSUMER_SYSRTC0_IN0` writer"]
pub type W = crate::W<ConsumerSysrtc0In0Spec>;
#[doc = "Field `PRSSEL` reader - IN0 async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - IN0 async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - IN0 async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - IN0 async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<ConsumerSysrtc0In0Spec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "IN0 consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_sysrtc0_in0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_sysrtc0_in0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerSysrtc0In0Spec;
impl crate::RegisterSpec for ConsumerSysrtc0In0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_sysrtc0_in0::R`](R) reader structure"]
impl crate::Readable for ConsumerSysrtc0In0Spec {}
#[doc = "`write(|w| ..)` method takes [`consumer_sysrtc0_in0::W`](W) writer structure"]
impl crate::Writable for ConsumerSysrtc0In0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_SYSRTC0_IN0 to value 0"]
impl crate::Resettable for ConsumerSysrtc0In0Spec {}
