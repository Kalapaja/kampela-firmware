#[doc = "Register `MEMINFO` reader"]
pub type R = crate::R<MeminfoSpec>;
#[doc = "Field `FLASHPAGESIZE` reader - Flash Page Size"]
pub type FlashpagesizeR = crate::FieldReader;
#[doc = "Field `UDPAGESIZE` reader - User Data Page Size"]
pub type UdpagesizeR = crate::FieldReader;
#[doc = "Field `DILEN` reader - Length of DI Page"]
pub type DilenR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - Flash Page Size"]
    #[inline(always)]
    pub fn flashpagesize(&self) -> FlashpagesizeR {
        FlashpagesizeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - User Data Page Size"]
    #[inline(always)]
    pub fn udpagesize(&self) -> UdpagesizeR {
        UdpagesizeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Length of DI Page"]
    #[inline(always)]
    pub fn dilen(&self) -> DilenR {
        DilenR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Flash page size and misc. chip information\n\nYou can [`read`](crate::Reg::read) this register and get [`meminfo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MeminfoSpec;
impl crate::RegisterSpec for MeminfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`meminfo::R`](R) reader structure"]
impl crate::Readable for MeminfoSpec {}
#[doc = "`reset()` method sets MEMINFO to value 0"]
impl crate::Resettable for MeminfoSpec {}
