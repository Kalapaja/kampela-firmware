#[doc = "Register `SINGLE` reader"]
pub type R = crate::R<SingleSpec>;
#[doc = "Register `SINGLE` writer"]
pub type W = crate::W<SingleSpec>;
#[doc = "Field `PINNEG` reader - Negative Pin Select"]
pub type PinnegR = crate::FieldReader;
#[doc = "Field `PINNEG` writer - Negative Pin Select"]
pub type PinnegW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Negative Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Portneg {
    #[doc = "0: Ground (single-ended)"]
    Gnd = 0,
    #[doc = "2: Direct connection to DAC0_CH1"]
    Dac1 = 2,
    #[doc = "8: Port A - Select pin number using PINNEG"]
    Porta = 8,
    #[doc = "9: Port B - Select pin number using PINNEG"]
    Portb = 9,
    #[doc = "10: Port C - Select pin number using PINNEG"]
    Portc = 10,
    #[doc = "11: Port D - Select pin number using PINNEG"]
    Portd = 11,
}
impl From<Portneg> for u8 {
    #[inline(always)]
    fn from(variant: Portneg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Portneg {
    type Ux = u8;
}
impl crate::IsEnum for Portneg {}
#[doc = "Field `PORTNEG` reader - Negative Port Select"]
pub type PortnegR = crate::FieldReader<Portneg>;
impl PortnegR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Portneg> {
        match self.bits {
            0 => Some(Portneg::Gnd),
            2 => Some(Portneg::Dac1),
            8 => Some(Portneg::Porta),
            9 => Some(Portneg::Portb),
            10 => Some(Portneg::Portc),
            11 => Some(Portneg::Portd),
            _ => None,
        }
    }
    #[doc = "Ground (single-ended)"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == Portneg::Gnd
    }
    #[doc = "Direct connection to DAC0_CH1"]
    #[inline(always)]
    pub fn is_dac1(&self) -> bool {
        *self == Portneg::Dac1
    }
    #[doc = "Port A - Select pin number using PINNEG"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Portneg::Porta
    }
    #[doc = "Port B - Select pin number using PINNEG"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Portneg::Portb
    }
    #[doc = "Port C - Select pin number using PINNEG"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Portneg::Portc
    }
    #[doc = "Port D - Select pin number using PINNEG"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Portneg::Portd
    }
}
#[doc = "Field `PORTNEG` writer - Negative Port Select"]
pub type PortnegW<'a, REG> = crate::FieldWriter<'a, REG, 4, Portneg>;
impl<'a, REG> PortnegW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ground (single-ended)"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut crate::W<REG> {
        self.variant(Portneg::Gnd)
    }
    #[doc = "Direct connection to DAC0_CH1"]
    #[inline(always)]
    pub fn dac1(self) -> &'a mut crate::W<REG> {
        self.variant(Portneg::Dac1)
    }
    #[doc = "Port A - Select pin number using PINNEG"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Portneg::Porta)
    }
    #[doc = "Port B - Select pin number using PINNEG"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Portneg::Portb)
    }
    #[doc = "Port C - Select pin number using PINNEG"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Portneg::Portc)
    }
    #[doc = "Port D - Select pin number using PINNEG"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Portneg::Portd)
    }
}
#[doc = "Field `PINPOS` reader - Positive Pin Select"]
pub type PinposR = crate::FieldReader;
#[doc = "Field `PINPOS` writer - Positive Pin Select"]
pub type PinposW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Positive Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Portpos {
    #[doc = "0: Ground"]
    Gnd = 0,
    #[doc = "1: Supply Pin - Select specific supply using PINPOS"]
    Supply = 1,
    #[doc = "2: Direct connection to DAC0_CH0"]
    Dac0 = 2,
    #[doc = "8: Port A - Select pin number using PINPOS"]
    Porta = 8,
    #[doc = "9: Port B - Select pin number using PINPOS"]
    Portb = 9,
    #[doc = "10: Port C - Select pin number using PINPOS"]
    Portc = 10,
    #[doc = "11: Port D - Select pin number using PINPOS"]
    Portd = 11,
}
impl From<Portpos> for u8 {
    #[inline(always)]
    fn from(variant: Portpos) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Portpos {
    type Ux = u8;
}
impl crate::IsEnum for Portpos {}
#[doc = "Field `PORTPOS` reader - Positive Port Select"]
pub type PortposR = crate::FieldReader<Portpos>;
impl PortposR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Portpos> {
        match self.bits {
            0 => Some(Portpos::Gnd),
            1 => Some(Portpos::Supply),
            2 => Some(Portpos::Dac0),
            8 => Some(Portpos::Porta),
            9 => Some(Portpos::Portb),
            10 => Some(Portpos::Portc),
            11 => Some(Portpos::Portd),
            _ => None,
        }
    }
    #[doc = "Ground"]
    #[inline(always)]
    pub fn is_gnd(&self) -> bool {
        *self == Portpos::Gnd
    }
    #[doc = "Supply Pin - Select specific supply using PINPOS"]
    #[inline(always)]
    pub fn is_supply(&self) -> bool {
        *self == Portpos::Supply
    }
    #[doc = "Direct connection to DAC0_CH0"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        *self == Portpos::Dac0
    }
    #[doc = "Port A - Select pin number using PINPOS"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Portpos::Porta
    }
    #[doc = "Port B - Select pin number using PINPOS"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Portpos::Portb
    }
    #[doc = "Port C - Select pin number using PINPOS"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Portpos::Portc
    }
    #[doc = "Port D - Select pin number using PINPOS"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Portpos::Portd
    }
}
#[doc = "Field `PORTPOS` writer - Positive Port Select"]
pub type PortposW<'a, REG> = crate::FieldWriter<'a, REG, 4, Portpos>;
impl<'a, REG> PortposW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Ground"]
    #[inline(always)]
    pub fn gnd(self) -> &'a mut crate::W<REG> {
        self.variant(Portpos::Gnd)
    }
    #[doc = "Supply Pin - Select specific supply using PINPOS"]
    #[inline(always)]
    pub fn supply(self) -> &'a mut crate::W<REG> {
        self.variant(Portpos::Supply)
    }
    #[doc = "Direct connection to DAC0_CH0"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut crate::W<REG> {
        self.variant(Portpos::Dac0)
    }
    #[doc = "Port A - Select pin number using PINPOS"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Portpos::Porta)
    }
    #[doc = "Port B - Select pin number using PINPOS"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Portpos::Portb)
    }
    #[doc = "Port C - Select pin number using PINPOS"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Portpos::Portc)
    }
    #[doc = "Port D - Select pin number using PINPOS"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Portpos::Portd)
    }
}
#[doc = "Configuration Group Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfg {
    #[doc = "0: Use configuration group 0"]
    Config0 = 0,
    #[doc = "1: Use configuration group 1"]
    Config1 = 1,
}
impl From<Cfg> for bool {
    #[inline(always)]
    fn from(variant: Cfg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CFG` reader - Configuration Group Select"]
pub type CfgR = crate::BitReader<Cfg>;
impl CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfg {
        match self.bits {
            false => Cfg::Config0,
            true => Cfg::Config1,
        }
    }
    #[doc = "Use configuration group 0"]
    #[inline(always)]
    pub fn is_config0(&self) -> bool {
        *self == Cfg::Config0
    }
    #[doc = "Use configuration group 1"]
    #[inline(always)]
    pub fn is_config1(&self) -> bool {
        *self == Cfg::Config1
    }
}
#[doc = "Field `CFG` writer - Configuration Group Select"]
pub type CfgW<'a, REG> = crate::BitWriter<'a, REG, Cfg>;
impl<'a, REG> CfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Use configuration group 0"]
    #[inline(always)]
    pub fn config0(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg::Config0)
    }
    #[doc = "Use configuration group 1"]
    #[inline(always)]
    pub fn config1(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg::Config1)
    }
}
#[doc = "Field `CMP` reader - Comparison Enable"]
pub type CmpR = crate::BitReader;
#[doc = "Field `CMP` writer - Comparison Enable"]
pub type CmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Negative Pin Select"]
    #[inline(always)]
    pub fn pinneg(&self) -> PinnegR {
        PinnegR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Negative Port Select"]
    #[inline(always)]
    pub fn portneg(&self) -> PortnegR {
        PortnegR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Positive Pin Select"]
    #[inline(always)]
    pub fn pinpos(&self) -> PinposR {
        PinposR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Positive Port Select"]
    #[inline(always)]
    pub fn portpos(&self) -> PortposR {
        PortposR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Configuration Group Select"]
    #[inline(always)]
    pub fn cfg(&self) -> CfgR {
        CfgR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Comparison Enable"]
    #[inline(always)]
    pub fn cmp(&self) -> CmpR {
        CmpR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Negative Pin Select"]
    #[inline(always)]
    pub fn pinneg(&mut self) -> PinnegW<SingleSpec> {
        PinnegW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Negative Port Select"]
    #[inline(always)]
    pub fn portneg(&mut self) -> PortnegW<SingleSpec> {
        PortnegW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Positive Pin Select"]
    #[inline(always)]
    pub fn pinpos(&mut self) -> PinposW<SingleSpec> {
        PinposW::new(self, 8)
    }
    #[doc = "Bits 12:15 - Positive Port Select"]
    #[inline(always)]
    pub fn portpos(&mut self) -> PortposW<SingleSpec> {
        PortposW::new(self, 12)
    }
    #[doc = "Bit 16 - Configuration Group Select"]
    #[inline(always)]
    pub fn cfg(&mut self) -> CfgW<SingleSpec> {
        CfgW::new(self, 16)
    }
    #[doc = "Bit 17 - Comparison Enable"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CmpW<SingleSpec> {
        CmpW::new(self, 17)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`single::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`single::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SingleSpec;
impl crate::RegisterSpec for SingleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`single::R`](R) reader structure"]
impl crate::Readable for SingleSpec {}
#[doc = "`write(|w| ..)` method takes [`single::W`](W) writer structure"]
impl crate::Writable for SingleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SINGLE to value 0"]
impl crate::Resettable for SingleSpec {}
