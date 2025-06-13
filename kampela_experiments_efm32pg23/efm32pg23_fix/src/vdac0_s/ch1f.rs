#[doc = "Register `CH1F` writer"]
pub type W = crate::W<Ch1fSpec>;
#[doc = "Field `DATA` writer - Channel 1 Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl W {
    #[doc = "Bits 0:11 - Channel 1 Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<Ch1fSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1f::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1fSpec;
impl crate::RegisterSpec for Ch1fSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ch1f::W`](W) writer structure"]
impl crate::Writable for Ch1fSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH1F to value 0"]
impl crate::Resettable for Ch1fSpec {}
