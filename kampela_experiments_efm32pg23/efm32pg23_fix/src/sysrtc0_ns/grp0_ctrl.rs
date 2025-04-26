#[doc = "Register `GRP0_CTRL` reader"]
pub type R = crate::R<Grp0CtrlSpec>;
#[doc = "Register `GRP0_CTRL` writer"]
pub type W = crate::W<Grp0CtrlSpec>;
#[doc = "Field `CMP0EN` reader - Compare 0 Enable"]
pub type Cmp0enR = crate::BitReader;
#[doc = "Field `CMP0EN` writer - Compare 0 Enable"]
pub type Cmp0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1EN` reader - Compare 1 Enable"]
pub type Cmp1enR = crate::BitReader;
#[doc = "Field `CMP1EN` writer - Compare 1 Enable"]
pub type Cmp1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0EN` reader - Capture 0 Enable"]
pub type Cap0enR = crate::BitReader;
#[doc = "Field `CAP0EN` writer - Capture 0 Enable"]
pub type Cap0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Compare 0 Compare Match Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmp0cmoa {
    #[doc = "0: Cleared on the next cycle"]
    Clear = 0,
    #[doc = "1: Set on the next cycle"]
    Set = 1,
    #[doc = "2: Set on the next cycle, cleared on the cycle after"]
    Pulse = 2,
    #[doc = "3: Inverted on the next cycle"]
    Toggle = 3,
    #[doc = "4: Export this channel's CMP IF"]
    Cmpif = 4,
}
impl From<Cmp0cmoa> for u8 {
    #[inline(always)]
    fn from(variant: Cmp0cmoa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmp0cmoa {
    type Ux = u8;
}
impl crate::IsEnum for Cmp0cmoa {}
#[doc = "Field `CMP0CMOA` reader - Compare 0 Compare Match Output Action"]
pub type Cmp0cmoaR = crate::FieldReader<Cmp0cmoa>;
impl Cmp0cmoaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmp0cmoa> {
        match self.bits {
            0 => Some(Cmp0cmoa::Clear),
            1 => Some(Cmp0cmoa::Set),
            2 => Some(Cmp0cmoa::Pulse),
            3 => Some(Cmp0cmoa::Toggle),
            4 => Some(Cmp0cmoa::Cmpif),
            _ => None,
        }
    }
    #[doc = "Cleared on the next cycle"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Cmp0cmoa::Clear
    }
    #[doc = "Set on the next cycle"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cmp0cmoa::Set
    }
    #[doc = "Set on the next cycle, cleared on the cycle after"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == Cmp0cmoa::Pulse
    }
    #[doc = "Inverted on the next cycle"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Cmp0cmoa::Toggle
    }
    #[doc = "Export this channel's CMP IF"]
    #[inline(always)]
    pub fn is_cmpif(&self) -> bool {
        *self == Cmp0cmoa::Cmpif
    }
}
#[doc = "Field `CMP0CMOA` writer - Compare 0 Compare Match Output Action"]
pub type Cmp0cmoaW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cmp0cmoa>;
impl<'a, REG> Cmp0cmoaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Cleared on the next cycle"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0cmoa::Clear)
    }
    #[doc = "Set on the next cycle"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0cmoa::Set)
    }
    #[doc = "Set on the next cycle, cleared on the cycle after"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0cmoa::Pulse)
    }
    #[doc = "Inverted on the next cycle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0cmoa::Toggle)
    }
    #[doc = "Export this channel's CMP IF"]
    #[inline(always)]
    pub fn cmpif(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp0cmoa::Cmpif)
    }
}
#[doc = "Compare 1 Compare Match Output Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmp1cmoa {
    #[doc = "0: Cleared on the next cycle"]
    Clear = 0,
    #[doc = "1: Set on the next cycle"]
    Set = 1,
    #[doc = "2: Set on the next cycle, cleared on the cycle after"]
    Pulse = 2,
    #[doc = "3: Inverted on the next cycle"]
    Toggle = 3,
    #[doc = "4: Export this channel's CMP IF"]
    Cmpif = 4,
}
impl From<Cmp1cmoa> for u8 {
    #[inline(always)]
    fn from(variant: Cmp1cmoa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmp1cmoa {
    type Ux = u8;
}
impl crate::IsEnum for Cmp1cmoa {}
#[doc = "Field `CMP1CMOA` reader - Compare 1 Compare Match Output Action"]
pub type Cmp1cmoaR = crate::FieldReader<Cmp1cmoa>;
impl Cmp1cmoaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cmp1cmoa> {
        match self.bits {
            0 => Some(Cmp1cmoa::Clear),
            1 => Some(Cmp1cmoa::Set),
            2 => Some(Cmp1cmoa::Pulse),
            3 => Some(Cmp1cmoa::Toggle),
            4 => Some(Cmp1cmoa::Cmpif),
            _ => None,
        }
    }
    #[doc = "Cleared on the next cycle"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Cmp1cmoa::Clear
    }
    #[doc = "Set on the next cycle"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cmp1cmoa::Set
    }
    #[doc = "Set on the next cycle, cleared on the cycle after"]
    #[inline(always)]
    pub fn is_pulse(&self) -> bool {
        *self == Cmp1cmoa::Pulse
    }
    #[doc = "Inverted on the next cycle"]
    #[inline(always)]
    pub fn is_toggle(&self) -> bool {
        *self == Cmp1cmoa::Toggle
    }
    #[doc = "Export this channel's CMP IF"]
    #[inline(always)]
    pub fn is_cmpif(&self) -> bool {
        *self == Cmp1cmoa::Cmpif
    }
}
#[doc = "Field `CMP1CMOA` writer - Compare 1 Compare Match Output Action"]
pub type Cmp1cmoaW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cmp1cmoa>;
impl<'a, REG> Cmp1cmoaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Cleared on the next cycle"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp1cmoa::Clear)
    }
    #[doc = "Set on the next cycle"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp1cmoa::Set)
    }
    #[doc = "Set on the next cycle, cleared on the cycle after"]
    #[inline(always)]
    pub fn pulse(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp1cmoa::Pulse)
    }
    #[doc = "Inverted on the next cycle"]
    #[inline(always)]
    pub fn toggle(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp1cmoa::Toggle)
    }
    #[doc = "Export this channel's CMP IF"]
    #[inline(always)]
    pub fn cmpif(self) -> &'a mut crate::W<REG> {
        self.variant(Cmp1cmoa::Cmpif)
    }
}
#[doc = "Capture 0 Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cap0edge {
    #[doc = "0: Rising edges detected"]
    Rising = 0,
    #[doc = "1: Falling edges detected"]
    Falling = 1,
    #[doc = "2: Both edges detected"]
    Both = 2,
}
impl From<Cap0edge> for u8 {
    #[inline(always)]
    fn from(variant: Cap0edge) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cap0edge {
    type Ux = u8;
}
impl crate::IsEnum for Cap0edge {}
#[doc = "Field `CAP0EDGE` reader - Capture 0 Edge Select"]
pub type Cap0edgeR = crate::FieldReader<Cap0edge>;
impl Cap0edgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cap0edge> {
        match self.bits {
            0 => Some(Cap0edge::Rising),
            1 => Some(Cap0edge::Falling),
            2 => Some(Cap0edge::Both),
            _ => None,
        }
    }
    #[doc = "Rising edges detected"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Cap0edge::Rising
    }
    #[doc = "Falling edges detected"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Cap0edge::Falling
    }
    #[doc = "Both edges detected"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Cap0edge::Both
    }
}
#[doc = "Field `CAP0EDGE` writer - Capture 0 Edge Select"]
pub type Cap0edgeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cap0edge>;
impl<'a, REG> Cap0edgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Rising edges detected"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0edge::Rising)
    }
    #[doc = "Falling edges detected"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0edge::Falling)
    }
    #[doc = "Both edges detected"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0edge::Both)
    }
}
impl R {
    #[doc = "Bit 0 - Compare 0 Enable"]
    #[inline(always)]
    pub fn cmp0en(&self) -> Cmp0enR {
        Cmp0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 1 Enable"]
    #[inline(always)]
    pub fn cmp1en(&self) -> Cmp1enR {
        Cmp1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture 0 Enable"]
    #[inline(always)]
    pub fn cap0en(&self) -> Cap0enR {
        Cap0enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Compare 0 Compare Match Output Action"]
    #[inline(always)]
    pub fn cmp0cmoa(&self) -> Cmp0cmoaR {
        Cmp0cmoaR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Compare 1 Compare Match Output Action"]
    #[inline(always)]
    pub fn cmp1cmoa(&self) -> Cmp1cmoaR {
        Cmp1cmoaR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:10 - Capture 0 Edge Select"]
    #[inline(always)]
    pub fn cap0edge(&self) -> Cap0edgeR {
        Cap0edgeR::new(((self.bits >> 9) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Compare 0 Enable"]
    #[inline(always)]
    pub fn cmp0en(&mut self) -> Cmp0enW<Grp0CtrlSpec> {
        Cmp0enW::new(self, 0)
    }
    #[doc = "Bit 1 - Compare 1 Enable"]
    #[inline(always)]
    pub fn cmp1en(&mut self) -> Cmp1enW<Grp0CtrlSpec> {
        Cmp1enW::new(self, 1)
    }
    #[doc = "Bit 2 - Capture 0 Enable"]
    #[inline(always)]
    pub fn cap0en(&mut self) -> Cap0enW<Grp0CtrlSpec> {
        Cap0enW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Compare 0 Compare Match Output Action"]
    #[inline(always)]
    pub fn cmp0cmoa(&mut self) -> Cmp0cmoaW<Grp0CtrlSpec> {
        Cmp0cmoaW::new(self, 3)
    }
    #[doc = "Bits 6:8 - Compare 1 Compare Match Output Action"]
    #[inline(always)]
    pub fn cmp1cmoa(&mut self) -> Cmp1cmoaW<Grp0CtrlSpec> {
        Cmp1cmoaW::new(self, 6)
    }
    #[doc = "Bits 9:10 - Capture 0 Edge Select"]
    #[inline(always)]
    pub fn cap0edge(&mut self) -> Cap0edgeW<Grp0CtrlSpec> {
        Cap0edgeW::new(self, 9)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`grp0_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp0_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Grp0CtrlSpec;
impl crate::RegisterSpec for Grp0CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp0_ctrl::R`](R) reader structure"]
impl crate::Readable for Grp0CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`grp0_ctrl::W`](W) writer structure"]
impl crate::Writable for Grp0CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP0_CTRL to value 0"]
impl crate::Resettable for Grp0CtrlSpec {}
