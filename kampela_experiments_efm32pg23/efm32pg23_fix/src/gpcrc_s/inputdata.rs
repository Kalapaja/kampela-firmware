#[doc = "Register `INPUTDATA` writer"]
pub type W = crate::W<InputdataSpec>;
#[doc = "Field `INPUTDATA` writer - Input Data for 32-bit"]
pub type InputdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Input Data for 32-bit"]
    #[inline(always)]
    pub fn inputdata(&mut self) -> InputdataW<InputdataSpec> {
        InputdataW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputdata::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputdataSpec;
impl crate::RegisterSpec for InputdataSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`inputdata::W`](W) writer structure"]
impl crate::Writable for InputdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUTDATA to value 0"]
impl crate::Resettable for InputdataSpec {}
