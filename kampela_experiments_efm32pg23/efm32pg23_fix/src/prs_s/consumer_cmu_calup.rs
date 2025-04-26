#[doc = "Register `CONSUMER_CMU_CALUP` reader"]
pub type R = crate::R<ConsumerCmuCalupSpec>;
#[doc = "Register `CONSUMER_CMU_CALUP` writer"]
pub type W = crate::W<ConsumerCmuCalupSpec>;
#[doc = "Field `PRSSEL` reader - CALUP async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - CALUP async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - CALUP async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CALUP async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<ConsumerCmuCalupSpec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "CALUP Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_cmu_calup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_cmu_calup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerCmuCalupSpec;
impl crate::RegisterSpec for ConsumerCmuCalupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_cmu_calup::R`](R) reader structure"]
impl crate::Readable for ConsumerCmuCalupSpec {}
#[doc = "`write(|w| ..)` method takes [`consumer_cmu_calup::W`](W) writer structure"]
impl crate::Writable for ConsumerCmuCalupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_CMU_CALUP to value 0"]
impl crate::Resettable for ConsumerCmuCalupSpec {}
