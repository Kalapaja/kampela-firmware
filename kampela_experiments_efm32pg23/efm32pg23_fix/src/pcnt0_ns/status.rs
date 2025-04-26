#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Current Counter Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: Up counter mode (clockwise in EXTCLKQUAD mode with the EDGE bit in PCNTn_CTRL set to 0)."]
    Up = 0,
    #[doc = "1: Down counter mode."]
    Down = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Current Counter Direction"]
pub type DirR = crate::BitReader<Dir>;
impl DirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dir {
        match self.bits {
            false => Dir::Up,
            true => Dir::Down,
        }
    }
    #[doc = "Up counter mode (clockwise in EXTCLKQUAD mode with the EDGE bit in PCNTn_CTRL set to 0)."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Dir::Up
    }
    #[doc = "Down counter mode."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Dir::Down
    }
}
#[doc = "Field `TOPBV` reader - TOP Buffer Valid"]
pub type TopbvR = crate::BitReader;
#[doc = "Lock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pcntlockstatus {
    #[doc = "0: PCNT registers are unlocked"]
    Unlocked = 0,
    #[doc = "1: PCNT registers are locked"]
    Locked = 1,
}
impl From<Pcntlockstatus> for bool {
    #[inline(always)]
    fn from(variant: Pcntlockstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCNTLOCKSTATUS` reader - Lock Status"]
pub type PcntlockstatusR = crate::BitReader<Pcntlockstatus>;
impl PcntlockstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pcntlockstatus {
        match self.bits {
            false => Pcntlockstatus::Unlocked,
            true => Pcntlockstatus::Locked,
        }
    }
    #[doc = "PCNT registers are unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Pcntlockstatus::Unlocked
    }
    #[doc = "PCNT registers are locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Pcntlockstatus::Locked
    }
}
#[doc = "Field `CNTRUNNING` reader - Main Counter running status"]
pub type CntrunningR = crate::BitReader;
#[doc = "Field `AUXCNTRUNNING` reader - Aux Counter running status"]
pub type AuxcntrunningR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Current Counter Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TOP Buffer Valid"]
    #[inline(always)]
    pub fn topbv(&self) -> TopbvR {
        TopbvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Lock Status"]
    #[inline(always)]
    pub fn pcntlockstatus(&self) -> PcntlockstatusR {
        PcntlockstatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Main Counter running status"]
    #[inline(always)]
    pub fn cntrunning(&self) -> CntrunningR {
        CntrunningR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Aux Counter running status"]
    #[inline(always)]
    pub fn auxcntrunning(&self) -> AuxcntrunningR {
        AuxcntrunningR::new(((self.bits >> 4) & 1) != 0)
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
