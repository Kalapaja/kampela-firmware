#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Field `ACMPOUT` reader - Analog Comparator Output"]
pub type AcmpoutR = crate::BitReader;
#[doc = "Field `ACMPRDY` reader - Analog Comparator Ready"]
pub type AcmprdyR = crate::BitReader;
#[doc = "Field `INPUTCONFLICT` reader - INPUT conflict"]
pub type InputconflictR = crate::BitReader;
#[doc = "Field `PORTALLOCERR` reader - Port allocation error"]
pub type PortallocerrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Analog Comparator Output"]
    #[inline(always)]
    pub fn acmpout(&self) -> AcmpoutR {
        AcmpoutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Analog Comparator Ready"]
    #[inline(always)]
    pub fn acmprdy(&self) -> AcmprdyR {
        AcmprdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - INPUT conflict"]
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
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
