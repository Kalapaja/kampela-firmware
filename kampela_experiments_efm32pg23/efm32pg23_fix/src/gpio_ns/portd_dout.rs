#[doc = "Register `PORTD_DOUT` reader"]
pub type R = crate::R<PortdDoutSpec>;
#[doc = "Register `PORTD_DOUT` writer"]
pub type W = crate::W<PortdDoutSpec>;
#[doc = "Field `DOUT` reader - Data output"]
pub type DoutR = crate::FieldReader;
#[doc = "Field `DOUT` writer - Data output"]
pub type DoutW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Data output"]
    #[inline(always)]
    pub fn dout(&self) -> DoutR {
        DoutR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Data output"]
    #[inline(always)]
    pub fn dout(&mut self) -> DoutW<PortdDoutSpec> {
        DoutW::new(self, 0)
    }
}
#[doc = "data out\n\nYou can [`read`](crate::Reg::read) this register and get [`portd_dout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portd_dout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortdDoutSpec;
impl crate::RegisterSpec for PortdDoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`portd_dout::R`](R) reader structure"]
impl crate::Readable for PortdDoutSpec {}
#[doc = "`write(|w| ..)` method takes [`portd_dout::W`](W) writer structure"]
impl crate::Writable for PortdDoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PORTD_DOUT to value 0"]
impl crate::Resettable for PortdDoutSpec {}
