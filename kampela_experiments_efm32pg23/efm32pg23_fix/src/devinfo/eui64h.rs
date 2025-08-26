#[doc = "Register `EUI64H` reader"]
pub type R = crate::R<Eui64hSpec>;
#[doc = "Field `UNIQUEH` reader - UNIQUEH"]
pub type UniquehR = crate::FieldReader;
#[doc = "Field `OUI64` reader - OUI64"]
pub type Oui64R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - UNIQUEH"]
    #[inline(always)]
    pub fn uniqueh(&self) -> UniquehR {
        UniquehR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - OUI64"]
    #[inline(always)]
    pub fn oui64(&self) -> Oui64R {
        Oui64R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "MA-L compliant EUI64 OUI and Unique Identifier (high bits)\n\nYou can [`read`](crate::Reg::read) this register and get [`eui64h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eui64hSpec;
impl crate::RegisterSpec for Eui64hSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eui64h::R`](R) reader structure"]
impl crate::Readable for Eui64hSpec {}
#[doc = "`reset()` method sets EUI64H to value 0"]
impl crate::Resettable for Eui64hSpec {}
