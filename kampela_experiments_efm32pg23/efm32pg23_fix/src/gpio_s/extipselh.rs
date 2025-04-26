#[doc = "Register `EXTIPSELH` reader"]
pub type R = crate::R<ExtipselhSpec>;
#[doc = "Register `EXTIPSELH` writer"]
pub type W = crate::W<ExtipselhSpec>;
#[doc = "External Interrupt Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel0 {
    #[doc = "0: Port A group selected"]
    Porta = 0,
    #[doc = "1: Port B group selected"]
    Portb = 1,
    #[doc = "2: Port C group selected"]
    Portc = 2,
    #[doc = "3: Port D group selected"]
    Portd = 3,
}
impl From<Extipsel0> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel0 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel0 {}
#[doc = "Field `EXTIPSEL0` reader - External Interrupt Port Select"]
pub type Extipsel0R = crate::FieldReader<Extipsel0>;
impl Extipsel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipsel0 {
        match self.bits {
            0 => Extipsel0::Porta,
            1 => Extipsel0::Portb,
            2 => Extipsel0::Portc,
            3 => Extipsel0::Portd,
            _ => unreachable!(),
        }
    }
    #[doc = "Port A group selected"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel0::Porta
    }
    #[doc = "Port B group selected"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel0::Portb
    }
    #[doc = "Port C group selected"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel0::Portc
    }
    #[doc = "Port D group selected"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel0::Portd
    }
}
#[doc = "Field `EXTIPSEL0` writer - External Interrupt Port Select"]
pub type Extipsel0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipsel0, crate::Safe>;
impl<'a, REG> Extipsel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel0::Porta)
    }
    #[doc = "Port B group selected"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel0::Portb)
    }
    #[doc = "Port C group selected"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel0::Portc)
    }
    #[doc = "Port D group selected"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel0::Portd)
    }
}
#[doc = "External Interrupt Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel1 {
    #[doc = "0: Port A group selected"]
    Porta = 0,
    #[doc = "1: Port B group selected"]
    Portb = 1,
    #[doc = "2: Port C group selected"]
    Portc = 2,
    #[doc = "3: Port D group selected"]
    Portd = 3,
}
impl From<Extipsel1> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel1 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel1 {}
#[doc = "Field `EXTIPSEL1` reader - External Interrupt Port Select"]
pub type Extipsel1R = crate::FieldReader<Extipsel1>;
impl Extipsel1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipsel1 {
        match self.bits {
            0 => Extipsel1::Porta,
            1 => Extipsel1::Portb,
            2 => Extipsel1::Portc,
            3 => Extipsel1::Portd,
            _ => unreachable!(),
        }
    }
    #[doc = "Port A group selected"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel1::Porta
    }
    #[doc = "Port B group selected"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel1::Portb
    }
    #[doc = "Port C group selected"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel1::Portc
    }
    #[doc = "Port D group selected"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel1::Portd
    }
}
#[doc = "Field `EXTIPSEL1` writer - External Interrupt Port Select"]
pub type Extipsel1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipsel1, crate::Safe>;
impl<'a, REG> Extipsel1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel1::Porta)
    }
    #[doc = "Port B group selected"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel1::Portb)
    }
    #[doc = "Port C group selected"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel1::Portc)
    }
    #[doc = "Port D group selected"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel1::Portd)
    }
}
#[doc = "External Interrupt Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel2 {
    #[doc = "0: Port A group selected"]
    Porta = 0,
    #[doc = "1: Port B group selected"]
    Portb = 1,
    #[doc = "2: Port C group selected"]
    Portc = 2,
    #[doc = "3: Port D group selected"]
    Portd = 3,
}
impl From<Extipsel2> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel2 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel2 {}
#[doc = "Field `EXTIPSEL2` reader - External Interrupt Port Select"]
pub type Extipsel2R = crate::FieldReader<Extipsel2>;
impl Extipsel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipsel2 {
        match self.bits {
            0 => Extipsel2::Porta,
            1 => Extipsel2::Portb,
            2 => Extipsel2::Portc,
            3 => Extipsel2::Portd,
            _ => unreachable!(),
        }
    }
    #[doc = "Port A group selected"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel2::Porta
    }
    #[doc = "Port B group selected"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel2::Portb
    }
    #[doc = "Port C group selected"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel2::Portc
    }
    #[doc = "Port D group selected"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel2::Portd
    }
}
#[doc = "Field `EXTIPSEL2` writer - External Interrupt Port Select"]
pub type Extipsel2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipsel2, crate::Safe>;
impl<'a, REG> Extipsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel2::Porta)
    }
    #[doc = "Port B group selected"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel2::Portb)
    }
    #[doc = "Port C group selected"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel2::Portc)
    }
    #[doc = "Port D group selected"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel2::Portd)
    }
}
#[doc = "External Interrupt Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Extipsel3 {
    #[doc = "0: Port A group selected"]
    Porta = 0,
    #[doc = "1: Port B group selected"]
    Portb = 1,
    #[doc = "2: Port C group selected"]
    Portc = 2,
    #[doc = "3: Port D group selected"]
    Portd = 3,
}
impl From<Extipsel3> for u8 {
    #[inline(always)]
    fn from(variant: Extipsel3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Extipsel3 {
    type Ux = u8;
}
impl crate::IsEnum for Extipsel3 {}
#[doc = "Field `EXTIPSEL3` reader - External Interrupt Port Select"]
pub type Extipsel3R = crate::FieldReader<Extipsel3>;
impl Extipsel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extipsel3 {
        match self.bits {
            0 => Extipsel3::Porta,
            1 => Extipsel3::Portb,
            2 => Extipsel3::Portc,
            3 => Extipsel3::Portd,
            _ => unreachable!(),
        }
    }
    #[doc = "Port A group selected"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        *self == Extipsel3::Porta
    }
    #[doc = "Port B group selected"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        *self == Extipsel3::Portb
    }
    #[doc = "Port C group selected"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        *self == Extipsel3::Portc
    }
    #[doc = "Port D group selected"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        *self == Extipsel3::Portd
    }
}
#[doc = "Field `EXTIPSEL3` writer - External Interrupt Port Select"]
pub type Extipsel3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Extipsel3, crate::Safe>;
impl<'a, REG> Extipsel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Port A group selected"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel3::Porta)
    }
    #[doc = "Port B group selected"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel3::Portb)
    }
    #[doc = "Port C group selected"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel3::Portc)
    }
    #[doc = "Port D group selected"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut crate::W<REG> {
        self.variant(Extipsel3::Portd)
    }
}
impl R {
    #[doc = "Bits 0:1 - External Interrupt Port Select"]
    #[inline(always)]
    pub fn extipsel0(&self) -> Extipsel0R {
        Extipsel0R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Interrupt Port Select"]
    #[inline(always)]
    pub fn extipsel1(&self) -> Extipsel1R {
        Extipsel1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Interrupt Port Select"]
    #[inline(always)]
    pub fn extipsel2(&self) -> Extipsel2R {
        Extipsel2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External Interrupt Port Select"]
    #[inline(always)]
    pub fn extipsel3(&self) -> Extipsel3R {
        Extipsel3R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt Port Select"]
    #[inline(always)]
    pub fn extipsel0(&mut self) -> Extipsel0W<ExtipselhSpec> {
        Extipsel0W::new(self, 0)
    }
    #[doc = "Bits 4:5 - External Interrupt Port Select"]
    #[inline(always)]
    pub fn extipsel1(&mut self) -> Extipsel1W<ExtipselhSpec> {
        Extipsel1W::new(self, 4)
    }
    #[doc = "Bits 8:9 - External Interrupt Port Select"]
    #[inline(always)]
    pub fn extipsel2(&mut self) -> Extipsel2W<ExtipselhSpec> {
        Extipsel2W::new(self, 8)
    }
    #[doc = "Bits 12:13 - External Interrupt Port Select"]
    #[inline(always)]
    pub fn extipsel3(&mut self) -> Extipsel3W<ExtipselhSpec> {
        Extipsel3W::new(self, 12)
    }
}
#[doc = "External interrupt Port Select High\n\nYou can [`read`](crate::Reg::read) this register and get [`extipselh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extipselh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtipselhSpec;
impl crate::RegisterSpec for ExtipselhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extipselh::R`](R) reader structure"]
impl crate::Readable for ExtipselhSpec {}
#[doc = "`write(|w| ..)` method takes [`extipselh::W`](W) writer structure"]
impl crate::Writable for ExtipselhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTIPSELH to value 0"]
impl crate::Resettable for ExtipselhSpec {}
