#[doc = "Register `CH1CFG` reader"]
pub type R = crate::R<Ch1cfgSpec>;
#[doc = "Register `CH1CFG` writer"]
pub type W = crate::W<Ch1cfgSpec>;
#[doc = "Channel 1 Conversion Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Convmode {
    #[doc = "0: DAC channel 1 is set in continuous mode"]
    Continuous = 0,
    #[doc = "1: DAC channel 1 is set in sample/shut off mode"]
    Sampleoff = 1,
}
impl From<Convmode> for bool {
    #[inline(always)]
    fn from(variant: Convmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CONVMODE` reader - Channel 1 Conversion Mode"]
pub type ConvmodeR = crate::BitReader<Convmode>;
impl ConvmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Convmode {
        match self.bits {
            false => Convmode::Continuous,
            true => Convmode::Sampleoff,
        }
    }
    #[doc = "DAC channel 1 is set in continuous mode"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Convmode::Continuous
    }
    #[doc = "DAC channel 1 is set in sample/shut off mode"]
    #[inline(always)]
    pub fn is_sampleoff(&self) -> bool {
        *self == Convmode::Sampleoff
    }
}
#[doc = "Field `CONVMODE` writer - Channel 1 Conversion Mode"]
pub type ConvmodeW<'a, REG> = crate::BitWriter<'a, REG, Convmode>;
impl<'a, REG> ConvmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DAC channel 1 is set in continuous mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Convmode::Continuous)
    }
    #[doc = "DAC channel 1 is set in sample/shut off mode"]
    #[inline(always)]
    pub fn sampleoff(self) -> &'a mut crate::W<REG> {
        self.variant(Convmode::Sampleoff)
    }
}
#[doc = "Channel 1 Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Powermode {
    #[doc = "0: Default is High Power Mode"]
    Highpower = 0,
    #[doc = "1: Set this bit for Low Power Mode"]
    Lowpower = 1,
}
impl From<Powermode> for bool {
    #[inline(always)]
    fn from(variant: Powermode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POWERMODE` reader - Channel 1 Power Mode"]
pub type PowermodeR = crate::BitReader<Powermode>;
impl PowermodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Powermode {
        match self.bits {
            false => Powermode::Highpower,
            true => Powermode::Lowpower,
        }
    }
    #[doc = "Default is High Power Mode"]
    #[inline(always)]
    pub fn is_highpower(&self) -> bool {
        *self == Powermode::Highpower
    }
    #[doc = "Set this bit for Low Power Mode"]
    #[inline(always)]
    pub fn is_lowpower(&self) -> bool {
        *self == Powermode::Lowpower
    }
}
#[doc = "Field `POWERMODE` writer - Channel 1 Power Mode"]
pub type PowermodeW<'a, REG> = crate::BitWriter<'a, REG, Powermode>;
impl<'a, REG> PowermodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Default is High Power Mode"]
    #[inline(always)]
    pub fn highpower(self) -> &'a mut crate::W<REG> {
        self.variant(Powermode::Highpower)
    }
    #[doc = "Set this bit for Low Power Mode"]
    #[inline(always)]
    pub fn lowpower(self) -> &'a mut crate::W<REG> {
        self.variant(Powermode::Lowpower)
    }
}
#[doc = "Channel 1 Trigger Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Trigmode {
    #[doc = "0: No Conversion Trigger Source Selected for Channel 1"]
    None = 0,
    #[doc = "1: Channel 1 is triggered by Channel 1 FIFO (CH1F) write"]
    Sw = 1,
    #[doc = "2: Channel 1 is triggered by Sync PRS input.PRS Trigger should have the same clock group as VDAC."]
    Syncprs = 2,
    #[doc = "4: Channel 1 is triggered by Internal Timer Overflow"]
    Internaltimer = 4,
    #[doc = "5: Channel 1 is triggered by Async PRS input"]
    Asyncprs = 5,
}
impl From<Trigmode> for u8 {
    #[inline(always)]
    fn from(variant: Trigmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Trigmode {
    type Ux = u8;
}
impl crate::IsEnum for Trigmode {}
#[doc = "Field `TRIGMODE` reader - Channel 1 Trigger Mode"]
pub type TrigmodeR = crate::FieldReader<Trigmode>;
impl TrigmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Trigmode> {
        match self.bits {
            0 => Some(Trigmode::None),
            1 => Some(Trigmode::Sw),
            2 => Some(Trigmode::Syncprs),
            4 => Some(Trigmode::Internaltimer),
            5 => Some(Trigmode::Asyncprs),
            _ => None,
        }
    }
    #[doc = "No Conversion Trigger Source Selected for Channel 1"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Trigmode::None
    }
    #[doc = "Channel 1 is triggered by Channel 1 FIFO (CH1F) write"]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == Trigmode::Sw
    }
    #[doc = "Channel 1 is triggered by Sync PRS input.PRS Trigger should have the same clock group as VDAC."]
    #[inline(always)]
    pub fn is_syncprs(&self) -> bool {
        *self == Trigmode::Syncprs
    }
    #[doc = "Channel 1 is triggered by Internal Timer Overflow"]
    #[inline(always)]
    pub fn is_internaltimer(&self) -> bool {
        *self == Trigmode::Internaltimer
    }
    #[doc = "Channel 1 is triggered by Async PRS input"]
    #[inline(always)]
    pub fn is_asyncprs(&self) -> bool {
        *self == Trigmode::Asyncprs
    }
}
#[doc = "Field `TRIGMODE` writer - Channel 1 Trigger Mode"]
pub type TrigmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Trigmode>;
impl<'a, REG> TrigmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Conversion Trigger Source Selected for Channel 1"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Trigmode::None)
    }
    #[doc = "Channel 1 is triggered by Channel 1 FIFO (CH1F) write"]
    #[inline(always)]
    pub fn sw(self) -> &'a mut crate::W<REG> {
        self.variant(Trigmode::Sw)
    }
    #[doc = "Channel 1 is triggered by Sync PRS input.PRS Trigger should have the same clock group as VDAC."]
    #[inline(always)]
    pub fn syncprs(self) -> &'a mut crate::W<REG> {
        self.variant(Trigmode::Syncprs)
    }
    #[doc = "Channel 1 is triggered by Internal Timer Overflow"]
    #[inline(always)]
    pub fn internaltimer(self) -> &'a mut crate::W<REG> {
        self.variant(Trigmode::Internaltimer)
    }
    #[doc = "Channel 1 is triggered by Async PRS input"]
    #[inline(always)]
    pub fn asyncprs(self) -> &'a mut crate::W<REG> {
        self.variant(Trigmode::Asyncprs)
    }
}
#[doc = "Channel 1 Refresh Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refreshsource {
    #[doc = "0: No Refresh Source Selected"]
    None = 0,
    #[doc = "1: CH1 Refresh Triggered by Refresh Timer Overflow"]
    Refreshtimer = 1,
    #[doc = "2: CH1 Refresh Triggered by Sync PRS. PRS Trigger should have the same clock group as VDAC."]
    Syncprs = 2,
    #[doc = "3: CH1 Refresh Triggered by Async PRS"]
    Asyncprs = 3,
}
impl From<Refreshsource> for u8 {
    #[inline(always)]
    fn from(variant: Refreshsource) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refreshsource {
    type Ux = u8;
}
impl crate::IsEnum for Refreshsource {}
#[doc = "Field `REFRESHSOURCE` reader - Channel 1 Refresh Source"]
pub type RefreshsourceR = crate::FieldReader<Refreshsource>;
impl RefreshsourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refreshsource {
        match self.bits {
            0 => Refreshsource::None,
            1 => Refreshsource::Refreshtimer,
            2 => Refreshsource::Syncprs,
            3 => Refreshsource::Asyncprs,
            _ => unreachable!(),
        }
    }
    #[doc = "No Refresh Source Selected"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Refreshsource::None
    }
    #[doc = "CH1 Refresh Triggered by Refresh Timer Overflow"]
    #[inline(always)]
    pub fn is_refreshtimer(&self) -> bool {
        *self == Refreshsource::Refreshtimer
    }
    #[doc = "CH1 Refresh Triggered by Sync PRS. PRS Trigger should have the same clock group as VDAC."]
    #[inline(always)]
    pub fn is_syncprs(&self) -> bool {
        *self == Refreshsource::Syncprs
    }
    #[doc = "CH1 Refresh Triggered by Async PRS"]
    #[inline(always)]
    pub fn is_asyncprs(&self) -> bool {
        *self == Refreshsource::Asyncprs
    }
}
#[doc = "Field `REFRESHSOURCE` writer - Channel 1 Refresh Source"]
pub type RefreshsourceW<'a, REG> = crate::FieldWriter<'a, REG, 2, Refreshsource, crate::Safe>;
impl<'a, REG> RefreshsourceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Refresh Source Selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshsource::None)
    }
    #[doc = "CH1 Refresh Triggered by Refresh Timer Overflow"]
    #[inline(always)]
    pub fn refreshtimer(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshsource::Refreshtimer)
    }
    #[doc = "CH1 Refresh Triggered by Sync PRS. PRS Trigger should have the same clock group as VDAC."]
    #[inline(always)]
    pub fn syncprs(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshsource::Syncprs)
    }
    #[doc = "CH1 Refresh Triggered by Async PRS"]
    #[inline(always)]
    pub fn asyncprs(self) -> &'a mut crate::W<REG> {
        self.variant(Refreshsource::Asyncprs)
    }
}
#[doc = "Field `FIFODVL` reader - Channel 1 FIFO Low Watermark"]
pub type FifodvlR = crate::FieldReader;
#[doc = "Field `FIFODVL` writer - Channel 1 FIFO Low Watermark"]
pub type FifodvlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HIGHCAPLOADEN` reader - Channel 1 High Cap Load Mode Enable"]
pub type HighcaploadenR = crate::BitReader;
#[doc = "Field `HIGHCAPLOADEN` writer - Channel 1 High Cap Load Mode Enable"]
pub type HighcaploadenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEEPWARM` reader - Channel 1 Keepwarm Mode Enable"]
pub type KeepwarmR = crate::BitReader;
#[doc = "Field `KEEPWARM` writer - Channel 1 Keepwarm Mode Enable"]
pub type KeepwarmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 1 Conversion Mode"]
    #[inline(always)]
    pub fn convmode(&self) -> ConvmodeR {
        ConvmodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 1 Power Mode"]
    #[inline(always)]
    pub fn powermode(&self) -> PowermodeR {
        PowermodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Channel 1 Trigger Mode"]
    #[inline(always)]
    pub fn trigmode(&self) -> TrigmodeR {
        TrigmodeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Channel 1 Refresh Source"]
    #[inline(always)]
    pub fn refreshsource(&self) -> RefreshsourceR {
        RefreshsourceR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Channel 1 FIFO Low Watermark"]
    #[inline(always)]
    pub fn fifodvl(&self) -> FifodvlR {
        FifodvlR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 14 - Channel 1 High Cap Load Mode Enable"]
    #[inline(always)]
    pub fn highcaploaden(&self) -> HighcaploadenR {
        HighcaploadenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Channel 1 Keepwarm Mode Enable"]
    #[inline(always)]
    pub fn keepwarm(&self) -> KeepwarmR {
        KeepwarmR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 1 Conversion Mode"]
    #[inline(always)]
    pub fn convmode(&mut self) -> ConvmodeW<Ch1cfgSpec> {
        ConvmodeW::new(self, 0)
    }
    #[doc = "Bit 2 - Channel 1 Power Mode"]
    #[inline(always)]
    pub fn powermode(&mut self) -> PowermodeW<Ch1cfgSpec> {
        PowermodeW::new(self, 2)
    }
    #[doc = "Bits 4:6 - Channel 1 Trigger Mode"]
    #[inline(always)]
    pub fn trigmode(&mut self) -> TrigmodeW<Ch1cfgSpec> {
        TrigmodeW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Channel 1 Refresh Source"]
    #[inline(always)]
    pub fn refreshsource(&mut self) -> RefreshsourceW<Ch1cfgSpec> {
        RefreshsourceW::new(self, 8)
    }
    #[doc = "Bits 11:12 - Channel 1 FIFO Low Watermark"]
    #[inline(always)]
    pub fn fifodvl(&mut self) -> FifodvlW<Ch1cfgSpec> {
        FifodvlW::new(self, 11)
    }
    #[doc = "Bit 14 - Channel 1 High Cap Load Mode Enable"]
    #[inline(always)]
    pub fn highcaploaden(&mut self) -> HighcaploadenW<Ch1cfgSpec> {
        HighcaploadenW::new(self, 14)
    }
    #[doc = "Bit 16 - Channel 1 Keepwarm Mode Enable"]
    #[inline(always)]
    pub fn keepwarm(&mut self) -> KeepwarmW<Ch1cfgSpec> {
        KeepwarmW::new(self, 16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ch1cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch1cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch1cfgSpec;
impl crate::RegisterSpec for Ch1cfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1cfg::R`](R) reader structure"]
impl crate::Readable for Ch1cfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ch1cfg::W`](W) writer structure"]
impl crate::Writable for Ch1cfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH1CFG to value 0x10"]
impl crate::Resettable for Ch1cfgSpec {
    const RESET_VALUE: u32 = 0x10;
}
