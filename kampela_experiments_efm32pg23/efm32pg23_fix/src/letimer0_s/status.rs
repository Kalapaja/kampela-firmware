#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `RUNNING` reader - LETIMER Running"]
pub type RunningR = crate::BitReader;
#[doc = "LETIMER Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Letimerlockstatus {
    #[doc = "0: LETIMER registers are unlocked"]
    Unlocked = 0,
    #[doc = "1: LETIMER registers are locked"]
    Locked = 1,
}
impl From<Letimerlockstatus> for bool {
    #[inline(always)]
    fn from(variant: Letimerlockstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LETIMERLOCKSTATUS` reader - LETIMER Lock Status"]
pub type LetimerlockstatusR = crate::BitReader<Letimerlockstatus>;
impl LetimerlockstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Letimerlockstatus {
        match self.bits {
            false => Letimerlockstatus::Unlocked,
            true => Letimerlockstatus::Locked,
        }
    }
    #[doc = "LETIMER registers are unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Letimerlockstatus::Unlocked
    }
    #[doc = "LETIMER registers are locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Letimerlockstatus::Locked
    }
}
impl R {
    #[doc = "Bit 0 - LETIMER Running"]
    #[inline(always)]
    pub fn running(&self) -> RunningR {
        RunningR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LETIMER Lock Status"]
    #[inline(always)]
    pub fn letimerlockstatus(&self) -> LetimerlockstatusR {
        LetimerlockstatusR::new(((self.bits >> 1) & 1) != 0)
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
