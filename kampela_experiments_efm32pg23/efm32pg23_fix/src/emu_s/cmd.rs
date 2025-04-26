#[doc = "Register `CMD` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "Field `EM4UNLATCH` writer - EM4 unlatch"]
pub type Em4unlatchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEMPAVGREQ` writer - Temperature Average Request"]
pub type TempavgreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM01VSCALE1` writer - Scale voltage to Vscale1"]
pub type Em01vscale1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM01VSCALE2` writer - Scale voltage to Vscale2"]
pub type Em01vscale2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTCAUSECLR` writer - Reset Cause Clear"]
pub type RstcauseclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPERRCCLR` writer - Tamper Reset Cause Clear"]
pub type TamperrcclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 1 - EM4 unlatch"]
    #[inline(always)]
    pub fn em4unlatch(&mut self) -> Em4unlatchW<CmdSpec> {
        Em4unlatchW::new(self, 1)
    }
    #[doc = "Bit 4 - Temperature Average Request"]
    #[inline(always)]
    pub fn tempavgreq(&mut self) -> TempavgreqW<CmdSpec> {
        TempavgreqW::new(self, 4)
    }
    #[doc = "Bit 10 - Scale voltage to Vscale1"]
    #[inline(always)]
    pub fn em01vscale1(&mut self) -> Em01vscale1W<CmdSpec> {
        Em01vscale1W::new(self, 10)
    }
    #[doc = "Bit 11 - Scale voltage to Vscale2"]
    #[inline(always)]
    pub fn em01vscale2(&mut self) -> Em01vscale2W<CmdSpec> {
        Em01vscale2W::new(self, 11)
    }
    #[doc = "Bit 17 - Reset Cause Clear"]
    #[inline(always)]
    pub fn rstcauseclr(&mut self) -> RstcauseclrW<CmdSpec> {
        RstcauseclrW::new(self, 17)
    }
    #[doc = "Bit 18 - Tamper Reset Cause Clear"]
    #[inline(always)]
    pub fn tamperrcclr(&mut self) -> TamperrcclrW<CmdSpec> {
        TamperrcclrW::new(self, 18)
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
