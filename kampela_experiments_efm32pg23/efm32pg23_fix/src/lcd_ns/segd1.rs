#[doc = "Register `SEGD1` reader"]
pub type R = crate::R<Segd1Spec>;
#[doc = "Register `SEGD1` writer"]
pub type W = crate::W<Segd1Spec>;
#[doc = "Field `SEGD1` reader - COM1 Segment Data Low"]
pub type Segd1R = crate::FieldReader<u32>;
#[doc = "Field `SEGD1` writer - COM1 Segment Data Low"]
pub type Segd1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - COM1 Segment Data Low"]
    #[inline(always)]
    pub fn segd1(&self) -> Segd1R {
        Segd1R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - COM1 Segment Data Low"]
    #[inline(always)]
    pub fn segd1(&mut self) -> Segd1W<Segd1Spec> {
        Segd1W::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`segd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd1Spec;
impl crate::RegisterSpec for Segd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd1::R`](R) reader structure"]
impl crate::Readable for Segd1Spec {}
#[doc = "`write(|w| ..)` method takes [`segd1::W`](W) writer structure"]
impl crate::Writable for Segd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD1 to value 0"]
impl crate::Resettable for Segd1Spec {}
