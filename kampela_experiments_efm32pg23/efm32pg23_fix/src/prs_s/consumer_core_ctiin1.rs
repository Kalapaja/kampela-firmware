#[doc = "Register `CONSUMER_CORE_CTIIN1` reader"]
pub type R = crate::R<ConsumerCoreCtiin1Spec>;
#[doc = "Register `CONSUMER_CORE_CTIIN1` writer"]
pub type W = crate::W<ConsumerCoreCtiin1Spec>;
#[doc = "Field `PRSSEL` reader - CTI async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - CTI async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - CTI async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CTI async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<ConsumerCoreCtiin1Spec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "CTI Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_core_ctiin1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_core_ctiin1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerCoreCtiin1Spec;
impl crate::RegisterSpec for ConsumerCoreCtiin1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_core_ctiin1::R`](R) reader structure"]
impl crate::Readable for ConsumerCoreCtiin1Spec {}
#[doc = "`write(|w| ..)` method takes [`consumer_core_ctiin1::W`](W) writer structure"]
impl crate::Writable for ConsumerCoreCtiin1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_CORE_CTIIN1 to value 0"]
impl crate::Resettable for ConsumerCoreCtiin1Spec {}
