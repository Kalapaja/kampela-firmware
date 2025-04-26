#[doc = "Register `ROOTNSDATA0` reader"]
pub type R = crate::R<Rootnsdata0Spec>;
#[doc = "Register `ROOTNSDATA0` writer"]
pub type W = crate::W<Rootnsdata0Spec>;
#[doc = "Field `DATA` reader - Data"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<Rootnsdata0Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Generic data space for user to pass to root, e.g., address of struct in mem\n\nYou can [`read`](crate::Reg::read) this register and get [`rootnsdata0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rootnsdata0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rootnsdata0Spec;
impl crate::RegisterSpec for Rootnsdata0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rootnsdata0::R`](R) reader structure"]
impl crate::Readable for Rootnsdata0Spec {}
#[doc = "`write(|w| ..)` method takes [`rootnsdata0::W`](W) writer structure"]
impl crate::Writable for Rootnsdata0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROOTNSDATA0 to value 0"]
impl crate::Resettable for Rootnsdata0Spec {}
