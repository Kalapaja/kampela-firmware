#[doc = "Register `EUI48H` reader"]
pub type R = crate::R<Eui48hSpec>;
#[doc = "Field `OUI48H` reader - OUI48H"]
pub type Oui48hR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - OUI48H"]
    #[inline(always)]
    pub fn oui48h(&self) -> Oui48hR {
        Oui48hR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MA-L compliant EUI48 OUI (high bits)\n\nYou can [`read`](crate::Reg::read) this register and get [`eui48h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eui48hSpec;
impl crate::RegisterSpec for Eui48hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eui48h::R`](R) reader structure"]
impl crate::Readable for Eui48hSpec {}
#[doc = "`reset()` method sets EUI48H to value 0xffff_0000"]
impl crate::Resettable for Eui48hSpec {
    const RESET_VALUE: u32 = 0xffff_0000;
}
