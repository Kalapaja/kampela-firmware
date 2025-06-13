#[doc = "Register `PRSMODE` reader"]
pub type R = crate::R<PrsmodeSpec>;
#[doc = "Register `PRSMODE` writer"]
pub type W = crate::W<PrsmodeSpec>;
#[doc = "PRS Start Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prsstartmode {
    #[doc = "0: PRS cannot start the LETIMER"]
    None = 0,
    #[doc = "1: Rising edge of selected PRS input can start the LETIMER"]
    Rising = 1,
    #[doc = "2: Falling edge of selected PRS input can start the LETIMER"]
    Falling = 2,
    #[doc = "3: Both the rising or falling edge of the selected PRS input can start the LETIMER"]
    Both = 3,
}
impl From<Prsstartmode> for u8 {
    #[inline(always)]
    fn from(variant: Prsstartmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prsstartmode {
    type Ux = u8;
}
impl crate::IsEnum for Prsstartmode {}
#[doc = "Field `PRSSTARTMODE` reader - PRS Start Mode"]
pub type PrsstartmodeR = crate::FieldReader<Prsstartmode>;
impl PrsstartmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prsstartmode {
        match self.bits {
            0 => Prsstartmode::None,
            1 => Prsstartmode::Rising,
            2 => Prsstartmode::Falling,
            3 => Prsstartmode::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "PRS cannot start the LETIMER"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Prsstartmode::None
    }
    #[doc = "Rising edge of selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Prsstartmode::Rising
    }
    #[doc = "Falling edge of selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Prsstartmode::Falling
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Prsstartmode::Both
    }
}
#[doc = "Field `PRSSTARTMODE` writer - PRS Start Mode"]
pub type PrsstartmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Prsstartmode, crate::Safe>;
impl<'a, REG> PrsstartmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS cannot start the LETIMER"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstartmode::None)
    }
    #[doc = "Rising edge of selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstartmode::Rising)
    }
    #[doc = "Falling edge of selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstartmode::Falling)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can start the LETIMER"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstartmode::Both)
    }
}
#[doc = "PRS Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prsstopmode {
    #[doc = "0: PRS cannot stop the LETIMER"]
    None = 0,
    #[doc = "1: Rising edge of selected PRS input can stop the LETIMER"]
    Rising = 1,
    #[doc = "2: Falling edge of selected PRS input can stop the LETIMER"]
    Falling = 2,
    #[doc = "3: Both the rising or falling edge of the selected PRS input can stop the LETIMER"]
    Both = 3,
}
impl From<Prsstopmode> for u8 {
    #[inline(always)]
    fn from(variant: Prsstopmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prsstopmode {
    type Ux = u8;
}
impl crate::IsEnum for Prsstopmode {}
#[doc = "Field `PRSSTOPMODE` reader - PRS Stop Mode"]
pub type PrsstopmodeR = crate::FieldReader<Prsstopmode>;
impl PrsstopmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prsstopmode {
        match self.bits {
            0 => Prsstopmode::None,
            1 => Prsstopmode::Rising,
            2 => Prsstopmode::Falling,
            3 => Prsstopmode::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "PRS cannot stop the LETIMER"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Prsstopmode::None
    }
    #[doc = "Rising edge of selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Prsstopmode::Rising
    }
    #[doc = "Falling edge of selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Prsstopmode::Falling
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Prsstopmode::Both
    }
}
#[doc = "Field `PRSSTOPMODE` writer - PRS Stop Mode"]
pub type PrsstopmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Prsstopmode, crate::Safe>;
impl<'a, REG> PrsstopmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS cannot stop the LETIMER"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstopmode::None)
    }
    #[doc = "Rising edge of selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstopmode::Rising)
    }
    #[doc = "Falling edge of selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstopmode::Falling)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can stop the LETIMER"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Prsstopmode::Both)
    }
}
#[doc = "PRS Clear Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prsclearmode {
    #[doc = "0: PRS cannot clear the LETIMER"]
    None = 0,
    #[doc = "1: Rising edge of selected PRS input can clear the LETIMER"]
    Rising = 1,
    #[doc = "2: Falling edge of selected PRS input can clear the LETIMER"]
    Falling = 2,
    #[doc = "3: Both the rising or falling edge of the selected PRS input can clear the LETIMER"]
    Both = 3,
}
impl From<Prsclearmode> for u8 {
    #[inline(always)]
    fn from(variant: Prsclearmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prsclearmode {
    type Ux = u8;
}
impl crate::IsEnum for Prsclearmode {}
#[doc = "Field `PRSCLEARMODE` reader - PRS Clear Mode"]
pub type PrsclearmodeR = crate::FieldReader<Prsclearmode>;
impl PrsclearmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prsclearmode {
        match self.bits {
            0 => Prsclearmode::None,
            1 => Prsclearmode::Rising,
            2 => Prsclearmode::Falling,
            3 => Prsclearmode::Both,
            _ => unreachable!(),
        }
    }
    #[doc = "PRS cannot clear the LETIMER"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Prsclearmode::None
    }
    #[doc = "Rising edge of selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Prsclearmode::Rising
    }
    #[doc = "Falling edge of selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Prsclearmode::Falling
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        *self == Prsclearmode::Both
    }
}
#[doc = "Field `PRSCLEARMODE` writer - PRS Clear Mode"]
pub type PrsclearmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Prsclearmode, crate::Safe>;
impl<'a, REG> PrsclearmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "PRS cannot clear the LETIMER"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Prsclearmode::None)
    }
    #[doc = "Rising edge of selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Prsclearmode::Rising)
    }
    #[doc = "Falling edge of selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Prsclearmode::Falling)
    }
    #[doc = "Both the rising or falling edge of the selected PRS input can clear the LETIMER"]
    #[inline(always)]
    pub fn both(self) -> &'a mut crate::W<REG> {
        self.variant(Prsclearmode::Both)
    }
}
impl R {
    #[doc = "Bits 18:19 - PRS Start Mode"]
    #[inline(always)]
    pub fn prsstartmode(&self) -> PrsstartmodeR {
        PrsstartmodeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 22:23 - PRS Stop Mode"]
    #[inline(always)]
    pub fn prsstopmode(&self) -> PrsstopmodeR {
        PrsstopmodeR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:27 - PRS Clear Mode"]
    #[inline(always)]
    pub fn prsclearmode(&self) -> PrsclearmodeR {
        PrsclearmodeR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 18:19 - PRS Start Mode"]
    #[inline(always)]
    pub fn prsstartmode(&mut self) -> PrsstartmodeW<PrsmodeSpec> {
        PrsstartmodeW::new(self, 18)
    }
    #[doc = "Bits 22:23 - PRS Stop Mode"]
    #[inline(always)]
    pub fn prsstopmode(&mut self) -> PrsstopmodeW<PrsmodeSpec> {
        PrsstopmodeW::new(self, 22)
    }
    #[doc = "Bits 26:27 - PRS Clear Mode"]
    #[inline(always)]
    pub fn prsclearmode(&mut self) -> PrsclearmodeW<PrsmodeSpec> {
        PrsclearmodeW::new(self, 26)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`prsmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prsmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrsmodeSpec;
impl crate::RegisterSpec for PrsmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prsmode::R`](R) reader structure"]
impl crate::Readable for PrsmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`prsmode::W`](W) writer structure"]
impl crate::Writable for PrsmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRSMODE to value 0"]
impl crate::Resettable for PrsmodeSpec {}
