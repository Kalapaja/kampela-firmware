#[doc = "Register `CTRLX` reader"]
pub type R = crate::R<CtrlxSpec>;
#[doc = "Register `CTRLX` writer"]
pub type W = crate::W<CtrlxSpec>;
#[doc = "Debug halt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbghalt {
    #[doc = "0: Continue to transmit until TX buffer is empty"]
    Disable = 0,
    #[doc = "1: Negate RTS to stop link partner's transmission during debug HALT. NOTE** The core clock should be equal to or faster than the peripheral clock; otherwise, each single step could transmit multiple frames instead of just transmitting one frame."]
    Enable = 1,
}
impl From<Dbghalt> for bool {
    #[inline(always)]
    fn from(variant: Dbghalt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGHALT` reader - Debug halt"]
pub type DbghaltR = crate::BitReader<Dbghalt>;
impl DbghaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbghalt {
        match self.bits {
            false => Dbghalt::Disable,
            true => Dbghalt::Enable,
        }
    }
    #[doc = "Continue to transmit until TX buffer is empty"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Dbghalt::Disable
    }
    #[doc = "Negate RTS to stop link partner's transmission during debug HALT. NOTE** The core clock should be equal to or faster than the peripheral clock; otherwise, each single step could transmit multiple frames instead of just transmitting one frame."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Dbghalt::Enable
    }
}
#[doc = "Field `DBGHALT` writer - Debug halt"]
pub type DbghaltW<'a, REG> = crate::BitWriter<'a, REG, Dbghalt>;
impl<'a, REG> DbghaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue to transmit until TX buffer is empty"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Dbghalt::Disable)
    }
    #[doc = "Negate RTS to stop link partner's transmission during debug HALT. NOTE** The core clock should be equal to or faster than the peripheral clock; otherwise, each single step could transmit multiple frames instead of just transmitting one frame."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Dbghalt::Enable)
    }
}
#[doc = "CTS Pin Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsinv {
    #[doc = "0: The USn_CTS pin is low true"]
    Disable = 0,
    #[doc = "1: The USn_CTS pin is high true"]
    Enable = 1,
}
impl From<Ctsinv> for bool {
    #[inline(always)]
    fn from(variant: Ctsinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSINV` reader - CTS Pin Inversion"]
pub type CtsinvR = crate::BitReader<Ctsinv>;
impl CtsinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsinv {
        match self.bits {
            false => Ctsinv::Disable,
            true => Ctsinv::Enable,
        }
    }
    #[doc = "The USn_CTS pin is low true"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ctsinv::Disable
    }
    #[doc = "The USn_CTS pin is high true"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ctsinv::Enable
    }
}
#[doc = "Field `CTSINV` writer - CTS Pin Inversion"]
pub type CtsinvW<'a, REG> = crate::BitWriter<'a, REG, Ctsinv>;
impl<'a, REG> CtsinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The USn_CTS pin is low true"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsinv::Disable)
    }
    #[doc = "The USn_CTS pin is high true"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsinv::Enable)
    }
}
#[doc = "CTS Function enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctsen {
    #[doc = "0: Ingore CTS"]
    Disable = 0,
    #[doc = "1: Stop transmitting when CTS is negated"]
    Enable = 1,
}
impl From<Ctsen> for bool {
    #[inline(always)]
    fn from(variant: Ctsen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTSEN` reader - CTS Function enabled"]
pub type CtsenR = crate::BitReader<Ctsen>;
impl CtsenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctsen {
        match self.bits {
            false => Ctsen::Disable,
            true => Ctsen::Enable,
        }
    }
    #[doc = "Ingore CTS"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Ctsen::Disable
    }
    #[doc = "Stop transmitting when CTS is negated"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Ctsen::Enable
    }
}
#[doc = "Field `CTSEN` writer - CTS Function enabled"]
pub type CtsenW<'a, REG> = crate::BitWriter<'a, REG, Ctsen>;
impl<'a, REG> CtsenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Ingore CTS"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::Disable)
    }
    #[doc = "Stop transmitting when CTS is negated"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Ctsen::Enable)
    }
}
#[doc = "RTS Pin Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rtsinv {
    #[doc = "0: The USn_RTS pin is low true"]
    Disable = 0,
    #[doc = "1: The USn_RTS pin is high true"]
    Enable = 1,
}
impl From<Rtsinv> for bool {
    #[inline(always)]
    fn from(variant: Rtsinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTSINV` reader - RTS Pin Inversion"]
pub type RtsinvR = crate::BitReader<Rtsinv>;
impl RtsinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rtsinv {
        match self.bits {
            false => Rtsinv::Disable,
            true => Rtsinv::Enable,
        }
    }
    #[doc = "The USn_RTS pin is low true"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rtsinv::Disable
    }
    #[doc = "The USn_RTS pin is high true"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rtsinv::Enable
    }
}
#[doc = "Field `RTSINV` writer - RTS Pin Inversion"]
pub type RtsinvW<'a, REG> = crate::BitWriter<'a, REG, Rtsinv>;
impl<'a, REG> RtsinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The USn_RTS pin is low true"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsinv::Disable)
    }
    #[doc = "The USn_RTS pin is high true"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rtsinv::Enable)
    }
}
#[doc = "Field `RXPRSEN` reader - PRS RX Enable"]
pub type RxprsenR = crate::BitReader;
#[doc = "Field `RXPRSEN` writer - PRS RX Enable"]
pub type RxprsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKPRSEN` reader - PRS CLK Enable"]
pub type ClkprsenR = crate::BitReader;
#[doc = "Field `CLKPRSEN` writer - PRS CLK Enable"]
pub type ClkprsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Debug halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DbghaltR {
        DbghaltR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTS Pin Inversion"]
    #[inline(always)]
    pub fn ctsinv(&self) -> CtsinvR {
        CtsinvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTS Function enabled"]
    #[inline(always)]
    pub fn ctsen(&self) -> CtsenR {
        CtsenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTS Pin Inversion"]
    #[inline(always)]
    pub fn rtsinv(&self) -> RtsinvR {
        RtsinvR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprsen(&self) -> RxprsenR {
        RxprsenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - PRS CLK Enable"]
    #[inline(always)]
    pub fn clkprsen(&self) -> ClkprsenR {
        ClkprsenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug halt"]
    #[inline(always)]
    pub fn dbghalt(&mut self) -> DbghaltW<CtrlxSpec> {
        DbghaltW::new(self, 0)
    }
    #[doc = "Bit 1 - CTS Pin Inversion"]
    #[inline(always)]
    pub fn ctsinv(&mut self) -> CtsinvW<CtrlxSpec> {
        CtsinvW::new(self, 1)
    }
    #[doc = "Bit 2 - CTS Function enabled"]
    #[inline(always)]
    pub fn ctsen(&mut self) -> CtsenW<CtrlxSpec> {
        CtsenW::new(self, 2)
    }
    #[doc = "Bit 3 - RTS Pin Inversion"]
    #[inline(always)]
    pub fn rtsinv(&mut self) -> RtsinvW<CtrlxSpec> {
        RtsinvW::new(self, 3)
    }
    #[doc = "Bit 7 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprsen(&mut self) -> RxprsenW<CtrlxSpec> {
        RxprsenW::new(self, 7)
    }
    #[doc = "Bit 15 - PRS CLK Enable"]
    #[inline(always)]
    pub fn clkprsen(&mut self) -> ClkprsenW<CtrlxSpec> {
        ClkprsenW::new(self, 15)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrlx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrlx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlxSpec;
impl crate::RegisterSpec for CtrlxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlx::R`](R) reader structure"]
impl crate::Readable for CtrlxSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrlx::W`](W) writer structure"]
impl crate::Writable for CtrlxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRLX to value 0"]
impl crate::Resettable for CtrlxSpec {}
