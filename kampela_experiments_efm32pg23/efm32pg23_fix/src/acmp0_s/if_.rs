#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `RISE` reader - Rising Edge Triggered Interrupt Flag"]
pub type RiseR = crate::BitReader;
#[doc = "Field `RISE` writer - Rising Edge Triggered Interrupt Flag"]
pub type RiseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FALL` reader - Falling Edge Triggered Interrupt Flag"]
pub type FallR = crate::BitReader;
#[doc = "Field `FALL` writer - Falling Edge Triggered Interrupt Flag"]
pub type FallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMPRDY` reader - ACMP ready Interrupt flag"]
pub type AcmprdyR = crate::BitReader;
#[doc = "Field `ACMPRDY` writer - ACMP ready Interrupt flag"]
pub type AcmprdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUTCONFLICT` reader - Input conflict"]
pub type InputconflictR = crate::BitReader;
#[doc = "Field `INPUTCONFLICT` writer - Input conflict"]
pub type InputconflictW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTALLOCERR` reader - Port allocation error"]
pub type PortallocerrR = crate::BitReader;
#[doc = "Field `PORTALLOCERR` writer - Port allocation error"]
pub type PortallocerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rising Edge Triggered Interrupt Flag"]
    #[inline(always)]
    pub fn rise(&self) -> RiseR {
        RiseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling Edge Triggered Interrupt Flag"]
    #[inline(always)]
    pub fn fall(&self) -> FallR {
        FallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ACMP ready Interrupt flag"]
    #[inline(always)]
    pub fn acmprdy(&self) -> AcmprdyR {
        AcmprdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input conflict"]
    #[inline(always)]
    pub fn inputconflict(&self) -> InputconflictR {
        InputconflictR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port allocation error"]
    #[inline(always)]
    pub fn portallocerr(&self) -> PortallocerrR {
        PortallocerrR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising Edge Triggered Interrupt Flag"]
    #[inline(always)]
    pub fn rise(&mut self) -> RiseW<IfSpec> {
        RiseW::new(self, 0)
    }
    #[doc = "Bit 1 - Falling Edge Triggered Interrupt Flag"]
    #[inline(always)]
    pub fn fall(&mut self) -> FallW<IfSpec> {
        FallW::new(self, 1)
    }
    #[doc = "Bit 2 - ACMP ready Interrupt flag"]
    #[inline(always)]
    pub fn acmprdy(&mut self) -> AcmprdyW<IfSpec> {
        AcmprdyW::new(self, 2)
    }
    #[doc = "Bit 3 - Input conflict"]
    #[inline(always)]
    pub fn inputconflict(&mut self) -> InputconflictW<IfSpec> {
        InputconflictW::new(self, 3)
    }
    #[doc = "Bit 4 - Port allocation error"]
    #[inline(always)]
    pub fn portallocerr(&mut self) -> PortallocerrW<IfSpec> {
        PortallocerrW::new(self, 4)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`if_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSpec;
impl crate::RegisterSpec for IfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_::R`](R) reader structure"]
impl crate::Readable for IfSpec {}
#[doc = "`write(|w| ..)` method takes [`if_::W`](W) writer structure"]
impl crate::Writable for IfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IfSpec {}
