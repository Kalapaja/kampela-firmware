#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `PPUPRIV` reader - PPU Privilege Interrupt Enable"]
pub type PpuprivR = crate::BitReader;
#[doc = "Field `PPUPRIV` writer - PPU Privilege Interrupt Enable"]
pub type PpuprivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPUINST` reader - PPU Instruction Interrupt Enable"]
pub type PpuinstR = crate::BitReader;
#[doc = "Field `PPUINST` writer - PPU Instruction Interrupt Enable"]
pub type PpuinstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPUSEC` reader - PPU Security Interrupt Enable"]
pub type PpusecR = crate::BitReader;
#[doc = "Field `PPUSEC` writer - PPU Security Interrupt Enable"]
pub type PpusecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMPUSEC` reader - BMPU Security Interrupt Enable"]
pub type BmpusecR = crate::BitReader;
#[doc = "Field `BMPUSEC` writer - BMPU Security Interrupt Enable"]
pub type BmpusecW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PPU Privilege Interrupt Enable"]
    #[inline(always)]
    pub fn ppupriv(&self) -> PpuprivR {
        PpuprivR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PPU Instruction Interrupt Enable"]
    #[inline(always)]
    pub fn ppuinst(&self) -> PpuinstR {
        PpuinstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - PPU Security Interrupt Enable"]
    #[inline(always)]
    pub fn ppusec(&self) -> PpusecR {
        PpusecR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BMPU Security Interrupt Enable"]
    #[inline(always)]
    pub fn bmpusec(&self) -> BmpusecR {
        BmpusecR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PPU Privilege Interrupt Enable"]
    #[inline(always)]
    pub fn ppupriv(&mut self) -> PpuprivW<IenSpec> {
        PpuprivW::new(self, 0)
    }
    #[doc = "Bit 2 - PPU Instruction Interrupt Enable"]
    #[inline(always)]
    pub fn ppuinst(&mut self) -> PpuinstW<IenSpec> {
        PpuinstW::new(self, 2)
    }
    #[doc = "Bit 16 - PPU Security Interrupt Enable"]
    #[inline(always)]
    pub fn ppusec(&mut self) -> PpusecW<IenSpec> {
        PpusecW::new(self, 16)
    }
    #[doc = "Bit 17 - BMPU Security Interrupt Enable"]
    #[inline(always)]
    pub fn bmpusec(&mut self) -> BmpusecW<IenSpec> {
        BmpusecW::new(self, 17)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
