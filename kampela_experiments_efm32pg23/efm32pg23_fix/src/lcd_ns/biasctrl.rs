#[doc = "Register `BIASCTRL` reader"]
pub type R = crate::R<BiasctrlSpec>;
#[doc = "Register `BIASCTRL` writer"]
pub type W = crate::W<BiasctrlSpec>;
#[doc = "Field `RESISTOR` reader - Resistor strength"]
pub type ResistorR = crate::FieldReader;
#[doc = "Field `RESISTOR` writer - Resistor strength"]
pub type ResistorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BUFDRV` reader - Buffer Drive Strength"]
pub type BufdrvR = crate::FieldReader;
#[doc = "Field `BUFDRV` writer - Buffer Drive Strength"]
pub type BufdrvW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BUFBIAS` reader - Buffer Bias Setting"]
pub type BufbiasR = crate::FieldReader;
#[doc = "Field `BUFBIAS` writer - Buffer Bias Setting"]
pub type BufbiasW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Mode Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Use step down control with VLCD less than VDDX. Use VLCD\\[4:0\\] to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    Stepdown = 0,
    #[doc = "1: Use the charge pump to pump VLCD above VDDX."]
    Chargepump = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Mode Setting"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Stepdown,
            true => Mode::Chargepump,
        }
    }
    #[doc = "Use step down control with VLCD less than VDDX. Use VLCD\\[4:0\\] to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    #[inline(always)]
    pub fn is_stepdown(&self) -> bool {
        *self == Mode::Stepdown
    }
    #[doc = "Use the charge pump to pump VLCD above VDDX."]
    #[inline(always)]
    pub fn is_chargepump(&self) -> bool {
        *self == Mode::Chargepump
    }
}
#[doc = "Field `MODE` writer - Mode Setting"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use step down control with VLCD less than VDDX. Use VLCD\\[4:0\\] to control VLCD level, and use SPEED to adjust VLCD drive strength."]
    #[inline(always)]
    pub fn stepdown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Stepdown)
    }
    #[doc = "Use the charge pump to pump VLCD above VDDX."]
    #[inline(always)]
    pub fn chargepump(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Chargepump)
    }
}
#[doc = "Field `VLCD` reader - VLCD voltage level"]
pub type VlcdR = crate::FieldReader;
#[doc = "Field `VLCD` writer - VLCD voltage level"]
pub type VlcdW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "VDDX select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vddxsel {
    #[doc = "0: Connect charge pump to digital DVDD supply"]
    Dvdd = 0,
    #[doc = "1: Connect charge pump to analog AVDD supply"]
    Avdd = 1,
}
impl From<Vddxsel> for bool {
    #[inline(always)]
    fn from(variant: Vddxsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDXSEL` reader - VDDX select"]
pub type VddxselR = crate::BitReader<Vddxsel>;
impl VddxselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vddxsel {
        match self.bits {
            false => Vddxsel::Dvdd,
            true => Vddxsel::Avdd,
        }
    }
    #[doc = "Connect charge pump to digital DVDD supply"]
    #[inline(always)]
    pub fn is_dvdd(&self) -> bool {
        *self == Vddxsel::Dvdd
    }
    #[doc = "Connect charge pump to analog AVDD supply"]
    #[inline(always)]
    pub fn is_avdd(&self) -> bool {
        *self == Vddxsel::Avdd
    }
}
#[doc = "Field `VDDXSEL` writer - VDDX select"]
pub type VddxselW<'a, REG> = crate::BitWriter<'a, REG, Vddxsel>;
impl<'a, REG> VddxselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Connect charge pump to digital DVDD supply"]
    #[inline(always)]
    pub fn dvdd(self) -> &'a mut crate::W<REG> {
        self.variant(Vddxsel::Dvdd)
    }
    #[doc = "Connect charge pump to analog AVDD supply"]
    #[inline(always)]
    pub fn avdd(self) -> &'a mut crate::W<REG> {
        self.variant(Vddxsel::Avdd)
    }
}
#[doc = "LCD Gate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lcdgate {
    #[doc = "0: LCD BIAS voltages driven onto pins."]
    Ungate = 0,
    #[doc = "1: LCD BIAS MUX tristated at the pins."]
    Gate = 1,
}
impl From<Lcdgate> for bool {
    #[inline(always)]
    fn from(variant: Lcdgate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LCDGATE` reader - LCD Gate"]
pub type LcdgateR = crate::BitReader<Lcdgate>;
impl LcdgateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lcdgate {
        match self.bits {
            false => Lcdgate::Ungate,
            true => Lcdgate::Gate,
        }
    }
    #[doc = "LCD BIAS voltages driven onto pins."]
    #[inline(always)]
    pub fn is_ungate(&self) -> bool {
        *self == Lcdgate::Ungate
    }
    #[doc = "LCD BIAS MUX tristated at the pins."]
    #[inline(always)]
    pub fn is_gate(&self) -> bool {
        *self == Lcdgate::Gate
    }
}
#[doc = "Field `LCDGATE` writer - LCD Gate"]
pub type LcdgateW<'a, REG> = crate::BitWriter<'a, REG, Lcdgate>;
impl<'a, REG> LcdgateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LCD BIAS voltages driven onto pins."]
    #[inline(always)]
    pub fn ungate(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdgate::Ungate)
    }
    #[doc = "LCD BIAS MUX tristated at the pins."]
    #[inline(always)]
    pub fn gate(self) -> &'a mut crate::W<REG> {
        self.variant(Lcdgate::Gate)
    }
}
#[doc = "DMA Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmamode {
    #[doc = "0: No DMA requests are generated"]
    Dmadisable = 0,
    #[doc = "1: DMA request on frame counter event. This will also start a DMA transfer during EM23."]
    Dmafc = 1,
    #[doc = "2: DMA request on display counter event. This will also start a DMA transfer during EM23."]
    Dmadisplay = 2,
}
impl From<Dmamode> for u8 {
    #[inline(always)]
    fn from(variant: Dmamode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmamode {
    type Ux = u8;
}
impl crate::IsEnum for Dmamode {}
#[doc = "Field `DMAMODE` reader - DMA Mode"]
pub type DmamodeR = crate::FieldReader<Dmamode>;
impl DmamodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmamode> {
        match self.bits {
            0 => Some(Dmamode::Dmadisable),
            1 => Some(Dmamode::Dmafc),
            2 => Some(Dmamode::Dmadisplay),
            _ => None,
        }
    }
    #[doc = "No DMA requests are generated"]
    #[inline(always)]
    pub fn is_dmadisable(&self) -> bool {
        *self == Dmamode::Dmadisable
    }
    #[doc = "DMA request on frame counter event. This will also start a DMA transfer during EM23."]
    #[inline(always)]
    pub fn is_dmafc(&self) -> bool {
        *self == Dmamode::Dmafc
    }
    #[doc = "DMA request on display counter event. This will also start a DMA transfer during EM23."]
    #[inline(always)]
    pub fn is_dmadisplay(&self) -> bool {
        *self == Dmamode::Dmadisplay
    }
}
#[doc = "Field `DMAMODE` writer - DMA Mode"]
pub type DmamodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmamode>;
impl<'a, REG> DmamodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No DMA requests are generated"]
    #[inline(always)]
    pub fn dmadisable(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamode::Dmadisable)
    }
    #[doc = "DMA request on frame counter event. This will also start a DMA transfer during EM23."]
    #[inline(always)]
    pub fn dmafc(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamode::Dmafc)
    }
    #[doc = "DMA request on display counter event. This will also start a DMA transfer during EM23."]
    #[inline(always)]
    pub fn dmadisplay(self) -> &'a mut crate::W<REG> {
        self.variant(Dmamode::Dmadisplay)
    }
}
impl R {
    #[doc = "Bits 0:3 - Resistor strength"]
    #[inline(always)]
    pub fn resistor(&self) -> ResistorR {
        ResistorR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Buffer Drive Strength"]
    #[inline(always)]
    pub fn bufdrv(&self) -> BufdrvR {
        BufdrvR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Buffer Bias Setting"]
    #[inline(always)]
    pub fn bufbias(&self) -> BufbiasR {
        BufbiasR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Mode Setting"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:20 - VLCD voltage level"]
    #[inline(always)]
    pub fn vlcd(&self) -> VlcdR {
        VlcdR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - VDDX select"]
    #[inline(always)]
    pub fn vddxsel(&self) -> VddxselR {
        VddxselR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 26 - LCD Gate"]
    #[inline(always)]
    pub fn lcdgate(&self) -> LcdgateR {
        LcdgateR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 30:31 - DMA Mode"]
    #[inline(always)]
    pub fn dmamode(&self) -> DmamodeR {
        DmamodeR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Resistor strength"]
    #[inline(always)]
    pub fn resistor(&mut self) -> ResistorW<BiasctrlSpec> {
        ResistorW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Buffer Drive Strength"]
    #[inline(always)]
    pub fn bufdrv(&mut self) -> BufdrvW<BiasctrlSpec> {
        BufdrvW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Buffer Bias Setting"]
    #[inline(always)]
    pub fn bufbias(&mut self) -> BufbiasW<BiasctrlSpec> {
        BufbiasW::new(self, 8)
    }
    #[doc = "Bit 12 - Mode Setting"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<BiasctrlSpec> {
        ModeW::new(self, 12)
    }
    #[doc = "Bits 16:20 - VLCD voltage level"]
    #[inline(always)]
    pub fn vlcd(&mut self) -> VlcdW<BiasctrlSpec> {
        VlcdW::new(self, 16)
    }
    #[doc = "Bit 22 - VDDX select"]
    #[inline(always)]
    pub fn vddxsel(&mut self) -> VddxselW<BiasctrlSpec> {
        VddxselW::new(self, 22)
    }
    #[doc = "Bit 26 - LCD Gate"]
    #[inline(always)]
    pub fn lcdgate(&mut self) -> LcdgateW<BiasctrlSpec> {
        LcdgateW::new(self, 26)
    }
    #[doc = "Bits 30:31 - DMA Mode"]
    #[inline(always)]
    pub fn dmamode(&mut self) -> DmamodeW<BiasctrlSpec> {
        DmamodeW::new(self, 30)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`biasctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`biasctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BiasctrlSpec;
impl crate::RegisterSpec for BiasctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`biasctrl::R`](R) reader structure"]
impl crate::Readable for BiasctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`biasctrl::W`](W) writer structure"]
impl crate::Writable for BiasctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BIASCTRL to value 0x001f_0000"]
impl crate::Resettable for BiasctrlSpec {
    const RESET_VALUE: u32 = 0x001f_0000;
}
