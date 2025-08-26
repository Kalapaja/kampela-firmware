#[doc = "Register `BMPUFSADDR` reader"]
pub type R = crate::R<BmpufsaddrSpec>;
#[doc = "Field `BMPUFSADDR` reader - Fault Address"]
pub type BmpufsaddrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Fault Address"]
    #[inline(always)]
    pub fn bmpufsaddr(&self) -> BmpufsaddrR {
        BmpufsaddrR::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`bmpufsaddr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmpufsaddrSpec;
impl crate::RegisterSpec for BmpufsaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmpufsaddr::R`](R) reader structure"]
impl crate::Readable for BmpufsaddrSpec {}
#[doc = "`reset()` method sets BMPUFSADDR to value 0"]
impl crate::Resettable for BmpufsaddrSpec {}
