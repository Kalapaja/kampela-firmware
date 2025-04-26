#[doc = "Register `CONSUMER_CORE_M33RXEV` reader"]
pub type R = crate::R<ConsumerCoreM33rxevSpec>;
#[doc = "Register `CONSUMER_CORE_M33RXEV` writer"]
pub type W = crate::W<ConsumerCoreM33rxevSpec>;
#[doc = "Field `PRSSEL` reader - M33 async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - M33 async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - M33 async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - M33 async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<ConsumerCoreM33rxevSpec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "M33 Consumer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_core_m33rxev::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_core_m33rxev::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerCoreM33rxevSpec;
impl crate::RegisterSpec for ConsumerCoreM33rxevSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_core_m33rxev::R`](R) reader structure"]
impl crate::Readable for ConsumerCoreM33rxevSpec {}
#[doc = "`write(|w| ..)` method takes [`consumer_core_m33rxev::W`](W) writer structure"]
impl crate::Writable for ConsumerCoreM33rxevSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_CORE_M33RXEV to value 0"]
impl crate::Resettable for ConsumerCoreM33rxevSpec {}
