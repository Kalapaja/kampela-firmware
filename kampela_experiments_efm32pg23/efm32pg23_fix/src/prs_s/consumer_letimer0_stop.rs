#[doc = "Register `CONSUMER_LETIMER0_STOP` reader"]
pub type R = crate::R<ConsumerLetimer0StopSpec>;
#[doc = "Register `CONSUMER_LETIMER0_STOP` writer"]
pub type W = crate::W<ConsumerLetimer0StopSpec>;
#[doc = "Field `PRSSEL` reader - STOP async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - STOP async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - STOP async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - STOP async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<ConsumerLetimer0StopSpec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "STOP Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_letimer0_stop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_letimer0_stop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerLetimer0StopSpec;
impl crate::RegisterSpec for ConsumerLetimer0StopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_letimer0_stop::R`](R) reader structure"]
impl crate::Readable for ConsumerLetimer0StopSpec {}
#[doc = "`write(|w| ..)` method takes [`consumer_letimer0_stop::W`](W) writer structure"]
impl crate::Writable for ConsumerLetimer0StopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_LETIMER0_STOP to value 0"]
impl crate::Resettable for ConsumerLetimer0StopSpec {}
