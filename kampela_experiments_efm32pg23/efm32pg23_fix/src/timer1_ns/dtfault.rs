#[doc = "Register `DTFAULT` reader"]
pub type R = crate::R<DtfaultSpec>;
#[doc = "Field `DTPRS0F` reader - DTI PRS 0 Fault"]
pub type Dtprs0fR = crate::BitReader;
#[doc = "Field `DTPRS1F` reader - DTI PRS 1 Fault"]
pub type Dtprs1fR = crate::BitReader;
#[doc = "Field `DTDBGF` reader - DTI Debugger Fault"]
pub type DtdbgfR = crate::BitReader;
#[doc = "Field `DTLOCKUPF` reader - DTI Lockup Fault"]
pub type DtlockupfR = crate::BitReader;
#[doc = "Field `DTEM23F` reader - DTI EM23 Entry Fault"]
pub type Dtem23fR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DTI PRS 0 Fault"]
    #[inline(always)]
    pub fn dtprs0f(&self) -> Dtprs0fR {
        Dtprs0fR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTI PRS 1 Fault"]
    #[inline(always)]
    pub fn dtprs1f(&self) -> Dtprs1fR {
        Dtprs1fR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DTI Debugger Fault"]
    #[inline(always)]
    pub fn dtdbgf(&self) -> DtdbgfR {
        DtdbgfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTI Lockup Fault"]
    #[inline(always)]
    pub fn dtlockupf(&self) -> DtlockupfR {
        DtlockupfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DTI EM23 Entry Fault"]
    #[inline(always)]
    pub fn dtem23f(&self) -> Dtem23fR {
        Dtem23fR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dtfault::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtfaultSpec;
impl crate::RegisterSpec for DtfaultSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtfault::R`](R) reader structure"]
impl crate::Readable for DtfaultSpec {}
#[doc = "`reset()` method sets DTFAULT to value 0"]
impl crate::Resettable for DtfaultSpec {}
