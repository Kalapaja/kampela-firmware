#[doc = "Register `EM4WUEN` reader"]
pub type R = crate::R<Em4wuenSpec>;
#[doc = "Register `EM4WUEN` writer"]
pub type W = crate::W<Em4wuenSpec>;
#[doc = "Field `EM4WUEN` reader - EM4 wake up enable"]
pub type Em4wuenR = crate::FieldReader<u16>;
#[doc = "Field `EM4WUEN` writer - EM4 wake up enable"]
pub type Em4wuenW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 16:27 - EM4 wake up enable"]
    #[inline(always)]
    pub fn em4wuen(&self) -> Em4wuenR {
        Em4wuenR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - EM4 wake up enable"]
    #[inline(always)]
    pub fn em4wuen(&mut self) -> Em4wuenW<Em4wuenSpec> {
        Em4wuenW::new(self, 16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`em4wuen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`em4wuen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Em4wuenSpec;
impl crate::RegisterSpec for Em4wuenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`em4wuen::R`](R) reader structure"]
impl crate::Readable for Em4wuenSpec {}
#[doc = "`write(|w| ..)` method takes [`em4wuen::W`](W) writer structure"]
impl crate::Writable for Em4wuenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EM4WUEN to value 0"]
impl crate::Resettable for Em4wuenSpec {}
