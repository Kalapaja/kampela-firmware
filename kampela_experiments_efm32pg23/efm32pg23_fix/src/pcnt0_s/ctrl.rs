#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `S1CDIR` reader - Count Direction Determined By S1"]
pub type S1cdirR = crate::BitReader;
#[doc = "Field `S1CDIR` writer - Count Direction Determined By S1"]
pub type S1cdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Non-Quadrature Mode Counter Direction Co\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cntdir {
    #[doc = "0: Up counter mode."]
    Up = 0,
    #[doc = "1: Down counter mode."]
    Down = 1,
}
impl From<Cntdir> for bool {
    #[inline(always)]
    fn from(variant: Cntdir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTDIR` reader - Non-Quadrature Mode Counter Direction Co"]
pub type CntdirR = crate::BitReader<Cntdir>;
impl CntdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntdir {
        match self.bits {
            false => Cntdir::Up,
            true => Cntdir::Down,
        }
    }
    #[doc = "Up counter mode."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Cntdir::Up
    }
    #[doc = "Down counter mode."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Cntdir::Down
    }
}
#[doc = "Field `CNTDIR` writer - Non-Quadrature Mode Counter Direction Co"]
pub type CntdirW<'a, REG> = crate::BitWriter<'a, REG, Cntdir>;
impl<'a, REG> CntdirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Up counter mode."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Cntdir::Up)
    }
    #[doc = "Down counter mode."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Cntdir::Down)
    }
}
#[doc = "Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    #[doc = "0: Positive edges on the PCNTn_S0IN inputs are counted in OVSSINGLE mode. Does not invert PCNTn_S1IN input in OVSSINGLE and EXTCLKSINGLE modes"]
    Pos = 0,
    #[doc = "1: Negative edges on the PCNTn_S0IN inputs are counted in OVSSINGLE mode. Inverts the PCNTn_S1IN input in OVSSINGLE and EXTCLKSINGLE modes"]
    Neg = 1,
}
impl From<Edge> for bool {
    #[inline(always)]
    fn from(variant: Edge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDGE` reader - Edge Select"]
pub type EdgeR = crate::BitReader<Edge>;
impl EdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edge {
        match self.bits {
            false => Edge::Pos,
            true => Edge::Neg,
        }
    }
    #[doc = "Positive edges on the PCNTn_S0IN inputs are counted in OVSSINGLE mode. Does not invert PCNTn_S1IN input in OVSSINGLE and EXTCLKSINGLE modes"]
    #[inline(always)]
    pub fn is_pos(&self) -> bool {
        *self == Edge::Pos
    }
    #[doc = "Negative edges on the PCNTn_S0IN inputs are counted in OVSSINGLE mode. Inverts the PCNTn_S1IN input in OVSSINGLE and EXTCLKSINGLE modes"]
    #[inline(always)]
    pub fn is_neg(&self) -> bool {
        *self == Edge::Neg
    }
}
#[doc = "Field `EDGE` writer - Edge Select"]
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG, Edge>;
impl<'a, REG> EdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Positive edges on the PCNTn_S0IN inputs are counted in OVSSINGLE mode. Does not invert PCNTn_S1IN input in OVSSINGLE and EXTCLKSINGLE modes"]
    #[inline(always)]
    pub fn pos(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::Pos)
    }
    #[doc = "Negative edges on the PCNTn_S0IN inputs are counted in OVSSINGLE mode. Inverts the PCNTn_S1IN input in OVSSINGLE and EXTCLKSINGLE modes"]
    #[inline(always)]
    pub fn neg(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::Neg)
    }
}
#[doc = "Controls When the Counter Counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cntev {
    #[doc = "0: Counts up on up-count and down on down-count events."]
    Both = 0,
    #[doc = "1: Only counts up on up-count events."]
    Up = 1,
    #[doc = "2: Only counts down on down-count events."]
    Down = 2,
}
impl From<Cntev> for u8 {
    #[inline(always)]
    fn from(variant: Cntev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cntev {
    type Ux = u8;
}
impl crate::IsEnum for Cntev {}
#[doc = "Field `CNTEV` reader - Controls When the Counter Counts"]
pub type CntevR = crate::FieldReader<Cntev>;
impl CntevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cntev> {
        match self.bits {
            0 => Some(Cntev::Both),
            1 => Some(Cntev::Up),
            2 => Some(Cntev::Down),
            _ => None,
        }
    }
    #[doc = "Counts up on up-count and down on down-count events."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Cntev::Both
    }
    #[doc = "Only counts up on up-count events."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Cntev::Up
    }
    #[doc = "Only counts down on down-count events."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Cntev::Down
    }
}
#[doc = "Field `CNTEV` writer - Controls When the Counter Counts"]
pub type CntevW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cntev>;
impl<'a, REG> CntevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counts up on up-count and down on down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Cntev::Both)
    }
    #[doc = "Only counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Cntev::Up)
    }
    #[doc = "Only counts down on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Cntev::Down)
    }
}
#[doc = "Controls When the Aux Counter Counts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Auxcntev {
    #[doc = "0: Counts up on both up-count and down-count events."]
    Both = 0,
    #[doc = "1: Counts up on up-count events."]
    Up = 1,
    #[doc = "2: Counts up on down-count events."]
    Down = 2,
}
impl From<Auxcntev> for u8 {
    #[inline(always)]
    fn from(variant: Auxcntev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Auxcntev {
    type Ux = u8;
}
impl crate::IsEnum for Auxcntev {}
#[doc = "Field `AUXCNTEV` reader - Controls When the Aux Counter Counts"]
pub type AuxcntevR = crate::FieldReader<Auxcntev>;
impl AuxcntevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Auxcntev> {
        match self.bits {
            0 => Some(Auxcntev::Both),
            1 => Some(Auxcntev::Up),
            2 => Some(Auxcntev::Down),
            _ => None,
        }
    }
    #[doc = "Counts up on both up-count and down-count events."]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Auxcntev::Both
    }
    #[doc = "Counts up on up-count events."]
    #[inline(always)]
    pub fn is_up(&self) -> bool {
        *self == Auxcntev::Up
    }
    #[doc = "Counts up on down-count events."]
    #[inline(always)]
    pub fn is_down(&self) -> bool {
        *self == Auxcntev::Down
    }
}
#[doc = "Field `AUXCNTEV` writer - Controls When the Aux Counter Counts"]
pub type AuxcntevW<'a, REG> = crate::FieldWriter<'a, REG, 2, Auxcntev>;
impl<'a, REG> AuxcntevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Counts up on both up-count and down-count events."]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Auxcntev::Both)
    }
    #[doc = "Counts up on up-count events."]
    #[inline(always)]
    pub fn up(self) -> &'a mut crate::W<REG> {
        self.variant(Auxcntev::Up)
    }
    #[doc = "Counts up on down-count events."]
    #[inline(always)]
    pub fn down(self) -> &'a mut crate::W<REG> {
        self.variant(Auxcntev::Down)
    }
}
impl R {
    #[doc = "Bit 0 - Count Direction Determined By S1"]
    #[inline(always)]
    pub fn s1cdir(&self) -> S1cdirR {
        S1cdirR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Non-Quadrature Mode Counter Direction Co"]
    #[inline(always)]
    pub fn cntdir(&self) -> CntdirR {
        CntdirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Edge Select"]
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Controls When the Counter Counts"]
    #[inline(always)]
    pub fn cntev(&self) -> CntevR {
        CntevR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Controls When the Aux Counter Counts"]
    #[inline(always)]
    pub fn auxcntev(&self) -> AuxcntevR {
        AuxcntevR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Count Direction Determined By S1"]
    #[inline(always)]
    pub fn s1cdir(&mut self) -> S1cdirW<CtrlSpec> {
        S1cdirW::new(self, 0)
    }
    #[doc = "Bit 1 - Non-Quadrature Mode Counter Direction Co"]
    #[inline(always)]
    pub fn cntdir(&mut self) -> CntdirW<CtrlSpec> {
        CntdirW::new(self, 1)
    }
    #[doc = "Bit 2 - Edge Select"]
    #[inline(always)]
    pub fn edge(&mut self) -> EdgeW<CtrlSpec> {
        EdgeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Controls When the Counter Counts"]
    #[inline(always)]
    pub fn cntev(&mut self) -> CntevW<CtrlSpec> {
        CntevW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Controls When the Aux Counter Counts"]
    #[inline(always)]
    pub fn auxcntev(&mut self) -> AuxcntevW<CtrlSpec> {
        AuxcntevW::new(self, 6)
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
