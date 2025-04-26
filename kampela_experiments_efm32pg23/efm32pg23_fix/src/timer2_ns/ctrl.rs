#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Timer Rising Input Edge Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Risea {
    #[doc = "0: No action"]
    None = 0,
    #[doc = "1: Start counter without reload"]
    Start = 1,
    #[doc = "2: Stop counter without reload"]
    Stop = 2,
    #[doc = "3: Reload and start counter"]
    Reloadstart = 3,
}
impl From<Risea> for u8 {
    #[inline(always)]
    fn from(variant: Risea) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Risea {
    type Ux = u8;
}
impl crate::IsEnum for Risea {}
#[doc = "Field `RISEA` reader - Timer Rising Input Edge Action"]
pub type RiseaR = crate::FieldReader<Risea>;
impl RiseaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Risea {
        match self.bits {
            0 => Risea::None,
            1 => Risea::Start,
            2 => Risea::Stop,
            3 => Risea::Reloadstart,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Risea::None
    }
    #[doc = "Start counter without reload"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Risea::Start
    }
    #[doc = "Stop counter without reload"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Risea::Stop
    }
    #[doc = "Reload and start counter"]
    #[inline(always)]
    pub fn is_reloadstart(&self) -> bool {
        *self == Risea::Reloadstart
    }
}
#[doc = "Field `RISEA` writer - Timer Rising Input Edge Action"]
pub type RiseaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Risea, crate::Safe>;
impl<'a, REG> RiseaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Risea::None)
    }
    #[doc = "Start counter without reload"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Risea::Start)
    }
    #[doc = "Stop counter without reload"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Risea::Stop)
    }
    #[doc = "Reload and start counter"]
    #[inline(always)]
    pub fn reloadstart(self) -> &'a mut crate::W<REG> {
        self.variant(Risea::Reloadstart)
    }
}
#[doc = "Timer Falling Input Edge Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Falla {
    #[doc = "0: No action"]
    None = 0,
    #[doc = "1: Start counter without reload"]
    Start = 1,
    #[doc = "2: Stop counter without reload"]
    Stop = 2,
    #[doc = "3: Reload and start counter"]
    Reloadstart = 3,
}
impl From<Falla> for u8 {
    #[inline(always)]
    fn from(variant: Falla) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Falla {
    type Ux = u8;
}
impl crate::IsEnum for Falla {}
#[doc = "Field `FALLA` reader - Timer Falling Input Edge Action"]
pub type FallaR = crate::FieldReader<Falla>;
impl FallaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Falla {
        match self.bits {
            0 => Falla::None,
            1 => Falla::Start,
            2 => Falla::Stop,
            3 => Falla::Reloadstart,
            _ => unreachable!(),
        }
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Falla::None
    }
    #[doc = "Start counter without reload"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == Falla::Start
    }
    #[doc = "Stop counter without reload"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Falla::Stop
    }
    #[doc = "Reload and start counter"]
    #[inline(always)]
    pub fn is_reloadstart(&self) -> bool {
        *self == Falla::Reloadstart
    }
}
#[doc = "Field `FALLA` writer - Timer Falling Input Edge Action"]
pub type FallaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Falla, crate::Safe>;
impl<'a, REG> FallaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Falla::None)
    }
    #[doc = "Start counter without reload"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(Falla::Start)
    }
    #[doc = "Stop counter without reload"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Falla::Stop)
    }
    #[doc = "Reload and start counter"]
    #[inline(always)]
    pub fn reloadstart(self) -> &'a mut crate::W<REG> {
        self.variant(Falla::Reloadstart)
    }
}
#[doc = "Field `X2CNT` reader - 2x Count Mode"]
pub type X2cntR = crate::BitReader;
#[doc = "Field `X2CNT` writer - 2x Count Mode"]
pub type X2cntW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Timer Rising Input Edge Action"]
    #[inline(always)]
    pub fn risea(&self) -> RiseaR {
        RiseaR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Timer Falling Input Edge Action"]
    #[inline(always)]
    pub fn falla(&self) -> FallaR {
        FallaR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - 2x Count Mode"]
    #[inline(always)]
    pub fn x2cnt(&self) -> X2cntR {
        X2cntR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Rising Input Edge Action"]
    #[inline(always)]
    pub fn risea(&mut self) -> RiseaW<CtrlSpec> {
        RiseaW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Timer Falling Input Edge Action"]
    #[inline(always)]
    pub fn falla(&mut self) -> FallaW<CtrlSpec> {
        FallaW::new(self, 2)
    }
    #[doc = "Bit 4 - 2x Count Mode"]
    #[inline(always)]
    pub fn x2cnt(&mut self) -> X2cntW<CtrlSpec> {
        X2cntW::new(self, 4)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
