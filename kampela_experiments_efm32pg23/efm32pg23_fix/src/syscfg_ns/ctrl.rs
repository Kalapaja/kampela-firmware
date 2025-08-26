#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ADDRFAULTEN` reader - Invalid Address Bus Fault Response Enabl"]
pub type AddrfaultenR = crate::BitReader;
#[doc = "Field `ADDRFAULTEN` writer - Invalid Address Bus Fault Response Enabl"]
pub type AddrfaultenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKDISFAULTEN` reader - Disabled Clkbus Bus Fault Enable"]
pub type ClkdisfaultenR = crate::BitReader;
#[doc = "Field `CLKDISFAULTEN` writer - Disabled Clkbus Bus Fault Enable"]
pub type ClkdisfaultenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMECCERRFAULTEN` reader - Two bit ECC error bus fault response ena"]
pub type RameccerrfaultenR = crate::BitReader;
#[doc = "Field `RAMECCERRFAULTEN` writer - Two bit ECC error bus fault response ena"]
pub type RameccerrfaultenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Invalid Address Bus Fault Response Enabl"]
    #[inline(always)]
    pub fn addrfaulten(&self) -> AddrfaultenR {
        AddrfaultenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disabled Clkbus Bus Fault Enable"]
    #[inline(always)]
    pub fn clkdisfaulten(&self) -> ClkdisfaultenR {
        ClkdisfaultenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - Two bit ECC error bus fault response ena"]
    #[inline(always)]
    pub fn rameccerrfaulten(&self) -> RameccerrfaultenR {
        RameccerrfaultenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invalid Address Bus Fault Response Enabl"]
    #[inline(always)]
    pub fn addrfaulten(&mut self) -> AddrfaultenW<CtrlSpec> {
        AddrfaultenW::new(self, 0)
    }
    #[doc = "Bit 1 - Disabled Clkbus Bus Fault Enable"]
    #[inline(always)]
    pub fn clkdisfaulten(&mut self) -> ClkdisfaultenW<CtrlSpec> {
        ClkdisfaultenW::new(self, 1)
    }
    #[doc = "Bit 5 - Two bit ECC error bus fault response ena"]
    #[inline(always)]
    pub fn rameccerrfaulten(&mut self) -> RameccerrfaultenW<CtrlSpec> {
        RameccerrfaultenW::new(self, 5)
    }
}
#[doc = "Configure to provide general RAM configuration.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x23"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x23;
}
