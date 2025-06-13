#[doc = "Register `ROOTNSDATA1` reader"]
pub type R = crate::R<Rootnsdata1Spec>;
#[doc = "Register `ROOTNSDATA1` writer"]
pub type W = crate::W<Rootnsdata1Spec>;
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
    pub fn data(&mut self) -> DataW<Rootnsdata1Spec> {
        DataW::new(self, 0)
    }
}
#[doc = "Generic data space for user to pass to root, e.g., address of struct in mem\n\nYou can [`read`](crate::Reg::read) this register and get [`rootnsdata1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rootnsdata1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rootnsdata1Spec;
impl crate::RegisterSpec for Rootnsdata1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rootnsdata1::R`](R) reader structure"]
impl crate::Readable for Rootnsdata1Spec {}
#[doc = "`write(|w| ..)` method takes [`rootnsdata1::W`](W) writer structure"]
impl crate::Writable for Rootnsdata1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ROOTNSDATA1 to value 0"]
impl crate::Resettable for Rootnsdata1Spec {}
