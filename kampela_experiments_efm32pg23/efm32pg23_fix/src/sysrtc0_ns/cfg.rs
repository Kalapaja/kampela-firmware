#[doc = "Register `CFG` reader"]
pub type R = crate::R<CfgSpec>;
#[doc = "Register `CFG` writer"]
pub type W = crate::W<CfgSpec>;
#[doc = "Debug Mode Run Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Debugrun {
    #[doc = "0: SYSRTC is frozen in debug mode"]
    Disable = 0,
    #[doc = "1: SYSRTC is running in debug mode"]
    Enable = 1,
}
impl From<Debugrun> for bool {
    #[inline(always)]
    fn from(variant: Debugrun) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DebugrunR = crate::BitReader<Debugrun>;
impl DebugrunR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Debugrun {
        match self.bits {
            false => Debugrun::Disable,
            true => Debugrun::Enable,
        }
    }
    #[doc = "SYSRTC is frozen in debug mode"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Debugrun::Disable
    }
    #[doc = "SYSRTC is running in debug mode"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Debugrun::Enable
    }
}
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DebugrunW<'a, REG> = crate::BitWriter<'a, REG, Debugrun>;
impl<'a, REG> DebugrunW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SYSRTC is frozen in debug mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Debugrun::Disable)
    }
    #[doc = "SYSRTC is running in debug mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Debugrun::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DebugrunR {
        DebugrunR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DebugrunW<CfgSpec> {
        DebugrunW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgSpec;
impl crate::RegisterSpec for CfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg::R`](R) reader structure"]
impl crate::Readable for CfgSpec {}
#[doc = "`write(|w| ..)` method takes [`cfg::W`](W) writer structure"]
impl crate::Writable for CfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CfgSpec {}
