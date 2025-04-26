#[doc = "Register `REP1` reader"]
pub type R = crate::R<Rep1Spec>;
#[doc = "Register `REP1` writer"]
pub type W = crate::W<Rep1Spec>;
#[doc = "Field `REP1` reader - Repeat Counter 1"]
pub type Rep1R = crate::FieldReader;
#[doc = "Field `REP1` writer - Repeat Counter 1"]
pub type Rep1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Repeat Counter 1"]
    #[inline(always)]
    pub fn rep1(&self) -> Rep1R {
        Rep1R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repeat Counter 1"]
    #[inline(always)]
    pub fn rep1(&mut self) -> Rep1W<Rep1Spec> {
        Rep1W::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rep1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rep1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rep1Spec;
impl crate::RegisterSpec for Rep1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rep1::R`](R) reader structure"]
impl crate::Readable for Rep1Spec {}
#[doc = "`write(|w| ..)` method takes [`rep1::W`](W) writer structure"]
impl crate::Writable for Rep1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REP1 to value 0"]
impl crate::Resettable for Rep1Spec {}
