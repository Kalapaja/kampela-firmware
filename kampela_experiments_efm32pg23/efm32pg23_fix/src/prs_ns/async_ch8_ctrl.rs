#[doc = "Register `ASYNC_CH8_CTRL` reader"]
pub type R = crate::R<AsyncCh8CtrlSpec>;
#[doc = "Register `ASYNC_CH8_CTRL` writer"]
pub type W = crate::W<AsyncCh8CtrlSpec>;
#[doc = "Signal Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sigsel {
    #[doc = "0: NONE"]
    None = 0,
}
impl From<Sigsel> for u8 {
    #[inline(always)]
    fn from(variant: Sigsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sigsel {
    type Ux = u8;
}
impl crate::IsEnum for Sigsel {}
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SigselR = crate::FieldReader<Sigsel>;
impl SigselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sigsel> {
        match self.bits {
            0 => Some(Sigsel::None),
            _ => None,
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sigsel::None
    }
}
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SigselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sigsel>;
impl<'a, REG> SigselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "NONE"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sigsel::None)
    }
}
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SourceselR = crate::FieldReader;
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SourceselW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Function Select\n\nValue on reset: 12"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fnsel {
    #[doc = "0: Logical 0"]
    LogicalZero = 0,
    #[doc = "1: A NOR B"]
    ANorB = 1,
    #[doc = "2: (!A) AND B"]
    NotAAndB = 2,
    #[doc = "3: !A"]
    NotA = 3,
    #[doc = "4: A AND (!B)"]
    AAndNotB = 4,
    #[doc = "5: !B"]
    NotB = 5,
    #[doc = "6: A XOR B"]
    AXorB = 6,
    #[doc = "7: A NAND B"]
    ANandB = 7,
    #[doc = "8: A AND B"]
    AAndB = 8,
    #[doc = "9: A XNOR B"]
    AXnorB = 9,
    #[doc = "10: B"]
    B = 10,
    #[doc = "11: (!A) OR B"]
    NotAOrB = 11,
    #[doc = "12: A"]
    A = 12,
    #[doc = "13: A OR (!B)"]
    AOrNotB = 13,
    #[doc = "14: A OR B"]
    AOrB = 14,
    #[doc = "15: Logical 1"]
    LogicalOne = 15,
}
impl From<Fnsel> for u8 {
    #[inline(always)]
    fn from(variant: Fnsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fnsel {
    type Ux = u8;
}
impl crate::IsEnum for Fnsel {}
#[doc = "Field `FNSEL` reader - Function Select"]
pub type FnselR = crate::FieldReader<Fnsel>;
impl FnselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fnsel {
        match self.bits {
            0 => Fnsel::LogicalZero,
            1 => Fnsel::ANorB,
            2 => Fnsel::NotAAndB,
            3 => Fnsel::NotA,
            4 => Fnsel::AAndNotB,
            5 => Fnsel::NotB,
            6 => Fnsel::AXorB,
            7 => Fnsel::ANandB,
            8 => Fnsel::AAndB,
            9 => Fnsel::AXnorB,
            10 => Fnsel::B,
            11 => Fnsel::NotAOrB,
            12 => Fnsel::A,
            13 => Fnsel::AOrNotB,
            14 => Fnsel::AOrB,
            15 => Fnsel::LogicalOne,
            _ => unreachable!(),
        }
    }
    #[doc = "Logical 0"]
    #[inline(always)]
    pub fn is_logical_zero(&self) -> bool {
        *self == Fnsel::LogicalZero
    }
    #[doc = "A NOR B"]
    #[inline(always)]
    pub fn is_a_nor_b(&self) -> bool {
        *self == Fnsel::ANorB
    }
    #[doc = "(!A) AND B"]
    #[inline(always)]
    pub fn is_not_a_and_b(&self) -> bool {
        *self == Fnsel::NotAAndB
    }
    #[doc = "!A"]
    #[inline(always)]
    pub fn is_not_a(&self) -> bool {
        *self == Fnsel::NotA
    }
    #[doc = "A AND (!B)"]
    #[inline(always)]
    pub fn is_a_and_not_b(&self) -> bool {
        *self == Fnsel::AAndNotB
    }
    #[doc = "!B"]
    #[inline(always)]
    pub fn is_not_b(&self) -> bool {
        *self == Fnsel::NotB
    }
    #[doc = "A XOR B"]
    #[inline(always)]
    pub fn is_a_xor_b(&self) -> bool {
        *self == Fnsel::AXorB
    }
    #[doc = "A NAND B"]
    #[inline(always)]
    pub fn is_a_nand_b(&self) -> bool {
        *self == Fnsel::ANandB
    }
    #[doc = "A AND B"]
    #[inline(always)]
    pub fn is_a_and_b(&self) -> bool {
        *self == Fnsel::AAndB
    }
    #[doc = "A XNOR B"]
    #[inline(always)]
    pub fn is_a_xnor_b(&self) -> bool {
        *self == Fnsel::AXnorB
    }
    #[doc = "B"]
    #[inline(always)]
    pub fn is_b(&self) -> bool {
        *self == Fnsel::B
    }
    #[doc = "(!A) OR B"]
    #[inline(always)]
    pub fn is_not_a_or_b(&self) -> bool {
        *self == Fnsel::NotAOrB
    }
    #[doc = "A"]
    #[inline(always)]
    pub fn is_a(&self) -> bool {
        *self == Fnsel::A
    }
    #[doc = "A OR (!B)"]
    #[inline(always)]
    pub fn is_a_or_not_b(&self) -> bool {
        *self == Fnsel::AOrNotB
    }
    #[doc = "A OR B"]
    #[inline(always)]
    pub fn is_a_or_b(&self) -> bool {
        *self == Fnsel::AOrB
    }
    #[doc = "Logical 1"]
    #[inline(always)]
    pub fn is_logical_one(&self) -> bool {
        *self == Fnsel::LogicalOne
    }
}
#[doc = "Field `FNSEL` writer - Function Select"]
pub type FnselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Fnsel, crate::Safe>;
impl<'a, REG> FnselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Logical 0"]
    #[inline(always)]
    pub fn logical_zero(self) -> &'a mut crate::W<REG> {
        self.variant(Fnsel::LogicalZero)
    }
    #[doc = "A NOR B"]
    #[inline(always)]
    pub fn a_nor_b(self) -> &'a mut crate::W<REG> {
        self.variant(Fnsel::ANorB)
    }
    #[doc = "(!A) AND B"]
    #[inline(always)]
    pub fn not_a_and_b(self) -> &'a mut crate::W<REG> {
        self.variant(Fnsel::NotAAndB)
    }
    #[doc = "!A"]
    #[inline(always)]
    pub fn not_a(self) -> &'a mut crate::W<REG> {
        self.variant(Fnsel::NotA)
    }
    #[doc = "A AND (!B)"]
    #[inline(always)]
    pub fn a_and_not_b(self) -> &'a mut crate::W<REG> {
        self.variant(Fnsel::AAndNotB)
    }
    #[doc = "!B"]
    #[inline(always)]
    pub fn not_b(self) -> &'a mut crate::W<REG> {
        self.variant(Fnsel::NotB)
    }
    #[doc = "A XOR B"]
    #[inline(always)]
    pub fn a_xor_b(self) -> &'a mut crate::W<REG> {
        self.variant(Fnsel::AXorB)
    }
    #[doc = "A NAND B"]
    #[inline(always)]
    pub fn a_nand_b(self) -> &'a mut crate::W<REG> {
        self.variant(Fnsel::ANandB)
    }
    #[doc = "A AND B"]
    #[inline(always)]
    pub fn a_and_b(self) -> &'a mut crate::W<REG> {
        self.variant(Fnsel::AAndB)
    }
    #[doc = "A XNOR B"]
    #[inline(always)]
    pub fn a_xnor_b(self) -> &'a mut crate::W<REG> {
        self.variant(Fnsel::AXnorB)
    }
    #[doc = "B"]
    #[inline(always)]
    pub fn b(self) -> &'a mut crate::W<REG> {
        self.variant(Fnsel::B)
    }
    #[doc = "(!A) OR B"]
    #[inline(always)]
    pub fn not_a_or_b(self) -> &'a mut crate::W<REG> {
        self.variant(Fnsel::NotAOrB)
    }
    #[doc = "A"]
    #[inline(always)]
    pub fn a(self) -> &'a mut crate::W<REG> {
        self.variant(Fnsel::A)
    }
    #[doc = "A OR (!B)"]
    #[inline(always)]
    pub fn a_or_not_b(self) -> &'a mut crate::W<REG> {
        self.variant(Fnsel::AOrNotB)
    }
    #[doc = "A OR B"]
    #[inline(always)]
    pub fn a_or_b(self) -> &'a mut crate::W<REG> {
        self.variant(Fnsel::AOrB)
    }
    #[doc = "Logical 1"]
    #[inline(always)]
    pub fn logical_one(self) -> &'a mut crate::W<REG> {
        self.variant(Fnsel::LogicalOne)
    }
}
#[doc = "Field `AUXSEL` reader - Aux Select"]
pub type AuxselR = crate::FieldReader;
#[doc = "Field `AUXSEL` writer - Aux Select"]
pub type AuxselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SigselR {
        SigselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SourceselR {
        SourceselR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:19 - Function Select"]
    #[inline(always)]
    pub fn fnsel(&self) -> FnselR {
        FnselR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Aux Select"]
    #[inline(always)]
    pub fn auxsel(&self) -> AuxselR {
        AuxselR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&mut self) -> SigselW<AsyncCh8CtrlSpec> {
        SigselW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&mut self) -> SourceselW<AsyncCh8CtrlSpec> {
        SourceselW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Function Select"]
    #[inline(always)]
    pub fn fnsel(&mut self) -> FnselW<AsyncCh8CtrlSpec> {
        FnselW::new(self, 16)
    }
    #[doc = "Bits 24:27 - Aux Select"]
    #[inline(always)]
    pub fn auxsel(&mut self) -> AuxselW<AsyncCh8CtrlSpec> {
        AuxselW::new(self, 24)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`async_ch8_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`async_ch8_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsyncCh8CtrlSpec;
impl crate::RegisterSpec for AsyncCh8CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`async_ch8_ctrl::R`](R) reader structure"]
impl crate::Readable for AsyncCh8CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`async_ch8_ctrl::W`](W) writer structure"]
impl crate::Writable for AsyncCh8CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ASYNC_CH8_CTRL to value 0x000c_0000"]
impl crate::Resettable for AsyncCh8CtrlSpec {
    const RESET_VALUE: u32 = 0x000c_0000;
}
