#[doc = "Register `CC0_ICF` reader"]
pub type R = crate::R<Cc0IcfSpec>;
#[doc = "Field `ICF` reader - Input Capture FIFO"]
pub type IcfR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Input Capture FIFO"]
    #[inline(always)]
    pub fn icf(&self) -> IcfR {
        IcfR::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc0_icf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc0IcfSpec;
impl crate::RegisterSpec for Cc0IcfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc0_icf::R`](R) reader structure"]
impl crate::Readable for Cc0IcfSpec {}
#[doc = "`reset()` method sets CC0_ICF to value 0"]
impl crate::Resettable for Cc0IcfSpec {}
