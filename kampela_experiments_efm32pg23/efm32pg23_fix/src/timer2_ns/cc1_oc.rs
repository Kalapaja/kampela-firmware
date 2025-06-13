#[doc = "Register `CC1_OC` reader"]
pub type R = crate::R<Cc1OcSpec>;
#[doc = "Register `CC1_OC` writer"]
pub type W = crate::W<Cc1OcSpec>;
#[doc = "Field `OC` reader - Output Compare Value"]
pub type OcR = crate::FieldReader<u16>;
#[doc = "Field `OC` writer - Output Compare Value"]
pub type OcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output Compare Value"]
    #[inline(always)]
    pub fn oc(&self) -> OcR {
        OcR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output Compare Value"]
    #[inline(always)]
    pub fn oc(&mut self) -> OcW<Cc1OcSpec> {
        OcW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_oc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc1_oc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc1OcSpec;
impl crate::RegisterSpec for Cc1OcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc1_oc::R`](R) reader structure"]
impl crate::Readable for Cc1OcSpec {}
#[doc = "`write(|w| ..)` method takes [`cc1_oc::W`](W) writer structure"]
impl crate::Writable for Cc1OcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC1_OC to value 0"]
impl crate::Resettable for Cc1OcSpec {}
