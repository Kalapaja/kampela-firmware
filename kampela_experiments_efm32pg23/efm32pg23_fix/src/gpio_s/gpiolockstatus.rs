#[doc = "Register `GPIOLOCKSTATUS` reader"]
pub type R = crate::R<GpiolockstatusSpec>;
#[doc = "GPIO LOCK status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: Registers are unlocked"]
    Unlocked = 0,
    #[doc = "1: Registers are locked"]
    Locked = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - GPIO LOCK status"]
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
    #[doc = "Registers are unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lock::Unlocked
    }
    #[doc = "Registers are locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lock::Locked
    }
}
impl R {
    #[doc = "Bit 0 - GPIO LOCK status"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`gpiolockstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpiolockstatusSpec;
impl crate::RegisterSpec for GpiolockstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiolockstatus::R`](R) reader structure"]
impl crate::Readable for GpiolockstatusSpec {}
#[doc = "`reset()` method sets GPIOLOCKSTATUS to value 0"]
impl crate::Resettable for GpiolockstatusSpec {}
