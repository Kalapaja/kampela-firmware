#[doc = "Register `IPVERSION` reader"]
pub type R = crate::R<IpversionSpec>;
#[doc = "Field `IPVERSION` reader - IP version ID"]
pub type IpversionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IP version ID"]
    #[inline(always)]
    pub fn ipversion(&self) -> IpversionR {
        IpversionR::new(self.bits)
    }
}
#[doc = "The read only IPVERSION field gives the version for this module. There may be minor software changes required for modules with different values of IPVERSION.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpversionSpec;
impl crate::RegisterSpec for IpversionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipversion::R`](R) reader structure"]
impl crate::Readable for IpversionSpec {}
#[doc = "`reset()` method sets IPVERSION to value 0"]
impl crate::Resettable for IpversionSpec {}
