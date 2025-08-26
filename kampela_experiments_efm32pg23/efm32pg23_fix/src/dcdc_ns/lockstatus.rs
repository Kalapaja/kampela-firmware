#[doc = "Register `LOCKSTATUS` reader"]
pub type R = crate::R<LockstatusSpec>;
#[doc = "Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: Unlocked State"]
    Unlocked = 0,
    #[doc = "1: LOCKED STATE"]
    Locked = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Lock Status"]
pub type LockR = crate::BitReader<Lock>;
impl LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lock {
        match self.bits {
            false => Lock::Unlocked,
            true => Lock::Locked,
        }
    }
    #[doc = "Unlocked State"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lock::Unlocked
    }
    #[doc = "LOCKED STATE"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lock::Locked
    }
}
impl R {
    #[doc = "Bit 0 - Lock Status"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`lockstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockstatusSpec;
impl crate::RegisterSpec for LockstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lockstatus::R`](R) reader structure"]
impl crate::Readable for LockstatusSpec {}
#[doc = "`reset()` method sets LOCKSTATUS to value 0"]
impl crate::Resettable for LockstatusSpec {}
