#[doc = "Register `SEGD2` reader"]
pub type R = crate::R<Segd2Spec>;
#[doc = "Register `SEGD2` writer"]
pub type W = crate::W<Segd2Spec>;
#[doc = "Field `SEGD2` reader - COM2 Segment Data Low"]
pub type Segd2R = crate::FieldReader<u32>;
#[doc = "Field `SEGD2` writer - COM2 Segment Data Low"]
pub type Segd2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - COM2 Segment Data Low"]
    #[inline(always)]
    pub fn segd2(&self) -> Segd2R {
        Segd2R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - COM2 Segment Data Low"]
    #[inline(always)]
    pub fn segd2(&mut self) -> Segd2W<Segd2Spec> {
        Segd2W::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`segd2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd2Spec;
impl crate::RegisterSpec for Segd2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd2::R`](R) reader structure"]
impl crate::Readable for Segd2Spec {}
#[doc = "`write(|w| ..)` method takes [`segd2::W`](W) writer structure"]
impl crate::Writable for Segd2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD2 to value 0"]
impl crate::Resettable for Segd2Spec {}
