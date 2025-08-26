#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Lock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lock {
    #[doc = "0: All EMU lockable registers are unlocked."]
    Unlocked = 0,
    #[doc = "1: All EMU lockable registers are locked."]
    Locked = 1,
}
impl From<Lock> for bool {
    #[inline(always)]
    fn from(variant: Lock) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCK` reader - Lock status"]
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
    #[doc = "All EMU lockable registers are unlocked."]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Lock::Unlocked
    }
    #[doc = "All EMU lockable registers are locked."]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Lock::Locked
    }
}
#[doc = "Field `FIRSTTEMPDONE` reader - First Temp done"]
pub type FirsttempdoneR = crate::BitReader;
#[doc = "Field `TEMPACTIVE` reader - Temp active"]
pub type TempactiveR = crate::BitReader;
#[doc = "Field `TEMPAVGACTIVE` reader - Temp Average active"]
pub type TempavgactiveR = crate::BitReader;
#[doc = "Field `VSCALEBUSY` reader - Vscale busy"]
pub type VscalebusyR = crate::BitReader;
#[doc = "Field `VSCALEFAILED` reader - Vscale failed"]
pub type VscalefailedR = crate::BitReader;
#[doc = "Vscale status\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Vscale {
    #[doc = "0: Voltage scaling set to 0.9v"]
    Vscale0 = 0,
    #[doc = "1: Voltage scaling set to 1.0v"]
    Vscale1 = 1,
    #[doc = "2: Voltage scaling set to 1.1v"]
    Vscale2 = 2,
}
impl From<Vscale> for u8 {
    #[inline(always)]
    fn from(variant: Vscale) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Vscale {
    type Ux = u8;
}
impl crate::IsEnum for Vscale {}
#[doc = "Field `VSCALE` reader - Vscale status"]
pub type VscaleR = crate::FieldReader<Vscale>;
impl VscaleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Vscale> {
        match self.bits {
            0 => Some(Vscale::Vscale0),
            1 => Some(Vscale::Vscale1),
            2 => Some(Vscale::Vscale2),
            _ => None,
        }
    }
    #[doc = "Voltage scaling set to 0.9v"]
    #[inline(always)]
    pub fn is_vscale0(&self) -> bool {
        *self == Vscale::Vscale0
    }
    #[doc = "Voltage scaling set to 1.0v"]
    #[inline(always)]
    pub fn is_vscale1(&self) -> bool {
        *self == Vscale::Vscale1
    }
    #[doc = "Voltage scaling set to 1.1v"]
    #[inline(always)]
    pub fn is_vscale2(&self) -> bool {
        *self == Vscale::Vscale2
    }
}
#[doc = "Field `EM4IORET` reader - EM4 IO retention status"]
pub type Em4ioretR = crate::BitReader;
#[doc = "Field `EM2ENTERED` reader - EM2 entered"]
pub type Em2enteredR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Lock status"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - First Temp done"]
    #[inline(always)]
    pub fn firsttempdone(&self) -> FirsttempdoneR {
        FirsttempdoneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Temp active"]
    #[inline(always)]
    pub fn tempactive(&self) -> TempactiveR {
        TempactiveR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Temp Average active"]
    #[inline(always)]
    pub fn tempavgactive(&self) -> TempavgactiveR {
        TempavgactiveR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Vscale busy"]
    #[inline(always)]
    pub fn vscalebusy(&self) -> VscalebusyR {
        VscalebusyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Vscale failed"]
    #[inline(always)]
    pub fn vscalefailed(&self) -> VscalefailedR {
        VscalefailedR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Vscale status"]
    #[inline(always)]
    pub fn vscale(&self) -> VscaleR {
        VscaleR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 12 - EM4 IO retention status"]
    #[inline(always)]
    pub fn em4ioret(&self) -> Em4ioretR {
        Em4ioretR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - EM2 entered"]
    #[inline(always)]
    pub fn em2entered(&self) -> Em2enteredR {
        Em2enteredR::new(((self.bits >> 14) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x80"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x80;
}
