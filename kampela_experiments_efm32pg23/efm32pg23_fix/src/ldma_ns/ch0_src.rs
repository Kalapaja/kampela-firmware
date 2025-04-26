#[doc = "Register `CH0_SRC` reader"]
pub type R = crate::R<Ch0SrcSpec>;
#[doc = "Register `CH0_SRC` writer"]
pub type W = crate::W<Ch0SrcSpec>;
#[doc = "Field `SRCADDR` reader - Source Data Address"]
pub type SrcaddrR = crate::FieldReader<u32>;
#[doc = "Field `SRCADDR` writer - Source Data Address"]
pub type SrcaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    pub fn srcaddr(&self) -> SrcaddrR {
        SrcaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Data Address"]
    #[inline(always)]
    pub fn srcaddr(&mut self) -> SrcaddrW<Ch0SrcSpec> {
        SrcaddrW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch0_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch0_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch0SrcSpec;
impl crate::RegisterSpec for Ch0SrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch0_src::R`](R) reader structure"]
impl crate::Readable for Ch0SrcSpec {}
#[doc = "`write(|w| ..)` method takes [`ch0_src::W`](W) writer structure"]
impl crate::Writable for Ch0SrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH0_SRC to value 0"]
impl crate::Resettable for Ch0SrcSpec {}
