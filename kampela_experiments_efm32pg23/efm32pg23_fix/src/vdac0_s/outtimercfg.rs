#[doc = "Register `OUTTIMERCFG` reader"]
pub type R = crate::R<OuttimercfgSpec>;
#[doc = "Register `OUTTIMERCFG` writer"]
pub type W = crate::W<OuttimercfgSpec>;
#[doc = "Field `CH0OUTHOLDTIME` reader - CH0 Output Hold Time"]
pub type Ch0outholdtimeR = crate::FieldReader<u16>;
#[doc = "Field `CH0OUTHOLDTIME` writer - CH0 Output Hold Time"]
pub type Ch0outholdtimeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CH1OUTHOLDTIME` reader - CH1 Output Hold Time"]
pub type Ch1outholdtimeR = crate::FieldReader<u16>;
#[doc = "Field `CH1OUTHOLDTIME` writer - CH1 Output Hold Time"]
pub type Ch1outholdtimeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - CH0 Output Hold Time"]
    #[inline(always)]
    pub fn ch0outholdtime(&self) -> Ch0outholdtimeR {
        Ch0outholdtimeR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 15:24 - CH1 Output Hold Time"]
    #[inline(always)]
    pub fn ch1outholdtime(&self) -> Ch1outholdtimeR {
        Ch1outholdtimeR::new(((self.bits >> 15) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - CH0 Output Hold Time"]
    #[inline(always)]
    pub fn ch0outholdtime(&mut self) -> Ch0outholdtimeW<OuttimercfgSpec> {
        Ch0outholdtimeW::new(self, 0)
    }
    #[doc = "Bits 15:24 - CH1 Output Hold Time"]
    #[inline(always)]
    pub fn ch1outholdtime(&mut self) -> Ch1outholdtimeW<OuttimercfgSpec> {
        Ch1outholdtimeW::new(self, 15)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`outtimercfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`outtimercfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OuttimercfgSpec;
impl crate::RegisterSpec for OuttimercfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`outtimercfg::R`](R) reader structure"]
impl crate::Readable for OuttimercfgSpec {}
#[doc = "`write(|w| ..)` method takes [`outtimercfg::W`](W) writer structure"]
impl crate::Writable for OuttimercfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUTTIMERCFG to value 0"]
impl crate::Resettable for OuttimercfgSpec {}
