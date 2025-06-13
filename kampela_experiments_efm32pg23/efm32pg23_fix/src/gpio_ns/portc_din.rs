#[doc = "Register `PORTC_DIN` reader"]
pub type R = crate::R<PortcDinSpec>;
#[doc = "Field `DIN` reader - Data input"]
pub type DinR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Data input"]
    #[inline(always)]
    pub fn din(&self) -> DinR {
        DinR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "data in\n\nYou can [`read`](crate::Reg::read) this register and get [`portc_din::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortcDinSpec;
impl crate::RegisterSpec for PortcDinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`portc_din::R`](R) reader structure"]
impl crate::Readable for PortcDinSpec {}
#[doc = "`reset()` method sets PORTC_DIN to value 0"]
impl crate::Resettable for PortcDinSpec {}
