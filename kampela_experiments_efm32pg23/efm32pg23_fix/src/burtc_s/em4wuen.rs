#[doc = "Register `EM4WUEN` reader"]
pub type R = crate::R<Em4wuenSpec>;
#[doc = "Register `EM4WUEN` writer"]
pub type W = crate::W<Em4wuenSpec>;
#[doc = "Field `OFEM4WUEN` reader - Overflow EM4 Wakeup Enable"]
pub type Ofem4wuenR = crate::BitReader;
#[doc = "Field `OFEM4WUEN` writer - Overflow EM4 Wakeup Enable"]
pub type Ofem4wuenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMPEM4WUEN` reader - Compare Match EM4 Wakeup Enable"]
pub type Compem4wuenR = crate::BitReader;
#[doc = "Field `COMPEM4WUEN` writer - Compare Match EM4 Wakeup Enable"]
pub type Compem4wuenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow EM4 Wakeup Enable"]
    #[inline(always)]
    pub fn ofem4wuen(&self) -> Ofem4wuenR {
        Ofem4wuenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Match EM4 Wakeup Enable"]
    #[inline(always)]
    pub fn compem4wuen(&self) -> Compem4wuenR {
        Compem4wuenR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow EM4 Wakeup Enable"]
    #[inline(always)]
    pub fn ofem4wuen(&mut self) -> Ofem4wuenW<Em4wuenSpec> {
        Ofem4wuenW::new(self, 0)
    }
    #[doc = "Bit 1 - Compare Match EM4 Wakeup Enable"]
    #[inline(always)]
    pub fn compem4wuen(&mut self) -> Compem4wuenW<Em4wuenSpec> {
        Compem4wuenW::new(self, 1)
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
