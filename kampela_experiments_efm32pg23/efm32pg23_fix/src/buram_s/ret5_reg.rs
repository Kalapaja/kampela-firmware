#[doc = "Register `RET5_REG` reader"]
pub type R = crate::R<Ret5RegSpec>;
#[doc = "Register `RET5_REG` writer"]
pub type W = crate::W<Ret5RegSpec>;
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
    pub fn retreg(&mut self) -> RetregW<Ret5RegSpec> {
        RetregW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ret5_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ret5_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ret5RegSpec;
impl crate::RegisterSpec for Ret5RegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ret5_reg::R`](R) reader structure"]
impl crate::Readable for Ret5RegSpec {}
#[doc = "`write(|w| ..)` method takes [`ret5_reg::W`](W) writer structure"]
impl crate::Writable for Ret5RegSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RET5_REG to value 0"]
impl crate::Resettable for Ret5RegSpec {}
