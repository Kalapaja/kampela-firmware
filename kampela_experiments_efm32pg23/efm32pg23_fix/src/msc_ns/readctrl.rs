#[doc = "Register `READCTRL` reader"]
pub type R = crate::R<ReadctrlSpec>;
#[doc = "Register `READCTRL` writer"]
pub type W = crate::W<ReadctrlSpec>;
#[doc = "Read Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Zero wait-states inserted in fetch or read transfers"]
    Ws0 = 0,
    #[doc = "1: One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details"]
    Ws1 = 1,
    #[doc = "2: Two wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details"]
    Ws2 = 2,
    #[doc = "3: Three wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details"]
    Ws3 = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Read Mode"]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::Ws0,
            1 => Mode::Ws1,
            2 => Mode::Ws2,
            3 => Mode::Ws3,
            _ => unreachable!(),
        }
    }
    #[doc = "Zero wait-states inserted in fetch or read transfers"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == Mode::Ws0
    }
    #[doc = "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == Mode::Ws1
    }
    #[doc = "Two wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == Mode::Ws2
    }
    #[doc = "Three wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn is_ws3(&self) -> bool {
        *self == Mode::Ws3
    }
}
#[doc = "Field `MODE` writer - Read Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Zero wait-states inserted in fetch or read transfers"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ws0)
    }
    #[doc = "One wait-state inserted for each fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ws1)
    }
    #[doc = "Two wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ws2)
    }
    #[doc = "Three wait-states inserted for eatch fetch or read transfer. See Flash Wait-States table for details"]
    #[inline(always)]
    pub fn ws3(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ws3)
    }
}
impl R {
    #[doc = "Bits 20:21 - Read Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 20:21 - Read Mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<ReadctrlSpec> {
        ModeW::new(self, 20)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`readctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`readctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadctrlSpec;
impl crate::RegisterSpec for ReadctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`readctrl::R`](R) reader structure"]
impl crate::Readable for ReadctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`readctrl::W`](W) writer structure"]
impl crate::Writable for ReadctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets READCTRL to value 0x0020_0000"]
impl crate::Resettable for ReadctrlSpec {
    const RESET_VALUE: u32 = 0x0020_0000;
}
