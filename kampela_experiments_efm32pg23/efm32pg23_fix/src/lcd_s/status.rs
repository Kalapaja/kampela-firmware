#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `ASTATE` reader - Current Animation State"]
pub type AstateR = crate::FieldReader;
#[doc = "Field `BLINK` reader - Blink State"]
pub type BlinkR = crate::BitReader;
#[doc = "Field `LOADBUSY` reader - Load Synchronization is busy"]
pub type LoadbusyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Current Animation State"]
    #[inline(always)]
    pub fn astate(&self) -> AstateR {
        AstateR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Blink State"]
    #[inline(always)]
    pub fn blink(&self) -> BlinkR {
        BlinkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Load Synchronization is busy"]
    #[inline(always)]
    pub fn loadbusy(&self) -> LoadbusyR {
        LoadbusyR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
