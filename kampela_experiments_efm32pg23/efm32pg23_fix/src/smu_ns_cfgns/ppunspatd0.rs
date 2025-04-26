#[doc = "Register `PPUNSPATD0` reader"]
pub type R = crate::R<Ppunspatd0Spec>;
#[doc = "Register `PPUNSPATD0` writer"]
pub type W = crate::W<Ppunspatd0Spec>;
#[doc = "Field `SCRATCHPAD` reader - SCRATCHPAD Privileged Access"]
pub type ScratchpadR = crate::BitReader;
#[doc = "Field `SCRATCHPAD` writer - SCRATCHPAD Privileged Access"]
pub type ScratchpadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EMU` reader - EMU Privileged Access"]
pub type EmuR = crate::BitReader;
#[doc = "Field `EMU` writer - EMU Privileged Access"]
pub type EmuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMU` reader - CMU Privileged Access"]
pub type CmuR = crate::BitReader;
#[doc = "Field `CMU` writer - CMU Privileged Access"]
pub type CmuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFRCO0` reader - HFRCO0 Privileged Access"]
pub type Hfrco0R = crate::BitReader;
#[doc = "Field `HFRCO0` writer - HFRCO0 Privileged Access"]
pub type Hfrco0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSRCO` reader - FSRCO Privileged Access"]
pub type FsrcoR = crate::BitReader;
#[doc = "Field `FSRCO` writer - FSRCO Privileged Access"]
pub type FsrcoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPLL0` reader - DPLL0 Privileged Access"]
pub type Dpll0R = crate::BitReader;
#[doc = "Field `DPLL0` writer - DPLL0 Privileged Access"]
pub type Dpll0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFXO` reader - LFXO Privileged Access"]
pub type LfxoR = crate::BitReader;
#[doc = "Field `LFXO` writer - LFXO Privileged Access"]
pub type LfxoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFRCO` reader - LFRCO Privileged Access"]
pub type LfrcoR = crate::BitReader;
#[doc = "Field `LFRCO` writer - LFRCO Privileged Access"]
pub type LfrcoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULFRCO` reader - ULFRCO Privileged Access"]
pub type UlfrcoR = crate::BitReader;
#[doc = "Field `ULFRCO` writer - ULFRCO Privileged Access"]
pub type UlfrcoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSC` reader - MSC Privileged Access"]
pub type MscR = crate::BitReader;
#[doc = "Field `MSC` writer - MSC Privileged Access"]
pub type MscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICACHE0` reader - ICACHE0 Privileged Access"]
pub type Icache0R = crate::BitReader;
#[doc = "Field `ICACHE0` writer - ICACHE0 Privileged Access"]
pub type Icache0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRS` reader - PRS Privileged Access"]
pub type PrsR = crate::BitReader;
#[doc = "Field `PRS` writer - PRS Privileged Access"]
pub type PrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIO` reader - GPIO Privileged Access"]
pub type GpioR = crate::BitReader;
#[doc = "Field `GPIO` writer - GPIO Privileged Access"]
pub type GpioW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDMA` reader - LDMA Privileged Access"]
pub type LdmaR = crate::BitReader;
#[doc = "Field `LDMA` writer - LDMA Privileged Access"]
pub type LdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDMAXBAR` reader - LDMAXBAR Privileged Access"]
pub type LdmaxbarR = crate::BitReader;
#[doc = "Field `LDMAXBAR` writer - LDMAXBAR Privileged Access"]
pub type LdmaxbarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER0` reader - TIMER0 Privileged Access"]
pub type Timer0R = crate::BitReader;
#[doc = "Field `TIMER0` writer - TIMER0 Privileged Access"]
pub type Timer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER1` reader - TIMER1 Privileged Access"]
pub type Timer1R = crate::BitReader;
#[doc = "Field `TIMER1` writer - TIMER1 Privileged Access"]
pub type Timer1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER2` reader - TIMER2 Privileged Access"]
pub type Timer2R = crate::BitReader;
#[doc = "Field `TIMER2` writer - TIMER2 Privileged Access"]
pub type Timer2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER3` reader - TIMER3 Privileged Access"]
pub type Timer3R = crate::BitReader;
#[doc = "Field `TIMER3` writer - TIMER3 Privileged Access"]
pub type Timer3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER4` reader - TIMER4 Privileged Access"]
pub type Timer4R = crate::BitReader;
#[doc = "Field `TIMER4` writer - TIMER4 Privileged Access"]
pub type Timer4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART0` reader - USART0 Privileged Access"]
pub type Usart0R = crate::BitReader;
#[doc = "Field `USART0` writer - USART0 Privileged Access"]
pub type Usart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURTC` reader - BURTC Privileged Access"]
pub type BurtcR = crate::BitReader;
#[doc = "Field `BURTC` writer - BURTC Privileged Access"]
pub type BurtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1` reader - I2C1 Privileged Access"]
pub type I2c1R = crate::BitReader;
#[doc = "Field `I2C1` writer - I2C1 Privileged Access"]
pub type I2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHIPTESTCTRL` reader - CHIPTESTCTRL Privileged Access"]
pub type ChiptestctrlR = crate::BitReader;
#[doc = "Field `CHIPTESTCTRL` writer - CHIPTESTCTRL Privileged Access"]
pub type ChiptestctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFGCFGNS` reader - SYSCFGCFGNS Privileged Access"]
pub type SyscfgcfgnsR = crate::BitReader;
#[doc = "Field `SYSCFGCFGNS` writer - SYSCFGCFGNS Privileged Access"]
pub type SyscfgcfgnsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYSCFG` reader - SYSCFG Privileged Access"]
pub type SyscfgR = crate::BitReader;
#[doc = "Field `SYSCFG` writer - SYSCFG Privileged Access"]
pub type SyscfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BURAM` reader - BURAM Privileged Access"]
pub type BuramR = crate::BitReader;
#[doc = "Field `BURAM` writer - BURAM Privileged Access"]
pub type BuramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPCRC` reader - GPCRC Privileged Access"]
pub type GpcrcR = crate::BitReader;
#[doc = "Field `GPCRC` writer - GPCRC Privileged Access"]
pub type GpcrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC` reader - DCDC Privileged Access"]
pub type DcdcR = crate::BitReader;
#[doc = "Field `DCDC` writer - DCDC Privileged Access"]
pub type DcdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HOSTMAILBOX` reader - HOSTMAILBOX Privileged Access"]
pub type HostmailboxR = crate::BitReader;
#[doc = "Field `HOSTMAILBOX` writer - HOSTMAILBOX Privileged Access"]
pub type HostmailboxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EUSART1` reader - EUSART1 Privileged Access"]
pub type Eusart1R = crate::BitReader;
#[doc = "Field `EUSART1` writer - EUSART1 Privileged Access"]
pub type Eusart1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EUSART2` reader - EUSART2 Privileged Access"]
pub type Eusart2R = crate::BitReader;
#[doc = "Field `EUSART2` writer - EUSART2 Privileged Access"]
pub type Eusart2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCRATCHPAD Privileged Access"]
    #[inline(always)]
    pub fn scratchpad(&self) -> ScratchpadR {
        ScratchpadR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EMU Privileged Access"]
    #[inline(always)]
    pub fn emu(&self) -> EmuR {
        EmuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMU Privileged Access"]
    #[inline(always)]
    pub fn cmu(&self) -> CmuR {
        CmuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HFRCO0 Privileged Access"]
    #[inline(always)]
    pub fn hfrco0(&self) -> Hfrco0R {
        Hfrco0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FSRCO Privileged Access"]
    #[inline(always)]
    pub fn fsrco(&self) -> FsrcoR {
        FsrcoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DPLL0 Privileged Access"]
    #[inline(always)]
    pub fn dpll0(&self) -> Dpll0R {
        Dpll0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LFXO Privileged Access"]
    #[inline(always)]
    pub fn lfxo(&self) -> LfxoR {
        LfxoR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LFRCO Privileged Access"]
    #[inline(always)]
    pub fn lfrco(&self) -> LfrcoR {
        LfrcoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ULFRCO Privileged Access"]
    #[inline(always)]
    pub fn ulfrco(&self) -> UlfrcoR {
        UlfrcoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MSC Privileged Access"]
    #[inline(always)]
    pub fn msc(&self) -> MscR {
        MscR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ICACHE0 Privileged Access"]
    #[inline(always)]
    pub fn icache0(&self) -> Icache0R {
        Icache0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PRS Privileged Access"]
    #[inline(always)]
    pub fn prs(&self) -> PrsR {
        PrsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPIO Privileged Access"]
    #[inline(always)]
    pub fn gpio(&self) -> GpioR {
        GpioR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LDMA Privileged Access"]
    #[inline(always)]
    pub fn ldma(&self) -> LdmaR {
        LdmaR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LDMAXBAR Privileged Access"]
    #[inline(always)]
    pub fn ldmaxbar(&self) -> LdmaxbarR {
        LdmaxbarR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIMER0 Privileged Access"]
    #[inline(always)]
    pub fn timer0(&self) -> Timer0R {
        Timer0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIMER1 Privileged Access"]
    #[inline(always)]
    pub fn timer1(&self) -> Timer1R {
        Timer1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIMER2 Privileged Access"]
    #[inline(always)]
    pub fn timer2(&self) -> Timer2R {
        Timer2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIMER3 Privileged Access"]
    #[inline(always)]
    pub fn timer3(&self) -> Timer3R {
        Timer3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - TIMER4 Privileged Access"]
    #[inline(always)]
    pub fn timer4(&self) -> Timer4R {
        Timer4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - USART0 Privileged Access"]
    #[inline(always)]
    pub fn usart0(&self) -> Usart0R {
        Usart0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BURTC Privileged Access"]
    #[inline(always)]
    pub fn burtc(&self) -> BurtcR {
        BurtcR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C1 Privileged Access"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2c1R {
        I2c1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - CHIPTESTCTRL Privileged Access"]
    #[inline(always)]
    pub fn chiptestctrl(&self) -> ChiptestctrlR {
        ChiptestctrlR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SYSCFGCFGNS Privileged Access"]
    #[inline(always)]
    pub fn syscfgcfgns(&self) -> SyscfgcfgnsR {
        SyscfgcfgnsR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SYSCFG Privileged Access"]
    #[inline(always)]
    pub fn syscfg(&self) -> SyscfgR {
        SyscfgR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - BURAM Privileged Access"]
    #[inline(always)]
    pub fn buram(&self) -> BuramR {
        BuramR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - GPCRC Privileged Access"]
    #[inline(always)]
    pub fn gpcrc(&self) -> GpcrcR {
        GpcrcR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DCDC Privileged Access"]
    #[inline(always)]
    pub fn dcdc(&self) -> DcdcR {
        DcdcR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - HOSTMAILBOX Privileged Access"]
    #[inline(always)]
    pub fn hostmailbox(&self) -> HostmailboxR {
        HostmailboxR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - EUSART1 Privileged Access"]
    #[inline(always)]
    pub fn eusart1(&self) -> Eusart1R {
        Eusart1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - EUSART2 Privileged Access"]
    #[inline(always)]
    pub fn eusart2(&self) -> Eusart2R {
        Eusart2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCRATCHPAD Privileged Access"]
    #[inline(always)]
    pub fn scratchpad(&mut self) -> ScratchpadW<Ppunspatd0Spec> {
        ScratchpadW::new(self, 0)
    }
    #[doc = "Bit 1 - EMU Privileged Access"]
    #[inline(always)]
    pub fn emu(&mut self) -> EmuW<Ppunspatd0Spec> {
        EmuW::new(self, 1)
    }
    #[doc = "Bit 2 - CMU Privileged Access"]
    #[inline(always)]
    pub fn cmu(&mut self) -> CmuW<Ppunspatd0Spec> {
        CmuW::new(self, 2)
    }
    #[doc = "Bit 3 - HFRCO0 Privileged Access"]
    #[inline(always)]
    pub fn hfrco0(&mut self) -> Hfrco0W<Ppunspatd0Spec> {
        Hfrco0W::new(self, 3)
    }
    #[doc = "Bit 4 - FSRCO Privileged Access"]
    #[inline(always)]
    pub fn fsrco(&mut self) -> FsrcoW<Ppunspatd0Spec> {
        FsrcoW::new(self, 4)
    }
    #[doc = "Bit 5 - DPLL0 Privileged Access"]
    #[inline(always)]
    pub fn dpll0(&mut self) -> Dpll0W<Ppunspatd0Spec> {
        Dpll0W::new(self, 5)
    }
    #[doc = "Bit 6 - LFXO Privileged Access"]
    #[inline(always)]
    pub fn lfxo(&mut self) -> LfxoW<Ppunspatd0Spec> {
        LfxoW::new(self, 6)
    }
    #[doc = "Bit 7 - LFRCO Privileged Access"]
    #[inline(always)]
    pub fn lfrco(&mut self) -> LfrcoW<Ppunspatd0Spec> {
        LfrcoW::new(self, 7)
    }
    #[doc = "Bit 8 - ULFRCO Privileged Access"]
    #[inline(always)]
    pub fn ulfrco(&mut self) -> UlfrcoW<Ppunspatd0Spec> {
        UlfrcoW::new(self, 8)
    }
    #[doc = "Bit 9 - MSC Privileged Access"]
    #[inline(always)]
    pub fn msc(&mut self) -> MscW<Ppunspatd0Spec> {
        MscW::new(self, 9)
    }
    #[doc = "Bit 10 - ICACHE0 Privileged Access"]
    #[inline(always)]
    pub fn icache0(&mut self) -> Icache0W<Ppunspatd0Spec> {
        Icache0W::new(self, 10)
    }
    #[doc = "Bit 11 - PRS Privileged Access"]
    #[inline(always)]
    pub fn prs(&mut self) -> PrsW<Ppunspatd0Spec> {
        PrsW::new(self, 11)
    }
    #[doc = "Bit 12 - GPIO Privileged Access"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GpioW<Ppunspatd0Spec> {
        GpioW::new(self, 12)
    }
    #[doc = "Bit 13 - LDMA Privileged Access"]
    #[inline(always)]
    pub fn ldma(&mut self) -> LdmaW<Ppunspatd0Spec> {
        LdmaW::new(self, 13)
    }
    #[doc = "Bit 14 - LDMAXBAR Privileged Access"]
    #[inline(always)]
    pub fn ldmaxbar(&mut self) -> LdmaxbarW<Ppunspatd0Spec> {
        LdmaxbarW::new(self, 14)
    }
    #[doc = "Bit 15 - TIMER0 Privileged Access"]
    #[inline(always)]
    pub fn timer0(&mut self) -> Timer0W<Ppunspatd0Spec> {
        Timer0W::new(self, 15)
    }
    #[doc = "Bit 16 - TIMER1 Privileged Access"]
    #[inline(always)]
    pub fn timer1(&mut self) -> Timer1W<Ppunspatd0Spec> {
        Timer1W::new(self, 16)
    }
    #[doc = "Bit 17 - TIMER2 Privileged Access"]
    #[inline(always)]
    pub fn timer2(&mut self) -> Timer2W<Ppunspatd0Spec> {
        Timer2W::new(self, 17)
    }
    #[doc = "Bit 18 - TIMER3 Privileged Access"]
    #[inline(always)]
    pub fn timer3(&mut self) -> Timer3W<Ppunspatd0Spec> {
        Timer3W::new(self, 18)
    }
    #[doc = "Bit 19 - TIMER4 Privileged Access"]
    #[inline(always)]
    pub fn timer4(&mut self) -> Timer4W<Ppunspatd0Spec> {
        Timer4W::new(self, 19)
    }
    #[doc = "Bit 20 - USART0 Privileged Access"]
    #[inline(always)]
    pub fn usart0(&mut self) -> Usart0W<Ppunspatd0Spec> {
        Usart0W::new(self, 20)
    }
    #[doc = "Bit 21 - BURTC Privileged Access"]
    #[inline(always)]
    pub fn burtc(&mut self) -> BurtcW<Ppunspatd0Spec> {
        BurtcW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C1 Privileged Access"]
    #[inline(always)]
    pub fn i2c1(&mut self) -> I2c1W<Ppunspatd0Spec> {
        I2c1W::new(self, 22)
    }
    #[doc = "Bit 23 - CHIPTESTCTRL Privileged Access"]
    #[inline(always)]
    pub fn chiptestctrl(&mut self) -> ChiptestctrlW<Ppunspatd0Spec> {
        ChiptestctrlW::new(self, 23)
    }
    #[doc = "Bit 24 - SYSCFGCFGNS Privileged Access"]
    #[inline(always)]
    pub fn syscfgcfgns(&mut self) -> SyscfgcfgnsW<Ppunspatd0Spec> {
        SyscfgcfgnsW::new(self, 24)
    }
    #[doc = "Bit 25 - SYSCFG Privileged Access"]
    #[inline(always)]
    pub fn syscfg(&mut self) -> SyscfgW<Ppunspatd0Spec> {
        SyscfgW::new(self, 25)
    }
    #[doc = "Bit 26 - BURAM Privileged Access"]
    #[inline(always)]
    pub fn buram(&mut self) -> BuramW<Ppunspatd0Spec> {
        BuramW::new(self, 26)
    }
    #[doc = "Bit 27 - GPCRC Privileged Access"]
    #[inline(always)]
    pub fn gpcrc(&mut self) -> GpcrcW<Ppunspatd0Spec> {
        GpcrcW::new(self, 27)
    }
    #[doc = "Bit 28 - DCDC Privileged Access"]
    #[inline(always)]
    pub fn dcdc(&mut self) -> DcdcW<Ppunspatd0Spec> {
        DcdcW::new(self, 28)
    }
    #[doc = "Bit 29 - HOSTMAILBOX Privileged Access"]
    #[inline(always)]
    pub fn hostmailbox(&mut self) -> HostmailboxW<Ppunspatd0Spec> {
        HostmailboxW::new(self, 29)
    }
    #[doc = "Bit 30 - EUSART1 Privileged Access"]
    #[inline(always)]
    pub fn eusart1(&mut self) -> Eusart1W<Ppunspatd0Spec> {
        Eusart1W::new(self, 30)
    }
    #[doc = "Bit 31 - EUSART2 Privileged Access"]
    #[inline(always)]
    pub fn eusart2(&mut self) -> Eusart2W<Ppunspatd0Spec> {
        Eusart2W::new(self, 31)
    }
}
#[doc = "Set peripheral bits to 1 to mark as privileged access only\n\nYou can [`read`](crate::Reg::read) this register and get [`ppunspatd0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppunspatd0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ppunspatd0Spec;
impl crate::RegisterSpec for Ppunspatd0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppunspatd0::R`](R) reader structure"]
impl crate::Readable for Ppunspatd0Spec {}
#[doc = "`write(|w| ..)` method takes [`ppunspatd0::W`](W) writer structure"]
impl crate::Writable for Ppunspatd0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPUNSPATD0 to value 0"]
impl crate::Resettable for Ppunspatd0Spec {}
