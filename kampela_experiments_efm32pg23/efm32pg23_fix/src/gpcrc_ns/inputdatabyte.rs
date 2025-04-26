#[doc = "Register `INPUTDATABYTE` writer"]
pub type W = crate::W<InputdatabyteSpec>;
#[doc = "Field `INPUTDATABYTE` writer - Input Data for 8-bit"]
pub type InputdatabyteW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Input Data for 8-bit"]
    #[inline(always)]
    pub fn inputdatabyte(&mut self) -> InputdatabyteW<InputdatabyteSpec> {
        InputdatabyteW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputdatabyte::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputdatabyteSpec;
impl crate::RegisterSpec for InputdatabyteSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`inputdatabyte::W`](W) writer structure"]
impl crate::Writable for InputdatabyteSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUTDATABYTE to value 0"]
impl crate::Resettable for InputdatabyteSpec {}
