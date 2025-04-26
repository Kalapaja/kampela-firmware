#[doc = "Register `FRAME` reader"]
pub type R = crate::R<FrameSpec>;
#[doc = "Register `FRAME` writer"]
pub type W = crate::W<FrameSpec>;
#[doc = "Data-Bit Mode\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Databits {
    #[doc = "1: Each frame contains 4 data bits"]
    Four = 1,
    #[doc = "2: Each frame contains 5 data bits"]
    Five = 2,
    #[doc = "3: Each frame contains 6 data bits"]
    Six = 3,
    #[doc = "4: Each frame contains 7 data bits"]
    Seven = 4,
    #[doc = "5: Each frame contains 8 data bits"]
    Eight = 5,
    #[doc = "6: Each frame contains 9 data bits"]
    Nine = 6,
    #[doc = "7: Each frame contains 10 data bits"]
    Ten = 7,
    #[doc = "8: Each frame contains 11 data bits"]
    Eleven = 8,
    #[doc = "9: Each frame contains 12 data bits"]
    Twelve = 9,
    #[doc = "10: Each frame contains 13 data bits"]
    Thirteen = 10,
    #[doc = "11: Each frame contains 14 data bits"]
    Fourteen = 11,
    #[doc = "12: Each frame contains 15 data bits"]
    Fifteen = 12,
    #[doc = "13: Each frame contains 16 data bits"]
    Sixteen = 13,
}
impl From<Databits> for u8 {
    #[inline(always)]
    fn from(variant: Databits) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Databits {
    type Ux = u8;
}
impl crate::IsEnum for Databits {}
#[doc = "Field `DATABITS` reader - Data-Bit Mode"]
pub type DatabitsR = crate::FieldReader<Databits>;
impl DatabitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Databits> {
        match self.bits {
            1 => Some(Databits::Four),
            2 => Some(Databits::Five),
            3 => Some(Databits::Six),
            4 => Some(Databits::Seven),
            5 => Some(Databits::Eight),
            6 => Some(Databits::Nine),
            7 => Some(Databits::Ten),
            8 => Some(Databits::Eleven),
            9 => Some(Databits::Twelve),
            10 => Some(Databits::Thirteen),
            11 => Some(Databits::Fourteen),
            12 => Some(Databits::Fifteen),
            13 => Some(Databits::Sixteen),
            _ => None,
        }
    }
    #[doc = "Each frame contains 4 data bits"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Databits::Four
    }
    #[doc = "Each frame contains 5 data bits"]
    #[inline(always)]
    pub fn is_five(&self) -> bool {
        *self == Databits::Five
    }
    #[doc = "Each frame contains 6 data bits"]
    #[inline(always)]
    pub fn is_six(&self) -> bool {
        *self == Databits::Six
    }
    #[doc = "Each frame contains 7 data bits"]
    #[inline(always)]
    pub fn is_seven(&self) -> bool {
        *self == Databits::Seven
    }
    #[doc = "Each frame contains 8 data bits"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == Databits::Eight
    }
    #[doc = "Each frame contains 9 data bits"]
    #[inline(always)]
    pub fn is_nine(&self) -> bool {
        *self == Databits::Nine
    }
    #[doc = "Each frame contains 10 data bits"]
    #[inline(always)]
    pub fn is_ten(&self) -> bool {
        *self == Databits::Ten
    }
    #[doc = "Each frame contains 11 data bits"]
    #[inline(always)]
    pub fn is_eleven(&self) -> bool {
        *self == Databits::Eleven
    }
    #[doc = "Each frame contains 12 data bits"]
    #[inline(always)]
    pub fn is_twelve(&self) -> bool {
        *self == Databits::Twelve
    }
    #[doc = "Each frame contains 13 data bits"]
    #[inline(always)]
    pub fn is_thirteen(&self) -> bool {
        *self == Databits::Thirteen
    }
    #[doc = "Each frame contains 14 data bits"]
    #[inline(always)]
    pub fn is_fourteen(&self) -> bool {
        *self == Databits::Fourteen
    }
    #[doc = "Each frame contains 15 data bits"]
    #[inline(always)]
    pub fn is_fifteen(&self) -> bool {
        *self == Databits::Fifteen
    }
    #[doc = "Each frame contains 16 data bits"]
    #[inline(always)]
    pub fn is_sixteen(&self) -> bool {
        *self == Databits::Sixteen
    }
}
#[doc = "Field `DATABITS` writer - Data-Bit Mode"]
pub type DatabitsW<'a, REG> = crate::FieldWriter<'a, REG, 4, Databits>;
impl<'a, REG> DatabitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Each frame contains 4 data bits"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Databits::Four)
    }
    #[doc = "Each frame contains 5 data bits"]
    #[inline(always)]
    pub fn five(self) -> &'a mut crate::W<REG> {
        self.variant(Databits::Five)
    }
    #[doc = "Each frame contains 6 data bits"]
    #[inline(always)]
    pub fn six(self) -> &'a mut crate::W<REG> {
        self.variant(Databits::Six)
    }
    #[doc = "Each frame contains 7 data bits"]
    #[inline(always)]
    pub fn seven(self) -> &'a mut crate::W<REG> {
        self.variant(Databits::Seven)
    }
    #[doc = "Each frame contains 8 data bits"]
    #[inline(always)]
    pub fn eight(self) -> &'a mut crate::W<REG> {
        self.variant(Databits::Eight)
    }
    #[doc = "Each frame contains 9 data bits"]
    #[inline(always)]
    pub fn nine(self) -> &'a mut crate::W<REG> {
        self.variant(Databits::Nine)
    }
    #[doc = "Each frame contains 10 data bits"]
    #[inline(always)]
    pub fn ten(self) -> &'a mut crate::W<REG> {
        self.variant(Databits::Ten)
    }
    #[doc = "Each frame contains 11 data bits"]
    #[inline(always)]
    pub fn eleven(self) -> &'a mut crate::W<REG> {
        self.variant(Databits::Eleven)
    }
    #[doc = "Each frame contains 12 data bits"]
    #[inline(always)]
    pub fn twelve(self) -> &'a mut crate::W<REG> {
        self.variant(Databits::Twelve)
    }
    #[doc = "Each frame contains 13 data bits"]
    #[inline(always)]
    pub fn thirteen(self) -> &'a mut crate::W<REG> {
        self.variant(Databits::Thirteen)
    }
    #[doc = "Each frame contains 14 data bits"]
    #[inline(always)]
    pub fn fourteen(self) -> &'a mut crate::W<REG> {
        self.variant(Databits::Fourteen)
    }
    #[doc = "Each frame contains 15 data bits"]
    #[inline(always)]
    pub fn fifteen(self) -> &'a mut crate::W<REG> {
        self.variant(Databits::Fifteen)
    }
    #[doc = "Each frame contains 16 data bits"]
    #[inline(always)]
    pub fn sixteen(self) -> &'a mut crate::W<REG> {
        self.variant(Databits::Sixteen)
    }
}
#[doc = "Parity-Bit Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Parity {
    #[doc = "0: Parity bits are not used"]
    None = 0,
    #[doc = "2: Even parity are used. Parity bits are automatically generated and checked by hardware."]
    Even = 2,
    #[doc = "3: Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    Odd = 3,
}
impl From<Parity> for u8 {
    #[inline(always)]
    fn from(variant: Parity) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Parity {
    type Ux = u8;
}
impl crate::IsEnum for Parity {}
#[doc = "Field `PARITY` reader - Parity-Bit Mode"]
pub type ParityR = crate::FieldReader<Parity>;
impl ParityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Parity> {
        match self.bits {
            0 => Some(Parity::None),
            2 => Some(Parity::Even),
            3 => Some(Parity::Odd),
            _ => None,
        }
    }
    #[doc = "Parity bits are not used"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Parity::None
    }
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Parity::Even
    }
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Parity::Odd
    }
}
#[doc = "Field `PARITY` writer - Parity-Bit Mode"]
pub type ParityW<'a, REG> = crate::FieldWriter<'a, REG, 2, Parity>;
impl<'a, REG> ParityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Parity bits are not used"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::None)
    }
    #[doc = "Even parity are used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::Even)
    }
    #[doc = "Odd parity is used. Parity bits are automatically generated and checked by hardware."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::Odd)
    }
}
#[doc = "Stop-Bit Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stopbits {
    #[doc = "0: The transmitter generates a half stop bit. Stop-bits are not verified by receiver"]
    Half = 0,
    #[doc = "1: One stop bit is generated and verified"]
    One = 1,
    #[doc = "2: The transmitter generates one and a half stop bit. The receiver verifies the first stop bit"]
    Oneandahalf = 2,
    #[doc = "3: The transmitter generates two stop bits. The receiver checks the first stop-bit only"]
    Two = 3,
}
impl From<Stopbits> for u8 {
    #[inline(always)]
    fn from(variant: Stopbits) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stopbits {
    type Ux = u8;
}
impl crate::IsEnum for Stopbits {}
#[doc = "Field `STOPBITS` reader - Stop-Bit Mode"]
pub type StopbitsR = crate::FieldReader<Stopbits>;
impl StopbitsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopbits {
        match self.bits {
            0 => Stopbits::Half,
            1 => Stopbits::One,
            2 => Stopbits::Oneandahalf,
            3 => Stopbits::Two,
            _ => unreachable!(),
        }
    }
    #[doc = "The transmitter generates a half stop bit. Stop-bits are not verified by receiver"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == Stopbits::Half
    }
    #[doc = "One stop bit is generated and verified"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Stopbits::One
    }
    #[doc = "The transmitter generates one and a half stop bit. The receiver verifies the first stop bit"]
    #[inline(always)]
    pub fn is_oneandahalf(&self) -> bool {
        *self == Stopbits::Oneandahalf
    }
    #[doc = "The transmitter generates two stop bits. The receiver checks the first stop-bit only"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Stopbits::Two
    }
}
#[doc = "Field `STOPBITS` writer - Stop-Bit Mode"]
pub type StopbitsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Stopbits, crate::Safe>;
impl<'a, REG> StopbitsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The transmitter generates a half stop bit. Stop-bits are not verified by receiver"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(Stopbits::Half)
    }
    #[doc = "One stop bit is generated and verified"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Stopbits::One)
    }
    #[doc = "The transmitter generates one and a half stop bit. The receiver verifies the first stop bit"]
    #[inline(always)]
    pub fn oneandahalf(self) -> &'a mut crate::W<REG> {
        self.variant(Stopbits::Oneandahalf)
    }
    #[doc = "The transmitter generates two stop bits. The receiver checks the first stop-bit only"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Stopbits::Two)
    }
}
impl R {
    #[doc = "Bits 0:3 - Data-Bit Mode"]
    #[inline(always)]
    pub fn databits(&self) -> DatabitsR {
        DatabitsR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Parity-Bit Mode"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Stop-Bit Mode"]
    #[inline(always)]
    pub fn stopbits(&self) -> StopbitsR {
        StopbitsR::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data-Bit Mode"]
    #[inline(always)]
    pub fn databits(&mut self) -> DatabitsW<FrameSpec> {
        DatabitsW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Parity-Bit Mode"]
    #[inline(always)]
    pub fn parity(&mut self) -> ParityW<FrameSpec> {
        ParityW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Stop-Bit Mode"]
    #[inline(always)]
    pub fn stopbits(&mut self) -> StopbitsW<FrameSpec> {
        StopbitsW::new(self, 12)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`frame::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frame::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FrameSpec;
impl crate::RegisterSpec for FrameSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frame::R`](R) reader structure"]
impl crate::Readable for FrameSpec {}
#[doc = "`write(|w| ..)` method takes [`frame::W`](W) writer structure"]
impl crate::Writable for FrameSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FRAME to value 0x1005"]
impl crate::Resettable for FrameSpec {
    const RESET_VALUE: u32 = 0x1005;
}
