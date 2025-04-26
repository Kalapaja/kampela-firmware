#[doc = "Register `PPUNSPATD1` reader"]
pub type R = crate::R<Ppunspatd1Spec>;
#[doc = "Register `PPUNSPATD1` writer"]
pub type W = crate::W<Ppunspatd1Spec>;
#[doc = "Field `SYSRTC` reader - SYSRTC Privileged Access"]
pub type SysrtcR = crate::BitReader;
#[doc = "Field `SYSRTC` writer - SYSRTC Privileged Access"]
pub type SysrtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD` reader - LCD Privileged Access"]
pub type LcdR = crate::BitReader;
#[doc = "Field `LCD` writer - LCD Privileged Access"]
pub type LcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEYSCAN` reader - KEYSCAN Privileged Access"]
pub type KeyscanR = crate::BitReader;
#[doc = "Field `KEYSCAN` writer - KEYSCAN Privileged Access"]
pub type KeyscanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMEM` reader - DMEM Privileged Access"]
pub type DmemR = crate::BitReader;
#[doc = "Field `DMEM` writer - DMEM Privileged Access"]
pub type DmemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCDRF` reader - LCDRF Privileged Access"]
pub type LcdrfR = crate::BitReader;
#[doc = "Field `LCDRF` writer - LCDRF Privileged Access"]
pub type LcdrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMU` reader - SMU Privileged Access"]
pub type SmuR = crate::BitReader;
#[doc = "Field `SMU` writer - SMU Privileged Access"]
pub type SmuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMUCFGNS` reader - SMUCFGNS Privileged Access"]
pub type SmucfgnsR = crate::BitReader;
#[doc = "Field `SMUCFGNS` writer - SMUCFGNS Privileged Access"]
pub type SmucfgnsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LETIMER0` reader - LETIMER0 Privileged Access"]
pub type Letimer0R = crate::BitReader;
#[doc = "Field `LETIMER0` writer - LETIMER0 Privileged Access"]
pub type Letimer0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IADC0` reader - IADC0 Privileged Access"]
pub type Iadc0R = crate::BitReader;
#[doc = "Field `IADC0` writer - IADC0 Privileged Access"]
pub type Iadc0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP0` reader - ACMP0 Privileged Access"]
pub type Acmp0R = crate::BitReader;
#[doc = "Field `ACMP0` writer - ACMP0 Privileged Access"]
pub type Acmp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMP1` reader - ACMP1 Privileged Access"]
pub type Acmp1R = crate::BitReader;
#[doc = "Field `ACMP1` writer - ACMP1 Privileged Access"]
pub type Acmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMUXCP0` reader - AMUXCP0 Privileged Access"]
pub type Amuxcp0R = crate::BitReader;
#[doc = "Field `AMUXCP0` writer - AMUXCP0 Privileged Access"]
pub type Amuxcp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDAC0` reader - VDAC0 Privileged Access"]
pub type Vdac0R = crate::BitReader;
#[doc = "Field `VDAC0` writer - VDAC0 Privileged Access"]
pub type Vdac0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCNT` reader - PCNT Privileged Access"]
pub type PcntR = crate::BitReader;
#[doc = "Field `PCNT` writer - PCNT Privileged Access"]
pub type PcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LESENSE` reader - LESENSE Privileged Access"]
pub type LesenseR = crate::BitReader;
#[doc = "Field `LESENSE` writer - LESENSE Privileged Access"]
pub type LesenseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFRCO1` reader - HFRCO1 Privileged Access"]
pub type Hfrco1R = crate::BitReader;
#[doc = "Field `HFRCO1` writer - HFRCO1 Privileged Access"]
pub type Hfrco1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFXO0` reader - HFXO0 Privileged Access"]
pub type Hfxo0R = crate::BitReader;
#[doc = "Field `HFXO0` writer - HFXO0 Privileged Access"]
pub type Hfxo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C0` reader - I2C0 Privileged Access"]
pub type I2c0R = crate::BitReader;
#[doc = "Field `I2C0` writer - I2C0 Privileged Access"]
pub type I2c0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG0` reader - WDOG0 Privileged Access"]
pub type Wdog0R = crate::BitReader;
#[doc = "Field `WDOG0` writer - WDOG0 Privileged Access"]
pub type Wdog0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDOG1` reader - WDOG1 Privileged Access"]
pub type Wdog1R = crate::BitReader;
#[doc = "Field `WDOG1` writer - WDOG1 Privileged Access"]
pub type Wdog1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EUSART0` reader - EUSART0 Privileged Access"]
pub type Eusart0R = crate::BitReader;
#[doc = "Field `EUSART0` writer - EUSART0 Privileged Access"]
pub type Eusart0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEMAILBOX` reader - SEMAILBOX Privileged Access"]
pub type SemailboxR = crate::BitReader;
#[doc = "Field `SEMAILBOX` writer - SEMAILBOX Privileged Access"]
pub type SemailboxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSRTC Privileged Access"]
    #[inline(always)]
    pub fn sysrtc(&self) -> SysrtcR {
        SysrtcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LCD Privileged Access"]
    #[inline(always)]
    pub fn lcd(&self) -> LcdR {
        LcdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - KEYSCAN Privileged Access"]
    #[inline(always)]
    pub fn keyscan(&self) -> KeyscanR {
        KeyscanR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMEM Privileged Access"]
    #[inline(always)]
    pub fn dmem(&self) -> DmemR {
        DmemR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCDRF Privileged Access"]
    #[inline(always)]
    pub fn lcdrf(&self) -> LcdrfR {
        LcdrfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - SMU Privileged Access"]
    #[inline(always)]
    pub fn smu(&self) -> SmuR {
        SmuR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SMUCFGNS Privileged Access"]
    #[inline(always)]
    pub fn smucfgns(&self) -> SmucfgnsR {
        SmucfgnsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LETIMER0 Privileged Access"]
    #[inline(always)]
    pub fn letimer0(&self) -> Letimer0R {
        Letimer0R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - IADC0 Privileged Access"]
    #[inline(always)]
    pub fn iadc0(&self) -> Iadc0R {
        Iadc0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ACMP0 Privileged Access"]
    #[inline(always)]
    pub fn acmp0(&self) -> Acmp0R {
        Acmp0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ACMP1 Privileged Access"]
    #[inline(always)]
    pub fn acmp1(&self) -> Acmp1R {
        Acmp1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AMUXCP0 Privileged Access"]
    #[inline(always)]
    pub fn amuxcp0(&self) -> Amuxcp0R {
        Amuxcp0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - VDAC0 Privileged Access"]
    #[inline(always)]
    pub fn vdac0(&self) -> Vdac0R {
        Vdac0R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PCNT Privileged Access"]
    #[inline(always)]
    pub fn pcnt(&self) -> PcntR {
        PcntR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - LESENSE Privileged Access"]
    #[inline(always)]
    pub fn lesense(&self) -> LesenseR {
        LesenseR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HFRCO1 Privileged Access"]
    #[inline(always)]
    pub fn hfrco1(&self) -> Hfrco1R {
        Hfrco1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HFXO0 Privileged Access"]
    #[inline(always)]
    pub fn hfxo0(&self) -> Hfxo0R {
        Hfxo0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - I2C0 Privileged Access"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2c0R {
        I2c0R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - WDOG0 Privileged Access"]
    #[inline(always)]
    pub fn wdog0(&self) -> Wdog0R {
        Wdog0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WDOG1 Privileged Access"]
    #[inline(always)]
    pub fn wdog1(&self) -> Wdog1R {
        Wdog1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EUSART0 Privileged Access"]
    #[inline(always)]
    pub fn eusart0(&self) -> Eusart0R {
        Eusart0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SEMAILBOX Privileged Access"]
    #[inline(always)]
    pub fn semailbox(&self) -> SemailboxR {
        SemailboxR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSRTC Privileged Access"]
    #[inline(always)]
    pub fn sysrtc(&mut self) -> SysrtcW<Ppunspatd1Spec> {
        SysrtcW::new(self, 0)
    }
    #[doc = "Bit 1 - LCD Privileged Access"]
    #[inline(always)]
    pub fn lcd(&mut self) -> LcdW<Ppunspatd1Spec> {
        LcdW::new(self, 1)
    }
    #[doc = "Bit 2 - KEYSCAN Privileged Access"]
    #[inline(always)]
    pub fn keyscan(&mut self) -> KeyscanW<Ppunspatd1Spec> {
        KeyscanW::new(self, 2)
    }
    #[doc = "Bit 3 - DMEM Privileged Access"]
    #[inline(always)]
    pub fn dmem(&mut self) -> DmemW<Ppunspatd1Spec> {
        DmemW::new(self, 3)
    }
    #[doc = "Bit 4 - LCDRF Privileged Access"]
    #[inline(always)]
    pub fn lcdrf(&mut self) -> LcdrfW<Ppunspatd1Spec> {
        LcdrfW::new(self, 4)
    }
    #[doc = "Bit 7 - SMU Privileged Access"]
    #[inline(always)]
    pub fn smu(&mut self) -> SmuW<Ppunspatd1Spec> {
        SmuW::new(self, 7)
    }
    #[doc = "Bit 8 - SMUCFGNS Privileged Access"]
    #[inline(always)]
    pub fn smucfgns(&mut self) -> SmucfgnsW<Ppunspatd1Spec> {
        SmucfgnsW::new(self, 8)
    }
    #[doc = "Bit 9 - LETIMER0 Privileged Access"]
    #[inline(always)]
    pub fn letimer0(&mut self) -> Letimer0W<Ppunspatd1Spec> {
        Letimer0W::new(self, 9)
    }
    #[doc = "Bit 10 - IADC0 Privileged Access"]
    #[inline(always)]
    pub fn iadc0(&mut self) -> Iadc0W<Ppunspatd1Spec> {
        Iadc0W::new(self, 10)
    }
    #[doc = "Bit 11 - ACMP0 Privileged Access"]
    #[inline(always)]
    pub fn acmp0(&mut self) -> Acmp0W<Ppunspatd1Spec> {
        Acmp0W::new(self, 11)
    }
    #[doc = "Bit 12 - ACMP1 Privileged Access"]
    #[inline(always)]
    pub fn acmp1(&mut self) -> Acmp1W<Ppunspatd1Spec> {
        Acmp1W::new(self, 12)
    }
    #[doc = "Bit 13 - AMUXCP0 Privileged Access"]
    #[inline(always)]
    pub fn amuxcp0(&mut self) -> Amuxcp0W<Ppunspatd1Spec> {
        Amuxcp0W::new(self, 13)
    }
    #[doc = "Bit 14 - VDAC0 Privileged Access"]
    #[inline(always)]
    pub fn vdac0(&mut self) -> Vdac0W<Ppunspatd1Spec> {
        Vdac0W::new(self, 14)
    }
    #[doc = "Bit 15 - PCNT Privileged Access"]
    #[inline(always)]
    pub fn pcnt(&mut self) -> PcntW<Ppunspatd1Spec> {
        PcntW::new(self, 15)
    }
    #[doc = "Bit 16 - LESENSE Privileged Access"]
    #[inline(always)]
    pub fn lesense(&mut self) -> LesenseW<Ppunspatd1Spec> {
        LesenseW::new(self, 16)
    }
    #[doc = "Bit 17 - HFRCO1 Privileged Access"]
    #[inline(always)]
    pub fn hfrco1(&mut self) -> Hfrco1W<Ppunspatd1Spec> {
        Hfrco1W::new(self, 17)
    }
    #[doc = "Bit 18 - HFXO0 Privileged Access"]
    #[inline(always)]
    pub fn hfxo0(&mut self) -> Hfxo0W<Ppunspatd1Spec> {
        Hfxo0W::new(self, 18)
    }
    #[doc = "Bit 19 - I2C0 Privileged Access"]
    #[inline(always)]
    pub fn i2c0(&mut self) -> I2c0W<Ppunspatd1Spec> {
        I2c0W::new(self, 19)
    }
    #[doc = "Bit 20 - WDOG0 Privileged Access"]
    #[inline(always)]
    pub fn wdog0(&mut self) -> Wdog0W<Ppunspatd1Spec> {
        Wdog0W::new(self, 20)
    }
    #[doc = "Bit 21 - WDOG1 Privileged Access"]
    #[inline(always)]
    pub fn wdog1(&mut self) -> Wdog1W<Ppunspatd1Spec> {
        Wdog1W::new(self, 21)
    }
    #[doc = "Bit 22 - EUSART0 Privileged Access"]
    #[inline(always)]
    pub fn eusart0(&mut self) -> Eusart0W<Ppunspatd1Spec> {
        Eusart0W::new(self, 22)
    }
    #[doc = "Bit 23 - SEMAILBOX Privileged Access"]
    #[inline(always)]
    pub fn semailbox(&mut self) -> SemailboxW<Ppunspatd1Spec> {
        SemailboxW::new(self, 23)
    }
}
#[doc = "Set peripheral bits to 1 to mark as privileged access only\n\nYou can [`read`](crate::Reg::read) this register and get [`ppunspatd1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppunspatd1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ppunspatd1Spec;
impl crate::RegisterSpec for Ppunspatd1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppunspatd1::R`](R) reader structure"]
impl crate::Readable for Ppunspatd1Spec {}
#[doc = "`write(|w| ..)` method takes [`ppunspatd1::W`](W) writer structure"]
impl crate::Writable for Ppunspatd1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPUNSPATD1 to value 0"]
impl crate::Resettable for Ppunspatd1Spec {}
