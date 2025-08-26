#[doc = "Register `CONSUMER_WDOG0_SRC1` reader"]
pub type R = crate::R<ConsumerWdog0Src1Spec>;
#[doc = "Register `CONSUMER_WDOG0_SRC1` writer"]
pub type W = crate::W<ConsumerWdog0Src1Spec>;
#[doc = "Field `PRSSEL` reader - SRC1 async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - SRC1 async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - SRC1 async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SRC1 async channel select"]
    #[inline(always)]
    pub fn prssel(&mut self) -> PrsselW<ConsumerWdog0Src1Spec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "SRC1 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_wdog0_src1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_wdog0_src1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerWdog0Src1Spec;
impl crate::RegisterSpec for ConsumerWdog0Src1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_wdog0_src1::R`](R) reader structure"]
impl crate::Readable for ConsumerWdog0Src1Spec {}
#[doc = "`write(|w| ..)` method takes [`consumer_wdog0_src1::W`](W) writer structure"]
impl crate::Writable for ConsumerWdog0Src1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSUMER_WDOG0_SRC1 to value 0"]
impl crate::Resettable for ConsumerWdog0Src1Spec {}
