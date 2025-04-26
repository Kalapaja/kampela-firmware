#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `ERASE` reader - Host Erase Done Interrupt Read Flag"]
pub type EraseR = crate::BitReader;
#[doc = "Field `ERASE` writer - Host Erase Done Interrupt Read Flag"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` reader - Host Write Done Interrupt Read Flag"]
pub type WriteR = crate::BitReader;
#[doc = "Field `WRITE` writer - Host Write Done Interrupt Read Flag"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDATAOV` reader - Host write buffer overflow"]
pub type WdataovR = crate::BitReader;
#[doc = "Field `WDATAOV` writer - Host write buffer overflow"]
pub type WdataovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUPF` reader - Flash Power Up Sequence Complete Flag"]
pub type PwrupfR = crate::BitReader;
#[doc = "Field `PWRUPF` writer - Flash Power Up Sequence Complete Flag"]
pub type PwrupfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWROFF` reader - Flash Power Off Sequence Complete Flag"]
pub type PwroffR = crate::BitReader;
#[doc = "Field `PWROFF` writer - Flash Power Off Sequence Complete Flag"]
pub type PwroffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Host Erase Done Interrupt Read Flag"]
    #[inline(always)]
    pub fn erase(&self) -> EraseR {
        EraseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host Write Done Interrupt Read Flag"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host write buffer overflow"]
    #[inline(always)]
    pub fn wdataov(&self) -> WdataovR {
        WdataovR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash Power Up Sequence Complete Flag"]
    #[inline(always)]
    pub fn pwrupf(&self) -> PwrupfR {
        PwrupfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flash Power Off Sequence Complete Flag"]
    #[inline(always)]
    pub fn pwroff(&self) -> PwroffR {
        PwroffR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Host Erase Done Interrupt Read Flag"]
    #[inline(always)]
    pub fn erase(&mut self) -> EraseW<IfSpec> {
        EraseW::new(self, 0)
    }
    #[doc = "Bit 1 - Host Write Done Interrupt Read Flag"]
    #[inline(always)]
    pub fn write(&mut self) -> WriteW<IfSpec> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 2 - Host write buffer overflow"]
    #[inline(always)]
    pub fn wdataov(&mut self) -> WdataovW<IfSpec> {
        WdataovW::new(self, 2)
    }
    #[doc = "Bit 8 - Flash Power Up Sequence Complete Flag"]
    #[inline(always)]
    pub fn pwrupf(&mut self) -> PwrupfW<IfSpec> {
        PwrupfW::new(self, 8)
    }
    #[doc = "Bit 9 - Flash Power Off Sequence Complete Flag"]
    #[inline(always)]
    pub fn pwroff(&mut self) -> PwroffW<IfSpec> {
        PwroffW::new(self, 9)
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
