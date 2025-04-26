#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `PPUPRIV` reader - PPU Privilege Interrupt Flag"]
pub type PpuprivR = crate::BitReader;
#[doc = "Field `PPUPRIV` writer - PPU Privilege Interrupt Flag"]
pub type PpuprivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPUINST` reader - PPU Instruction Interrupt Flag"]
pub type PpuinstR = crate::BitReader;
#[doc = "Field `PPUINST` writer - PPU Instruction Interrupt Flag"]
pub type PpuinstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPUSEC` reader - PPU Security Interrupt Flag"]
pub type PpusecR = crate::BitReader;
#[doc = "Field `PPUSEC` writer - PPU Security Interrupt Flag"]
pub type PpusecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMPUSEC` reader - BMPU Security Interrupt Flag"]
pub type BmpusecR = crate::BitReader;
#[doc = "Field `BMPUSEC` writer - BMPU Security Interrupt Flag"]
pub type BmpusecW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PPU Privilege Interrupt Flag"]
    #[inline(always)]
    pub fn ppupriv(&self) -> PpuprivR {
        PpuprivR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PPU Instruction Interrupt Flag"]
    #[inline(always)]
    pub fn ppuinst(&self) -> PpuinstR {
        PpuinstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - PPU Security Interrupt Flag"]
    #[inline(always)]
    pub fn ppusec(&self) -> PpusecR {
        PpusecR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BMPU Security Interrupt Flag"]
    #[inline(always)]
    pub fn bmpusec(&self) -> BmpusecR {
        BmpusecR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PPU Privilege Interrupt Flag"]
    #[inline(always)]
    pub fn ppupriv(&mut self) -> PpuprivW<IfSpec> {
        PpuprivW::new(self, 0)
    }
    #[doc = "Bit 2 - PPU Instruction Interrupt Flag"]
    #[inline(always)]
    pub fn ppuinst(&mut self) -> PpuinstW<IfSpec> {
        PpuinstW::new(self, 2)
    }
    #[doc = "Bit 16 - PPU Security Interrupt Flag"]
    #[inline(always)]
    pub fn ppusec(&mut self) -> PpusecW<IfSpec> {
        PpusecW::new(self, 16)
    }
    #[doc = "Bit 17 - BMPU Security Interrupt Flag"]
    #[inline(always)]
    pub fn bmpusec(&mut self) -> BmpusecW<IfSpec> {
        BmpusecW::new(self, 17)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
