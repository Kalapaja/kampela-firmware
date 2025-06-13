#[doc = "Register `SADDRMASK` reader"]
pub type R = crate::R<SaddrmaskSpec>;
#[doc = "Register `SADDRMASK` writer"]
pub type W = crate::W<SaddrmaskSpec>;
#[doc = "Field `SADDRMASK` reader - Follower Address Mask"]
pub type SaddrmaskR = crate::FieldReader;
#[doc = "Field `SADDRMASK` writer - Follower Address Mask"]
pub type SaddrmaskW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:7 - Follower Address Mask"]
    #[inline(always)]
    pub fn saddrmask(&self) -> SaddrmaskR {
        SaddrmaskR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Follower Address Mask"]
    #[inline(always)]
    pub fn saddrmask(&mut self) -> SaddrmaskW<SaddrmaskSpec> {
        SaddrmaskW::new(self, 1)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`saddrmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddrmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaddrmaskSpec;
impl crate::RegisterSpec for SaddrmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`saddrmask::R`](R) reader structure"]
impl crate::Readable for SaddrmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`saddrmask::W`](W) writer structure"]
impl crate::Writable for SaddrmaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SADDRMASK to value 0"]
impl crate::Resettable for SaddrmaskSpec {}
