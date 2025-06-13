#[doc = "Register `BACTRL` reader"]
pub type R = crate::R<BactrlSpec>;
#[doc = "Register `BACTRL` writer"]
pub type W = crate::W<BactrlSpec>;
#[doc = "Field `BLINKEN` reader - Blink Enable"]
pub type BlinkenR = crate::BitReader;
#[doc = "Field `BLINKEN` writer - Blink Enable"]
pub type BlinkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Blank Display\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Blank {
    #[doc = "0: Display is not \"blanked\""]
    Disable = 0,
    #[doc = "1: Display is \"blanked\""]
    Enable = 1,
}
impl From<Blank> for bool {
    #[inline(always)]
    fn from(variant: Blank) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BLANK` reader - Blank Display"]
pub type BlankR = crate::BitReader<Blank>;
impl BlankR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blank {
        match self.bits {
            false => Blank::Disable,
            true => Blank::Enable,
        }
    }
    #[doc = "Display is not \"blanked\""]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Blank::Disable
    }
    #[doc = "Display is \"blanked\""]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Blank::Enable
    }
}
#[doc = "Field `BLANK` writer - Blank Display"]
pub type BlankW<'a, REG> = crate::BitWriter<'a, REG, Blank>;
impl<'a, REG> BlankW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Display is not \"blanked\""]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Blank::Disable)
    }
    #[doc = "Display is \"blanked\""]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Blank::Enable)
    }
}
#[doc = "Field `AEN` reader - Animation Enable"]
pub type AenR = crate::BitReader;
#[doc = "Field `AEN` writer - Animation Enable"]
pub type AenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Animate Register A Shift Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aregasc {
    #[doc = "0: No Shift operation on Animation Register A"]
    Noshift = 0,
    #[doc = "1: Animation Register A is shifted left"]
    Shiftleft = 1,
    #[doc = "2: Animation Register A is shifted right"]
    Shiftright = 2,
}
impl From<Aregasc> for u8 {
    #[inline(always)]
    fn from(variant: Aregasc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aregasc {
    type Ux = u8;
}
impl crate::IsEnum for Aregasc {}
#[doc = "Field `AREGASC` reader - Animate Register A Shift Control"]
pub type AregascR = crate::FieldReader<Aregasc>;
impl AregascR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aregasc> {
        match self.bits {
            0 => Some(Aregasc::Noshift),
            1 => Some(Aregasc::Shiftleft),
            2 => Some(Aregasc::Shiftright),
            _ => None,
        }
    }
    #[doc = "No Shift operation on Animation Register A"]
    #[inline(always)]
    pub fn is_noshift(&self) -> bool {
        *self == Aregasc::Noshift
    }
    #[doc = "Animation Register A is shifted left"]
    #[inline(always)]
    pub fn is_shiftleft(&self) -> bool {
        *self == Aregasc::Shiftleft
    }
    #[doc = "Animation Register A is shifted right"]
    #[inline(always)]
    pub fn is_shiftright(&self) -> bool {
        *self == Aregasc::Shiftright
    }
}
#[doc = "Field `AREGASC` writer - Animate Register A Shift Control"]
pub type AregascW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aregasc>;
impl<'a, REG> AregascW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Shift operation on Animation Register A"]
    #[inline(always)]
    pub fn noshift(self) -> &'a mut crate::W<REG> {
        self.variant(Aregasc::Noshift)
    }
    #[doc = "Animation Register A is shifted left"]
    #[inline(always)]
    pub fn shiftleft(self) -> &'a mut crate::W<REG> {
        self.variant(Aregasc::Shiftleft)
    }
    #[doc = "Animation Register A is shifted right"]
    #[inline(always)]
    pub fn shiftright(self) -> &'a mut crate::W<REG> {
        self.variant(Aregasc::Shiftright)
    }
}
#[doc = "Animate Register B Shift Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Aregbsc {
    #[doc = "0: No Shift operation on Animation Register B"]
    Noshift = 0,
    #[doc = "1: Animation Register B is shifted left"]
    Shiftleft = 1,
    #[doc = "2: Animation Register B is shifted right"]
    Shiftright = 2,
}
impl From<Aregbsc> for u8 {
    #[inline(always)]
    fn from(variant: Aregbsc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Aregbsc {
    type Ux = u8;
}
impl crate::IsEnum for Aregbsc {}
#[doc = "Field `AREGBSC` reader - Animate Register B Shift Control"]
pub type AregbscR = crate::FieldReader<Aregbsc>;
impl AregbscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Aregbsc> {
        match self.bits {
            0 => Some(Aregbsc::Noshift),
            1 => Some(Aregbsc::Shiftleft),
            2 => Some(Aregbsc::Shiftright),
            _ => None,
        }
    }
    #[doc = "No Shift operation on Animation Register B"]
    #[inline(always)]
    pub fn is_noshift(&self) -> bool {
        *self == Aregbsc::Noshift
    }
    #[doc = "Animation Register B is shifted left"]
    #[inline(always)]
    pub fn is_shiftleft(&self) -> bool {
        *self == Aregbsc::Shiftleft
    }
    #[doc = "Animation Register B is shifted right"]
    #[inline(always)]
    pub fn is_shiftright(&self) -> bool {
        *self == Aregbsc::Shiftright
    }
}
#[doc = "Field `AREGBSC` writer - Animate Register B Shift Control"]
pub type AregbscW<'a, REG> = crate::FieldWriter<'a, REG, 2, Aregbsc>;
impl<'a, REG> AregbscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Shift operation on Animation Register B"]
    #[inline(always)]
    pub fn noshift(self) -> &'a mut crate::W<REG> {
        self.variant(Aregbsc::Noshift)
    }
    #[doc = "Animation Register B is shifted left"]
    #[inline(always)]
    pub fn shiftleft(self) -> &'a mut crate::W<REG> {
        self.variant(Aregbsc::Shiftleft)
    }
    #[doc = "Animation Register B is shifted right"]
    #[inline(always)]
    pub fn shiftright(self) -> &'a mut crate::W<REG> {
        self.variant(Aregbsc::Shiftright)
    }
}
#[doc = "Animate Logic Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alogsel {
    #[doc = "0: AREGA and AREGB AND'ed"]
    And = 0,
    #[doc = "1: AREGA and AREGB OR'ed"]
    Or = 1,
}
impl From<Alogsel> for bool {
    #[inline(always)]
    fn from(variant: Alogsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALOGSEL` reader - Animate Logic Function Select"]
pub type AlogselR = crate::BitReader<Alogsel>;
impl AlogselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Alogsel {
        match self.bits {
            false => Alogsel::And,
            true => Alogsel::Or,
        }
    }
    #[doc = "AREGA and AREGB AND'ed"]
    #[inline(always)]
    pub fn is_and(&self) -> bool {
        *self == Alogsel::And
    }
    #[doc = "AREGA and AREGB OR'ed"]
    #[inline(always)]
    pub fn is_or(&self) -> bool {
        *self == Alogsel::Or
    }
}
#[doc = "Field `ALOGSEL` writer - Animate Logic Function Select"]
pub type AlogselW<'a, REG> = crate::BitWriter<'a, REG, Alogsel>;
impl<'a, REG> AlogselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "AREGA and AREGB AND'ed"]
    #[inline(always)]
    pub fn and(self) -> &'a mut crate::W<REG> {
        self.variant(Alogsel::And)
    }
    #[doc = "AREGA and AREGB OR'ed"]
    #[inline(always)]
    pub fn or(self) -> &'a mut crate::W<REG> {
        self.variant(Alogsel::Or)
    }
}
#[doc = "Field `FCEN` reader - Frame Counter Enable"]
pub type FcenR = crate::BitReader;
#[doc = "Field `FCEN` writer - Frame Counter Enable"]
pub type FcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Display Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Displaycnten {
    #[doc = "0: Disable the display counter"]
    Disable = 0,
    #[doc = "1: Enable the display counter"]
    Enable = 1,
}
impl From<Displaycnten> for bool {
    #[inline(always)]
    fn from(variant: Displaycnten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISPLAYCNTEN` reader - Display Counter Enable"]
pub type DisplaycntenR = crate::BitReader<Displaycnten>;
impl DisplaycntenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Displaycnten {
        match self.bits {
            false => Displaycnten::Disable,
            true => Displaycnten::Enable,
        }
    }
    #[doc = "Disable the display counter"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Displaycnten::Disable
    }
    #[doc = "Enable the display counter"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Displaycnten::Enable
    }
}
#[doc = "Field `DISPLAYCNTEN` writer - Display Counter Enable"]
pub type DisplaycntenW<'a, REG> = crate::BitWriter<'a, REG, Displaycnten>;
impl<'a, REG> DisplaycntenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the display counter"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Displaycnten::Disable)
    }
    #[doc = "Enable the display counter"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Displaycnten::Enable)
    }
}
#[doc = "Animation Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aloc {
    #[doc = "0: Animation appears on segments 0 to 7"]
    Seg0to7 = 0,
    #[doc = "1: Animation appears on segments 8 to 15"]
    Seg8to15 = 1,
}
impl From<Aloc> for bool {
    #[inline(always)]
    fn from(variant: Aloc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALOC` reader - Animation Location"]
pub type AlocR = crate::BitReader<Aloc>;
impl AlocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aloc {
        match self.bits {
            false => Aloc::Seg0to7,
            true => Aloc::Seg8to15,
        }
    }
    #[doc = "Animation appears on segments 0 to 7"]
    #[inline(always)]
    pub fn is_seg0to7(&self) -> bool {
        *self == Aloc::Seg0to7
    }
    #[doc = "Animation appears on segments 8 to 15"]
    #[inline(always)]
    pub fn is_seg8to15(&self) -> bool {
        *self == Aloc::Seg8to15
    }
}
#[doc = "Field `ALOC` writer - Animation Location"]
pub type AlocW<'a, REG> = crate::BitWriter<'a, REG, Aloc>;
impl<'a, REG> AlocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Animation appears on segments 0 to 7"]
    #[inline(always)]
    pub fn seg0to7(self) -> &'a mut crate::W<REG> {
        self.variant(Aloc::Seg0to7)
    }
    #[doc = "Animation appears on segments 8 to 15"]
    #[inline(always)]
    pub fn seg8to15(self) -> &'a mut crate::W<REG> {
        self.variant(Aloc::Seg8to15)
    }
}
impl R {
    #[doc = "Bit 0 - Blink Enable"]
    #[inline(always)]
    pub fn blinken(&self) -> BlinkenR {
        BlinkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Blank Display"]
    #[inline(always)]
    pub fn blank(&self) -> BlankR {
        BlankR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Animation Enable"]
    #[inline(always)]
    pub fn aen(&self) -> AenR {
        AenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Animate Register A Shift Control"]
    #[inline(always)]
    pub fn aregasc(&self) -> AregascR {
        AregascR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Animate Register B Shift Control"]
    #[inline(always)]
    pub fn aregbsc(&self) -> AregbscR {
        AregbscR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Animate Logic Function Select"]
    #[inline(always)]
    pub fn alogsel(&self) -> AlogselR {
        AlogselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Frame Counter Enable"]
    #[inline(always)]
    pub fn fcen(&self) -> FcenR {
        FcenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Display Counter Enable"]
    #[inline(always)]
    pub fn displaycnten(&self) -> DisplaycntenR {
        DisplaycntenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 28 - Animation Location"]
    #[inline(always)]
    pub fn aloc(&self) -> AlocR {
        AlocR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Blink Enable"]
    #[inline(always)]
    pub fn blinken(&mut self) -> BlinkenW<BactrlSpec> {
        BlinkenW::new(self, 0)
    }
    #[doc = "Bit 1 - Blank Display"]
    #[inline(always)]
    pub fn blank(&mut self) -> BlankW<BactrlSpec> {
        BlankW::new(self, 1)
    }
    #[doc = "Bit 2 - Animation Enable"]
    #[inline(always)]
    pub fn aen(&mut self) -> AenW<BactrlSpec> {
        AenW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Animate Register A Shift Control"]
    #[inline(always)]
    pub fn aregasc(&mut self) -> AregascW<BactrlSpec> {
        AregascW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Animate Register B Shift Control"]
    #[inline(always)]
    pub fn aregbsc(&mut self) -> AregbscW<BactrlSpec> {
        AregbscW::new(self, 5)
    }
    #[doc = "Bit 7 - Animate Logic Function Select"]
    #[inline(always)]
    pub fn alogsel(&mut self) -> AlogselW<BactrlSpec> {
        AlogselW::new(self, 7)
    }
    #[doc = "Bit 8 - Frame Counter Enable"]
    #[inline(always)]
    pub fn fcen(&mut self) -> FcenW<BactrlSpec> {
        FcenW::new(self, 8)
    }
    #[doc = "Bit 9 - Display Counter Enable"]
    #[inline(always)]
    pub fn displaycnten(&mut self) -> DisplaycntenW<BactrlSpec> {
        DisplaycntenW::new(self, 9)
    }
    #[doc = "Bit 28 - Animation Location"]
    #[inline(always)]
    pub fn aloc(&mut self) -> AlocW<BactrlSpec> {
        AlocW::new(self, 28)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`bactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BactrlSpec;
impl crate::RegisterSpec for BactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bactrl::R`](R) reader structure"]
impl crate::Readable for BactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bactrl::W`](W) writer structure"]
impl crate::Writable for BactrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BACTRL to value 0"]
impl crate::Resettable for BactrlSpec {}
