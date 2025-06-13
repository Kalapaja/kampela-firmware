#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `ERASE` reader - Erase Done Interrupt enable"]
pub type EraseR = crate::BitReader;
#[doc = "Field `ERASE` writer - Erase Done Interrupt enable"]
pub type EraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE` reader - Write Done Interrupt enable"]
pub type WriteR = crate::BitReader;
#[doc = "Field `WRITE` writer - Write Done Interrupt enable"]
pub type WriteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDATAOV` reader - write data buffer overflow irq enable"]
pub type WdataovR = crate::BitReader;
#[doc = "Field `WDATAOV` writer - write data buffer overflow irq enable"]
pub type WdataovW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRUPF` reader - Flash Power Up Seq done irq enable"]
pub type PwrupfR = crate::BitReader;
#[doc = "Field `PWRUPF` writer - Flash Power Up Seq done irq enable"]
pub type PwrupfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWROFF` reader - Flash Power Off Seq done irq enable"]
pub type PwroffR = crate::BitReader;
#[doc = "Field `PWROFF` writer - Flash Power Off Seq done irq enable"]
pub type PwroffW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Erase Done Interrupt enable"]
    #[inline(always)]
    pub fn erase(&self) -> EraseR {
        EraseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Done Interrupt enable"]
    #[inline(always)]
    pub fn write(&self) -> WriteR {
        WriteR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - write data buffer overflow irq enable"]
    #[inline(always)]
    pub fn wdataov(&self) -> WdataovR {
        WdataovR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash Power Up Seq done irq enable"]
    #[inline(always)]
    pub fn pwrupf(&self) -> PwrupfR {
        PwrupfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flash Power Off Seq done irq enable"]
    #[inline(always)]
    pub fn pwroff(&self) -> PwroffR {
        PwroffR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Erase Done Interrupt enable"]
    #[inline(always)]
    pub fn erase(&mut self) -> EraseW<IenSpec> {
        EraseW::new(self, 0)
    }
    #[doc = "Bit 1 - Write Done Interrupt enable"]
    #[inline(always)]
    pub fn write(&mut self) -> WriteW<IenSpec> {
        WriteW::new(self, 1)
    }
    #[doc = "Bit 2 - write data buffer overflow irq enable"]
    #[inline(always)]
    pub fn wdataov(&mut self) -> WdataovW<IenSpec> {
        WdataovW::new(self, 2)
    }
    #[doc = "Bit 8 - Flash Power Up Seq done irq enable"]
    #[inline(always)]
    pub fn pwrupf(&mut self) -> PwrupfW<IenSpec> {
        PwrupfW::new(self, 8)
    }
    #[doc = "Bit 9 - Flash Power Off Seq done irq enable"]
    #[inline(always)]
    pub fn pwroff(&mut self) -> PwroffW<IenSpec> {
        PwroffW::new(self, 9)
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
