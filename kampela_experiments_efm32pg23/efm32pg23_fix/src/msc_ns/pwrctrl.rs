#[doc = "Register `PWRCTRL` reader"]
pub type R = crate::R<PwrctrlSpec>;
#[doc = "Register `PWRCTRL` writer"]
pub type W = crate::W<PwrctrlSpec>;
#[doc = "Field `PWROFFONEM1ENTRY` reader - Power down Flash macro when enter EM1"]
pub type Pwroffonem1entryR = crate::BitReader;
#[doc = "Field `PWROFFONEM1ENTRY` writer - Power down Flash macro when enter EM1"]
pub type Pwroffonem1entryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWROFFONEM1PENTRY` reader - Power down Flash macro when enter EM1P"]
pub type Pwroffonem1pentryR = crate::BitReader;
#[doc = "Field `PWROFFONEM1PENTRY` writer - Power down Flash macro when enter EM1P"]
pub type Pwroffonem1pentryW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWROFFENTRYAGAIN` reader - POWER down flash again in EM1/EM1p"]
pub type PwroffentryagainR = crate::BitReader;
#[doc = "Field `PWROFFENTRYAGAIN` writer - POWER down flash again in EM1/EM1p"]
pub type PwroffentryagainW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWROFFDLY` reader - Power down delay"]
pub type PwroffdlyR = crate::FieldReader;
#[doc = "Field `PWROFFDLY` writer - Power down delay"]
pub type PwroffdlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Power down Flash macro when enter EM1"]
    #[inline(always)]
    pub fn pwroffonem1entry(&self) -> Pwroffonem1entryR {
        Pwroffonem1entryR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Power down Flash macro when enter EM1P"]
    #[inline(always)]
    pub fn pwroffonem1pentry(&self) -> Pwroffonem1pentryR {
        Pwroffonem1pentryR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - POWER down flash again in EM1/EM1p"]
    #[inline(always)]
    pub fn pwroffentryagain(&self) -> PwroffentryagainR {
        PwroffentryagainR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Power down delay"]
    #[inline(always)]
    pub fn pwroffdly(&self) -> PwroffdlyR {
        PwroffdlyR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Power down Flash macro when enter EM1"]
    #[inline(always)]
    pub fn pwroffonem1entry(&mut self) -> Pwroffonem1entryW<PwrctrlSpec> {
        Pwroffonem1entryW::new(self, 0)
    }
    #[doc = "Bit 1 - Power down Flash macro when enter EM1P"]
    #[inline(always)]
    pub fn pwroffonem1pentry(&mut self) -> Pwroffonem1pentryW<PwrctrlSpec> {
        Pwroffonem1pentryW::new(self, 1)
    }
    #[doc = "Bit 4 - POWER down flash again in EM1/EM1p"]
    #[inline(always)]
    pub fn pwroffentryagain(&mut self) -> PwroffentryagainW<PwrctrlSpec> {
        PwroffentryagainW::new(self, 4)
    }
    #[doc = "Bits 16:23 - Power down delay"]
    #[inline(always)]
    pub fn pwroffdly(&mut self) -> PwroffdlyW<PwrctrlSpec> {
        PwroffdlyW::new(self, 16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrctrlSpec;
impl crate::RegisterSpec for PwrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrctrl::R`](R) reader structure"]
impl crate::Readable for PwrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrctrl::W`](W) writer structure"]
impl crate::Writable for PwrctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRCTRL to value 0x0010_0002"]
impl crate::Resettable for PwrctrlSpec {
    const RESET_VALUE: u32 = 0x0010_0002;
}
