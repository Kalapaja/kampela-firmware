#[doc = "Register `PORTC_DOUT` reader"]
pub type R = crate::R<PortcDoutSpec>;
#[doc = "Register `PORTC_DOUT` writer"]
pub type W = crate::W<PortcDoutSpec>;
#[doc = "Field `DOUT` reader - Data output"]
pub type DoutR = crate::FieldReader<u16>;
#[doc = "Field `DOUT` writer - Data output"]
pub type DoutW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Data output"]
    #[inline(always)]
    pub fn dout(&self) -> DoutR {
        DoutR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Data output"]
    #[inline(always)]
    pub fn dout(&mut self) -> DoutW<PortcDoutSpec> {
        DoutW::new(self, 0)
    }
}
#[doc = "data out\n\nYou can [`read`](crate::Reg::read) this register and get [`portc_dout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`portc_dout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortcDoutSpec;
impl crate::RegisterSpec for PortcDoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`portc_dout::R`](R) reader structure"]
impl crate::Readable for PortcDoutSpec {}
#[doc = "`write(|w| ..)` method takes [`portc_dout::W`](W) writer structure"]
impl crate::Writable for PortcDoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PORTC_DOUT to value 0"]
impl crate::Resettable for PortcDoutSpec {}
