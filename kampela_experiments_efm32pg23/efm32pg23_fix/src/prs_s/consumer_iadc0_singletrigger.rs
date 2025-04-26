#[doc = "Register `CONSUMER_IADC0_SINGLETRIGGER` reader"]
pub type R = crate::R<ConsumerIadc0SingletriggerSpec>;
#[doc = "Register `CONSUMER_IADC0_SINGLETRIGGER` writer"]
pub type W = crate::W<ConsumerIadc0SingletriggerSpec>;
#[doc = "Field `PRSSEL` reader - SINGLE async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - SINGLE async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPRSSEL` reader - SINGLE sync channel select"]
pub type SprsselR = crate::FieldReader;
#[doc = "Field `SPRSSEL` writer - SINGLE sync channel select"]
pub type SprsselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - SINGLE async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - SINGLE sync channel select"]
    #[inline(always)]
    pub fn sprssel(&self) -> SprsselR {
        SprsselR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SINGLE async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<ConsumerIadc0SingletriggerSpec> {
        PrsselW::new(self, 0)
    }
    #[doc = "Bits 8:9 - SINGLE sync channel select"]
    #[inline(always)]
    pub fn sprssel(&mut self) -> SprsselW<ConsumerIadc0SingletriggerSpec> {
        SprsselW::new(self, 8)
    }
}
#[doc = "SINGLE Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_iadc0_singletrigger::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_iadc0_singletrigger::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerIadc0SingletriggerSpec;
impl crate::RegisterSpec for ConsumerIadc0SingletriggerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_iadc0_singletrigger::R`](R) reader structure"]
impl crate::Readable for ConsumerIadc0SingletriggerSpec {}
#[doc = "`write(|w| ..)` method takes [`consumer_iadc0_singletrigger::W`](W) writer structure"]
impl crate::Writable for ConsumerIadc0SingletriggerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_IADC0_SINGLETRIGGER to value 0"]
impl crate::Resettable for ConsumerIadc0SingletriggerSpec {}
