#[doc = "Register `SYNCSWCLR` writer"]
pub type W = crate::W<SyncswclrSpec>;
#[doc = "Field `SYNCSWCLR` writer - DMA SYNC Software Trigger Clear"]
pub type SyncswclrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - DMA SYNC Software Trigger Clear"]
    #[inline(always)]
    pub fn syncswclr(&mut self) -> SyncswclrW<SyncswclrSpec> {
        SyncswclrW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syncswclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncswclrSpec;
impl crate::RegisterSpec for SyncswclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`syncswclr::W`](W) writer structure"]
impl crate::Writable for SyncswclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNCSWCLR to value 0"]
impl crate::Resettable for SyncswclrSpec {}
