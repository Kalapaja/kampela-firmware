#[doc = "Register `SYNCSWSET` writer"]
pub type W = crate::W<SyncswsetSpec>;
#[doc = "Field `SYNCSWSET` writer - DMA SYNC Software Trigger Set"]
pub type SyncswsetW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - DMA SYNC Software Trigger Set"]
    #[inline(always)]
    pub fn syncswset(&mut self) -> SyncswsetW<SyncswsetSpec> {
        SyncswsetW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syncswset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncswsetSpec;
impl crate::RegisterSpec for SyncswsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`syncswset::W`](W) writer structure"]
impl crate::Writable for SyncswsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNCSWSET to value 0"]
impl crate::Resettable for SyncswsetSpec {}
