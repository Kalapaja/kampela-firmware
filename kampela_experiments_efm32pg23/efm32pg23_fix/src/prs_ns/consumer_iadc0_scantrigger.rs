#[doc = "Register `CONSUMER_IADC0_SCANTRIGGER` reader"]
pub type R = crate::R<ConsumerIadc0ScantriggerSpec>;
#[doc = "Register `CONSUMER_IADC0_SCANTRIGGER` writer"]
pub type W = crate::W<ConsumerIadc0ScantriggerSpec>;
#[doc = "Field `PRSSEL` reader - SCAN async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - SCAN async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPRSSEL` reader - SCAN sync channel select"]
pub type SprsselR = crate::FieldReader;
#[doc = "Field `SPRSSEL` writer - SCAN sync channel select"]
pub type SprsselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - SCAN async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - SCAN sync channel select"]
    #[inline(always)]
    pub fn sprssel(&self) -> SprsselR {
        SprsselR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SCAN async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<ConsumerIadc0ScantriggerSpec> {
        PrsselW::new(self, 0)
    }
    #[doc = "Bits 8:9 - SCAN sync channel select"]
    #[inline(always)]
    pub fn sprssel(&mut self) -> SprsselW<ConsumerIadc0ScantriggerSpec> {
        SprsselW::new(self, 8)
    }
}
#[doc = "SCAN consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_iadc0_scantrigger::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_iadc0_scantrigger::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerIadc0ScantriggerSpec;
impl crate::RegisterSpec for ConsumerIadc0ScantriggerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_iadc0_scantrigger::R`](R) reader structure"]
impl crate::Readable for ConsumerIadc0ScantriggerSpec {}
#[doc = "`write(|w| ..)` method takes [`consumer_iadc0_scantrigger::W`](W) writer structure"]
impl crate::Writable for ConsumerIadc0ScantriggerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_IADC0_SCANTRIGGER to value 0"]
impl crate::Resettable for ConsumerIadc0ScantriggerSpec {}
