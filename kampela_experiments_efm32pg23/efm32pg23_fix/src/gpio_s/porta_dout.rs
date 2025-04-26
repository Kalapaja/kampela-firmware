#[doc = "Register `PORTA_DOUT` reader"]
pub type R = crate::R<PortaDoutSpec>;
#[doc = "Register `PORTA_DOUT` writer"]
pub type W = crate::W<PortaDoutSpec>;
#[doc = "Field `DOUT` reader - Data output"]
pub type DoutR = crate::FieldReader<u16>;
#[doc = "Field `DOUT` writer - Data output"]
pub type DoutW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Data output"]
    #[inline(always)]
    pub fn dout(&self) -> DoutR {
        DoutR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Data output"]
    #[inline(always)]
    pub fn dout(&mut self) -> DoutW<PortaDoutSpec> {
        DoutW::new(self, 0)
    }
}
#[doc = "data out\n\nYou can [`read`](crate::Reg::read) this register and get [`porta_dout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porta_dout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PortaDoutSpec;
impl crate::RegisterSpec for PortaDoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`porta_dout::R`](R) reader structure"]
impl crate::Readable for PortaDoutSpec {}
#[doc = "`write(|w| ..)` method takes [`porta_dout::W`](W) writer structure"]
impl crate::Writable for PortaDoutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PORTA_DOUT to value 0"]
impl crate::Resettable for PortaDoutSpec {}
