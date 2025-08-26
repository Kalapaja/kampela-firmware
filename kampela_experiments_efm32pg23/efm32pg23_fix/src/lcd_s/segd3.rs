#[doc = "Register `SEGD3` reader"]
pub type R = crate::R<Segd3Spec>;
#[doc = "Register `SEGD3` writer"]
pub type W = crate::W<Segd3Spec>;
#[doc = "Field `SEGD3` reader - COM3 Segment Data Low"]
pub type Segd3R = crate::FieldReader<u32>;
#[doc = "Field `SEGD3` writer - COM3 Segment Data Low"]
pub type Segd3W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - COM3 Segment Data Low"]
    #[inline(always)]
    pub fn segd3(&self) -> Segd3R {
        Segd3R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - COM3 Segment Data Low"]
    #[inline(always)]
    pub fn segd3(&mut self) -> Segd3W<Segd3Spec> {
        Segd3W::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`segd3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`segd3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Segd3Spec;
impl crate::RegisterSpec for Segd3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`segd3::R`](R) reader structure"]
impl crate::Readable for Segd3Spec {}
#[doc = "`write(|w| ..)` method takes [`segd3::W`](W) writer structure"]
impl crate::Writable for Segd3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEGD3 to value 0"]
impl crate::Resettable for Segd3Spec {}
