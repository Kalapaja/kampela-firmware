#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `RISE` reader - Rising edge interrupt enable"]
pub type RiseR = crate::BitReader;
#[doc = "Field `RISE` writer - Rising edge interrupt enable"]
pub type RiseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FALL` reader - Falling edge interrupt enable"]
pub type FallR = crate::BitReader;
#[doc = "Field `FALL` writer - Falling edge interrupt enable"]
pub type FallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMPRDY` reader - ACMP ready interrupt enable"]
pub type AcmprdyR = crate::BitReader;
#[doc = "Field `ACMPRDY` writer - ACMP ready interrupt enable"]
pub type AcmprdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INPUTCONFLICT` reader - Input conflict interrupt enable"]
pub type InputconflictR = crate::BitReader;
#[doc = "Field `INPUTCONFLICT` writer - Input conflict interrupt enable"]
pub type InputconflictW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTALLOCERR` reader - Port allocation error interrupt enable"]
pub type PortallocerrR = crate::BitReader;
#[doc = "Field `PORTALLOCERR` writer - Port allocation error interrupt enable"]
pub type PortallocerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rising edge interrupt enable"]
    #[inline(always)]
    pub fn rise(&self) -> RiseR {
        RiseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling edge interrupt enable"]
    #[inline(always)]
    pub fn fall(&self) -> FallR {
        FallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ACMP ready interrupt enable"]
    #[inline(always)]
    pub fn acmprdy(&self) -> AcmprdyR {
        AcmprdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Input conflict interrupt enable"]
    #[inline(always)]
    pub fn inputconflict(&self) -> InputconflictR {
        InputconflictR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port allocation error interrupt enable"]
    #[inline(always)]
    pub fn portallocerr(&self) -> PortallocerrR {
        PortallocerrR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising edge interrupt enable"]
    #[inline(always)]
    pub fn rise(&mut self) -> RiseW<IenSpec> {
        RiseW::new(self, 0)
    }
    #[doc = "Bit 1 - Falling edge interrupt enable"]
    #[inline(always)]
    pub fn fall(&mut self) -> FallW<IenSpec> {
        FallW::new(self, 1)
    }
    #[doc = "Bit 2 - ACMP ready interrupt enable"]
    #[inline(always)]
    pub fn acmprdy(&mut self) -> AcmprdyW<IenSpec> {
        AcmprdyW::new(self, 2)
    }
    #[doc = "Bit 3 - Input conflict interrupt enable"]
    #[inline(always)]
    pub fn inputconflict(&mut self) -> InputconflictW<IenSpec> {
        InputconflictW::new(self, 3)
    }
    #[doc = "Bit 4 - Port allocation error interrupt enable"]
    #[inline(always)]
    pub fn portallocerr(&mut self) -> PortallocerrW<IenSpec> {
        PortallocerrW::new(self, 4)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
