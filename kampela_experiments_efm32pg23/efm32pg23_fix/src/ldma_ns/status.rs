#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `ANYBUSY` reader - Any DMA Channel Busy"]
pub type AnybusyR = crate::BitReader;
#[doc = "Field `ANYREQ` reader - Any DMA Channel Request Pending"]
pub type AnyreqR = crate::BitReader;
#[doc = "Field `CHGRANT` reader - Granted Channel Number"]
pub type ChgrantR = crate::FieldReader;
#[doc = "Field `CHERROR` reader - Errant Channel Number"]
pub type CherrorR = crate::FieldReader;
#[doc = "Field `FIFOLEVEL` reader - FIFO Level"]
pub type FifolevelR = crate::FieldReader;
#[doc = "Field `CHNUM` reader - Number of Channels"]
pub type ChnumR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Any DMA Channel Busy"]
    #[inline(always)]
    pub fn anybusy(&self) -> AnybusyR {
        AnybusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Any DMA Channel Request Pending"]
    #[inline(always)]
    pub fn anyreq(&self) -> AnyreqR {
        AnyreqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Granted Channel Number"]
    #[inline(always)]
    pub fn chgrant(&self) -> ChgrantR {
        ChgrantR::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Errant Channel Number"]
    #[inline(always)]
    pub fn cherror(&self) -> CherrorR {
        CherrorR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - FIFO Level"]
    #[inline(always)]
    pub fn fifolevel(&self) -> FifolevelR {
        FifolevelR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Number of Channels"]
    #[inline(always)]
    pub fn chnum(&self) -> ChnumR {
        ChnumR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x0810_0000"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x0810_0000;
}
