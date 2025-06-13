#[doc = "Register `OVSCTRL` reader"]
pub type R = crate::R<OvsctrlSpec>;
#[doc = "Register `OVSCTRL` writer"]
pub type W = crate::W<OvsctrlSpec>;
#[doc = "Field `FILTLEN` reader - Configure Filter Length for Inputs S0IN"]
pub type FiltlenR = crate::FieldReader;
#[doc = "Field `FILTLEN` writer - Configure Filter Length for Inputs S0IN"]
pub type FiltlenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FLUTTERRM` reader - Flutter Remove"]
pub type FlutterrmR = crate::BitReader;
#[doc = "Field `FLUTTERRM` writer - Flutter Remove"]
pub type FlutterrmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Configure Filter Length for Inputs S0IN"]
    #[inline(always)]
    pub fn filtlen(&self) -> FiltlenR {
        FiltlenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - Flutter Remove"]
    #[inline(always)]
    pub fn flutterrm(&self) -> FlutterrmR {
        FlutterrmR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configure Filter Length for Inputs S0IN"]
    #[inline(always)]
    pub fn filtlen(&mut self) -> FiltlenW<OvsctrlSpec> {
        FiltlenW::new(self, 0)
    }
    #[doc = "Bit 12 - Flutter Remove"]
    #[inline(always)]
    pub fn flutterrm(&mut self) -> FlutterrmW<OvsctrlSpec> {
        FlutterrmW::new(self, 12)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ovsctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ovsctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OvsctrlSpec;
impl crate::RegisterSpec for OvsctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ovsctrl::R`](R) reader structure"]
impl crate::Readable for OvsctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ovsctrl::W`](W) writer structure"]
impl crate::Writable for OvsctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OVSCTRL to value 0"]
impl crate::Resettable for OvsctrlSpec {}
