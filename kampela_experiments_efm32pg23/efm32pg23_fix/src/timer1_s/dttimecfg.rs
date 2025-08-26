#[doc = "Register `DTTIMECFG` reader"]
pub type R = crate::R<DttimecfgSpec>;
#[doc = "Register `DTTIMECFG` writer"]
pub type W = crate::W<DttimecfgSpec>;
#[doc = "Field `DTPRESC` reader - DTI Prescaler Setting"]
pub type DtprescR = crate::FieldReader<u16>;
#[doc = "Field `DTPRESC` writer - DTI Prescaler Setting"]
pub type DtprescW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DTRISET` reader - DTI Rise-time"]
pub type DtrisetR = crate::FieldReader;
#[doc = "Field `DTRISET` writer - DTI Rise-time"]
pub type DtrisetW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DTFALLT` reader - DTI Fall-time"]
pub type DtfalltR = crate::FieldReader;
#[doc = "Field `DTFALLT` writer - DTI Fall-time"]
pub type DtfalltW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:9 - DTI Prescaler Setting"]
    #[inline(always)]
    pub fn dtpresc(&self) -> DtprescR {
        DtprescR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - DTI Rise-time"]
    #[inline(always)]
    pub fn dtriset(&self) -> DtrisetR {
        DtrisetR::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - DTI Fall-time"]
    #[inline(always)]
    pub fn dtfallt(&self) -> DtfalltR {
        DtfalltR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - DTI Prescaler Setting"]
    #[inline(always)]
    pub fn dtpresc(&mut self) -> DtprescW<DttimecfgSpec> {
        DtprescW::new(self, 0)
    }
    #[doc = "Bits 10:15 - DTI Rise-time"]
    #[inline(always)]
    pub fn dtriset(&mut self) -> DtrisetW<DttimecfgSpec> {
        DtrisetW::new(self, 10)
    }
    #[doc = "Bits 16:21 - DTI Fall-time"]
    #[inline(always)]
    pub fn dtfallt(&mut self) -> DtfalltW<DttimecfgSpec> {
        DtfalltW::new(self, 16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dttimecfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dttimecfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DttimecfgSpec;
impl crate::RegisterSpec for DttimecfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dttimecfg::R`](R) reader structure"]
impl crate::Readable for DttimecfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dttimecfg::W`](W) writer structure"]
impl crate::Writable for DttimecfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTTIMECFG to value 0"]
impl crate::Resettable for DttimecfgSpec {}
