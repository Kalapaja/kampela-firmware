#[doc = "Register `IPVERSION` reader"]
pub type R = crate::R<IpversionSpec>;
#[doc = "Field `IPVERSION` reader - New BitField"]
pub type IpversionR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - New BitField"]
    #[inline(always)]
    pub fn ipversion(&self) -> IpversionR {
        IpversionR::new((self.bits & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ipversion::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IpversionSpec;
impl crate::RegisterSpec for IpversionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipversion::R`](R) reader structure"]
impl crate::Readable for IpversionSpec {}
#[doc = "`reset()` method sets IPVERSION to value 0x01"]
impl crate::Resettable for IpversionSpec {
    const RESET_VALUE: u32 = 0x01;
}
