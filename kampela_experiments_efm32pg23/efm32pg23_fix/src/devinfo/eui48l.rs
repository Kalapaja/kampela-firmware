#[doc = "Register `EUI48L` reader"]
pub type R = crate::R<Eui48lSpec>;
#[doc = "Field `UNIQUEID` reader - Unique ID"]
pub type UniqueidR = crate::FieldReader<u32>;
#[doc = "Field `OUI48L` reader - OUI48L"]
pub type Oui48lR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - Unique ID"]
    #[inline(always)]
    pub fn uniqueid(&self) -> UniqueidR {
        UniqueidR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - OUI48L"]
    #[inline(always)]
    pub fn oui48l(&self) -> Oui48lR {
        Oui48lR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "MA-L compliant EUI48 OUI (low bits) and Unique Identifier (24-bit)\n\nYou can [`read`](crate::Reg::read) this register and get [`eui48l::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eui48lSpec;
impl crate::RegisterSpec for Eui48lSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eui48l::R`](R) reader structure"]
impl crate::Readable for Eui48lSpec {}
#[doc = "`reset()` method sets EUI48L to value 0"]
impl crate::Resettable for Eui48lSpec {}
