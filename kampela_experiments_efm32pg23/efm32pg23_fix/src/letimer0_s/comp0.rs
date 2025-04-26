#[doc = "Register `COMP0` reader"]
pub type R = crate::R<Comp0Spec>;
#[doc = "Register `COMP0` writer"]
pub type W = crate::W<Comp0Spec>;
#[doc = "Field `COMP0` reader - Compare Value 0"]
pub type Comp0R = crate::FieldReader<u32>;
#[doc = "Field `COMP0` writer - Compare Value 0"]
pub type Comp0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Compare Value 0"]
    #[inline(always)]
    pub fn comp0(&self) -> Comp0R {
        Comp0R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Compare Value 0"]
    #[inline(always)]
    pub fn comp0(&mut self) -> Comp0W<Comp0Spec> {
        Comp0W::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`comp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp0Spec;
impl crate::RegisterSpec for Comp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp0::R`](R) reader structure"]
impl crate::Readable for Comp0Spec {}
#[doc = "`write(|w| ..)` method takes [`comp0::W`](W) writer structure"]
impl crate::Writable for Comp0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP0 to value 0"]
impl crate::Resettable for Comp0Spec {}
