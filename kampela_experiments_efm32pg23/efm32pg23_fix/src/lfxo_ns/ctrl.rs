#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `FORCEEN` reader - LFXO Force Enable"]
pub type ForceenR = crate::BitReader;
#[doc = "Field `FORCEEN` writer - LFXO Force Enable"]
pub type ForceenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISONDEMAND` reader - LFXO Disable On-demand requests"]
pub type DisondemandR = crate::BitReader;
#[doc = "Field `DISONDEMAND` writer - LFXO Disable On-demand requests"]
pub type DisondemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAILDETEN` reader - LFXO Failure Detection Enable"]
pub type FaildetenR = crate::BitReader;
#[doc = "Field `FAILDETEN` writer - LFXO Failure Detection Enable"]
pub type FaildetenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAILDETEM4WUEN` reader - LFXO Failure Detection EM4WU Enable"]
pub type Faildetem4wuenR = crate::BitReader;
#[doc = "Field `FAILDETEM4WUEN` writer - LFXO Failure Detection EM4WU Enable"]
pub type Faildetem4wuenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LFXO Force Enable"]
    #[inline(always)]
    pub fn forceen(&self) -> ForceenR {
        ForceenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LFXO Disable On-demand requests"]
    #[inline(always)]
    pub fn disondemand(&self) -> DisondemandR {
        DisondemandR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - LFXO Failure Detection Enable"]
    #[inline(always)]
    pub fn faildeten(&self) -> FaildetenR {
        FaildetenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LFXO Failure Detection EM4WU Enable"]
    #[inline(always)]
    pub fn faildetem4wuen(&self) -> Faildetem4wuenR {
        Faildetem4wuenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LFXO Force Enable"]
    #[inline(always)]
    pub fn forceen(&mut self) -> ForceenW<CtrlSpec> {
        ForceenW::new(self, 0)
    }
    #[doc = "Bit 1 - LFXO Disable On-demand requests"]
    #[inline(always)]
    pub fn disondemand(&mut self) -> DisondemandW<CtrlSpec> {
        DisondemandW::new(self, 1)
    }
    #[doc = "Bit 4 - LFXO Failure Detection Enable"]
    #[inline(always)]
    pub fn faildeten(&mut self) -> FaildetenW<CtrlSpec> {
        FaildetenW::new(self, 4)
    }
    #[doc = "Bit 5 - LFXO Failure Detection EM4WU Enable"]
    #[inline(always)]
    pub fn faildetem4wuen(&mut self) -> Faildetem4wuenW<CtrlSpec> {
        Faildetem4wuenW::new(self, 5)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x02"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x02;
}
