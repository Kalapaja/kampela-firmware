#[doc = "Register `AREGB` reader"]
pub type R = crate::R<AregbSpec>;
#[doc = "Register `AREGB` writer"]
pub type W = crate::W<AregbSpec>;
#[doc = "Field `AREGB` reader - Animation Register B Data"]
pub type AregbR = crate::FieldReader;
#[doc = "Field `AREGB` writer - Animation Register B Data"]
pub type AregbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Animation Register B Data"]
    #[inline(always)]
    pub fn aregb(&self) -> AregbR {
        AregbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Animation Register B Data"]
    #[inline(always)]
    pub fn aregb(&mut self) -> AregbW<AregbSpec> {
        AregbW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`aregb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aregb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AregbSpec;
impl crate::RegisterSpec for AregbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aregb::R`](R) reader structure"]
impl crate::Readable for AregbSpec {}
#[doc = "`write(|w| ..)` method takes [`aregb::W`](W) writer structure"]
impl crate::Writable for AregbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AREGB to value 0"]
impl crate::Resettable for AregbSpec {}
