#[doc = "Register `CHEN` writer"]
pub type W = crate::W<ChenSpec>;
#[doc = "Field `CHEN` writer - Channel Enables"]
pub type ChenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Channel Enables"]
    #[inline(always)]
    pub fn chen(&mut self) -> ChenW<ChenSpec> {
        ChenW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChenSpec;
impl crate::RegisterSpec for ChenSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chen::W`](W) writer structure"]
impl crate::Writable for ChenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHEN to value 0"]
impl crate::Resettable for ChenSpec {}
