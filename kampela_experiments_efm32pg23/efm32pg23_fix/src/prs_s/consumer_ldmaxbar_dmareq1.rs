#[doc = "Register `CONSUMER_LDMAXBAR_DMAREQ1` reader"]
pub type R = crate::R<ConsumerLdmaxbarDmareq1Spec>;
#[doc = "Register `CONSUMER_LDMAXBAR_DMAREQ1` writer"]
pub type W = crate::W<ConsumerLdmaxbarDmareq1Spec>;
#[doc = "Field `PRSSEL` reader - DMAREQ1 async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - DMAREQ1 async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DMAREQ1 async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DMAREQ1 async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<ConsumerLdmaxbarDmareq1Spec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "DMAREQ1 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_ldmaxbar_dmareq1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_ldmaxbar_dmareq1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerLdmaxbarDmareq1Spec;
impl crate::RegisterSpec for ConsumerLdmaxbarDmareq1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_ldmaxbar_dmareq1::R`](R) reader structure"]
impl crate::Readable for ConsumerLdmaxbarDmareq1Spec {}
#[doc = "`write(|w| ..)` method takes [`consumer_ldmaxbar_dmareq1::W`](W) writer structure"]
impl crate::Writable for ConsumerLdmaxbarDmareq1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_LDMAXBAR_DMAREQ1 to value 0"]
impl crate::Resettable for ConsumerLdmaxbarDmareq1Spec {}
