#[doc = "Register `CONSUMER_LETIMER0_START` reader"]
pub type R = crate::R<ConsumerLetimer0StartSpec>;
#[doc = "Register `CONSUMER_LETIMER0_START` writer"]
pub type W = crate::W<ConsumerLetimer0StartSpec>;
#[doc = "Field `PRSSEL` reader - START async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - START async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - START async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - START async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<ConsumerLetimer0StartSpec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "START Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_letimer0_start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_letimer0_start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerLetimer0StartSpec;
impl crate::RegisterSpec for ConsumerLetimer0StartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_letimer0_start::R`](R) reader structure"]
impl crate::Readable for ConsumerLetimer0StartSpec {}
#[doc = "`write(|w| ..)` method takes [`consumer_letimer0_start::W`](W) writer structure"]
impl crate::Writable for ConsumerLetimer0StartSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_LETIMER0_START to value 0"]
impl crate::Resettable for ConsumerLetimer0StartSpec {}
