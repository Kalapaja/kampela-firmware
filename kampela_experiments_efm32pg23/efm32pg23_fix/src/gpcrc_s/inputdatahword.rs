#[doc = "Register `INPUTDATAHWORD` writer"]
pub type W = crate::W<InputdatahwordSpec>;
#[doc = "Field `INPUTDATAHWORD` writer - Input Data for 16-bit"]
pub type InputdatahwordW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - Input Data for 16-bit"]
    #[inline(always)]
    pub fn inputdatahword(&mut self) -> InputdatahwordW<InputdatahwordSpec> {
        InputdatahwordW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inputdatahword::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InputdatahwordSpec;
impl crate::RegisterSpec for InputdatahwordSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`inputdatahword::W`](W) writer structure"]
impl crate::Writable for InputdatahwordSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INPUTDATAHWORD to value 0"]
impl crate::Resettable for InputdatahwordSpec {}
