#[doc = "Register `TIMINGCFG` reader"]
pub type R = crate::R<TimingcfgSpec>;
#[doc = "Register `TIMINGCFG` writer"]
pub type W = crate::W<TimingcfgSpec>;
#[doc = "TX Delay Transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txdelay {
    #[doc = "0: Frames are transmitted immediately."]
    None = 0,
    #[doc = "1: Transmission of new frames is delayed by a single bit period."]
    Single = 1,
    #[doc = "2: Transmission of new frames is delayed by a two bit periods."]
    Double = 2,
    #[doc = "3: Transmission of new frames is delayed by a three bit periods."]
    Tripple = 3,
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
#[doc = "Field `TXDELAY` reader - TX Delay Transmission"]
pub type TxdelayR = crate::FieldReader<Txdelay>;
impl TxdelayR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdelay {
        match self.bits {
            0 => Txdelay::None,
            1 => Txdelay::Single,
            2 => Txdelay::Double,
            3 => Txdelay::Tripple,
            _ => unreachable!(),
        }
    }
    #[doc = "Frames are transmitted immediately."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Txdelay::None
    }
    #[doc = "Transmission of new frames is delayed by a single bit period."]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == Txdelay::Single
    }
    #[doc = "Transmission of new frames is delayed by a two bit periods."]
    #[inline(always)]
    pub fn is_double(&self) -> bool {
        *self == Txdelay::Double
    }
    #[doc = "Transmission of new frames is delayed by a three bit periods."]
    #[inline(always)]
    pub fn is_tripple(&self) -> bool {
        *self == Txdelay::Tripple
    }
}
#[doc = "Field `TXDELAY` writer - TX Delay Transmission"]
pub type TxdelayW<'a, REG> = crate::FieldWriter<'a, REG, 2, Txdelay, crate::Safe>;
impl<'a, REG> TxdelayW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Frames are transmitted immediately."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::None)
    }
    #[doc = "Transmission of new frames is delayed by a single bit period."]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::Single)
    }
    #[doc = "Transmission of new frames is delayed by a two bit periods."]
    #[inline(always)]
    pub fn double(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::Double)
    }
    #[doc = "Transmission of new frames is delayed by a three bit periods."]
    #[inline(always)]
    pub fn tripple(self) -> &'a mut crate::W<REG> {
        self.variant(Txdelay::Tripple)
    }
}
#[doc = "Chip Select Setup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cssetup {
    #[doc = "0: CS is asserted half or 1 baud-time before the start of transmission depending on CLKPHASE equal to 1 or 0 respectively"]
    Zero = 0,
    #[doc = "1: CS is asserted 1 additional baud-time before start of transmission"]
    One = 1,
    #[doc = "2: CS is asserted 2 additional baud-times before start of transmission"]
    Two = 2,
    #[doc = "3: CS is asserted 3 additional baud-times before start of transmission"]
    Three = 3,
    #[doc = "4: CS is asserted 4 additional baud-times before start of transmission"]
    Four = 4,
    #[doc = "5: CS is asserted 5 additional baud-times before start of transmission"]
    Five = 5,
    #[doc = "6: CS is asserted 6 additional baud-times before start of transmission"]
    Six = 6,
    #[doc = "7: CS is asserted 7 additional baud-times before start of transmission"]
    Seven = 7,
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
            4 => Cssetup::Four,
            5 => Cssetup::Five,
            6 => Cssetup::Six,
            7 => Cssetup::Seven,
            _ => unreachable!(),
        }
    }
    #[doc = "CS is asserted half or 1 baud-time before the start of transmission depending on CLKPHASE equal to 1 or 0 respectively"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Cssetup::Zero
    }
    #[doc = "CS is asserted 1 additional baud-time before start of transmission"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Cssetup::One
    }
    #[doc = "CS is asserted 2 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Cssetup::Two
    }
    #[doc = "CS is asserted 3 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Cssetup::Three
    }
    #[doc = "CS is asserted 4 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Cssetup::Four
    }
    #[doc = "CS is asserted 5 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == Cssetup::Five
    }
    #[doc = "CS is asserted 6 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == Cssetup::Six
    }
    #[doc = "CS is asserted 7 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == Cssetup::Seven
    }
}
#[doc = "Field `CSSETUP` writer - Chip Select Setup"]
pub type CssetupW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cssetup, crate::Safe>;
impl<'a, REG> CssetupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CS is asserted half or 1 baud-time before the start of transmission depending on CLKPHASE equal to 1 or 0 respectively"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Cssetup::Zero)
    }
    #[doc = "CS is asserted 1 additional baud-time before start of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Cssetup::One)
    }
    #[doc = "CS is asserted 2 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Cssetup::Two)
    }
    #[doc = "CS is asserted 3 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Cssetup::Three)
    }
    #[doc = "CS is asserted 4 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Cssetup::Four)
    }
    #[doc = "CS is asserted 5 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn five(self) -> &'a mut crate::W<REG> {
        self.variant(Cssetup::Five)
    }
    #[doc = "CS is asserted 6 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn six(self) -> &'a mut crate::W<REG> {
        self.variant(Cssetup::Six)
    }
    #[doc = "CS is asserted 7 additional baud-times before start of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(Cssetup::Seven)
    }
}
#[doc = "Chip Select Hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cshold {
    #[doc = "0: CS is de-asserted half or 1 baud-time after the end of transmission depending on CLKPHASE equal to 1 or 0 respectively"]
    Zero = 0,
    #[doc = "1: CS is de-asserted 1 additional baud-time after the end of transmission"]
    One = 1,
    #[doc = "2: CS is de-asserted 2 additional baud-times after the end of transmission"]
    Two = 2,
    #[doc = "3: CS is de-asserted 3 additional baud-times after the end of transmission"]
    Three = 3,
    #[doc = "4: CS is de-asserted 4 additional baud-times after the end of transmission"]
    Four = 4,
    #[doc = "5: CS is de-asserted 5 additional baud-times after the end of transmission"]
    Five = 5,
    #[doc = "6: CS is de-asserted 6 additional baud-times after the end of transmission"]
    Six = 6,
    #[doc = "7: CS is de-asserted 7 additional baud-times after the end of transmission"]
    Seven = 7,
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
            4 => Cshold::Four,
            5 => Cshold::Five,
            6 => Cshold::Six,
            7 => Cshold::Seven,
            _ => unreachable!(),
        }
    }
    #[doc = "CS is de-asserted half or 1 baud-time after the end of transmission depending on CLKPHASE equal to 1 or 0 respectively"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Cshold::Zero
    }
    #[doc = "CS is de-asserted 1 additional baud-time after the end of transmission"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Cshold::One
    }
    #[doc = "CS is de-asserted 2 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Cshold::Two
    }
    #[doc = "CS is de-asserted 3 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Cshold::Three
    }
    #[doc = "CS is de-asserted 4 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Cshold::Four
    }
    #[doc = "CS is de-asserted 5 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == Cshold::Five
    }
    #[doc = "CS is de-asserted 6 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == Cshold::Six
    }
    #[doc = "CS is de-asserted 7 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == Cshold::Seven
    }
}
#[doc = "Field `CSHOLD` writer - Chip Select Hold"]
pub type CsholdW<'a, REG> = crate::FieldWriter<'a, REG, 3, Cshold, crate::Safe>;
impl<'a, REG> CsholdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CS is de-asserted half or 1 baud-time after the end of transmission depending on CLKPHASE equal to 1 or 0 respectively"]
    #[inline(always)]
    pub fn zero(self) -> &'a mut crate::W<REG> {
        self.variant(Cshold::Zero)
    }
    #[doc = "CS is de-asserted 1 additional baud-time after the end of transmission"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Cshold::One)
    }
    #[doc = "CS is de-asserted 2 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Cshold::Two)
    }
    #[doc = "CS is de-asserted 3 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Cshold::Three)
    }
    #[doc = "CS is de-asserted 4 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Cshold::Four)
    }
    #[doc = "CS is de-asserted 5 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn five(self) -> &'a mut crate::W<REG> {
        self.variant(Cshold::Five)
    }
    #[doc = "CS is de-asserted 6 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn six(self) -> &'a mut crate::W<REG> {
        self.variant(Cshold::Six)
    }
    #[doc = "CS is de-asserted 7 additional baud-times after the end of transmission"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(Cshold::Seven)
    }
}
#[doc = "Inter-Character Spacing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ics {
    #[doc = "0: There is no space between charcters"]
    Zero = 0,
    #[doc = "1: Create a space of 1 baud-times between frames"]
    One = 1,
    #[doc = "2: Create a space of 2 baud-times between frames"]
    Two = 2,
    #[doc = "3: Create a space of 3 baud-times between frames"]
    Three = 3,
    #[doc = "4: Create a space of 4 baud-times between frames"]
    Four = 4,
    #[doc = "5: Create a space of 5 baud-times between frames"]
    Five = 5,
    #[doc = "6: Create a space of 6 baud-times between frames"]
    Six = 6,
    #[doc = "7: Create a space of 7 baud-times between frames"]
    Seven = 7,
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
#[doc = "Field `ICS` reader - Inter-Character Spacing"]
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
            4 => Ics::Four,
            5 => Ics::Five,
            6 => Ics::Six,
            7 => Ics::Seven,
            _ => unreachable!(),
        }
    }
    #[doc = "There is no space between charcters"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == Ics::Zero
    }
    #[doc = "Create a space of 1 baud-times between frames"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Ics::One
    }
    #[doc = "Create a space of 2 baud-times between frames"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Ics::Two
    }
    #[doc = "Create a space of 3 baud-times between frames"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        *self == Ics::Three
    }
    #[doc = "Create a space of 4 baud-times between frames"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Ics::Four
    }
    #[doc = "Create a space of 5 baud-times between frames"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == Ics::Five
    }
    #[doc = "Create a space of 6 baud-times between frames"]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == Ics::Six
    }
    #[doc = "Create a space of 7 baud-times between frames"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == Ics::Seven
    }
}
#[doc = "Field `ICS` writer - Inter-Character Spacing"]
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
    #[doc = "Create a space of 1 baud-times between frames"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Ics::One)
    }
    #[doc = "Create a space of 2 baud-times between frames"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Ics::Two)
    }
    #[doc = "Create a space of 3 baud-times between frames"]
    #[inline(always)]
    pub fn three(self) -> &'a mut crate::W<REG> {
        self.variant(Ics::Three)
    }
    #[doc = "Create a space of 4 baud-times between frames"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Ics::Four)
    }
    #[doc = "Create a space of 5 baud-times between frames"]
    #[inline(always)]
    pub fn five(self) -> &'a mut crate::W<REG> {
        self.variant(Ics::Five)
    }
    #[doc = "Create a space of 6 baud-times between frames"]
    #[inline(always)]
    pub fn six(self) -> &'a mut crate::W<REG> {
        self.variant(Ics::Six)
    }
    #[doc = "Create a space of 7 baud-times between frames"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(Ics::Seven)
    }
}
#[doc = "Field `SETUPWINDOW` reader - Setup Window"]
pub type SetupwindowR = crate::FieldReader;
#[doc = "Field `SETUPWINDOW` writer - Setup Window"]
pub type SetupwindowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&self) -> TxdelayR {
        TxdelayR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Chip Select Setup"]
    #[inline(always)]
    pub fn cssetup(&self) -> CssetupR {
        CssetupR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Chip Select Hold"]
    #[inline(always)]
    pub fn cshold(&self) -> CsholdR {
        CsholdR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Inter-Character Spacing"]
    #[inline(always)]
    pub fn ics(&self) -> IcsR {
        IcsR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Setup Window"]
    #[inline(always)]
    pub fn setupwindow(&self) -> SetupwindowR {
        SetupwindowR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TX Delay Transmission"]
    #[inline(always)]
    pub fn txdelay(&mut self) -> TxdelayW<TimingcfgSpec> {
        TxdelayW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Chip Select Setup"]
    #[inline(always)]
    pub fn cssetup(&mut self) -> CssetupW<TimingcfgSpec> {
        CssetupW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Chip Select Hold"]
    #[inline(always)]
    pub fn cshold(&mut self) -> CsholdW<TimingcfgSpec> {
        CsholdW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Inter-Character Spacing"]
    #[inline(always)]
    pub fn ics(&mut self) -> IcsW<TimingcfgSpec> {
        IcsW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Setup Window"]
    #[inline(always)]
    pub fn setupwindow(&mut self) -> SetupwindowW<TimingcfgSpec> {
        SetupwindowW::new(self, 16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`timingcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimingcfgSpec;
impl crate::RegisterSpec for TimingcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timingcfg::R`](R) reader structure"]
impl crate::Readable for TimingcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`timingcfg::W`](W) writer structure"]
impl crate::Writable for TimingcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMINGCFG to value 0x0005_0000"]
impl crate::Resettable for TimingcfgSpec {
    const RESET_VALUE: u32 = 0x0005_0000;
}
