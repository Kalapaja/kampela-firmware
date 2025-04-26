#[doc = "Register `DELAY` reader"]
pub type R = crate::R<DelaySpec>;
#[doc = "Register `DELAY` writer"]
pub type W = crate::W<DelaySpec>;
#[doc = "Scan Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scandly {
    #[doc = "0: 2ms Scan Delay"]
    Scandly2 = 0,
    #[doc = "1: 4ms Scan Delay"]
    Scandly4 = 1,
    #[doc = "2: 6ms Scan Delay"]
    Scandly6 = 2,
    #[doc = "3: 8ms Scan Delay"]
    Scandly8 = 3,
    #[doc = "4: 10ms Scan Delay"]
    Scandly10 = 4,
    #[doc = "5: 12ms Scan Delay"]
    Scandly12 = 5,
    #[doc = "6: 14ms Scan Delay"]
    Scandly14 = 6,
    #[doc = "7: 16ms Scan Delay"]
    Scandly16 = 7,
    #[doc = "8: 18ms Scan Delay"]
    Scandly18 = 8,
    #[doc = "9: 20ms Scan Delay"]
    Scandly20 = 9,
    #[doc = "10: 22ms Scan Delay"]
    Scandly22 = 10,
    #[doc = "11: 24ms Scan Delay"]
    Scandly24 = 11,
    #[doc = "12: 26ms Scan Delay"]
    Scandly26 = 12,
    #[doc = "13: 28ms Scan Delay"]
    Scandly28 = 13,
    #[doc = "14: 30ms Scan Delay"]
    Scandly30 = 14,
    #[doc = "15: 32ms Scan Delay"]
    Scandly32 = 15,
}
impl From<Scandly> for u8 {
    #[inline(always)]
    fn from(variant: Scandly) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scandly {
    type Ux = u8;
}
impl crate::IsEnum for Scandly {}
#[doc = "Field `SCANDLY` reader - Scan Delay"]
pub type ScandlyR = crate::FieldReader<Scandly>;
impl ScandlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scandly {
        match self.bits {
            0 => Scandly::Scandly2,
            1 => Scandly::Scandly4,
            2 => Scandly::Scandly6,
            3 => Scandly::Scandly8,
            4 => Scandly::Scandly10,
            5 => Scandly::Scandly12,
            6 => Scandly::Scandly14,
            7 => Scandly::Scandly16,
            8 => Scandly::Scandly18,
            9 => Scandly::Scandly20,
            10 => Scandly::Scandly22,
            11 => Scandly::Scandly24,
            12 => Scandly::Scandly26,
            13 => Scandly::Scandly28,
            14 => Scandly::Scandly30,
            15 => Scandly::Scandly32,
            _ => unreachable!(),
        }
    }
    #[doc = "2ms Scan Delay"]
    #[inline(always)]
    pub fn is_scandly2(&self) -> bool {
        *self == Scandly::Scandly2
    }
    #[doc = "4ms Scan Delay"]
    #[inline(always)]
    pub fn is_scandly4(&self) -> bool {
        *self == Scandly::Scandly4
    }
    #[doc = "6ms Scan Delay"]
    #[inline(always)]
    pub fn is_scandly6(&self) -> bool {
        *self == Scandly::Scandly6
    }
    #[doc = "8ms Scan Delay"]
    #[inline(always)]
    pub fn is_scandly8(&self) -> bool {
        *self == Scandly::Scandly8
    }
    #[doc = "10ms Scan Delay"]
    #[inline(always)]
    pub fn is_scandly10(&self) -> bool {
        *self == Scandly::Scandly10
    }
    #[doc = "12ms Scan Delay"]
    #[inline(always)]
    pub fn is_scandly12(&self) -> bool {
        *self == Scandly::Scandly12
    }
    #[doc = "14ms Scan Delay"]
    #[inline(always)]
    pub fn is_scandly14(&self) -> bool {
        *self == Scandly::Scandly14
    }
    #[doc = "16ms Scan Delay"]
    #[inline(always)]
    pub fn is_scandly16(&self) -> bool {
        *self == Scandly::Scandly16
    }
    #[doc = "18ms Scan Delay"]
    #[inline(always)]
    pub fn is_scandly18(&self) -> bool {
        *self == Scandly::Scandly18
    }
    #[doc = "20ms Scan Delay"]
    #[inline(always)]
    pub fn is_scandly20(&self) -> bool {
        *self == Scandly::Scandly20
    }
    #[doc = "22ms Scan Delay"]
    #[inline(always)]
    pub fn is_scandly22(&self) -> bool {
        *self == Scandly::Scandly22
    }
    #[doc = "24ms Scan Delay"]
    #[inline(always)]
    pub fn is_scandly24(&self) -> bool {
        *self == Scandly::Scandly24
    }
    #[doc = "26ms Scan Delay"]
    #[inline(always)]
    pub fn is_scandly26(&self) -> bool {
        *self == Scandly::Scandly26
    }
    #[doc = "28ms Scan Delay"]
    #[inline(always)]
    pub fn is_scandly28(&self) -> bool {
        *self == Scandly::Scandly28
    }
    #[doc = "30ms Scan Delay"]
    #[inline(always)]
    pub fn is_scandly30(&self) -> bool {
        *self == Scandly::Scandly30
    }
    #[doc = "32ms Scan Delay"]
    #[inline(always)]
    pub fn is_scandly32(&self) -> bool {
        *self == Scandly::Scandly32
    }
}
#[doc = "Field `SCANDLY` writer - Scan Delay"]
pub type ScandlyW<'a, REG> = crate::FieldWriter<'a, REG, 4, Scandly, crate::Safe>;
impl<'a, REG> ScandlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2ms Scan Delay"]
    #[inline(always)]
    pub fn scandly2(self) -> &'a mut crate::W<REG> {
        self.variant(Scandly::Scandly2)
    }
    #[doc = "4ms Scan Delay"]
    #[inline(always)]
    pub fn scandly4(self) -> &'a mut crate::W<REG> {
        self.variant(Scandly::Scandly4)
    }
    #[doc = "6ms Scan Delay"]
    #[inline(always)]
    pub fn scandly6(self) -> &'a mut crate::W<REG> {
        self.variant(Scandly::Scandly6)
    }
    #[doc = "8ms Scan Delay"]
    #[inline(always)]
    pub fn scandly8(self) -> &'a mut crate::W<REG> {
        self.variant(Scandly::Scandly8)
    }
    #[doc = "10ms Scan Delay"]
    #[inline(always)]
    pub fn scandly10(self) -> &'a mut crate::W<REG> {
        self.variant(Scandly::Scandly10)
    }
    #[doc = "12ms Scan Delay"]
    #[inline(always)]
    pub fn scandly12(self) -> &'a mut crate::W<REG> {
        self.variant(Scandly::Scandly12)
    }
    #[doc = "14ms Scan Delay"]
    #[inline(always)]
    pub fn scandly14(self) -> &'a mut crate::W<REG> {
        self.variant(Scandly::Scandly14)
    }
    #[doc = "16ms Scan Delay"]
    #[inline(always)]
    pub fn scandly16(self) -> &'a mut crate::W<REG> {
        self.variant(Scandly::Scandly16)
    }
    #[doc = "18ms Scan Delay"]
    #[inline(always)]
    pub fn scandly18(self) -> &'a mut crate::W<REG> {
        self.variant(Scandly::Scandly18)
    }
    #[doc = "20ms Scan Delay"]
    #[inline(always)]
    pub fn scandly20(self) -> &'a mut crate::W<REG> {
        self.variant(Scandly::Scandly20)
    }
    #[doc = "22ms Scan Delay"]
    #[inline(always)]
    pub fn scandly22(self) -> &'a mut crate::W<REG> {
        self.variant(Scandly::Scandly22)
    }
    #[doc = "24ms Scan Delay"]
    #[inline(always)]
    pub fn scandly24(self) -> &'a mut crate::W<REG> {
        self.variant(Scandly::Scandly24)
    }
    #[doc = "26ms Scan Delay"]
    #[inline(always)]
    pub fn scandly26(self) -> &'a mut crate::W<REG> {
        self.variant(Scandly::Scandly26)
    }
    #[doc = "28ms Scan Delay"]
    #[inline(always)]
    pub fn scandly28(self) -> &'a mut crate::W<REG> {
        self.variant(Scandly::Scandly28)
    }
    #[doc = "30ms Scan Delay"]
    #[inline(always)]
    pub fn scandly30(self) -> &'a mut crate::W<REG> {
        self.variant(Scandly::Scandly30)
    }
    #[doc = "32ms Scan Delay"]
    #[inline(always)]
    pub fn scandly32(self) -> &'a mut crate::W<REG> {
        self.variant(Scandly::Scandly32)
    }
}
#[doc = "Debounce Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Debdly {
    #[doc = "0: 2ms Debounce Delay"]
    Debdly2 = 0,
    #[doc = "1: 4ms Debounce Delay"]
    Debdly4 = 1,
    #[doc = "2: 6ms Debounce Delay"]
    Debdly6 = 2,
    #[doc = "3: 8ms Debounce Delay"]
    Debdly8 = 3,
    #[doc = "4: 10ms Debounce Delay"]
    Debdly10 = 4,
    #[doc = "5: 12ms Debounce Delay"]
    Debdly12 = 5,
    #[doc = "6: 14ms Debounce Delay"]
    Debdly14 = 6,
    #[doc = "7: 16ms Debounce Delay"]
    Debdly16 = 7,
    #[doc = "8: 18ms Debounce Delay"]
    Debdly18 = 8,
    #[doc = "9: 20ms Debounce Delay"]
    Debdly20 = 9,
    #[doc = "10: 22ms Debounce Delay"]
    Debdly22 = 10,
    #[doc = "11: 24ms Debounce Delay"]
    Debdly24 = 11,
    #[doc = "12: 26ms Debounce Delay"]
    Debdly26 = 12,
    #[doc = "13: 28ms Debounce Delay"]
    Debdly28 = 13,
    #[doc = "14: 30ms Debounce Delay"]
    Debdly30 = 14,
    #[doc = "15: 32ms Debounce Delay"]
    Debdly32 = 15,
}
impl From<Debdly> for u8 {
    #[inline(always)]
    fn from(variant: Debdly) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Debdly {
    type Ux = u8;
}
impl crate::IsEnum for Debdly {}
#[doc = "Field `DEBDLY` reader - Debounce Delay"]
pub type DebdlyR = crate::FieldReader<Debdly>;
impl DebdlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Debdly {
        match self.bits {
            0 => Debdly::Debdly2,
            1 => Debdly::Debdly4,
            2 => Debdly::Debdly6,
            3 => Debdly::Debdly8,
            4 => Debdly::Debdly10,
            5 => Debdly::Debdly12,
            6 => Debdly::Debdly14,
            7 => Debdly::Debdly16,
            8 => Debdly::Debdly18,
            9 => Debdly::Debdly20,
            10 => Debdly::Debdly22,
            11 => Debdly::Debdly24,
            12 => Debdly::Debdly26,
            13 => Debdly::Debdly28,
            14 => Debdly::Debdly30,
            15 => Debdly::Debdly32,
            _ => unreachable!(),
        }
    }
    #[doc = "2ms Debounce Delay"]
    #[inline(always)]
    pub fn is_debdly2(&self) -> bool {
        *self == Debdly::Debdly2
    }
    #[doc = "4ms Debounce Delay"]
    #[inline(always)]
    pub fn is_debdly4(&self) -> bool {
        *self == Debdly::Debdly4
    }
    #[doc = "6ms Debounce Delay"]
    #[inline(always)]
    pub fn is_debdly6(&self) -> bool {
        *self == Debdly::Debdly6
    }
    #[doc = "8ms Debounce Delay"]
    #[inline(always)]
    pub fn is_debdly8(&self) -> bool {
        *self == Debdly::Debdly8
    }
    #[doc = "10ms Debounce Delay"]
    #[inline(always)]
    pub fn is_debdly10(&self) -> bool {
        *self == Debdly::Debdly10
    }
    #[doc = "12ms Debounce Delay"]
    #[inline(always)]
    pub fn is_debdly12(&self) -> bool {
        *self == Debdly::Debdly12
    }
    #[doc = "14ms Debounce Delay"]
    #[inline(always)]
    pub fn is_debdly14(&self) -> bool {
        *self == Debdly::Debdly14
    }
    #[doc = "16ms Debounce Delay"]
    #[inline(always)]
    pub fn is_debdly16(&self) -> bool {
        *self == Debdly::Debdly16
    }
    #[doc = "18ms Debounce Delay"]
    #[inline(always)]
    pub fn is_debdly18(&self) -> bool {
        *self == Debdly::Debdly18
    }
    #[doc = "20ms Debounce Delay"]
    #[inline(always)]
    pub fn is_debdly20(&self) -> bool {
        *self == Debdly::Debdly20
    }
    #[doc = "22ms Debounce Delay"]
    #[inline(always)]
    pub fn is_debdly22(&self) -> bool {
        *self == Debdly::Debdly22
    }
    #[doc = "24ms Debounce Delay"]
    #[inline(always)]
    pub fn is_debdly24(&self) -> bool {
        *self == Debdly::Debdly24
    }
    #[doc = "26ms Debounce Delay"]
    #[inline(always)]
    pub fn is_debdly26(&self) -> bool {
        *self == Debdly::Debdly26
    }
    #[doc = "28ms Debounce Delay"]
    #[inline(always)]
    pub fn is_debdly28(&self) -> bool {
        *self == Debdly::Debdly28
    }
    #[doc = "30ms Debounce Delay"]
    #[inline(always)]
    pub fn is_debdly30(&self) -> bool {
        *self == Debdly::Debdly30
    }
    #[doc = "32ms Debounce Delay"]
    #[inline(always)]
    pub fn is_debdly32(&self) -> bool {
        *self == Debdly::Debdly32
    }
}
#[doc = "Field `DEBDLY` writer - Debounce Delay"]
pub type DebdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4, Debdly, crate::Safe>;
impl<'a, REG> DebdlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly2(self) -> &'a mut crate::W<REG> {
        self.variant(Debdly::Debdly2)
    }
    #[doc = "4ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly4(self) -> &'a mut crate::W<REG> {
        self.variant(Debdly::Debdly4)
    }
    #[doc = "6ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly6(self) -> &'a mut crate::W<REG> {
        self.variant(Debdly::Debdly6)
    }
    #[doc = "8ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly8(self) -> &'a mut crate::W<REG> {
        self.variant(Debdly::Debdly8)
    }
    #[doc = "10ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly10(self) -> &'a mut crate::W<REG> {
        self.variant(Debdly::Debdly10)
    }
    #[doc = "12ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly12(self) -> &'a mut crate::W<REG> {
        self.variant(Debdly::Debdly12)
    }
    #[doc = "14ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly14(self) -> &'a mut crate::W<REG> {
        self.variant(Debdly::Debdly14)
    }
    #[doc = "16ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly16(self) -> &'a mut crate::W<REG> {
        self.variant(Debdly::Debdly16)
    }
    #[doc = "18ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly18(self) -> &'a mut crate::W<REG> {
        self.variant(Debdly::Debdly18)
    }
    #[doc = "20ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly20(self) -> &'a mut crate::W<REG> {
        self.variant(Debdly::Debdly20)
    }
    #[doc = "22ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly22(self) -> &'a mut crate::W<REG> {
        self.variant(Debdly::Debdly22)
    }
    #[doc = "24ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly24(self) -> &'a mut crate::W<REG> {
        self.variant(Debdly::Debdly24)
    }
    #[doc = "26ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly26(self) -> &'a mut crate::W<REG> {
        self.variant(Debdly::Debdly26)
    }
    #[doc = "28ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly28(self) -> &'a mut crate::W<REG> {
        self.variant(Debdly::Debdly28)
    }
    #[doc = "30ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly30(self) -> &'a mut crate::W<REG> {
        self.variant(Debdly::Debdly30)
    }
    #[doc = "32ms Debounce Delay"]
    #[inline(always)]
    pub fn debdly32(self) -> &'a mut crate::W<REG> {
        self.variant(Debdly::Debdly32)
    }
}
#[doc = "Row stable Delay\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Stabdly {
    #[doc = "0: 2ms Row Stable Delay"]
    Stabdly2 = 0,
    #[doc = "1: 4ms Row Stable Delay"]
    Stabdly4 = 1,
    #[doc = "2: 6ms Row Stable Delay"]
    Stabdly6 = 2,
    #[doc = "3: 8ms Row Stable Delay"]
    Stabdly8 = 3,
    #[doc = "4: 10ms Row Stable Delay"]
    Stabdly10 = 4,
    #[doc = "5: 12ms Row Stable Delay"]
    Stabdly12 = 5,
    #[doc = "6: 14ms Row Stable Delay"]
    Stabdly14 = 6,
    #[doc = "7: 16ms Row Stable Delay"]
    Stabdly16 = 7,
    #[doc = "8: 18ms Row Stable Delay"]
    Stabdly18 = 8,
    #[doc = "9: 20ms Row Stable Delay"]
    Stabdly20 = 9,
    #[doc = "10: 22ms Row Stable Delay"]
    Stabdly22 = 10,
    #[doc = "11: 24ms Row Stable Delay"]
    Stabdly24 = 11,
    #[doc = "12: 26ms Row Stable Delay"]
    Stabdly26 = 12,
    #[doc = "13: 28ms Row Stable Delay"]
    Stabdly28 = 13,
    #[doc = "14: 30ms Row Stable Delay"]
    Stabdly30 = 14,
    #[doc = "15: 32ms Row Stable Delay"]
    Stabdly32 = 15,
}
impl From<Stabdly> for u8 {
    #[inline(always)]
    fn from(variant: Stabdly) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Stabdly {
    type Ux = u8;
}
impl crate::IsEnum for Stabdly {}
#[doc = "Field `STABDLY` reader - Row stable Delay"]
pub type StabdlyR = crate::FieldReader<Stabdly>;
impl StabdlyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stabdly {
        match self.bits {
            0 => Stabdly::Stabdly2,
            1 => Stabdly::Stabdly4,
            2 => Stabdly::Stabdly6,
            3 => Stabdly::Stabdly8,
            4 => Stabdly::Stabdly10,
            5 => Stabdly::Stabdly12,
            6 => Stabdly::Stabdly14,
            7 => Stabdly::Stabdly16,
            8 => Stabdly::Stabdly18,
            9 => Stabdly::Stabdly20,
            10 => Stabdly::Stabdly22,
            11 => Stabdly::Stabdly24,
            12 => Stabdly::Stabdly26,
            13 => Stabdly::Stabdly28,
            14 => Stabdly::Stabdly30,
            15 => Stabdly::Stabdly32,
            _ => unreachable!(),
        }
    }
    #[doc = "2ms Row Stable Delay"]
    #[inline(always)]
    pub fn is_stabdly2(&self) -> bool {
        *self == Stabdly::Stabdly2
    }
    #[doc = "4ms Row Stable Delay"]
    #[inline(always)]
    pub fn is_stabdly4(&self) -> bool {
        *self == Stabdly::Stabdly4
    }
    #[doc = "6ms Row Stable Delay"]
    #[inline(always)]
    pub fn is_stabdly6(&self) -> bool {
        *self == Stabdly::Stabdly6
    }
    #[doc = "8ms Row Stable Delay"]
    #[inline(always)]
    pub fn is_stabdly8(&self) -> bool {
        *self == Stabdly::Stabdly8
    }
    #[doc = "10ms Row Stable Delay"]
    #[inline(always)]
    pub fn is_stabdly10(&self) -> bool {
        *self == Stabdly::Stabdly10
    }
    #[doc = "12ms Row Stable Delay"]
    #[inline(always)]
    pub fn is_stabdly12(&self) -> bool {
        *self == Stabdly::Stabdly12
    }
    #[doc = "14ms Row Stable Delay"]
    #[inline(always)]
    pub fn is_stabdly14(&self) -> bool {
        *self == Stabdly::Stabdly14
    }
    #[doc = "16ms Row Stable Delay"]
    #[inline(always)]
    pub fn is_stabdly16(&self) -> bool {
        *self == Stabdly::Stabdly16
    }
    #[doc = "18ms Row Stable Delay"]
    #[inline(always)]
    pub fn is_stabdly18(&self) -> bool {
        *self == Stabdly::Stabdly18
    }
    #[doc = "20ms Row Stable Delay"]
    #[inline(always)]
    pub fn is_stabdly20(&self) -> bool {
        *self == Stabdly::Stabdly20
    }
    #[doc = "22ms Row Stable Delay"]
    #[inline(always)]
    pub fn is_stabdly22(&self) -> bool {
        *self == Stabdly::Stabdly22
    }
    #[doc = "24ms Row Stable Delay"]
    #[inline(always)]
    pub fn is_stabdly24(&self) -> bool {
        *self == Stabdly::Stabdly24
    }
    #[doc = "26ms Row Stable Delay"]
    #[inline(always)]
    pub fn is_stabdly26(&self) -> bool {
        *self == Stabdly::Stabdly26
    }
    #[doc = "28ms Row Stable Delay"]
    #[inline(always)]
    pub fn is_stabdly28(&self) -> bool {
        *self == Stabdly::Stabdly28
    }
    #[doc = "30ms Row Stable Delay"]
    #[inline(always)]
    pub fn is_stabdly30(&self) -> bool {
        *self == Stabdly::Stabdly30
    }
    #[doc = "32ms Row Stable Delay"]
    #[inline(always)]
    pub fn is_stabdly32(&self) -> bool {
        *self == Stabdly::Stabdly32
    }
}
#[doc = "Field `STABDLY` writer - Row stable Delay"]
pub type StabdlyW<'a, REG> = crate::FieldWriter<'a, REG, 4, Stabdly, crate::Safe>;
impl<'a, REG> StabdlyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly2(self) -> &'a mut crate::W<REG> {
        self.variant(Stabdly::Stabdly2)
    }
    #[doc = "4ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly4(self) -> &'a mut crate::W<REG> {
        self.variant(Stabdly::Stabdly4)
    }
    #[doc = "6ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly6(self) -> &'a mut crate::W<REG> {
        self.variant(Stabdly::Stabdly6)
    }
    #[doc = "8ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly8(self) -> &'a mut crate::W<REG> {
        self.variant(Stabdly::Stabdly8)
    }
    #[doc = "10ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly10(self) -> &'a mut crate::W<REG> {
        self.variant(Stabdly::Stabdly10)
    }
    #[doc = "12ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly12(self) -> &'a mut crate::W<REG> {
        self.variant(Stabdly::Stabdly12)
    }
    #[doc = "14ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly14(self) -> &'a mut crate::W<REG> {
        self.variant(Stabdly::Stabdly14)
    }
    #[doc = "16ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly16(self) -> &'a mut crate::W<REG> {
        self.variant(Stabdly::Stabdly16)
    }
    #[doc = "18ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly18(self) -> &'a mut crate::W<REG> {
        self.variant(Stabdly::Stabdly18)
    }
    #[doc = "20ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly20(self) -> &'a mut crate::W<REG> {
        self.variant(Stabdly::Stabdly20)
    }
    #[doc = "22ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly22(self) -> &'a mut crate::W<REG> {
        self.variant(Stabdly::Stabdly22)
    }
    #[doc = "24ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly24(self) -> &'a mut crate::W<REG> {
        self.variant(Stabdly::Stabdly24)
    }
    #[doc = "26ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly26(self) -> &'a mut crate::W<REG> {
        self.variant(Stabdly::Stabdly26)
    }
    #[doc = "28ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly28(self) -> &'a mut crate::W<REG> {
        self.variant(Stabdly::Stabdly28)
    }
    #[doc = "30ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly30(self) -> &'a mut crate::W<REG> {
        self.variant(Stabdly::Stabdly30)
    }
    #[doc = "32ms Row Stable Delay"]
    #[inline(always)]
    pub fn stabdly32(self) -> &'a mut crate::W<REG> {
        self.variant(Stabdly::Stabdly32)
    }
}
impl R {
    #[doc = "Bits 8:11 - Scan Delay"]
    #[inline(always)]
    pub fn scandly(&self) -> ScandlyR {
        ScandlyR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Debounce Delay"]
    #[inline(always)]
    pub fn debdly(&self) -> DebdlyR {
        DebdlyR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Row stable Delay"]
    #[inline(always)]
    pub fn stabdly(&self) -> StabdlyR {
        StabdlyR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - Scan Delay"]
    #[inline(always)]
    pub fn scandly(&mut self) -> ScandlyW<DelaySpec> {
        ScandlyW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Debounce Delay"]
    #[inline(always)]
    pub fn debdly(&mut self) -> DebdlyW<DelaySpec> {
        DebdlyW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Row stable Delay"]
    #[inline(always)]
    pub fn stabdly(&mut self) -> StabdlyW<DelaySpec> {
        StabdlyW::new(self, 24)
    }
}
#[doc = "Delay\n\nYou can [`read`](crate::Reg::read) this register and get [`delay::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delay::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DelaySpec;
impl crate::RegisterSpec for DelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`delay::R`](R) reader structure"]
impl crate::Readable for DelaySpec {}
#[doc = "`write(|w| ..)` method takes [`delay::W`](W) writer structure"]
impl crate::Writable for DelaySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DELAY to value 0"]
impl crate::Resettable for DelaySpec {}
