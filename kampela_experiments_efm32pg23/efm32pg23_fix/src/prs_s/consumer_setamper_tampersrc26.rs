#[doc = "Register `CONSUMER_SETAMPER_TAMPERSRC26` reader"]
pub type R = crate::R<ConsumerSetamperTampersrc26Spec>;
#[doc = "Register `CONSUMER_SETAMPER_TAMPERSRC26` writer"]
pub type W = crate::W<ConsumerSetamperTampersrc26Spec>;
#[doc = "Field `PRSSEL` reader - TAMPERSRC26 async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - TAMPERSRC26 async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TAMPERSRC26 async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TAMPERSRC26 async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<ConsumerSetamperTampersrc26Spec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "TAMPERSRC26 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_setamper_tampersrc26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_setamper_tampersrc26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerSetamperTampersrc26Spec;
impl crate::RegisterSpec for ConsumerSetamperTampersrc26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_setamper_tampersrc26::R`](R) reader structure"]
impl crate::Readable for ConsumerSetamperTampersrc26Spec {}
#[doc = "`write(|w| ..)` method takes [`consumer_setamper_tampersrc26::W`](W) writer structure"]
impl crate::Writable for ConsumerSetamperTampersrc26Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_SETAMPER_TAMPERSRC26 to value 0"]
impl crate::Resettable for ConsumerSetamperTampersrc26Spec {}
