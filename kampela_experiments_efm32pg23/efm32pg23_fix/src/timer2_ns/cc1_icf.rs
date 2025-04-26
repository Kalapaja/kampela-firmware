#[doc = "Register `CC1_ICF` reader"]
pub type R = crate::R<Cc1IcfSpec>;
#[doc = "Field `ICF` reader - Input Capture FIFO"]
pub type IcfR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Input Capture FIFO"]
    #[inline(always)]
    pub fn icf(&self) -> IcfR {
        IcfR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc1_icf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc1IcfSpec;
impl crate::RegisterSpec for Cc1IcfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc1_icf::R`](R) reader structure"]
impl crate::Readable for Cc1IcfSpec {}
#[doc = "`reset()` method sets CC1_ICF to value 0"]
impl crate::Resettable for Cc1IcfSpec {}
