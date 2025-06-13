#[doc = "Register `WDATA` reader"]
pub type R = crate::R<WdataSpec>;
#[doc = "Register `WDATA` writer"]
pub type W = crate::W<WdataSpec>;
#[doc = "Field `DATAW` reader - Write Data"]
pub type DatawR = crate::FieldReader<u32>;
#[doc = "Field `DATAW` writer - Write Data"]
pub type DatawW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write Data"]
    #[inline(always)]
    pub fn dataw(&self) -> DatawR {
        DatawR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write Data"]
    #[inline(always)]
    pub fn dataw(&mut self) -> DatawW<WdataSpec> {
        DatawW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`wdata::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdata::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdataSpec;
impl crate::RegisterSpec for WdataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdata::R`](R) reader structure"]
impl crate::Readable for WdataSpec {}
#[doc = "`write(|w| ..)` method takes [`wdata::W`](W) writer structure"]
impl crate::Writable for WdataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDATA to value 0"]
impl crate::Resettable for WdataSpec {}
