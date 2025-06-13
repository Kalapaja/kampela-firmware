#[doc = "Register `COMP1` reader"]
pub type R = crate::R<Comp1Spec>;
#[doc = "Register `COMP1` writer"]
pub type W = crate::W<Comp1Spec>;
#[doc = "Field `COMP1` reader - Compare Value 1"]
pub type Comp1R = crate::FieldReader<u32>;
#[doc = "Field `COMP1` writer - Compare Value 1"]
pub type Comp1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Compare Value 1"]
    #[inline(always)]
    pub fn comp1(&self) -> Comp1R {
        Comp1R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Compare Value 1"]
    #[inline(always)]
    pub fn comp1(&mut self) -> Comp1W<Comp1Spec> {
        Comp1W::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`comp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`comp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Comp1Spec;
impl crate::RegisterSpec for Comp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp1::R`](R) reader structure"]
impl crate::Readable for Comp1Spec {}
#[doc = "`write(|w| ..)` method takes [`comp1::W`](W) writer structure"]
impl crate::Writable for Comp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMP1 to value 0"]
impl crate::Resettable for Comp1Spec {}
