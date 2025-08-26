#[doc = "Register `EXTIRISE` reader"]
pub type R = crate::R<ExtiriseSpec>;
#[doc = "Register `EXTIRISE` writer"]
pub type W = crate::W<ExtiriseSpec>;
#[doc = "Field `EXTIRISE` reader - EXT Int Rise"]
pub type ExtiriseR = crate::FieldReader<u16>;
#[doc = "Field `EXTIRISE` writer - EXT Int Rise"]
pub type ExtiriseW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - EXT Int Rise"]
    #[inline(always)]
    pub fn extirise(&self) -> ExtiriseR {
        ExtiriseR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - EXT Int Rise"]
    #[inline(always)]
    pub fn extirise(&mut self) -> ExtiriseW<ExtiriseSpec> {
        ExtiriseW::new(self, 0)
    }
}
#[doc = "External Interrupt Rising Edge Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`extirise::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extirise::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtiriseSpec;
impl crate::RegisterSpec for ExtiriseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extirise::R`](R) reader structure"]
impl crate::Readable for ExtiriseSpec {}
#[doc = "`write(|w| ..)` method takes [`extirise::W`](W) writer structure"]
impl crate::Writable for ExtiriseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTIRISE to value 0"]
impl crate::Resettable for ExtiriseSpec {}
