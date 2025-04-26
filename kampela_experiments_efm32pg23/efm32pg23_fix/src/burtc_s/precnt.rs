#[doc = "Register `PRECNT` reader"]
pub type R = crate::R<PrecntSpec>;
#[doc = "Register `PRECNT` writer"]
pub type W = crate::W<PrecntSpec>;
#[doc = "Field `PRECNT` reader - Pre-Counter Value"]
pub type PrecntR = crate::FieldReader<u16>;
#[doc = "Field `PRECNT` writer - Pre-Counter Value"]
pub type PrecntW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - Pre-Counter Value"]
    #[inline(always)]
    pub fn precnt(&self) -> PrecntR {
        PrecntR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Pre-Counter Value"]
    #[inline(always)]
    pub fn precnt(&mut self) -> PrecntW<PrecntSpec> {
        PrecntW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`precnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`precnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrecntSpec;
impl crate::RegisterSpec for PrecntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`precnt::R`](R) reader structure"]
impl crate::Readable for PrecntSpec {}
#[doc = "`write(|w| ..)` method takes [`precnt::W`](W) writer structure"]
impl crate::Writable for PrecntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRECNT to value 0"]
impl crate::Resettable for PrecntSpec {}
