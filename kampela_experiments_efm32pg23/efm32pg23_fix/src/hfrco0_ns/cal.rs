#[doc = "Register `CAL` reader"]
pub type R = crate::R<CalSpec>;
#[doc = "Register `CAL` writer"]
pub type W = crate::W<CalSpec>;
#[doc = "Field `TUNING` reader - Tuning Value"]
pub type TuningR = crate::FieldReader;
#[doc = "Field `TUNING` writer - Tuning Value"]
pub type TuningW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FINETUNING` reader - Fine Tuning Value"]
pub type FinetuningR = crate::FieldReader;
#[doc = "Field `FINETUNING` writer - Fine Tuning Value"]
pub type FinetuningW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `LDOHP` reader - LDO High Power Mode"]
pub type LdohpR = crate::BitReader;
#[doc = "Field `LDOHP` writer - LDO High Power Mode"]
pub type LdohpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREQRANGE` reader - Frequency Range"]
pub type FreqrangeR = crate::FieldReader;
#[doc = "Field `FREQRANGE` writer - Frequency Range"]
pub type FreqrangeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CMPBIAS` reader - Comparator Bias Current"]
pub type CmpbiasR = crate::FieldReader;
#[doc = "Field `CMPBIAS` writer - Comparator Bias Current"]
pub type CmpbiasW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Locally Divide HFRCO Clock Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clkdiv {
    #[doc = "0: Divide by 1."]
    Div1 = 0,
    #[doc = "1: Divide by 2."]
    Div2 = 1,
    #[doc = "2: Divide by 4."]
    Div4 = 2,
}
impl From<Clkdiv> for u8 {
    #[inline(always)]
    fn from(variant: Clkdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clkdiv {
    type Ux = u8;
}
impl crate::IsEnum for Clkdiv {}
#[doc = "Field `CLKDIV` reader - Locally Divide HFRCO Clock Output"]
pub type ClkdivR = crate::FieldReader<Clkdiv>;
impl ClkdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clkdiv> {
        match self.bits {
            0 => Some(Clkdiv::Div1),
            1 => Some(Clkdiv::Div2),
            2 => Some(Clkdiv::Div4),
            _ => None,
        }
    }
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Clkdiv::Div1
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Clkdiv::Div2
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Clkdiv::Div4
    }
}
#[doc = "Field `CLKDIV` writer - Locally Divide HFRCO Clock Output"]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clkdiv>;
impl<'a, REG> ClkdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div1)
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div2)
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Clkdiv::Div4)
    }
}
#[doc = "Field `CMPSEL` reader - Comparator Load Select"]
pub type CmpselR = crate::FieldReader;
#[doc = "Field `CMPSEL` writer - Comparator Load Select"]
pub type CmpselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IREFTC` reader - Tempco Trim on Comparator Current"]
pub type IreftcR = crate::FieldReader;
#[doc = "Field `IREFTC` writer - Tempco Trim on Comparator Current"]
pub type IreftcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:6 - Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TuningR {
        TuningR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Fine Tuning Value"]
    #[inline(always)]
    pub fn finetuning(&self) -> FinetuningR {
        FinetuningR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - LDO High Power Mode"]
    #[inline(always)]
    pub fn ldohp(&self) -> LdohpR {
        LdohpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Frequency Range"]
    #[inline(always)]
    pub fn freqrange(&self) -> FreqrangeR {
        FreqrangeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Comparator Bias Current"]
    #[inline(always)]
    pub fn cmpbias(&self) -> CmpbiasR {
        CmpbiasR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Locally Divide HFRCO Clock Output"]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Comparator Load Select"]
    #[inline(always)]
    pub fn cmpsel(&self) -> CmpselR {
        CmpselR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31 - Tempco Trim on Comparator Current"]
    #[inline(always)]
    pub fn ireftc(&self) -> IreftcR {
        IreftcR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Tuning Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TuningW<CalSpec> {
        TuningW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Fine Tuning Value"]
    #[inline(always)]
    pub fn finetuning(&mut self) -> FinetuningW<CalSpec> {
        FinetuningW::new(self, 8)
    }
    #[doc = "Bit 15 - LDO High Power Mode"]
    #[inline(always)]
    pub fn ldohp(&mut self) -> LdohpW<CalSpec> {
        LdohpW::new(self, 15)
    }
    #[doc = "Bits 16:20 - Frequency Range"]
    #[inline(always)]
    pub fn freqrange(&mut self) -> FreqrangeW<CalSpec> {
        FreqrangeW::new(self, 16)
    }
    #[doc = "Bits 21:23 - Comparator Bias Current"]
    #[inline(always)]
    pub fn cmpbias(&mut self) -> CmpbiasW<CalSpec> {
        CmpbiasW::new(self, 21)
    }
    #[doc = "Bits 24:25 - Locally Divide HFRCO Clock Output"]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<CalSpec> {
        ClkdivW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Comparator Load Select"]
    #[inline(always)]
    pub fn cmpsel(&mut self) -> CmpselW<CalSpec> {
        CmpselW::new(self, 26)
    }
    #[doc = "Bits 28:31 - Tempco Trim on Comparator Current"]
    #[inline(always)]
    pub fn ireftc(&mut self) -> IreftcW<CalSpec> {
        IreftcW::new(self, 28)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalSpec;
impl crate::RegisterSpec for CalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal::R`](R) reader structure"]
impl crate::Readable for CalSpec {}
#[doc = "`write(|w| ..)` method takes [`cal::W`](W) writer structure"]
impl crate::Writable for CalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAL to value 0xa868_9f7f"]
impl crate::Resettable for CalSpec {
    const RESET_VALUE: u32 = 0xa868_9f7f;
}
