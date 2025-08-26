#[doc = "Register `DMEM0PORTMAPSEL` reader"]
pub type R = crate::R<Dmem0portmapselSpec>;
#[doc = "Register `DMEM0PORTMAPSEL` writer"]
pub type W = crate::W<Dmem0portmapselSpec>;
#[doc = "Field `LDMAPORTSEL` reader - LDMA portmap selection"]
pub type LdmaportselR = crate::BitReader;
#[doc = "Field `LDMAPORTSEL` writer - LDMA portmap selection"]
pub type LdmaportselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRWAESPORTSEL` reader - SRWAES portmap selection"]
pub type SrwaesportselR = crate::BitReader;
#[doc = "Field `SRWAESPORTSEL` writer - SRWAES portmap selection"]
pub type SrwaesportselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBSRWPORTSEL` reader - AHBSRW portmap selection"]
pub type AhbsrwportselR = crate::BitReader;
#[doc = "Field `AHBSRWPORTSEL` writer - AHBSRW portmap selection"]
pub type AhbsrwportselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRWECA0PORTSEL` reader - SRWECA0 portmap selection"]
pub type Srweca0portselR = crate::BitReader;
#[doc = "Field `SRWECA0PORTSEL` writer - SRWECA0 portmap selection"]
pub type Srweca0portselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRWECA1PORTSEL` reader - SRWECA1 portmap selection"]
pub type Srweca1portselR = crate::BitReader;
#[doc = "Field `SRWECA1PORTSEL` writer - SRWECA1 portmap selection"]
pub type Srweca1portselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LDMA portmap selection"]
    #[inline(always)]
    pub fn ldmaportsel(&self) -> LdmaportselR {
        LdmaportselR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRWAES portmap selection"]
    #[inline(always)]
    pub fn srwaesportsel(&self) -> SrwaesportselR {
        SrwaesportselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHBSRW portmap selection"]
    #[inline(always)]
    pub fn ahbsrwportsel(&self) -> AhbsrwportselR {
        AhbsrwportselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRWECA0 portmap selection"]
    #[inline(always)]
    pub fn srweca0portsel(&self) -> Srweca0portselR {
        Srweca0portselR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRWECA1 portmap selection"]
    #[inline(always)]
    pub fn srweca1portsel(&self) -> Srweca1portselR {
        Srweca1portselR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LDMA portmap selection"]
    #[inline(always)]
    pub fn ldmaportsel(&mut self) -> LdmaportselW<Dmem0portmapselSpec> {
        LdmaportselW::new(self, 0)
    }
    #[doc = "Bit 1 - SRWAES portmap selection"]
    #[inline(always)]
    pub fn srwaesportsel(&mut self) -> SrwaesportselW<Dmem0portmapselSpec> {
        SrwaesportselW::new(self, 1)
    }
    #[doc = "Bit 2 - AHBSRW portmap selection"]
    #[inline(always)]
    pub fn ahbsrwportsel(&mut self) -> AhbsrwportselW<Dmem0portmapselSpec> {
        AhbsrwportselW::new(self, 2)
    }
    #[doc = "Bit 3 - SRWECA0 portmap selection"]
    #[inline(always)]
    pub fn srweca0portsel(&mut self) -> Srweca0portselW<Dmem0portmapselSpec> {
        Srweca0portselW::new(self, 3)
    }
    #[doc = "Bit 4 - SRWECA1 portmap selection"]
    #[inline(always)]
    pub fn srweca1portsel(&mut self) -> Srweca1portselW<Dmem0portmapselSpec> {
        Srweca1portselW::new(self, 4)
    }
}
#[doc = "Configure DMEM0 port remap selection.\n\nYou can [`read`](crate::Reg::read) this register and get [`dmem0portmapsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmem0portmapsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dmem0portmapselSpec;
impl crate::RegisterSpec for Dmem0portmapselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmem0portmapsel::R`](R) reader structure"]
impl crate::Readable for Dmem0portmapselSpec {}
#[doc = "`write(|w| ..)` method takes [`dmem0portmapsel::W`](W) writer structure"]
impl crate::Writable for Dmem0portmapselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMEM0PORTMAPSEL to value 0x13"]
impl crate::Resettable for Dmem0portmapselSpec {
    const RESET_VALUE: u32 = 0x13;
}
