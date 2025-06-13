#[doc = "Register `RET27_REG` reader"]
pub type R = crate::R<Ret27RegSpec>;
#[doc = "Register `RET27_REG` writer"]
pub type W = crate::W<Ret27RegSpec>;
#[doc = "Field `RETREG` reader - Latch based Retention register"]
pub type RetregR = crate::FieldReader<u32>;
#[doc = "Field `RETREG` writer - Latch based Retention register"]
pub type RetregW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Latch based Retention register"]
    #[inline(always)]
    pub fn retreg(&self) -> RetregR {
        RetregR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Latch based Retention register"]
    #[inline(always)]
    pub fn retreg(&mut self) -> RetregW<Ret27RegSpec> {
        RetregW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ret27_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret27_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ret27RegSpec;
impl crate::RegisterSpec for Ret27RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ret27_reg::R`](R) reader structure"]
impl crate::Readable for Ret27RegSpec {}
#[doc = "`write(|w| ..)` method takes [`ret27_reg::W`](W) writer structure"]
impl crate::Writable for Ret27RegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RET27_REG to value 0"]
impl crate::Resettable for Ret27RegSpec {}
