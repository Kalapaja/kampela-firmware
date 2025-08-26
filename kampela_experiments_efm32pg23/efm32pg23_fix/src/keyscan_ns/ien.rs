#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `NOKEY` reader - No Key was pressed"]
pub type NokeyR = crate::BitReader;
#[doc = "Field `NOKEY` writer - No Key was pressed"]
pub type NokeyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEY` reader - A Key was pressed"]
pub type KeyR = crate::BitReader;
#[doc = "Field `KEY` writer - A Key was pressed"]
pub type KeyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANNED` reader - Completed Scanning"]
pub type ScannedR = crate::BitReader;
#[doc = "Field `SCANNED` writer - Completed Scanning"]
pub type ScannedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WAKEUP` reader - Wake up"]
pub type WakeupR = crate::BitReader;
#[doc = "Field `WAKEUP` writer - Wake up"]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - No Key was pressed"]
    #[inline(always)]
    pub fn nokey(&self) -> NokeyR {
        NokeyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A Key was pressed"]
    #[inline(always)]
    pub fn key(&self) -> KeyR {
        KeyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Completed Scanning"]
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
    #[doc = "Bit 0 - No Key was pressed"]
    #[inline(always)]
    pub fn nokey(&mut self) -> NokeyW<IenSpec> {
        NokeyW::new(self, 0)
    }
    #[doc = "Bit 1 - A Key was pressed"]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<IenSpec> {
        KeyW::new(self, 1)
    }
    #[doc = "Bit 2 - Completed Scanning"]
    #[inline(always)]
    pub fn scanned(&mut self) -> ScannedW<IenSpec> {
        ScannedW::new(self, 2)
    }
    #[doc = "Bit 3 - Wake up"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WakeupW<IenSpec> {
        WakeupW::new(self, 3)
    }
}
#[doc = "Interrupt Enables\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
