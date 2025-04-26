#[doc = "Register `CH0F` writer"]
pub type W = crate::W<Ch0fSpec>;
#[doc = "Field `DATA` writer - Channel 0 Data"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl W {
    #[doc = "Bits 0:11 - Channel 0 Data"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<Ch0fSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0f::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0fSpec;
impl crate::RegisterSpec for Ch0fSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ch0f::W`](W) writer structure"]
impl crate::Writable for Ch0fSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH0F to value 0"]
impl crate::Resettable for Ch0fSpec {}
