#[doc = "Register `CONSUMER_HFXO0_TIMEOUT` reader"]
pub type R = crate::R<ConsumerHfxo0TimeoutSpec>;
#[doc = "Register `CONSUMER_HFXO0_TIMEOUT` writer"]
pub type W = crate::W<ConsumerHfxo0TimeoutSpec>;
#[doc = "Field `PRSSEL` reader - TIMEOUT async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - TIMEOUT async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TIMEOUT async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TIMEOUT async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<ConsumerHfxo0TimeoutSpec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "TIMEOUT Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_hfxo0_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_hfxo0_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerHfxo0TimeoutSpec;
impl crate::RegisterSpec for ConsumerHfxo0TimeoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_hfxo0_timeout::R`](R) reader structure"]
impl crate::Readable for ConsumerHfxo0TimeoutSpec {}
#[doc = "`write(|w| ..)` method takes [`consumer_hfxo0_timeout::W`](W) writer structure"]
impl crate::Writable for ConsumerHfxo0TimeoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_HFXO0_TIMEOUT to value 0"]
impl crate::Resettable for ConsumerHfxo0TimeoutSpec {}
