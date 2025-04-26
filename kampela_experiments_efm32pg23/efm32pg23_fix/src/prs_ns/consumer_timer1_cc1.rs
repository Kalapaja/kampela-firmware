#[doc = "Register `CONSUMER_TIMER1_CC1` reader"]
pub type R = crate::R<ConsumerTimer1Cc1Spec>;
#[doc = "Register `CONSUMER_TIMER1_CC1` writer"]
pub type W = crate::W<ConsumerTimer1Cc1Spec>;
#[doc = "Field `PRSSEL` reader - CC1 async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - CC1 async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SPRSSEL` reader - CC1 sync channel select"]
pub type SprsselR = crate::FieldReader;
#[doc = "Field `SPRSSEL` writer - CC1 sync channel select"]
pub type SprsselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - CC1 async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - CC1 sync channel select"]
    #[inline(always)]
    pub fn sprssel(&self) -> SprsselR {
        SprsselR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CC1 async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<ConsumerTimer1Cc1Spec> {
        PrsselW::new(self, 0)
    }
    #[doc = "Bits 8:9 - CC1 sync channel select"]
    #[inline(always)]
    pub fn sprssel(&mut self) -> SprsselW<ConsumerTimer1Cc1Spec> {
        SprsselW::new(self, 8)
    }
}
#[doc = "CC1 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer1_cc1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer1_cc1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerTimer1Cc1Spec;
impl crate::RegisterSpec for ConsumerTimer1Cc1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_timer1_cc1::R`](R) reader structure"]
impl crate::Readable for ConsumerTimer1Cc1Spec {}
#[doc = "`write(|w| ..)` method takes [`consumer_timer1_cc1::W`](W) writer structure"]
impl crate::Writable for ConsumerTimer1Cc1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_TIMER1_CC1 to value 0"]
impl crate::Resettable for ConsumerTimer1Cc1Spec {}
