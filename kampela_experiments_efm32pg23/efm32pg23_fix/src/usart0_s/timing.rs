#[doc = "Register `TIMING` reader"]
pub type R = crate::R<TimingSpec>;
#[doc = "Register `TIMING` writer"]
pub type W = crate::W<TimingSpec>;
#[doc = "TX frame start delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txdelay {
    #[doc = "0: Disable - TXDELAY in USARTn_CTRL can be used for legacy"]
    Disable = 0,
    #[doc = "1: Start of transmission is delayed for 1 baud-times"]
    One = 1,
    #[doc = "2: Start of transmission is delayed for 2 baud-times"]
    Two = 2,
    #[doc = "3: Start of transmission is delayed for 3 baud-times"]
    Three = 3,
    #[doc = "4: Start of transmission is delayed for 7 baud-times"]
    Seven = 4,
    #[doc = "5: Start of transmission is delayed for TCMPVAL0 baud-times"]
    Tcmp0 = 5,
    #[doc = "6: Start of transmission is delayed for TCMPVAL1 baud-times"]
    Tcmp1 = 6,
    #[doc = "7: Start of transmission is delayed for TCMPVAL2 baud-times"]
    Tcmp2 = 7,
}
impl From<Txdelay> for u8 {
    #[inline(always)]
    fn from(variant: Txdelay) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txdelay {
    type Ux = u8;
}
impl crate::IsEnum for Txdelay {}
#[doc = "Field `TXDELAY` reader - TX frame start delay"]
pub type TxdelayR = crate::FieldReader<Txdelay>;
impl TxdelayR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdelay {
        match self.bits {
            0 => Txdelay::Disable,
            1 => Txdelay::One,
            2 => Txdelay::Two,
            3 => Txdelay::Three,
            4 => Txdelay::Seven,
            5 => Txdelay::Tcmp0,
            6 => Txdelay::Tcmp1,
            7 => Txdelay::Tcmp2,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable - TXDELAY in USARTn_CTRL can be used for legacy"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Txdelay::Disable
    }
    #[doc = "Start of transmission is delayed for 1 baud-times"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Txdelay::One
    }
    #[doc = "Start of transmission is delayed for 2 baud-times"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Txdelay::Two
    }
    #[doc = "Start of transmission is delayed for 3 baud-times"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Txdelay::Three
    }
    #[doc = "Start of transmission is delayed for 7 baud-times"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == Txdelay::Seven
    }
    #[doc = "Start of transmission is delayed for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == Txdelay::Tcmp0
    }
    #[doc = "Start of transmission is delayed for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == Txdelay::Tcmp1
    }
    #[doc = "Start of transmission is delayed for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == Txdelay::Tcmp2
    }
}
#[doc = "Field `TXDELAY` writer - TX frame start delay"]
pub type TxdelayW<'a, REG> = crate::FieldWriter<'a, REG, 3, Txdelay, crate::Safe>;
impl<'a, REG> TxdelayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable - TXDELAY in USARTn_CTRL can be used for legacy"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::Disable)
    }
    #[doc = "Start of transmission is delayed for 1 baud-times"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::One)
    }
    #[doc = "Start of transmission is delayed for 2 baud-times"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::Two)
    }
    #[doc = "Start of transmission is delayed for 3 baud-times"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::Three)
    }
    #[doc = "Start of transmission is delayed for 7 baud-times"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::Seven)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::Tcmp0)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::Tcmp1)
    }
    #[doc = "Start of transmission is delayed for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::Tcmp2)
    }
}
#[doc = "Chip Select Setup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cssetup {
    #[doc = "0: CS is not asserted before start of transmission"]
    Zero = 0,
    #[doc = "1: CS is asserted for 1 baud-times before start of transmission"]
    One = 1,
    #[doc = "2: CS is asserted for 2 baud-times before start of transmission"]
    Two = 2,
    #[doc = "3: CS is asserted for 3 baud-times before start of transmission"]
    Three = 3,
    #[doc = "4: CS is asserted for 7 baud-times before start of transmission"]
    Seven = 4,
    #[doc = "5: CS is asserted before the start of transmission for TCMPVAL0 baud-times"]
    Tcmp0 = 5,
    #[doc = "6: CS is asserted before the start of transmission for TCMPVAL1 baud-times"]
    Tcmp1 = 6,
    #[doc = "7: CS is asserted before the start of transmission for TCMPVAL2 baud-times"]
    Tcmp2 = 7,
}
impl From<Cssetup> for u8 {
    #[inline(always)]
    fn from(variant: Cssetup) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cssetup {
    type Ux = u8;
}
impl crate::IsEnum for Cssetup {}
#[doc = "Field `CSSETUP` reader - Chip Select Setup"]
pub type CssetupR = crate::FieldReader<Cssetup>;
impl CssetupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cssetup {
        match self.bits {
            0 => Cssetup::Zero,
            1 => Cssetup::One,
            2 => Cssetup::Two,
            3 => Cssetup::Three,
            4 => Cssetup::Seven,
            5 => Cssetup::Tcmp0,
            6 => Cssetup::Tcmp1,
            7 => Cssetup::Tcmp2,
            _ => unreachable!(),
        }
    }
    #[doc = "CS is not asserted before start of transmission"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Cssetup::Zero
    }
    #[doc = "CS is asserted for 1 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Cssetup::One
    }
    #[doc = "CS is asserted for 2 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Cssetup::Two
    }
    #[doc = "CS is asserted for 3 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Cssetup::Three
    }
    #[doc = "CS is asserted for 7 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == Cssetup::Seven
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == Cssetup::Tcmp0
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == Cssetup::Tcmp1
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == Cssetup::Tcmp2
    }
}
#[doc = "Field `CSSETUP` writer - Chip Select Setup"]
pub type CssetupW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cssetup, crate::Safe>;
impl<'a, REG> CssetupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CS is not asserted before start of transmission"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Cssetup::Zero)
    }
    #[doc = "CS is asserted for 1 baud-times before start of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Cssetup::One)
    }
    #[doc = "CS is asserted for 2 baud-times before start of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Cssetup::Two)
    }
    #[doc = "CS is asserted for 3 baud-times before start of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Cssetup::Three)
    }
    #[doc = "CS is asserted for 7 baud-times before start of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(Cssetup::Seven)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Cssetup::Tcmp0)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Cssetup::Tcmp1)
    }
    #[doc = "CS is asserted before the start of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(Cssetup::Tcmp2)
    }
}
#[doc = "Inter-character spacing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ics {
    #[doc = "0: There is no space between charcters"]
    Zero = 0,
    #[doc = "1: Create a space of 1 baud-times before start of transmission"]
    One = 1,
    #[doc = "2: Create a space of 2 baud-times before start of transmission"]
    Two = 2,
    #[doc = "3: Create a space of 3 baud-times before start of transmission"]
    Three = 3,
    #[doc = "4: Create a space of 7 baud-times before start of transmission"]
    Seven = 4,
    #[doc = "5: Create a space of before the start of transmission for TCMPVAL0 baud-times"]
    Tcmp0 = 5,
    #[doc = "6: Create a space of before the start of transmission for TCMPVAL1 baud-times"]
    Tcmp1 = 6,
    #[doc = "7: Create a space of before the start of transmission for TCMPVAL2 baud-times"]
    Tcmp2 = 7,
}
impl From<Ics> for u8 {
    #[inline(always)]
    fn from(variant: Ics) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ics {
    type Ux = u8;
}
impl crate::IsEnum for Ics {}
#[doc = "Field `ICS` reader - Inter-character spacing"]
pub type IcsR = crate::FieldReader<Ics>;
impl IcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ics {
        match self.bits {
            0 => Ics::Zero,
            1 => Ics::One,
            2 => Ics::Two,
            3 => Ics::Three,
            4 => Ics::Seven,
            5 => Ics::Tcmp0,
            6 => Ics::Tcmp1,
            7 => Ics::Tcmp2,
            _ => unreachable!(),
        }
    }
    #[doc = "There is no space between charcters"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Ics::Zero
    }
    #[doc = "Create a space of 1 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Ics::One
    }
    #[doc = "Create a space of 2 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Ics::Two
    }
    #[doc = "Create a space of 3 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Ics::Three
    }
    #[doc = "Create a space of 7 baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == Ics::Seven
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == Ics::Tcmp0
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == Ics::Tcmp1
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == Ics::Tcmp2
    }
}
#[doc = "Field `ICS` writer - Inter-character spacing"]
pub type IcsW<'a, REG> = crate::FieldWriter<'a, REG, 3, Ics, crate::Safe>;
impl<'a, REG> IcsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "There is no space between charcters"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Ics::Zero)
    }
    #[doc = "Create a space of 1 baud-times before start of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Ics::One)
    }
    #[doc = "Create a space of 2 baud-times before start of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Ics::Two)
    }
    #[doc = "Create a space of 3 baud-times before start of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Ics::Three)
    }
    #[doc = "Create a space of 7 baud-times before start of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(Ics::Seven)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Ics::Tcmp0)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Ics::Tcmp1)
    }
    #[doc = "Create a space of before the start of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(Ics::Tcmp2)
    }
}
#[doc = "Chip Select Hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cshold {
    #[doc = "0: Disable CS being asserted after the end of transmission"]
    Zero = 0,
    #[doc = "1: CS is asserted for 1 baud-times after the end of transmission"]
    One = 1,
    #[doc = "2: CS is asserted for 2 baud-times after the end of transmission"]
    Two = 2,
    #[doc = "3: CS is asserted for 3 baud-times after the end of transmission"]
    Three = 3,
    #[doc = "4: CS is asserted for 7 baud-times after the end of transmission"]
    Seven = 4,
    #[doc = "5: CS is asserted after the end of transmission for TCMPVAL0 baud-times"]
    Tcmp0 = 5,
    #[doc = "6: CS is asserted after the end of transmission for TCMPVAL1 baud-times"]
    Tcmp1 = 6,
    #[doc = "7: CS is asserted after the end of transmission for TCMPVAL2 baud-times"]
    Tcmp2 = 7,
}
impl From<Cshold> for u8 {
    #[inline(always)]
    fn from(variant: Cshold) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cshold {
    type Ux = u8;
}
impl crate::IsEnum for Cshold {}
#[doc = "Field `CSHOLD` reader - Chip Select Hold"]
pub type CsholdR = crate::FieldReader<Cshold>;
impl CsholdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cshold {
        match self.bits {
            0 => Cshold::Zero,
            1 => Cshold::One,
            2 => Cshold::Two,
            3 => Cshold::Three,
            4 => Cshold::Seven,
            5 => Cshold::Tcmp0,
            6 => Cshold::Tcmp1,
            7 => Cshold::Tcmp2,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable CS being asserted after the end of transmission"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Cshold::Zero
    }
    #[doc = "CS is asserted for 1 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Cshold::One
    }
    #[doc = "CS is asserted for 2 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Cshold::Two
    }
    #[doc = "CS is asserted for 3 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Cshold::Three
    }
    #[doc = "CS is asserted for 7 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == Cshold::Seven
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn is_tcmp0(&self) -> bool {
        *self == Cshold::Tcmp0
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == Cshold::Tcmp1
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn is_tcmp2(&self) -> bool {
        *self == Cshold::Tcmp2
    }
}
#[doc = "Field `CSHOLD` writer - Chip Select Hold"]
pub type CsholdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cshold, crate::Safe>;
impl<'a, REG> CsholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable CS being asserted after the end of transmission"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Cshold::Zero)
    }
    #[doc = "CS is asserted for 1 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Cshold::One)
    }
    #[doc = "CS is asserted for 2 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Cshold::Two)
    }
    #[doc = "CS is asserted for 3 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Cshold::Three)
    }
    #[doc = "CS is asserted for 7 baud-times after the end of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(Cshold::Seven)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL0 baud-times"]
    #[inline(always)]
    pub fn tcmp0(self) -> &'a mut crate::W<REG> {
        self.variant(Cshold::Tcmp0)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL1 baud-times"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Cshold::Tcmp1)
    }
    #[doc = "CS is asserted after the end of transmission for TCMPVAL2 baud-times"]
    #[inline(always)]
    pub fn tcmp2(self) -> &'a mut crate::W<REG> {
        self.variant(Cshold::Tcmp2)
    }
}
impl R {
    #[doc = "Bits 16:18 - TX frame start delay"]
    #[inline(always)]
    pub fn txdelay(&self) -> TxdelayR {
        TxdelayR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Chip Select Setup"]
    #[inline(always)]
    pub fn cssetup(&self) -> CssetupR {
        CssetupR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Inter-character spacing"]
    #[inline(always)]
    pub fn ics(&self) -> IcsR {
        IcsR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Chip Select Hold"]
    #[inline(always)]
    pub fn cshold(&self) -> CsholdR {
        CsholdR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 16:18 - TX frame start delay"]
    #[inline(always)]
    pub fn txdelay(&mut self) -> TxdelayW<TimingSpec> {
        TxdelayW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Chip Select Setup"]
    #[inline(always)]
    pub fn cssetup(&mut self) -> CssetupW<TimingSpec> {
        CssetupW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Inter-character spacing"]
    #[inline(always)]
    pub fn ics(&mut self) -> IcsW<TimingSpec> {
        IcsW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Chip Select Hold"]
    #[inline(always)]
    pub fn cshold(&mut self) -> CsholdW<TimingSpec> {
        CsholdW::new(self, 28)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`timing::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timing::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimingSpec;
impl crate::RegisterSpec for TimingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timing::R`](R) reader structure"]
impl crate::Readable for TimingSpec {}
#[doc = "`write(|w| ..)` method takes [`timing::W`](W) writer structure"]
impl crate::Writable for TimingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMING to value 0"]
impl crate::Resettable for TimingSpec {}
