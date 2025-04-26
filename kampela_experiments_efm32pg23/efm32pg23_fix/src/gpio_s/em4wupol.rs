#[doc = "Register `EM4WUPOL` reader"]
pub type R = crate::R<Em4wupolSpec>;
#[doc = "Register `EM4WUPOL` writer"]
pub type W = crate::W<Em4wupolSpec>;
#[doc = "Field `EM4WUPOL` reader - EM4 Wake-Up Polarity"]
pub type Em4wupolR = crate::FieldReader<u16>;
#[doc = "Field `EM4WUPOL` writer - EM4 Wake-Up Polarity"]
pub type Em4wupolW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 16:27 - EM4 Wake-Up Polarity"]
    #[inline(always)]
    pub fn em4wupol(&self) -> Em4wupolR {
        Em4wupolR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - EM4 Wake-Up Polarity"]
    #[inline(always)]
    pub fn em4wupol(&mut self) -> Em4wupolW<Em4wupolSpec> {
        Em4wupolW::new(self, 16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wupol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wupol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Em4wupolSpec;
impl crate::RegisterSpec for Em4wupolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em4wupol::R`](R) reader structure"]
impl crate::Readable for Em4wupolSpec {}
#[doc = "`write(|w| ..)` method takes [`em4wupol::W`](W) writer structure"]
impl crate::Writable for Em4wupolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EM4WUPOL to value 0"]
impl crate::Resettable for Em4wupolSpec {}
