#[doc = "Register `REQDIS` reader"]
pub type R = crate::R<ReqdisSpec>;
#[doc = "Register `REQDIS` writer"]
pub type W = crate::W<ReqdisSpec>;
#[doc = "Field `REQDIS` reader - DMA Request Disables"]
pub type ReqdisR = crate::FieldReader;
#[doc = "Field `REQDIS` writer - DMA Request Disables"]
pub type ReqdisW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DMA Request Disables"]
    #[inline(always)]
    pub fn reqdis(&self) -> ReqdisR {
        ReqdisR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DMA Request Disables"]
    #[inline(always)]
    pub fn reqdis(&mut self) -> ReqdisW<ReqdisSpec> {
        ReqdisW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`reqdis::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reqdis::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReqdisSpec;
impl crate::RegisterSpec for ReqdisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reqdis::R`](R) reader structure"]
impl crate::Readable for ReqdisSpec {}
#[doc = "`write(|w| ..)` method takes [`reqdis::W`](W) writer structure"]
impl crate::Writable for ReqdisSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REQDIS to value 0"]
impl crate::Resettable for ReqdisSpec {}
