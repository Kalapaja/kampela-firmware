#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "ADC Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Adcmode {
    #[doc = "0: High speed mode with a maximum ADC_CLK of 10 MHz."]
    Normal = 0,
}
impl From<Adcmode> for u8 {
    #[inline(always)]
    fn from(variant: Adcmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Adcmode {
    type Ux = u8;
}
impl crate::IsEnum for Adcmode {}
#[doc = "Field `ADCMODE` reader - ADC Mode"]
pub type AdcmodeR = crate::FieldReader<Adcmode>;
impl AdcmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Adcmode> {
        match self.bits {
            0 => Some(Adcmode::Normal),
            _ => None,
        }
    }
    #[doc = "High speed mode with a maximum ADC_CLK of 10 MHz."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Adcmode::Normal
    }
}
#[doc = "Field `ADCMODE` writer - ADC Mode"]
pub type AdcmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Adcmode>;
impl<'a, REG> AdcmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High speed mode with a maximum ADC_CLK of 10 MHz."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Adcmode::Normal)
    }
}
#[doc = "High Speed OSR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Osrhs {
    #[doc = "0: High speed over sampling of 2x."]
    Hispd2 = 0,
    #[doc = "1: High speed over sampling of 4x."]
    Hispd4 = 1,
    #[doc = "2: High speed over sampling of 8x."]
    Hispd8 = 2,
    #[doc = "3: High speed over sampling of 16x."]
    Hispd16 = 3,
    #[doc = "4: HIgh speed over sampling of 32x."]
    Hispd32 = 4,
    #[doc = "5: High speed over sampling of 64x."]
    Hispd64 = 5,
}
impl From<Osrhs> for u8 {
    #[inline(always)]
    fn from(variant: Osrhs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Osrhs {
    type Ux = u8;
}
impl crate::IsEnum for Osrhs {}
#[doc = "Field `OSRHS` reader - High Speed OSR"]
pub type OsrhsR = crate::FieldReader<Osrhs>;
impl OsrhsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Osrhs> {
        match self.bits {
            0 => Some(Osrhs::Hispd2),
            1 => Some(Osrhs::Hispd4),
            2 => Some(Osrhs::Hispd8),
            3 => Some(Osrhs::Hispd16),
            4 => Some(Osrhs::Hispd32),
            5 => Some(Osrhs::Hispd64),
            _ => None,
        }
    }
    #[doc = "High speed over sampling of 2x."]
    #[inline(always)]
    pub fn is_hispd2(&self) -> bool {
        *self == Osrhs::Hispd2
    }
    #[doc = "High speed over sampling of 4x."]
    #[inline(always)]
    pub fn is_hispd4(&self) -> bool {
        *self == Osrhs::Hispd4
    }
    #[doc = "High speed over sampling of 8x."]
    #[inline(always)]
    pub fn is_hispd8(&self) -> bool {
        *self == Osrhs::Hispd8
    }
    #[doc = "High speed over sampling of 16x."]
    #[inline(always)]
    pub fn is_hispd16(&self) -> bool {
        *self == Osrhs::Hispd16
    }
    #[doc = "HIgh speed over sampling of 32x."]
    #[inline(always)]
    pub fn is_hispd32(&self) -> bool {
        *self == Osrhs::Hispd32
    }
    #[doc = "High speed over sampling of 64x."]
    #[inline(always)]
    pub fn is_hispd64(&self) -> bool {
        *self == Osrhs::Hispd64
    }
}
#[doc = "Field `OSRHS` writer - High Speed OSR"]
pub type OsrhsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Osrhs>;
impl<'a, REG> OsrhsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "High speed over sampling of 2x."]
    #[inline(always)]
    pub fn hispd2(self) -> &'a mut crate::W<REG> {
        self.variant(Osrhs::Hispd2)
    }
    #[doc = "High speed over sampling of 4x."]
    #[inline(always)]
    pub fn hispd4(self) -> &'a mut crate::W<REG> {
        self.variant(Osrhs::Hispd4)
    }
    #[doc = "High speed over sampling of 8x."]
    #[inline(always)]
    pub fn hispd8(self) -> &'a mut crate::W<REG> {
        self.variant(Osrhs::Hispd8)
    }
    #[doc = "High speed over sampling of 16x."]
    #[inline(always)]
    pub fn hispd16(self) -> &'a mut crate::W<REG> {
        self.variant(Osrhs::Hispd16)
    }
    #[doc = "HIgh speed over sampling of 32x."]
    #[inline(always)]
    pub fn hispd32(self) -> &'a mut crate::W<REG> {
        self.variant(Osrhs::Hispd32)
    }
    #[doc = "High speed over sampling of 64x."]
    #[inline(always)]
    pub fn hispd64(self) -> &'a mut crate::W<REG> {
        self.variant(Osrhs::Hispd64)
    }
}
#[doc = "Analog Gain\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Analoggain {
    #[doc = "1: Analog gain of 0.5x."]
    Anagain0p5 = 1,
    #[doc = "2: Analog gain of 1x."]
    Anagain1 = 2,
    #[doc = "3: Analog gain of 2x."]
    Anagain2 = 3,
    #[doc = "4: Analog gain of 3x."]
    Anagain3 = 4,
    #[doc = "5: Analog gain of 4x."]
    Anagain4 = 5,
}
impl From<Analoggain> for u8 {
    #[inline(always)]
    fn from(variant: Analoggain) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Analoggain {
    type Ux = u8;
}
impl crate::IsEnum for Analoggain {}
#[doc = "Field `ANALOGGAIN` reader - Analog Gain"]
pub type AnaloggainR = crate::FieldReader<Analoggain>;
impl AnaloggainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Analoggain> {
        match self.bits {
            1 => Some(Analoggain::Anagain0p5),
            2 => Some(Analoggain::Anagain1),
            3 => Some(Analoggain::Anagain2),
            4 => Some(Analoggain::Anagain3),
            5 => Some(Analoggain::Anagain4),
            _ => None,
        }
    }
    #[doc = "Analog gain of 0.5x."]
    #[inline(always)]
    pub fn is_anagain0p5(&self) -> bool {
        *self == Analoggain::Anagain0p5
    }
    #[doc = "Analog gain of 1x."]
    #[inline(always)]
    pub fn is_anagain1(&self) -> bool {
        *self == Analoggain::Anagain1
    }
    #[doc = "Analog gain of 2x."]
    #[inline(always)]
    pub fn is_anagain2(&self) -> bool {
        *self == Analoggain::Anagain2
    }
    #[doc = "Analog gain of 3x."]
    #[inline(always)]
    pub fn is_anagain3(&self) -> bool {
        *self == Analoggain::Anagain3
    }
    #[doc = "Analog gain of 4x."]
    #[inline(always)]
    pub fn is_anagain4(&self) -> bool {
        *self == Analoggain::Anagain4
    }
}
#[doc = "Field `ANALOGGAIN` writer - Analog Gain"]
pub type AnaloggainW<'a, REG> = crate::FieldWriter<'a, REG, 3, Analoggain>;
impl<'a, REG> AnaloggainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Analog gain of 0.5x."]
    #[inline(always)]
    pub fn anagain0p5(self) -> &'a mut crate::W<REG> {
        self.variant(Analoggain::Anagain0p5)
    }
    #[doc = "Analog gain of 1x."]
    #[inline(always)]
    pub fn anagain1(self) -> &'a mut crate::W<REG> {
        self.variant(Analoggain::Anagain1)
    }
    #[doc = "Analog gain of 2x."]
    #[inline(always)]
    pub fn anagain2(self) -> &'a mut crate::W<REG> {
        self.variant(Analoggain::Anagain2)
    }
    #[doc = "Analog gain of 3x."]
    #[inline(always)]
    pub fn anagain3(self) -> &'a mut crate::W<REG> {
        self.variant(Analoggain::Anagain3)
    }
    #[doc = "Analog gain of 4x."]
    #[inline(always)]
    pub fn anagain4(self) -> &'a mut crate::W<REG> {
        self.variant(Analoggain::Anagain4)
    }
}
#[doc = "Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Refsel {
    #[doc = "0: Internal 1.21 V reference."]
    Vbgr = 0,
    #[doc = "1: External Reference. (Calibrated for 1.25V nominal.)"]
    Vref = 1,
    #[doc = "3: AVDD (unbuffered)"]
    Vddx = 3,
    #[doc = "4: AVDD (buffered) * 0.8"]
    Vddx0p8buf = 4,
}
impl From<Refsel> for u8 {
    #[inline(always)]
    fn from(variant: Refsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Refsel {
    type Ux = u8;
}
impl crate::IsEnum for Refsel {}
#[doc = "Field `REFSEL` reader - Reference Select"]
pub type RefselR = crate::FieldReader<Refsel>;
impl RefselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Refsel> {
        match self.bits {
            0 => Some(Refsel::Vbgr),
            1 => Some(Refsel::Vref),
            3 => Some(Refsel::Vddx),
            4 => Some(Refsel::Vddx0p8buf),
            _ => None,
        }
    }
    #[doc = "Internal 1.21 V reference."]
    #[inline(always)]
    pub fn is_vbgr(&self) -> bool {
        *self == Refsel::Vbgr
    }
    #[doc = "External Reference. (Calibrated for 1.25V nominal.)"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == Refsel::Vref
    }
    #[doc = "AVDD (unbuffered)"]
    #[inline(always)]
    pub fn is_vddx(&self) -> bool {
        *self == Refsel::Vddx
    }
    #[doc = "AVDD (buffered) * 0.8"]
    #[inline(always)]
    pub fn is_vddx0p8buf(&self) -> bool {
        *self == Refsel::Vddx0p8buf
    }
}
#[doc = "Field `REFSEL` writer - Reference Select"]
pub type RefselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Refsel>;
impl<'a, REG> RefselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Internal 1.21 V reference."]
    #[inline(always)]
    pub fn vbgr(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Vbgr)
    }
    #[doc = "External Reference. (Calibrated for 1.25V nominal.)"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Vref)
    }
    #[doc = "AVDD (unbuffered)"]
    #[inline(always)]
    pub fn vddx(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Vddx)
    }
    #[doc = "AVDD (buffered) * 0.8"]
    #[inline(always)]
    pub fn vddx0p8buf(self) -> &'a mut crate::W<REG> {
        self.variant(Refsel::Vddx0p8buf)
    }
}
#[doc = "Digital Averaging\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Digavg {
    #[doc = "0: Collect one output word (no digital averaging)."]
    Avg1 = 0,
    #[doc = "1: Collect and average 2 digital output words."]
    Avg2 = 1,
    #[doc = "2: Collect and average 4 digital output words."]
    Avg4 = 2,
    #[doc = "3: Collect and average 8 digital output words."]
    Avg8 = 3,
    #[doc = "4: Collect and average 16 digital output words."]
    Avg16 = 4,
}
impl From<Digavg> for u8 {
    #[inline(always)]
    fn from(variant: Digavg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Digavg {
    type Ux = u8;
}
impl crate::IsEnum for Digavg {}
#[doc = "Field `DIGAVG` reader - Digital Averaging"]
pub type DigavgR = crate::FieldReader<Digavg>;
impl DigavgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Digavg> {
        match self.bits {
            0 => Some(Digavg::Avg1),
            1 => Some(Digavg::Avg2),
            2 => Some(Digavg::Avg4),
            3 => Some(Digavg::Avg8),
            4 => Some(Digavg::Avg16),
            _ => None,
        }
    }
    #[doc = "Collect one output word (no digital averaging)."]
    #[inline(always)]
    pub fn is_avg1(&self) -> bool {
        *self == Digavg::Avg1
    }
    #[doc = "Collect and average 2 digital output words."]
    #[inline(always)]
    pub fn is_avg2(&self) -> bool {
        *self == Digavg::Avg2
    }
    #[doc = "Collect and average 4 digital output words."]
    #[inline(always)]
    pub fn is_avg4(&self) -> bool {
        *self == Digavg::Avg4
    }
    #[doc = "Collect and average 8 digital output words."]
    #[inline(always)]
    pub fn is_avg8(&self) -> bool {
        *self == Digavg::Avg8
    }
    #[doc = "Collect and average 16 digital output words."]
    #[inline(always)]
    pub fn is_avg16(&self) -> bool {
        *self == Digavg::Avg16
    }
}
#[doc = "Field `DIGAVG` writer - Digital Averaging"]
pub type DigavgW<'a, REG> = crate::FieldWriter<'a, REG, 3, Digavg>;
impl<'a, REG> DigavgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Collect one output word (no digital averaging)."]
    #[inline(always)]
    pub fn avg1(self) -> &'a mut crate::W<REG> {
        self.variant(Digavg::Avg1)
    }
    #[doc = "Collect and average 2 digital output words."]
    #[inline(always)]
    pub fn avg2(self) -> &'a mut crate::W<REG> {
        self.variant(Digavg::Avg2)
    }
    #[doc = "Collect and average 4 digital output words."]
    #[inline(always)]
    pub fn avg4(self) -> &'a mut crate::W<REG> {
        self.variant(Digavg::Avg4)
    }
    #[doc = "Collect and average 8 digital output words."]
    #[inline(always)]
    pub fn avg8(self) -> &'a mut crate::W<REG> {
        self.variant(Digavg::Avg8)
    }
    #[doc = "Collect and average 16 digital output words."]
    #[inline(always)]
    pub fn avg16(self) -> &'a mut crate::W<REG> {
        self.variant(Digavg::Avg16)
    }
}
#[doc = "Two's Complement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Twoscompl {
    #[doc = "0: Automatic: Single ended measurements are reported as unipolar and differential measurements are reported as bipolar."]
    Auto = 0,
    #[doc = "1: Force all measurements to result in unipolar output. Negative differential numbers will saturate to 0."]
    Forceunipolar = 1,
    #[doc = "2: Force all measurements to result in bipolar output. Single ended measurements are half the range, but allow for small negative measurements."]
    Forcebipolar = 2,
}
impl From<Twoscompl> for u8 {
    #[inline(always)]
    fn from(variant: Twoscompl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Twoscompl {
    type Ux = u8;
}
impl crate::IsEnum for Twoscompl {}
#[doc = "Field `TWOSCOMPL` reader - Two's Complement"]
pub type TwoscomplR = crate::FieldReader<Twoscompl>;
impl TwoscomplR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Twoscompl> {
        match self.bits {
            0 => Some(Twoscompl::Auto),
            1 => Some(Twoscompl::Forceunipolar),
            2 => Some(Twoscompl::Forcebipolar),
            _ => None,
        }
    }
    #[doc = "Automatic: Single ended measurements are reported as unipolar and differential measurements are reported as bipolar."]
    #[inline(always)]
    pub fn is_auto(&self) -> bool {
        *self == Twoscompl::Auto
    }
    #[doc = "Force all measurements to result in unipolar output. Negative differential numbers will saturate to 0."]
    #[inline(always)]
    pub fn is_forceunipolar(&self) -> bool {
        *self == Twoscompl::Forceunipolar
    }
    #[doc = "Force all measurements to result in bipolar output. Single ended measurements are half the range, but allow for small negative measurements."]
    #[inline(always)]
    pub fn is_forcebipolar(&self) -> bool {
        *self == Twoscompl::Forcebipolar
    }
}
#[doc = "Field `TWOSCOMPL` writer - Two's Complement"]
pub type TwoscomplW<'a, REG> = crate::FieldWriter<'a, REG, 2, Twoscompl>;
impl<'a, REG> TwoscomplW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Automatic: Single ended measurements are reported as unipolar and differential measurements are reported as bipolar."]
    #[inline(always)]
    pub fn auto(self) -> &'a mut crate::W<REG> {
        self.variant(Twoscompl::Auto)
    }
    #[doc = "Force all measurements to result in unipolar output. Negative differential numbers will saturate to 0."]
    #[inline(always)]
    pub fn forceunipolar(self) -> &'a mut crate::W<REG> {
        self.variant(Twoscompl::Forceunipolar)
    }
    #[doc = "Force all measurements to result in bipolar output. Single ended measurements are half the range, but allow for small negative measurements."]
    #[inline(always)]
    pub fn forcebipolar(self) -> &'a mut crate::W<REG> {
        self.variant(Twoscompl::Forcebipolar)
    }
}
impl R {
    #[doc = "Bits 0:1 - ADC Mode"]
    #[inline(always)]
    pub fn adcmode(&self) -> AdcmodeR {
        AdcmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - High Speed OSR"]
    #[inline(always)]
    pub fn osrhs(&self) -> OsrhsR {
        OsrhsR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Analog Gain"]
    #[inline(always)]
    pub fn analoggain(&self) -> AnaloggainR {
        AnaloggainR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Reference Select"]
    #[inline(always)]
    pub fn refsel(&self) -> RefselR {
        RefselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Digital Averaging"]
    #[inline(always)]
    pub fn digavg(&self) -> DigavgR {
        DigavgR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 28:29 - Two's Complement"]
    #[inline(always)]
    pub fn twoscompl(&self) -> TwoscomplR {
        TwoscomplR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC Mode"]
    #[inline(always)]
    pub fn adcmode(&mut self) -> AdcmodeW<Cfg1Spec> {
        AdcmodeW::new(self, 0)
    }
    #[doc = "Bits 2:4 - High Speed OSR"]
    #[inline(always)]
    pub fn osrhs(&mut self) -> OsrhsW<Cfg1Spec> {
        OsrhsW::new(self, 2)
    }
    #[doc = "Bits 12:14 - Analog Gain"]
    #[inline(always)]
    pub fn analoggain(&mut self) -> AnaloggainW<Cfg1Spec> {
        AnaloggainW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Reference Select"]
    #[inline(always)]
    pub fn refsel(&mut self) -> RefselW<Cfg1Spec> {
        RefselW::new(self, 16)
    }
    #[doc = "Bits 21:23 - Digital Averaging"]
    #[inline(always)]
    pub fn digavg(&mut self) -> DigavgW<Cfg1Spec> {
        DigavgW::new(self, 21)
    }
    #[doc = "Bits 28:29 - Two's Complement"]
    #[inline(always)]
    pub fn twoscompl(&mut self) -> TwoscomplW<Cfg1Spec> {
        TwoscomplW::new(self, 28)
    }
}
#[doc = "Configration\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG1 to value 0x2060"]
impl crate::Resettable for Cfg1Spec {
    const RESET_VALUE: u32 = 0x2060;
}
