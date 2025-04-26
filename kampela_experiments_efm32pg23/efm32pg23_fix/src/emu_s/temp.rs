#[doc = "Register `TEMP` reader"]
pub type R = crate::R<TempSpec>;
#[doc = "Field `TEMPLSB` reader - Temperature measured decimal part"]
pub type TemplsbR = crate::FieldReader;
#[doc = "Field `TEMP` reader - Temperature measured"]
pub type TempR = crate::FieldReader<u16>;
#[doc = "Field `TEMPAVG` reader - Averaged Temperature"]
pub type TempavgR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - Temperature measured decimal part"]
    #[inline(always)]
    pub fn templsb(&self) -> TemplsbR {
        TemplsbR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:10 - Temperature measured"]
    #[inline(always)]
    pub fn temp(&self) -> TempR {
        TempR::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:26 - Averaged Temperature"]
    #[inline(always)]
    pub fn tempavg(&self) -> TempavgR {
        TempavgR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`temp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TempSpec;
impl crate::RegisterSpec for TempSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`temp::R`](R) reader structure"]
impl crate::Readable for TempSpec {}
#[doc = "`reset()` method sets TEMP to value 0"]
impl crate::Resettable for TempSpec {}
