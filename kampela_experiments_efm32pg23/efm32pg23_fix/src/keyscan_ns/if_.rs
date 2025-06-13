#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `NOKEY` reader - No key was pressed"]
pub type NokeyR = crate::BitReader;
#[doc = "Field `NOKEY` writer - No key was pressed"]
pub type NokeyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` reader - A key was pressed"]
pub type KeyR = crate::BitReader;
#[doc = "Field `KEY` writer - A key was pressed"]
pub type KeyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANNED` reader - Completed scan"]
pub type ScannedR = crate::BitReader;
#[doc = "Field `SCANNED` writer - Completed scan"]
pub type ScannedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` reader - Wake up"]
pub type WakeupR = crate::BitReader;
#[doc = "Field `WAKEUP` writer - Wake up"]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No key was pressed"]
    #[inline(always)]
    pub fn nokey(&self) -> NokeyR {
        NokeyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A key was pressed"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Completed scan"]
    #[inline(always)]
    pub fn scanned(&self) -> ScannedR {
        ScannedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake up"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - No key was pressed"]
    #[inline(always)]
    pub fn nokey(&mut self) -> NokeyW<IfSpec> {
        NokeyW::new(self, 0)
    }
    #[doc = "Bit 1 - A key was pressed"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<IfSpec> {
        KeyW::new(self, 1)
    }
    #[doc = "Bit 2 - Completed scan"]
    #[inline(always)]
    pub fn scanned(&mut self) -> ScannedW<IfSpec> {
        ScannedW::new(self, 2)
    }
    #[doc = "Bit 3 - Wake up"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WakeupW<IfSpec> {
        WakeupW::new(self, 3)
    }
}
#[doc = "Interrupt Flags\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
