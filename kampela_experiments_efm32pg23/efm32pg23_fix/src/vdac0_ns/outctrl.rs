#[doc = "Register `OUTCTRL` reader"]
pub type R = crate::R<OutctrlSpec>;
#[doc = "Register `OUTCTRL` writer"]
pub type W = crate::W<OutctrlSpec>;
#[doc = "Field `MAINOUTENCH0` reader - CH0 Main Output Enable"]
pub type Mainoutench0R = crate::BitReader;
#[doc = "Field `MAINOUTENCH0` writer - CH0 Main Output Enable"]
pub type Mainoutench0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAINOUTENCH1` reader - CH1 Main Output Enable"]
pub type Mainoutench1R = crate::BitReader;
#[doc = "Field `MAINOUTENCH1` writer - CH1 Main Output Enable"]
pub type Mainoutench1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXOUTENCH0` reader - CH0 Alternative Output Enable"]
pub type Auxoutench0R = crate::BitReader;
#[doc = "Field `AUXOUTENCH0` writer - CH0 Alternative Output Enable"]
pub type Auxoutench0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUXOUTENCH1` reader - CH1 Alternative Output Enable"]
pub type Auxoutench1R = crate::BitReader;
#[doc = "Field `AUXOUTENCH1` writer - CH1 Alternative Output Enable"]
pub type Auxoutench1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTCH0` reader - CH1 Main and Alternative Output Short"]
pub type Shortch0R = crate::BitReader;
#[doc = "Field `SHORTCH0` writer - CH1 Main and Alternative Output Short"]
pub type Shortch0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHORTCH1` reader - CH0 Main and Alternative Output Short"]
pub type Shortch1R = crate::BitReader;
#[doc = "Field `SHORTCH1` writer - CH0 Main and Alternative Output Short"]
pub type Shortch1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "CH0 ABUS Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Abusportselch0 {
    #[doc = "0: No GPIO Selected for CH0 ABUS Output"]
    None = 0,
    #[doc = "1: Port A Selected"]
    Porta = 1,
    #[doc = "2: Port B Selected"]
    Portb = 2,
    #[doc = "3: Port C Selected"]
    Portc = 3,
    #[doc = "4: Port D Selected"]
    Portd = 4,
}
impl From<Abusportselch0> for u8 {
    #[inline(always)]
    fn from(variant: Abusportselch0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Abusportselch0 {
    type Ux = u8;
}
impl crate::IsEnum for Abusportselch0 {}
#[doc = "Field `ABUSPORTSELCH0` reader - CH0 ABUS Port Select"]
pub type Abusportselch0R = crate::FieldReader<Abusportselch0>;
impl Abusportselch0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Abusportselch0> {
        match self.bits {
            0 => Some(Abusportselch0::None),
            1 => Some(Abusportselch0::Porta),
            2 => Some(Abusportselch0::Portb),
            3 => Some(Abusportselch0::Portc),
            4 => Some(Abusportselch0::Portd),
            _ => None,
        }
    }
    #[doc = "No GPIO Selected for CH0 ABUS Output"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Abusportselch0::None
    }
    #[doc = "Port A Selected"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Abusportselch0::Porta
    }
    #[doc = "Port B Selected"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Abusportselch0::Portb
    }
    #[doc = "Port C Selected"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Abusportselch0::Portc
    }
    #[doc = "Port D Selected"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Abusportselch0::Portd
    }
}
#[doc = "Field `ABUSPORTSELCH0` writer - CH0 ABUS Port Select"]
pub type Abusportselch0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Abusportselch0>;
impl<'a, REG> Abusportselch0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No GPIO Selected for CH0 ABUS Output"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Abusportselch0::None)
    }
    #[doc = "Port A Selected"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Abusportselch0::Porta)
    }
    #[doc = "Port B Selected"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Abusportselch0::Portb)
    }
    #[doc = "Port C Selected"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Abusportselch0::Portc)
    }
    #[doc = "Port D Selected"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Abusportselch0::Portd)
    }
}
#[doc = "Field `ABUSPINSELCH0` reader - CH0 ABUS Pin Select"]
pub type Abuspinselch0R = crate::FieldReader;
#[doc = "Field `ABUSPINSELCH0` writer - CH0 ABUS Pin Select"]
pub type Abuspinselch0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "CH1 ABUS Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Abusportselch1 {
    #[doc = "0: No GPIO Selected for CH1 ABUS Output"]
    None = 0,
    #[doc = "1: Port A Selected"]
    Porta = 1,
    #[doc = "2: Port B Selected"]
    Portb = 2,
    #[doc = "3: Port C Selected"]
    Portc = 3,
    #[doc = "4: Port D Selected"]
    Portd = 4,
}
impl From<Abusportselch1> for u8 {
    #[inline(always)]
    fn from(variant: Abusportselch1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Abusportselch1 {
    type Ux = u8;
}
impl crate::IsEnum for Abusportselch1 {}
#[doc = "Field `ABUSPORTSELCH1` reader - CH1 ABUS Port Select"]
pub type Abusportselch1R = crate::FieldReader<Abusportselch1>;
impl Abusportselch1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Abusportselch1> {
        match self.bits {
            0 => Some(Abusportselch1::None),
            1 => Some(Abusportselch1::Porta),
            2 => Some(Abusportselch1::Portb),
            3 => Some(Abusportselch1::Portc),
            4 => Some(Abusportselch1::Portd),
            _ => None,
        }
    }
    #[doc = "No GPIO Selected for CH1 ABUS Output"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Abusportselch1::None
    }
    #[doc = "Port A Selected"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Abusportselch1::Porta
    }
    #[doc = "Port B Selected"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Abusportselch1::Portb
    }
    #[doc = "Port C Selected"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Abusportselch1::Portc
    }
    #[doc = "Port D Selected"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Abusportselch1::Portd
    }
}
#[doc = "Field `ABUSPORTSELCH1` writer - CH1 ABUS Port Select"]
pub type Abusportselch1W<'a, REG> = crate::FieldWriter<'a, REG, 3, Abusportselch1>;
impl<'a, REG> Abusportselch1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No GPIO Selected for CH1 ABUS Output"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Abusportselch1::None)
    }
    #[doc = "Port A Selected"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Abusportselch1::Porta)
    }
    #[doc = "Port B Selected"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Abusportselch1::Portb)
    }
    #[doc = "Port C Selected"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Abusportselch1::Portc)
    }
    #[doc = "Port D Selected"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Abusportselch1::Portd)
    }
}
#[doc = "Field `ABUSPINSELCH1` reader - CH1 ABUS Pin Select"]
pub type Abuspinselch1R = crate::FieldReader;
#[doc = "Field `ABUSPINSELCH1` writer - CH1 ABUS Pin Select"]
pub type Abuspinselch1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - CH0 Main Output Enable"]
    #[inline(always)]
    pub fn mainoutench0(&self) -> Mainoutench0R {
        Mainoutench0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1 Main Output Enable"]
    #[inline(always)]
    pub fn mainoutench1(&self) -> Mainoutench1R {
        Mainoutench1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - CH0 Alternative Output Enable"]
    #[inline(always)]
    pub fn auxoutench0(&self) -> Auxoutench0R {
        Auxoutench0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH1 Alternative Output Enable"]
    #[inline(always)]
    pub fn auxoutench1(&self) -> Auxoutench1R {
        Auxoutench1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CH1 Main and Alternative Output Short"]
    #[inline(always)]
    pub fn shortch0(&self) -> Shortch0R {
        Shortch0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CH0 Main and Alternative Output Short"]
    #[inline(always)]
    pub fn shortch1(&self) -> Shortch1R {
        Shortch1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:14 - CH0 ABUS Port Select"]
    #[inline(always)]
    pub fn abusportselch0(&self) -> Abusportselch0R {
        Abusportselch0R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:20 - CH0 ABUS Pin Select"]
    #[inline(always)]
    pub fn abuspinselch0(&self) -> Abuspinselch0R {
        Abuspinselch0R::new(((self.bits >> 15) & 0x3f) as u8)
    }
    #[doc = "Bits 22:24 - CH1 ABUS Port Select"]
    #[inline(always)]
    pub fn abusportselch1(&self) -> Abusportselch1R {
        Abusportselch1R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:30 - CH1 ABUS Pin Select"]
    #[inline(always)]
    pub fn abuspinselch1(&self) -> Abuspinselch1R {
        Abuspinselch1R::new(((self.bits >> 25) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CH0 Main Output Enable"]
    #[inline(always)]
    pub fn mainoutench0(&mut self) -> Mainoutench0W<OutctrlSpec> {
        Mainoutench0W::new(self, 0)
    }
    #[doc = "Bit 1 - CH1 Main Output Enable"]
    #[inline(always)]
    pub fn mainoutench1(&mut self) -> Mainoutench1W<OutctrlSpec> {
        Mainoutench1W::new(self, 1)
    }
    #[doc = "Bit 4 - CH0 Alternative Output Enable"]
    #[inline(always)]
    pub fn auxoutench0(&mut self) -> Auxoutench0W<OutctrlSpec> {
        Auxoutench0W::new(self, 4)
    }
    #[doc = "Bit 5 - CH1 Alternative Output Enable"]
    #[inline(always)]
    pub fn auxoutench1(&mut self) -> Auxoutench1W<OutctrlSpec> {
        Auxoutench1W::new(self, 5)
    }
    #[doc = "Bit 8 - CH1 Main and Alternative Output Short"]
    #[inline(always)]
    pub fn shortch0(&mut self) -> Shortch0W<OutctrlSpec> {
        Shortch0W::new(self, 8)
    }
    #[doc = "Bit 9 - CH0 Main and Alternative Output Short"]
    #[inline(always)]
    pub fn shortch1(&mut self) -> Shortch1W<OutctrlSpec> {
        Shortch1W::new(self, 9)
    }
    #[doc = "Bits 12:14 - CH0 ABUS Port Select"]
    #[inline(always)]
    pub fn abusportselch0(&mut self) -> Abusportselch0W<OutctrlSpec> {
        Abusportselch0W::new(self, 12)
    }
    #[doc = "Bits 15:20 - CH0 ABUS Pin Select"]
    #[inline(always)]
    pub fn abuspinselch0(&mut self) -> Abuspinselch0W<OutctrlSpec> {
        Abuspinselch0W::new(self, 15)
    }
    #[doc = "Bits 22:24 - CH1 ABUS Port Select"]
    #[inline(always)]
    pub fn abusportselch1(&mut self) -> Abusportselch1W<OutctrlSpec> {
        Abusportselch1W::new(self, 22)
    }
    #[doc = "Bits 25:30 - CH1 ABUS Pin Select"]
    #[inline(always)]
    pub fn abuspinselch1(&mut self) -> Abuspinselch1W<OutctrlSpec> {
        Abuspinselch1W::new(self, 25)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`outctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OutctrlSpec;
impl crate::RegisterSpec for OutctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outctrl::R`](R) reader structure"]
impl crate::Readable for OutctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`outctrl::W`](W) writer structure"]
impl crate::Writable for OutctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTCTRL to value 0"]
impl crate::Resettable for OutctrlSpec {}
