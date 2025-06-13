#[doc = "Register `RSTCAUSE` reader"]
pub type R = crate::R<RstcauseSpec>;
#[doc = "Field `POR` reader - Power On Reset"]
pub type PorR = crate::BitReader;
#[doc = "Field `PIN` reader - Pin Reset"]
pub type PinR = crate::BitReader;
#[doc = "Field `EM4` reader - EM4 Wakeup Reset"]
pub type Em4R = crate::BitReader;
#[doc = "Field `WDOG0` reader - Watchdog 0 Reset"]
pub type Wdog0R = crate::BitReader;
#[doc = "Field `WDOG1` reader - Watchdog 1 Reset"]
pub type Wdog1R = crate::BitReader;
#[doc = "Field `LOCKUP` reader - M33 Core Lockup Reset"]
pub type LockupR = crate::BitReader;
#[doc = "Field `SYSREQ` reader - M33 Core Sys Reset"]
pub type SysreqR = crate::BitReader;
#[doc = "Field `DVDDBOD` reader - HVBOD Reset"]
pub type DvddbodR = crate::BitReader;
#[doc = "Field `DVDDLEBOD` reader - LEBOD Reset"]
pub type DvddlebodR = crate::BitReader;
#[doc = "Field `DECBOD` reader - LVBOD Reset"]
pub type DecbodR = crate::BitReader;
#[doc = "Field `AVDDBOD` reader - LEBOD1 Reset"]
pub type AvddbodR = crate::BitReader;
#[doc = "Field `IOVDD0BOD` reader - LEBOD2 Reset"]
pub type Iovdd0bodR = crate::BitReader;
#[doc = "Field `SETAMPER` reader - SE Tamper event Reset"]
pub type SetamperR = crate::BitReader;
#[doc = "Field `VREGIN` reader - DCDC VREGIN comparator"]
pub type VreginR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Power On Reset"]
    #[inline(always)]
    pub fn por(&self) -> PorR {
        PorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Pin Reset"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EM4 Wakeup Reset"]
    #[inline(always)]
    pub fn em4(&self) -> Em4R {
        Em4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog 0 Reset"]
    #[inline(always)]
    pub fn wdog0(&self) -> Wdog0R {
        Wdog0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog 1 Reset"]
    #[inline(always)]
    pub fn wdog1(&self) -> Wdog1R {
        Wdog1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - M33 Core Lockup Reset"]
    #[inline(always)]
    pub fn lockup(&self) -> LockupR {
        LockupR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - M33 Core Sys Reset"]
    #[inline(always)]
    pub fn sysreq(&self) -> SysreqR {
        SysreqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - HVBOD Reset"]
    #[inline(always)]
    pub fn dvddbod(&self) -> DvddbodR {
        DvddbodR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - LEBOD Reset"]
    #[inline(always)]
    pub fn dvddlebod(&self) -> DvddlebodR {
        DvddlebodR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LVBOD Reset"]
    #[inline(always)]
    pub fn decbod(&self) -> DecbodR {
        DecbodR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LEBOD1 Reset"]
    #[inline(always)]
    pub fn avddbod(&self) -> AvddbodR {
        AvddbodR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LEBOD2 Reset"]
    #[inline(always)]
    pub fn iovdd0bod(&self) -> Iovdd0bodR {
        Iovdd0bodR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - SE Tamper event Reset"]
    #[inline(always)]
    pub fn setamper(&self) -> SetamperR {
        SetamperR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 31 - DCDC VREGIN comparator"]
    #[inline(always)]
    pub fn vregin(&self) -> VreginR {
        VreginR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`rstcause::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstcauseSpec;
impl crate::RegisterSpec for RstcauseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rstcause::R`](R) reader structure"]
impl crate::Readable for RstcauseSpec {}
#[doc = "`reset()` method sets RSTCAUSE to value 0"]
impl crate::Resettable for RstcauseSpec {}
