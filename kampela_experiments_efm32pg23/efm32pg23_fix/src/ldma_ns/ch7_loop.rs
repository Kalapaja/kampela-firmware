#[doc = "Register `CH7_LOOP` reader"]
pub type R = crate::R<Ch7LoopSpec>;
#[doc = "Register `CH7_LOOP` writer"]
pub type W = crate::W<Ch7LoopSpec>;
#[doc = "Field `LOOPCNT` reader - Linked Structure Sequence Loop Counter"]
pub type LoopcntR = crate::FieldReader;
#[doc = "Field `LOOPCNT` writer - Linked Structure Sequence Loop Counter"]
pub type LoopcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Linked Structure Sequence Loop Counter"]
    #[inline(always)]
    pub fn loopcnt(&self) -> LoopcntR {
        LoopcntR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Linked Structure Sequence Loop Counter"]
    #[inline(always)]
    pub fn loopcnt(&mut self) -> LoopcntW<Ch7LoopSpec> {
        LoopcntW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7_loop::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7_loop::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch7LoopSpec;
impl crate::RegisterSpec for Ch7LoopSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch7_loop::R`](R) reader structure"]
impl crate::Readable for Ch7LoopSpec {}
#[doc = "`write(|w| ..)` method takes [`ch7_loop::W`](W) writer structure"]
impl crate::Writable for Ch7LoopSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH7_LOOP to value 0"]
impl crate::Resettable for Ch7LoopSpec {}
