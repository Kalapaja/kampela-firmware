#[doc = "Register `SADDR` reader"]
pub type R = crate::R<SaddrSpec>;
#[doc = "Register `SADDR` writer"]
pub type W = crate::W<SaddrSpec>;
#[doc = "Field `ADDR` reader - Follower address"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Follower address"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - Follower address"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Follower address"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<SaddrSpec> {
        AddrW::new(self, 1)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`saddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaddrSpec;
impl crate::RegisterSpec for SaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saddr::R`](R) reader structure"]
impl crate::Readable for SaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`saddr::W`](W) writer structure"]
impl crate::Writable for SaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SADDR to value 0"]
impl crate::Resettable for SaddrSpec {}
