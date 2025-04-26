#[doc = "Register `PORTA_DIN` reader"]
pub type R = crate::R<PortaDinSpec>;
#[doc = "Field `DIN` reader - Data input"]
pub type DinR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:10 - Data input"]
    #[inline(always)]
    pub fn din(&self) -> DinR {
        DinR::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "data in\n\nYou can [`read`](crate::Reg::read) this register and get [`porta_din::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortaDinSpec;
impl crate::RegisterSpec for PortaDinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`porta_din::R`](R) reader structure"]
impl crate::Readable for PortaDinSpec {}
#[doc = "`reset()` method sets PORTA_DIN to value 0"]
impl crate::Resettable for PortaDinSpec {}
