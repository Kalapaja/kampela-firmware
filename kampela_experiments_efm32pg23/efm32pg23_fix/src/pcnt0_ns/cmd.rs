#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `CORERST` writer - PCNT Clock Domain Reset"]
pub type CorerstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTRST` writer - CNT Reset"]
pub type CntrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXCNTRST` writer - AUXCNT Reset"]
pub type AuxcntrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCNTIM` writer - Load CNT Immediately"]
pub type LcntimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTCNT` writer - Start Main Counter"]
pub type StartcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STARTAUXCNT` writer - Start Aux Counter"]
pub type StartauxcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPCNT` writer - Stop Main Counter"]
pub type StopcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPAUXCNT` writer - Stop Aux Counter"]
pub type StopauxcntW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - PCNT Clock Domain Reset"]
    #[inline(always)]
    pub fn corerst(&mut self) -> CorerstW<CmdSpec> {
        CorerstW::new(self, 0)
    }
    #[doc = "Bit 1 - CNT Reset"]
    #[inline(always)]
    pub fn cntrst(&mut self) -> CntrstW<CmdSpec> {
        CntrstW::new(self, 1)
    }
    #[doc = "Bit 2 - AUXCNT Reset"]
    #[inline(always)]
    pub fn auxcntrst(&mut self) -> AuxcntrstW<CmdSpec> {
        AuxcntrstW::new(self, 2)
    }
    #[doc = "Bit 4 - Load CNT Immediately"]
    #[inline(always)]
    pub fn lcntim(&mut self) -> LcntimW<CmdSpec> {
        LcntimW::new(self, 4)
    }
    #[doc = "Bit 8 - Start Main Counter"]
    #[inline(always)]
    pub fn startcnt(&mut self) -> StartcntW<CmdSpec> {
        StartcntW::new(self, 8)
    }
    #[doc = "Bit 9 - Start Aux Counter"]
    #[inline(always)]
    pub fn startauxcnt(&mut self) -> StartauxcntW<CmdSpec> {
        StartauxcntW::new(self, 9)
    }
    #[doc = "Bit 10 - Stop Main Counter"]
    #[inline(always)]
    pub fn stopcnt(&mut self) -> StopcntW<CmdSpec> {
        StopcntW::new(self, 10)
    }
    #[doc = "Bit 11 - Stop Aux Counter"]
    #[inline(always)]
    pub fn stopauxcnt(&mut self) -> StopauxcntW<CmdSpec> {
        StopauxcntW::new(self, 11)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CmdSpec {}
