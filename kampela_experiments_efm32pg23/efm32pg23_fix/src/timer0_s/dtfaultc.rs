#[doc = "Register `DTFAULTC` writer"]
pub type W = crate::W<DtfaultcSpec>;
#[doc = "Field `DTPRS0FC` writer - DTI PRS0 Fault Clear"]
pub type Dtprs0fcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTPRS1FC` writer - DTI PRS1 Fault Clear"]
pub type Dtprs1fcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTDBGFC` writer - DTI Debugger Fault Clear"]
pub type DtdbgfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTLOCKUPFC` writer - DTI Lockup Fault Clear"]
pub type DtlockupfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTEM23FC` writer - DTI EM23 Fault Clear"]
pub type Dtem23fcW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DTI PRS0 Fault Clear"]
    #[inline(always)]
    pub fn dtprs0fc(&mut self) -> Dtprs0fcW<DtfaultcSpec> {
        Dtprs0fcW::new(self, 0)
    }
    #[doc = "Bit 1 - DTI PRS1 Fault Clear"]
    #[inline(always)]
    pub fn dtprs1fc(&mut self) -> Dtprs1fcW<DtfaultcSpec> {
        Dtprs1fcW::new(self, 1)
    }
    #[doc = "Bit 2 - DTI Debugger Fault Clear"]
    #[inline(always)]
    pub fn dtdbgfc(&mut self) -> DtdbgfcW<DtfaultcSpec> {
        DtdbgfcW::new(self, 2)
    }
    #[doc = "Bit 3 - DTI Lockup Fault Clear"]
    #[inline(always)]
    pub fn dtlockupfc(&mut self) -> DtlockupfcW<DtfaultcSpec> {
        DtlockupfcW::new(self, 3)
    }
    #[doc = "Bit 4 - DTI EM23 Fault Clear"]
    #[inline(always)]
    pub fn dtem23fc(&mut self) -> Dtem23fcW<DtfaultcSpec> {
        Dtem23fcW::new(self, 4)
    }
}
#[doc = "No Description\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtfaultc::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtfaultcSpec;
impl crate::RegisterSpec for DtfaultcSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dtfaultc::W`](W) writer structure"]
impl crate::Writable for DtfaultcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTFAULTC to value 0"]
impl crate::Resettable for DtfaultcSpec {}
