#[doc = "Register `MISCLOCKWORD` reader"]
pub type R = crate::R<MisclockwordSpec>;
#[doc = "Register `MISCLOCKWORD` writer"]
pub type W = crate::W<MisclockwordSpec>;
#[doc = "Field `MELOCKBIT` reader - Mass Erase Lock"]
pub type MelockbitR = crate::BitReader;
#[doc = "Field `MELOCKBIT` writer - Mass Erase Lock"]
pub type MelockbitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDLOCKBIT` reader - User Data Lock"]
pub type UdlockbitR = crate::BitReader;
#[doc = "Field `UDLOCKBIT` writer - User Data Lock"]
pub type UdlockbitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mass Erase Lock"]
    #[inline(always)]
    pub fn melockbit(&self) -> MelockbitR {
        MelockbitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - User Data Lock"]
    #[inline(always)]
    pub fn udlockbit(&self) -> UdlockbitR {
        UdlockbitR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mass Erase Lock"]
    #[inline(always)]
    pub fn melockbit(&mut self) -> MelockbitW<MisclockwordSpec> {
        MelockbitW::new(self, 0)
    }
    #[doc = "Bit 4 - User Data Lock"]
    #[inline(always)]
    pub fn udlockbit(&mut self) -> UdlockbitW<MisclockwordSpec> {
        UdlockbitW::new(self, 4)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`misclockword::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misclockword::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisclockwordSpec;
impl crate::RegisterSpec for MisclockwordSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misclockword::R`](R) reader structure"]
impl crate::Readable for MisclockwordSpec {}
#[doc = "`write(|w| ..)` method takes [`misclockword::W`](W) writer structure"]
impl crate::Writable for MisclockwordSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISCLOCKWORD to value 0x11"]
impl crate::Resettable for MisclockwordSpec {
    const RESET_VALUE: u32 = 0x11;
}
