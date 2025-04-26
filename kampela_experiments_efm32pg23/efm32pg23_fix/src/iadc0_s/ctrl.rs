#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "EM23 Wakeup on Conversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Em23wuconvert {
    #[doc = "0: When using suspend mode, conversions performed in EM2 or EM3 should not wake up the DMA until the FIFO's DVL setting is reached. This saves more power for large OSR settings or infrequent sampling."]
    Wudvl = 0,
    #[doc = "1: When using suspend mode, conversions performed in EM2 or EM3 will wake up the DMA and keep it awake until the conversions are done, regardless of the DVL setting. This mode burns more power, but it is useful when the conversion rate is faster than the time for the DMA to cycle through wake up and going back to sleep as it converts more than 4 scan table entries. Without using the wake up on conversion mode, the FIFO may overflow while the DMA is going in and out of sleep."]
    Wuconvert = 1,
}
impl From<Em23wuconvert> for bool {
    #[inline(always)]
    fn from(variant: Em23wuconvert) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM23WUCONVERT` reader - EM23 Wakeup on Conversion"]
pub type Em23wuconvertR = crate::BitReader<Em23wuconvert>;
impl Em23wuconvertR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Em23wuconvert {
        match self.bits {
            false => Em23wuconvert::Wudvl,
            true => Em23wuconvert::Wuconvert,
        }
    }
    #[doc = "When using suspend mode, conversions performed in EM2 or EM3 should not wake up the DMA until the FIFO's DVL setting is reached. This saves more power for large OSR settings or infrequent sampling."]
    #[inline(always)]
    pub fn is_wudvl(&self) -> bool {
        *self == Em23wuconvert::Wudvl
    }
    #[doc = "When using suspend mode, conversions performed in EM2 or EM3 will wake up the DMA and keep it awake until the conversions are done, regardless of the DVL setting. This mode burns more power, but it is useful when the conversion rate is faster than the time for the DMA to cycle through wake up and going back to sleep as it converts more than 4 scan table entries. Without using the wake up on conversion mode, the FIFO may overflow while the DMA is going in and out of sleep."]
    #[inline(always)]
    pub fn is_wuconvert(&self) -> bool {
        *self == Em23wuconvert::Wuconvert
    }
}
#[doc = "Field `EM23WUCONVERT` writer - EM23 Wakeup on Conversion"]
pub type Em23wuconvertW<'a, REG> = crate::BitWriter<'a, REG, Em23wuconvert>;
impl<'a, REG> Em23wuconvertW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When using suspend mode, conversions performed in EM2 or EM3 should not wake up the DMA until the FIFO's DVL setting is reached. This saves more power for large OSR settings or infrequent sampling."]
    #[inline(always)]
    pub fn wudvl(self) -> &'a mut crate::W<REG> {
        self.variant(Em23wuconvert::Wudvl)
    }
    #[doc = "When using suspend mode, conversions performed in EM2 or EM3 will wake up the DMA and keep it awake until the conversions are done, regardless of the DVL setting. This mode burns more power, but it is useful when the conversion rate is faster than the time for the DMA to cycle through wake up and going back to sleep as it converts more than 4 scan table entries. Without using the wake up on conversion mode, the FIFO may overflow while the DMA is going in and out of sleep."]
    #[inline(always)]
    pub fn wuconvert(self) -> &'a mut crate::W<REG> {
        self.variant(Em23wuconvert::Wuconvert)
    }
}
#[doc = "ADC_CLK Suspend - PRS0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcclksuspend0 {
    #[doc = "0: Normal mode which does not disable the ADC_CLK."]
    Prswudis = 0,
    #[doc = "1: ADCCLKWUEN will gate off ADC_CLK until the trigger is detected provided the internal timer is not selected as the trigger. Once the trigger is detected the ADC_CLK will be started, the band gap will be started, the ADC will be warmed up, and the SCAN Table and the Single entry will be converted. Once the conversions are done, the ADC_CLK will be gated off."]
    Prswuen = 1,
}
impl From<Adcclksuspend0> for bool {
    #[inline(always)]
    fn from(variant: Adcclksuspend0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCCLKSUSPEND0` reader - ADC_CLK Suspend - PRS0"]
pub type Adcclksuspend0R = crate::BitReader<Adcclksuspend0>;
impl Adcclksuspend0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcclksuspend0 {
        match self.bits {
            false => Adcclksuspend0::Prswudis,
            true => Adcclksuspend0::Prswuen,
        }
    }
    #[doc = "Normal mode which does not disable the ADC_CLK."]
    #[inline(always)]
    pub fn is_prswudis(&self) -> bool {
        *self == Adcclksuspend0::Prswudis
    }
    #[doc = "ADCCLKWUEN will gate off ADC_CLK until the trigger is detected provided the internal timer is not selected as the trigger. Once the trigger is detected the ADC_CLK will be started, the band gap will be started, the ADC will be warmed up, and the SCAN Table and the Single entry will be converted. Once the conversions are done, the ADC_CLK will be gated off."]
    #[inline(always)]
    pub fn is_prswuen(&self) -> bool {
        *self == Adcclksuspend0::Prswuen
    }
}
#[doc = "Field `ADCCLKSUSPEND0` writer - ADC_CLK Suspend - PRS0"]
pub type Adcclksuspend0W<'a, REG> = crate::BitWriter<'a, REG, Adcclksuspend0>;
impl<'a, REG> Adcclksuspend0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode which does not disable the ADC_CLK."]
    #[inline(always)]
    pub fn prswudis(self) -> &'a mut crate::W<REG> {
        self.variant(Adcclksuspend0::Prswudis)
    }
    #[doc = "ADCCLKWUEN will gate off ADC_CLK until the trigger is detected provided the internal timer is not selected as the trigger. Once the trigger is detected the ADC_CLK will be started, the band gap will be started, the ADC will be warmed up, and the SCAN Table and the Single entry will be converted. Once the conversions are done, the ADC_CLK will be gated off."]
    #[inline(always)]
    pub fn prswuen(self) -> &'a mut crate::W<REG> {
        self.variant(Adcclksuspend0::Prswuen)
    }
}
#[doc = "ADC_CLK Suspend - PRS1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adcclksuspend1 {
    #[doc = "0: Normal mode which does not disable the ADC_CLK."]
    Prswudis = 0,
    #[doc = "1: ADCCLKWUEN will gate off ADC_CLK until the trigger is detected provided the internal timer is not selected as the trigger. Once the trigger is detected the ADC_CLK will be started, the band gap will be started, the ADC will be warmed up, and the SCAN Table and the Single entry will be converted. Once the conversions are done, the ADC_CLK will be gated off."]
    Prswuen = 1,
}
impl From<Adcclksuspend1> for bool {
    #[inline(always)]
    fn from(variant: Adcclksuspend1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCCLKSUSPEND1` reader - ADC_CLK Suspend - PRS1"]
pub type Adcclksuspend1R = crate::BitReader<Adcclksuspend1>;
impl Adcclksuspend1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adcclksuspend1 {
        match self.bits {
            false => Adcclksuspend1::Prswudis,
            true => Adcclksuspend1::Prswuen,
        }
    }
    #[doc = "Normal mode which does not disable the ADC_CLK."]
    #[inline(always)]
    pub fn is_prswudis(&self) -> bool {
        *self == Adcclksuspend1::Prswudis
    }
    #[doc = "ADCCLKWUEN will gate off ADC_CLK until the trigger is detected provided the internal timer is not selected as the trigger. Once the trigger is detected the ADC_CLK will be started, the band gap will be started, the ADC will be warmed up, and the SCAN Table and the Single entry will be converted. Once the conversions are done, the ADC_CLK will be gated off."]
    #[inline(always)]
    pub fn is_prswuen(&self) -> bool {
        *self == Adcclksuspend1::Prswuen
    }
}
#[doc = "Field `ADCCLKSUSPEND1` writer - ADC_CLK Suspend - PRS1"]
pub type Adcclksuspend1W<'a, REG> = crate::BitWriter<'a, REG, Adcclksuspend1>;
impl<'a, REG> Adcclksuspend1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode which does not disable the ADC_CLK."]
    #[inline(always)]
    pub fn prswudis(self) -> &'a mut crate::W<REG> {
        self.variant(Adcclksuspend1::Prswudis)
    }
    #[doc = "ADCCLKWUEN will gate off ADC_CLK until the trigger is detected provided the internal timer is not selected as the trigger. Once the trigger is detected the ADC_CLK will be started, the band gap will be started, the ADC will be warmed up, and the SCAN Table and the Single entry will be converted. Once the conversions are done, the ADC_CLK will be gated off."]
    #[inline(always)]
    pub fn prswuen(self) -> &'a mut crate::W<REG> {
        self.variant(Adcclksuspend1::Prswuen)
    }
}
#[doc = "Debug Halt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dbghalt {
    #[doc = "0: Continue operation as normal during debug mode"]
    Normal = 0,
    #[doc = "1: Complete the current conversion and then halt during debug mode"]
    Halt = 1,
}
impl From<Dbghalt> for bool {
    #[inline(always)]
    fn from(variant: Dbghalt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBGHALT` reader - Debug Halt"]
pub type DbghaltR = crate::BitReader<Dbghalt>;
impl DbghaltR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dbghalt {
        match self.bits {
            false => Dbghalt::Normal,
            true => Dbghalt::Halt,
        }
    }
    #[doc = "Continue operation as normal during debug mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Dbghalt::Normal
    }
    #[doc = "Complete the current conversion and then halt during debug mode"]
    #[inline(always)]
    pub fn is_halt(&self) -> bool {
        *self == Dbghalt::Halt
    }
}
#[doc = "Field `DBGHALT` writer - Debug Halt"]
pub type DbghaltW<'a, REG> = crate::BitWriter<'a, REG, Dbghalt>;
impl<'a, REG> DbghaltW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Continue operation as normal during debug mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Dbghalt::Normal)
    }
    #[doc = "Complete the current conversion and then halt during debug mode"]
    #[inline(always)]
    pub fn halt(self) -> &'a mut crate::W<REG> {
        self.variant(Dbghalt::Halt)
    }
}
#[doc = "Warmup Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Warmupmode {
    #[doc = "0: Shut down the IADC after conversions have completed."]
    Normal = 0,
    #[doc = "1: Switch to standby mode after conversions have completed. The next warmup time will require 1us."]
    Keepinstandby = 1,
    #[doc = "2: Keep IADC fully powered after conversions have completed."]
    Keepwarm = 2,
}
impl From<Warmupmode> for u8 {
    #[inline(always)]
    fn from(variant: Warmupmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Warmupmode {
    type Ux = u8;
}
impl crate::IsEnum for Warmupmode {}
#[doc = "Field `WARMUPMODE` reader - Warmup Mode"]
pub type WarmupmodeR = crate::FieldReader<Warmupmode>;
impl WarmupmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Warmupmode> {
        match self.bits {
            0 => Some(Warmupmode::Normal),
            1 => Some(Warmupmode::Keepinstandby),
            2 => Some(Warmupmode::Keepwarm),
            _ => None,
        }
    }
    #[doc = "Shut down the IADC after conversions have completed."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Warmupmode::Normal
    }
    #[doc = "Switch to standby mode after conversions have completed. The next warmup time will require 1us."]
    #[inline(always)]
    pub fn is_keepinstandby(&self) -> bool {
        *self == Warmupmode::Keepinstandby
    }
    #[doc = "Keep IADC fully powered after conversions have completed."]
    #[inline(always)]
    pub fn is_keepwarm(&self) -> bool {
        *self == Warmupmode::Keepwarm
    }
}
#[doc = "Field `WARMUPMODE` writer - Warmup Mode"]
pub type WarmupmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Warmupmode>;
impl<'a, REG> WarmupmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Shut down the IADC after conversions have completed."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupmode::Normal)
    }
    #[doc = "Switch to standby mode after conversions have completed. The next warmup time will require 1us."]
    #[inline(always)]
    pub fn keepinstandby(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupmode::Keepinstandby)
    }
    #[doc = "Keep IADC fully powered after conversions have completed."]
    #[inline(always)]
    pub fn keepwarm(self) -> &'a mut crate::W<REG> {
        self.variant(Warmupmode::Keepwarm)
    }
}
#[doc = "Field `TIMEBASE` reader - Time Base"]
pub type TimebaseR = crate::FieldReader;
#[doc = "Field `TIMEBASE` writer - Time Base"]
pub type TimebaseW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "High Speed Clock Rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Hsclkrate {
    #[doc = "0: Use CMU_CLK_ADC directly. The source clock must be 40 MHz or less."]
    Div1 = 0,
    #[doc = "1: Divide CMU_CLK_ADC by 2 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
    Div2 = 1,
    #[doc = "2: Divide CMU_CLK_ADC by 3 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
    Div3 = 2,
    #[doc = "3: Divide CMU_CLK_ADC by 4 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
    Div4 = 3,
}
impl From<Hsclkrate> for u8 {
    #[inline(always)]
    fn from(variant: Hsclkrate) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Hsclkrate {
    type Ux = u8;
}
impl crate::IsEnum for Hsclkrate {}
#[doc = "Field `HSCLKRATE` reader - High Speed Clock Rate"]
pub type HsclkrateR = crate::FieldReader<Hsclkrate>;
impl HsclkrateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Hsclkrate> {
        match self.bits {
            0 => Some(Hsclkrate::Div1),
            1 => Some(Hsclkrate::Div2),
            2 => Some(Hsclkrate::Div3),
            3 => Some(Hsclkrate::Div4),
            _ => None,
        }
    }
    #[doc = "Use CMU_CLK_ADC directly. The source clock must be 40 MHz or less."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Hsclkrate::Div1
    }
    #[doc = "Divide CMU_CLK_ADC by 2 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Hsclkrate::Div2
    }
    #[doc = "Divide CMU_CLK_ADC by 3 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == Hsclkrate::Div3
    }
    #[doc = "Divide CMU_CLK_ADC by 4 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Hsclkrate::Div4
    }
}
#[doc = "Field `HSCLKRATE` writer - High Speed Clock Rate"]
pub type HsclkrateW<'a, REG> = crate::FieldWriter<'a, REG, 3, Hsclkrate>;
impl<'a, REG> HsclkrateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use CMU_CLK_ADC directly. The source clock must be 40 MHz or less."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Hsclkrate::Div1)
    }
    #[doc = "Divide CMU_CLK_ADC by 2 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Hsclkrate::Div2)
    }
    #[doc = "Divide CMU_CLK_ADC by 3 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(Hsclkrate::Div3)
    }
    #[doc = "Divide CMU_CLK_ADC by 4 before using it. The resulting CLK_SRC_ADC must be 40 MHz or less."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Hsclkrate::Div4)
    }
}
impl R {
    #[doc = "Bit 0 - EM23 Wakeup on Conversion"]
    #[inline(always)]
    pub fn em23wuconvert(&self) -> Em23wuconvertR {
        Em23wuconvertR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC_CLK Suspend - PRS0"]
    #[inline(always)]
    pub fn adcclksuspend0(&self) -> Adcclksuspend0R {
        Adcclksuspend0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC_CLK Suspend - PRS1"]
    #[inline(always)]
    pub fn adcclksuspend1(&self) -> Adcclksuspend1R {
        Adcclksuspend1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DbghaltR {
        DbghaltR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Warmup Mode"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WarmupmodeR {
        WarmupmodeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:22 - Time Base"]
    #[inline(always)]
    pub fn timebase(&self) -> TimebaseR {
        TimebaseR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 28:30 - High Speed Clock Rate"]
    #[inline(always)]
    pub fn hsclkrate(&self) -> HsclkrateR {
        HsclkrateR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EM23 Wakeup on Conversion"]
    #[inline(always)]
    pub fn em23wuconvert(&mut self) -> Em23wuconvertW<CtrlSpec> {
        Em23wuconvertW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC_CLK Suspend - PRS0"]
    #[inline(always)]
    pub fn adcclksuspend0(&mut self) -> Adcclksuspend0W<CtrlSpec> {
        Adcclksuspend0W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC_CLK Suspend - PRS1"]
    #[inline(always)]
    pub fn adcclksuspend1(&mut self) -> Adcclksuspend1W<CtrlSpec> {
        Adcclksuspend1W::new(self, 2)
    }
    #[doc = "Bit 3 - Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&mut self) -> DbghaltW<CtrlSpec> {
        DbghaltW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Warmup Mode"]
    #[inline(always)]
    pub fn warmupmode(&mut self) -> WarmupmodeW<CtrlSpec> {
        WarmupmodeW::new(self, 4)
    }
    #[doc = "Bits 16:22 - Time Base"]
    #[inline(always)]
    pub fn timebase(&mut self) -> TimebaseW<CtrlSpec> {
        TimebaseW::new(self, 16)
    }
    #[doc = "Bits 28:30 - High Speed Clock Rate"]
    #[inline(always)]
    pub fn hsclkrate(&mut self) -> HsclkrateW<CtrlSpec> {
        HsclkrateW::new(self, 28)
    }
}
#[doc = "Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
