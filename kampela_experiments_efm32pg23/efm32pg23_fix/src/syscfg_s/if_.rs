#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `SW0` reader - Software Interrupt Flag"]
pub type Sw0R = crate::BitReader;
#[doc = "Field `SW0` writer - Software Interrupt Flag"]
pub type Sw0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW1` reader - Software Interrupt Flag"]
pub type Sw1R = crate::BitReader;
#[doc = "Field `SW1` writer - Software Interrupt Flag"]
pub type Sw1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW2` reader - Software Interrupt Flag"]
pub type Sw2R = crate::BitReader;
#[doc = "Field `SW2` writer - Software Interrupt Flag"]
pub type Sw2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW3` reader - Software Interrupt Flag"]
pub type Sw3R = crate::BitReader;
#[doc = "Field `SW3` writer - Software Interrupt Flag"]
pub type Sw3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIOC` reader - FPU Invalid Operation interrupt flag"]
pub type FpiocR = crate::BitReader;
#[doc = "Field `FPIOC` writer - FPU Invalid Operation interrupt flag"]
pub type FpiocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPDZC` reader - FPU Divide by zero interrupt flag"]
pub type FpdzcR = crate::BitReader;
#[doc = "Field `FPDZC` writer - FPU Divide by zero interrupt flag"]
pub type FpdzcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPUFC` reader - FPU Underflow interrupt flag"]
pub type FpufcR = crate::BitReader;
#[doc = "Field `FPUFC` writer - FPU Underflow interrupt flag"]
pub type FpufcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPOFC` reader - FPU Overflow interrupt flag"]
pub type FpofcR = crate::BitReader;
#[doc = "Field `FPOFC` writer - FPU Overflow interrupt flag"]
pub type FpofcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIDC` reader - FPU Input denormal interrupt flag"]
pub type FpidcR = crate::BitReader;
#[doc = "Field `FPIDC` writer - FPU Input denormal interrupt flag"]
pub type FpidcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIXC` reader - FPU Inexact interrupt flag"]
pub type FpixcR = crate::BitReader;
#[doc = "Field `FPIXC` writer - FPU Inexact interrupt flag"]
pub type FpixcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQRAMERR1B` reader - SEQRAM Error 1-bit Interrupt Flag"]
pub type Seqramerr1bR = crate::BitReader;
#[doc = "Field `SEQRAMERR1B` writer - SEQRAM Error 1-bit Interrupt Flag"]
pub type Seqramerr1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQRAMERR2B` reader - SEQRAM Error 2-bit Interrupt Flag"]
pub type Seqramerr2bR = crate::BitReader;
#[doc = "Field `SEQRAMERR2B` writer - SEQRAM Error 2-bit Interrupt Flag"]
pub type Seqramerr2bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRCRAMERR1B` reader - FRCRAM Error 1-bit Interrupt Flag"]
pub type Frcramerr1bR = crate::BitReader;
#[doc = "Field `FRCRAMERR1B` writer - FRCRAM Error 1-bit Interrupt Flag"]
pub type Frcramerr1bW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRCRAMERR2B` reader - FRCRAM Error 2-bit Interrupt Flag"]
pub type Frcramerr2bR = crate::BitReader;
#[doc = "Field `FRCRAMERR2B` writer - FRCRAM Error 2-bit Interrupt Flag"]
pub type Frcramerr2bW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Interrupt Flag"]
    #[inline(always)]
    pub fn sw0(&self) -> Sw0R {
        Sw0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Interrupt Flag"]
    #[inline(always)]
    pub fn sw1(&self) -> Sw1R {
        Sw1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software Interrupt Flag"]
    #[inline(always)]
    pub fn sw2(&self) -> Sw2R {
        Sw2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Interrupt Flag"]
    #[inline(always)]
    pub fn sw3(&self) -> Sw3R {
        Sw3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - FPU Invalid Operation interrupt flag"]
    #[inline(always)]
    pub fn fpioc(&self) -> FpiocR {
        FpiocR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FPU Divide by zero interrupt flag"]
    #[inline(always)]
    pub fn fpdzc(&self) -> FpdzcR {
        FpdzcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FPU Underflow interrupt flag"]
    #[inline(always)]
    pub fn fpufc(&self) -> FpufcR {
        FpufcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FPU Overflow interrupt flag"]
    #[inline(always)]
    pub fn fpofc(&self) -> FpofcR {
        FpofcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FPU Input denormal interrupt flag"]
    #[inline(always)]
    pub fn fpidc(&self) -> FpidcR {
        FpidcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FPU Inexact interrupt flag"]
    #[inline(always)]
    pub fn fpixc(&self) -> FpixcR {
        FpixcR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 24 - SEQRAM Error 1-bit Interrupt Flag"]
    #[inline(always)]
    pub fn seqramerr1b(&self) -> Seqramerr1bR {
        Seqramerr1bR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SEQRAM Error 2-bit Interrupt Flag"]
    #[inline(always)]
    pub fn seqramerr2b(&self) -> Seqramerr2bR {
        Seqramerr2bR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - FRCRAM Error 1-bit Interrupt Flag"]
    #[inline(always)]
    pub fn frcramerr1b(&self) -> Frcramerr1bR {
        Frcramerr1bR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - FRCRAM Error 2-bit Interrupt Flag"]
    #[inline(always)]
    pub fn frcramerr2b(&self) -> Frcramerr2bR {
        Frcramerr2bR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Interrupt Flag"]
    #[inline(always)]
    pub fn sw0(&mut self) -> Sw0W<IfSpec> {
        Sw0W::new(self, 0)
    }
    #[doc = "Bit 1 - Software Interrupt Flag"]
    #[inline(always)]
    pub fn sw1(&mut self) -> Sw1W<IfSpec> {
        Sw1W::new(self, 1)
    }
    #[doc = "Bit 2 - Software Interrupt Flag"]
    #[inline(always)]
    pub fn sw2(&mut self) -> Sw2W<IfSpec> {
        Sw2W::new(self, 2)
    }
    #[doc = "Bit 3 - Software Interrupt Flag"]
    #[inline(always)]
    pub fn sw3(&mut self) -> Sw3W<IfSpec> {
        Sw3W::new(self, 3)
    }
    #[doc = "Bit 8 - FPU Invalid Operation interrupt flag"]
    #[inline(always)]
    pub fn fpioc(&mut self) -> FpiocW<IfSpec> {
        FpiocW::new(self, 8)
    }
    #[doc = "Bit 9 - FPU Divide by zero interrupt flag"]
    #[inline(always)]
    pub fn fpdzc(&mut self) -> FpdzcW<IfSpec> {
        FpdzcW::new(self, 9)
    }
    #[doc = "Bit 10 - FPU Underflow interrupt flag"]
    #[inline(always)]
    pub fn fpufc(&mut self) -> FpufcW<IfSpec> {
        FpufcW::new(self, 10)
    }
    #[doc = "Bit 11 - FPU Overflow interrupt flag"]
    #[inline(always)]
    pub fn fpofc(&mut self) -> FpofcW<IfSpec> {
        FpofcW::new(self, 11)
    }
    #[doc = "Bit 12 - FPU Input denormal interrupt flag"]
    #[inline(always)]
    pub fn fpidc(&mut self) -> FpidcW<IfSpec> {
        FpidcW::new(self, 12)
    }
    #[doc = "Bit 13 - FPU Inexact interrupt flag"]
    #[inline(always)]
    pub fn fpixc(&mut self) -> FpixcW<IfSpec> {
        FpixcW::new(self, 13)
    }
    #[doc = "Bit 24 - SEQRAM Error 1-bit Interrupt Flag"]
    #[inline(always)]
    pub fn seqramerr1b(&mut self) -> Seqramerr1bW<IfSpec> {
        Seqramerr1bW::new(self, 24)
    }
    #[doc = "Bit 25 - SEQRAM Error 2-bit Interrupt Flag"]
    #[inline(always)]
    pub fn seqramerr2b(&mut self) -> Seqramerr2bW<IfSpec> {
        Seqramerr2bW::new(self, 25)
    }
    #[doc = "Bit 28 - FRCRAM Error 1-bit Interrupt Flag"]
    #[inline(always)]
    pub fn frcramerr1b(&mut self) -> Frcramerr1bW<IfSpec> {
        Frcramerr1bW::new(self, 28)
    }
    #[doc = "Bit 29 - FRCRAM Error 2-bit Interrupt Flag"]
    #[inline(always)]
    pub fn frcramerr2b(&mut self) -> Frcramerr2bW<IfSpec> {
        Frcramerr2bW::new(self, 29)
    }
}
#[doc = "Read to get system status.\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`write(|w| ..)` method takes [`if_::W`](W) writer structure"]
impl crate::Writable for IfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
