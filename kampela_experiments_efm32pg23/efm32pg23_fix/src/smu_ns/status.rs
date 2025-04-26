#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "SMU Lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Smulock {
    #[doc = "0: UNLOCKED"]
    Unlocked = 0,
    #[doc = "1: LOCKED"]
    Locked = 1,
}
impl From<Smulock> for bool {
    #[inline(always)]
    fn from(variant: Smulock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMULOCK` reader - SMU Lock"]
pub type SmulockR = crate::BitReader<Smulock>;
impl SmulockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Smulock {
        match self.bits {
            false => Smulock::Unlocked,
            true => Smulock::Locked,
        }
    }
    #[doc = "UNLOCKED"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Smulock::Unlocked
    }
    #[doc = "LOCKED"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Smulock::Locked
    }
}
#[doc = "Field `SMUPRGERR` reader - SMU Programming Error"]
pub type SmuprgerrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SMU Lock"]
    #[inline(always)]
    pub fn smulock(&self) -> SmulockR {
        SmulockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMU Programming Error"]
    #[inline(always)]
    pub fn smuprgerr(&self) -> SmuprgerrR {
        SmuprgerrR::new(((self.bits >> 1) & 1) != 0)
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
