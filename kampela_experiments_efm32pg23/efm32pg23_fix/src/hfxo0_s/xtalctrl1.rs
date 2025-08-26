#[doc = "Register `XTALCTRL1` reader"]
pub type R = crate::R<Xtalctrl1Spec>;
#[doc = "Register `XTALCTRL1` writer"]
pub type W = crate::W<Xtalctrl1Spec>;
#[doc = "Field `CTUNEXIBUFOUTANA` reader - BUFOUT Tuning Capacitance on XI"]
pub type CtunexibufoutanaR = crate::FieldReader;
#[doc = "Field `CTUNEXIBUFOUTANA` writer - BUFOUT Tuning Capacitance on XI"]
pub type CtunexibufoutanaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - BUFOUT Tuning Capacitance on XI"]
    #[inline(always)]
    pub fn ctunexibufoutana(&self) -> CtunexibufoutanaR {
        CtunexibufoutanaR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - BUFOUT Tuning Capacitance on XI"]
    #[inline(always)]
    pub fn ctunexibufoutana(&mut self) -> CtunexibufoutanaW<Xtalctrl1Spec> {
        CtunexibufoutanaW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`xtalctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xtalctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xtalctrl1Spec;
impl crate::RegisterSpec for Xtalctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xtalctrl1::R`](R) reader structure"]
impl crate::Readable for Xtalctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`xtalctrl1::W`](W) writer structure"]
impl crate::Writable for Xtalctrl1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XTALCTRL1 to value 0x3c"]
impl crate::Resettable for Xtalctrl1Spec {
    const RESET_VALUE: u32 = 0x3c;
}
