#[doc = "Register `ROOTLOCKSTATUS` reader"]
pub type R = crate::R<RootlockstatusSpec>;
#[doc = "Field `BUSLOCK` reader - Bus Lock"]
pub type BuslockR = crate::BitReader;
#[doc = "Field `REGLOCK` reader - Register Lock"]
pub type ReglockR = crate::BitReader;
#[doc = "Field `MFRLOCK` reader - Manufacture Lock"]
pub type MfrlockR = crate::BitReader;
#[doc = "Field `ROOTDBGLOCK` reader - Root Debug Lock"]
pub type RootdbglockR = crate::BitReader;
#[doc = "Field `USERDBGAPLOCK` reader - User Debug Access Port Lock"]
pub type UserdbgaplockR = crate::BitReader;
#[doc = "Field `USERDBGLOCK` reader - User Invasive Debug Lock"]
pub type UserdbglockR = crate::BitReader;
#[doc = "Field `USERNIDLOCK` reader - User Non-invasive Debug Lock"]
pub type UsernidlockR = crate::BitReader;
#[doc = "Field `USERSPIDLOCK` reader - User Secure Invasive Debug Lock"]
pub type UserspidlockR = crate::BitReader;
#[doc = "Field `USERSPNIDLOCK` reader - User Secure Non-invasive Debug Lock"]
pub type UserspnidlockR = crate::BitReader;
#[doc = "Field `EFUSEUNLOCKED` reader - E-Fuse Unlocked"]
pub type EfuseunlockedR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Bus Lock"]
    #[inline(always)]
    pub fn buslock(&self) -> BuslockR {
        BuslockR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Register Lock"]
    #[inline(always)]
    pub fn reglock(&self) -> ReglockR {
        ReglockR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Manufacture Lock"]
    #[inline(always)]
    pub fn mfrlock(&self) -> MfrlockR {
        MfrlockR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Root Debug Lock"]
    #[inline(always)]
    pub fn rootdbglock(&self) -> RootdbglockR {
        RootdbglockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - User Debug Access Port Lock"]
    #[inline(always)]
    pub fn userdbgaplock(&self) -> UserdbgaplockR {
        UserdbgaplockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - User Invasive Debug Lock"]
    #[inline(always)]
    pub fn userdbglock(&self) -> UserdbglockR {
        UserdbglockR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - User Non-invasive Debug Lock"]
    #[inline(always)]
    pub fn usernidlock(&self) -> UsernidlockR {
        UsernidlockR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - User Secure Invasive Debug Lock"]
    #[inline(always)]
    pub fn userspidlock(&self) -> UserspidlockR {
        UserspidlockR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - User Secure Non-invasive Debug Lock"]
    #[inline(always)]
    pub fn userspnidlock(&self) -> UserspnidlockR {
        UserspnidlockR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - E-Fuse Unlocked"]
    #[inline(always)]
    pub fn efuseunlocked(&self) -> EfuseunlockedR {
        EfuseunlockedR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "This register returns the status of the SE managed locks.\n\nYou can [`read`](crate::Reg::read) this register and get [`rootlockstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RootlockstatusSpec;
impl crate::RegisterSpec for RootlockstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rootlockstatus::R`](R) reader structure"]
impl crate::Readable for RootlockstatusSpec {}
#[doc = "`reset()` method sets ROOTLOCKSTATUS to value 0x007f_0107"]
impl crate::Resettable for RootlockstatusSpec {
    const RESET_VALUE: u32 = 0x007f_0107;
}
