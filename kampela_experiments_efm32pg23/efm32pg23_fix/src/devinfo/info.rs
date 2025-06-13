#[doc = "Register `INFO` reader"]
pub type R = crate::R<InfoSpec>;
#[doc = "Field `CRC` reader - CRC"]
pub type CrcR = crate::FieldReader<u16>;
#[doc = "Field `PRODREV` reader - Production Revision"]
pub type ProdrevR = crate::FieldReader;
#[doc = "Field `DEVINFOREV` reader - DI Page Version"]
pub type DevinforevR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - CRC"]
    #[inline(always)]
    pub fn crc(&self) -> CrcR {
        CrcR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Production Revision"]
    #[inline(always)]
    pub fn prodrev(&self) -> ProdrevR {
        ProdrevR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DI Page Version"]
    #[inline(always)]
    pub fn devinforev(&self) -> DevinforevR {
        DevinforevR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Version of the device info structure being used\n\nYou can [`read`](crate::Reg::read) this register and get [`info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InfoSpec;
impl crate::RegisterSpec for InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`info::R`](R) reader structure"]
impl crate::Readable for InfoSpec {}
#[doc = "`reset()` method sets INFO to value 0x0c00_0000"]
impl crate::Resettable for InfoSpec {
    const RESET_VALUE: u32 = 0x0c00_0000;
}
