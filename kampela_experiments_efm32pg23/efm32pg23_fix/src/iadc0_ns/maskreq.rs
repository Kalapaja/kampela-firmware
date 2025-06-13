#[doc = "Register `MASKREQ` reader"]
pub type R = crate::R<MaskreqSpec>;
#[doc = "Register `MASKREQ` writer"]
pub type W = crate::W<MaskreqSpec>;
#[doc = "Field `MASKREQ` reader - Scan Queue Mask Request"]
pub type MaskreqR = crate::FieldReader<u16>;
#[doc = "Field `MASKREQ` writer - Scan Queue Mask Request"]
pub type MaskreqW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Scan Queue Mask Request"]
    #[inline(always)]
    pub fn maskreq(&self) -> MaskreqR {
        MaskreqR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Scan Queue Mask Request"]
    #[inline(always)]
    pub fn maskreq(&mut self) -> MaskreqW<MaskreqSpec> {
        MaskreqW::new(self, 0)
    }
}
#[doc = "Mask Request\n\nYou can [`read`](crate::Reg::read) this register and get [`maskreq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskreq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskreqSpec;
impl crate::RegisterSpec for MaskreqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maskreq::R`](R) reader structure"]
impl crate::Readable for MaskreqSpec {}
#[doc = "`write(|w| ..)` method takes [`maskreq::W`](W) writer structure"]
impl crate::Writable for MaskreqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASKREQ to value 0"]
impl crate::Resettable for MaskreqSpec {}
