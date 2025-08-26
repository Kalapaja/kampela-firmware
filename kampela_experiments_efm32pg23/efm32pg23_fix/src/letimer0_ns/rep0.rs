#[doc = "Register `REP0` reader"]
pub type R = crate::R<Rep0Spec>;
#[doc = "Register `REP0` writer"]
pub type W = crate::W<Rep0Spec>;
#[doc = "Field `REP0` reader - Repeat Counter 0"]
pub type Rep0R = crate::FieldReader;
#[doc = "Field `REP0` writer - Repeat Counter 0"]
pub type Rep0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Repeat Counter 0"]
    #[inline(always)]
    pub fn rep0(&self) -> Rep0R {
        Rep0R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Repeat Counter 0"]
    #[inline(always)]
    pub fn rep0(&mut self) -> Rep0W<Rep0Spec> {
        Rep0W::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rep0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rep0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rep0Spec;
impl crate::RegisterSpec for Rep0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rep0::R`](R) reader structure"]
impl crate::Readable for Rep0Spec {}
#[doc = "`write(|w| ..)` method takes [`rep0::W`](W) writer structure"]
impl crate::Writable for Rep0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REP0 to value 0"]
impl crate::Resettable for Rep0Spec {}
