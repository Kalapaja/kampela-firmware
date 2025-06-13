#[doc = "Register `SEGD0` reader"]
pub type R = crate::R<Segd0Spec>;
#[doc = "Register `SEGD0` writer"]
pub type W = crate::W<Segd0Spec>;
#[doc = "Field `SEGD0` reader - COM0 Segment Data Low"]
pub type Segd0R = crate::FieldReader<u32>;
#[doc = "Field `SEGD0` writer - COM0 Segment Data Low"]
pub type Segd0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - COM0 Segment Data Low"]
    #[inline(always)]
    pub fn segd0(&self) -> Segd0R {
        Segd0R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - COM0 Segment Data Low"]
    #[inline(always)]
    pub fn segd0(&mut self) -> Segd0W<Segd0Spec> {
        Segd0W::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`segd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd0Spec;
impl crate::RegisterSpec for Segd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd0::R`](R) reader structure"]
impl crate::Readable for Segd0Spec {}
#[doc = "`write(|w| ..)` method takes [`segd0::W`](W) writer structure"]
impl crate::Writable for Segd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD0 to value 0"]
impl crate::Resettable for Segd0Spec {}
