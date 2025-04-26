#[doc = "Register `DTCTRL` reader"]
pub type R = crate::R<DtctrlSpec>;
#[doc = "Register `DTCTRL` writer"]
pub type W = crate::W<DtctrlSpec>;
#[doc = "Field `DTCINV` reader - DTI Complementary Output Invert."]
pub type DtcinvR = crate::BitReader;
#[doc = "Field `DTCINV` writer - DTI Complementary Output Invert."]
pub type DtcinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIPOL` reader - DTI Inactive Polarity"]
pub type DtipolR = crate::BitReader;
#[doc = "Field `DTIPOL` writer - DTI Inactive Polarity"]
pub type DtipolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DTI Complementary Output Invert."]
    #[inline(always)]
    pub fn dtcinv(&self) -> DtcinvR {
        DtcinvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTI Inactive Polarity"]
    #[inline(always)]
    pub fn dtipol(&self) -> DtipolR {
        DtipolR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTI Complementary Output Invert."]
    #[inline(always)]
    pub fn dtcinv(&mut self) -> DtcinvW<DtctrlSpec> {
        DtcinvW::new(self, 0)
    }
    #[doc = "Bit 1 - DTI Inactive Polarity"]
    #[inline(always)]
    pub fn dtipol(&mut self) -> DtipolW<DtctrlSpec> {
        DtipolW::new(self, 1)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dtctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtctrlSpec;
impl crate::RegisterSpec for DtctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtctrl::R`](R) reader structure"]
impl crate::Readable for DtctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dtctrl::W`](W) writer structure"]
impl crate::Writable for DtctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTCTRL to value 0"]
impl crate::Resettable for DtctrlSpec {}
