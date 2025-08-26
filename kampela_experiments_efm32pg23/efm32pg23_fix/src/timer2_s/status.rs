#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `RUNNING` reader - Running"]
pub type RunningR = crate::BitReader;
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: Counting up"]
    Up = 0,
    #[doc = "1: Counting down"]
    Down = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Direction"]
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
    #[doc = "Counting up"]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Dir::Up
    }
    #[doc = "Counting down"]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Dir::Down
    }
}
#[doc = "Field `TOPBV` reader - TOP Buffer Valid"]
pub type TopbvR = crate::BitReader;
#[doc = "Timer lock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Timerlockstatus {
    #[doc = "0: TIMER registers are unlocked"]
    Unlocked = 0,
    #[doc = "1: TIMER registers are locked"]
    Locked = 1,
}
impl From<Timerlockstatus> for bool {
    #[inline(always)]
    fn from(variant: Timerlockstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMERLOCKSTATUS` reader - Timer lock status"]
pub type TimerlockstatusR = crate::BitReader<Timerlockstatus>;
impl TimerlockstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Timerlockstatus {
        match self.bits {
            false => Timerlockstatus::Unlocked,
            true => Timerlockstatus::Locked,
        }
    }
    #[doc = "TIMER registers are unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Timerlockstatus::Unlocked
    }
    #[doc = "TIMER registers are locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Timerlockstatus::Locked
    }
}
#[doc = "DTI lock status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtilockstatus {
    #[doc = "0: DTI registers are unlocked"]
    Unlocked = 0,
    #[doc = "1: DTI registers are locked"]
    Locked = 1,
}
impl From<Dtilockstatus> for bool {
    #[inline(always)]
    fn from(variant: Dtilockstatus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTILOCKSTATUS` reader - DTI lock status"]
pub type DtilockstatusR = crate::BitReader<Dtilockstatus>;
impl DtilockstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtilockstatus {
        match self.bits {
            false => Dtilockstatus::Unlocked,
            true => Dtilockstatus::Locked,
        }
    }
    #[doc = "DTI registers are unlocked"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Dtilockstatus::Unlocked
    }
    #[doc = "DTI registers are locked"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Dtilockstatus::Locked
    }
}
#[doc = "Field `SYNCBUSY` reader - Sync Busy"]
pub type SyncbusyR = crate::BitReader;
#[doc = "Field `OCBV0` reader - Output Compare Buffer Valid"]
pub type Ocbv0R = crate::BitReader;
#[doc = "Field `OCBV1` reader - Output Compare Buffer Valid"]
pub type Ocbv1R = crate::BitReader;
#[doc = "Field `OCBV2` reader - Output Compare Buffer Valid"]
pub type Ocbv2R = crate::BitReader;
#[doc = "Field `ICFEMPTY0` reader - Input capture fifo empty"]
pub type Icfempty0R = crate::BitReader;
#[doc = "Field `ICFEMPTY1` reader - Input capture fifo empty"]
pub type Icfempty1R = crate::BitReader;
#[doc = "Field `ICFEMPTY2` reader - Input capture fifo empty"]
pub type Icfempty2R = crate::BitReader;
#[doc = "Compare/Capture Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccpol0 {
    #[doc = "0: CCx polarity low level/rising edge"]
    Lowrise = 0,
    #[doc = "1: CCx polarity high level/falling edge"]
    Highfall = 1,
}
impl From<Ccpol0> for bool {
    #[inline(always)]
    fn from(variant: Ccpol0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPOL0` reader - Compare/Capture Polarity"]
pub type Ccpol0R = crate::BitReader<Ccpol0>;
impl Ccpol0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccpol0 {
        match self.bits {
            false => Ccpol0::Lowrise,
            true => Ccpol0::Highfall,
        }
    }
    #[doc = "CCx polarity low level/rising edge"]
    #[inline(always)]
    pub fn is_lowrise(&self) -> bool {
        *self == Ccpol0::Lowrise
    }
    #[doc = "CCx polarity high level/falling edge"]
    #[inline(always)]
    pub fn is_highfall(&self) -> bool {
        *self == Ccpol0::Highfall
    }
}
#[doc = "Compare/Capture Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccpol1 {
    #[doc = "0: CCx polarity low level/rising edge"]
    Lowrise = 0,
    #[doc = "1: CCx polarity high level/falling edge"]
    Highfall = 1,
}
impl From<Ccpol1> for bool {
    #[inline(always)]
    fn from(variant: Ccpol1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPOL1` reader - Compare/Capture Polarity"]
pub type Ccpol1R = crate::BitReader<Ccpol1>;
impl Ccpol1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccpol1 {
        match self.bits {
            false => Ccpol1::Lowrise,
            true => Ccpol1::Highfall,
        }
    }
    #[doc = "CCx polarity low level/rising edge"]
    #[inline(always)]
    pub fn is_lowrise(&self) -> bool {
        *self == Ccpol1::Lowrise
    }
    #[doc = "CCx polarity high level/falling edge"]
    #[inline(always)]
    pub fn is_highfall(&self) -> bool {
        *self == Ccpol1::Highfall
    }
}
#[doc = "Compare/Capture Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccpol2 {
    #[doc = "0: CCx polarity low level/rising edge"]
    Lowrise = 0,
    #[doc = "1: CCx polarity high level/falling edge"]
    Highfall = 1,
}
impl From<Ccpol2> for bool {
    #[inline(always)]
    fn from(variant: Ccpol2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPOL2` reader - Compare/Capture Polarity"]
pub type Ccpol2R = crate::BitReader<Ccpol2>;
impl Ccpol2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccpol2 {
        match self.bits {
            false => Ccpol2::Lowrise,
            true => Ccpol2::Highfall,
        }
    }
    #[doc = "CCx polarity low level/rising edge"]
    #[inline(always)]
    pub fn is_lowrise(&self) -> bool {
        *self == Ccpol2::Lowrise
    }
    #[doc = "CCx polarity high level/falling edge"]
    #[inline(always)]
    pub fn is_highfall(&self) -> bool {
        *self == Ccpol2::Highfall
    }
}
impl R {
    #[doc = "Bit 0 - Running"]
    #[inline(always)]
    pub fn running(&self) -> RunningR {
        RunningR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TOP Buffer Valid"]
    #[inline(always)]
    pub fn topbv(&self) -> TopbvR {
        TopbvR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer lock status"]
    #[inline(always)]
    pub fn timerlockstatus(&self) -> TimerlockstatusR {
        TimerlockstatusR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DTI lock status"]
    #[inline(always)]
    pub fn dtilockstatus(&self) -> DtilockstatusR {
        DtilockstatusR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sync Busy"]
    #[inline(always)]
    pub fn syncbusy(&self) -> SyncbusyR {
        SyncbusyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Compare Buffer Valid"]
    #[inline(always)]
    pub fn ocbv0(&self) -> Ocbv0R {
        Ocbv0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output Compare Buffer Valid"]
    #[inline(always)]
    pub fn ocbv1(&self) -> Ocbv1R {
        Ocbv1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Output Compare Buffer Valid"]
    #[inline(always)]
    pub fn ocbv2(&self) -> Ocbv2R {
        Ocbv2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Input capture fifo empty"]
    #[inline(always)]
    pub fn icfempty0(&self) -> Icfempty0R {
        Icfempty0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Input capture fifo empty"]
    #[inline(always)]
    pub fn icfempty1(&self) -> Icfempty1R {
        Icfempty1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Input capture fifo empty"]
    #[inline(always)]
    pub fn icfempty2(&self) -> Icfempty2R {
        Icfempty2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 24 - Compare/Capture Polarity"]
    #[inline(always)]
    pub fn ccpol0(&self) -> Ccpol0R {
        Ccpol0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Compare/Capture Polarity"]
    #[inline(always)]
    pub fn ccpol1(&self) -> Ccpol1R {
        Ccpol1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Compare/Capture Polarity"]
    #[inline(always)]
    pub fn ccpol2(&self) -> Ccpol2R {
        Ccpol2R::new(((self.bits >> 26) & 1) != 0)
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
