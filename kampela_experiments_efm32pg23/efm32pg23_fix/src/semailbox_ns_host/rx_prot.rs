#[doc = "Register `RX_PROT` reader"]
pub type R = crate::R<RxProtSpec>;
#[doc = "Field `UNPROTECTED` reader - UNPROTECTED"]
pub type UnprotectedR = crate::BitReader;
#[doc = "Field `PRIVILEGED` reader - PRIVILEGED"]
pub type PrivilegedR = crate::BitReader;
#[doc = "Field `NONSECURE` reader - NONSECURE"]
pub type NonsecureR = crate::BitReader;
#[doc = "Field `USER` reader - USER"]
pub type UserR = crate::FieldReader;
impl R {
    #[doc = "Bit 21 - UNPROTECTED"]
    #[inline(always)]
    pub fn unprotected(&self) -> UnprotectedR {
        UnprotectedR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PRIVILEGED"]
    #[inline(always)]
    pub fn privileged(&self) -> PrivilegedR {
        PrivilegedR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - NONSECURE"]
    #[inline(always)]
    pub fn nonsecure(&self) -> NonsecureR {
        NonsecureR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - USER"]
    #[inline(always)]
    pub fn user(&self) -> UserR {
        UserR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "RX Protection register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_prot::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxProtSpec;
impl crate::RegisterSpec for RxProtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_prot::R`](R) reader structure"]
impl crate::Readable for RxProtSpec {}
#[doc = "`reset()` method sets RX_PROT to value 0"]
impl crate::Resettable for RxProtSpec {}
