#[doc = "Register `PORTD_DIN` reader"]
pub type R = crate::R<PortdDinSpec>;
#[doc = "Field `DIN` reader - Data input"]
pub type DinR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Data input"]
    #[inline(always)]
    pub fn din(&self) -> DinR {
        DinR::new((self.bits & 0x3f) as u8)
    }
}
#[doc = "data in\n\nYou can [`read`](crate::Reg::read) this register and get [`portd_din::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortdDinSpec;
impl crate::RegisterSpec for PortdDinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`portd_din::R`](R) reader structure"]
impl crate::Readable for PortdDinSpec {}
#[doc = "`reset()` method sets PORTD_DIN to value 0"]
impl crate::Resettable for PortdDinSpec {}
