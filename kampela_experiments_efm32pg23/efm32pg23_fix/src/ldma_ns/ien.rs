#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `CHDONE` reader - Enable or disable the done interrupt"]
pub type ChdoneR = crate::FieldReader;
#[doc = "Field `CHDONE` writer - Enable or disable the done interrupt"]
pub type ChdoneW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ERROR` reader - Enable or disable the error interrupt"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Enable or disable the error interrupt"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Enable or disable the done interrupt"]
    #[inline(always)]
    pub fn chdone(&self) -> ChdoneR {
        ChdoneR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 31 - Enable or disable the error interrupt"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enable or disable the done interrupt"]
    #[inline(always)]
    pub fn chdone(&mut self) -> ChdoneW<IenSpec> {
        ChdoneW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable or disable the error interrupt"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<IenSpec> {
        ErrorW::new(self, 31)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
