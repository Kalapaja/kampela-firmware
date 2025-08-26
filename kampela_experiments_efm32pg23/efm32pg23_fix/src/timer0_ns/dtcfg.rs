#[doc = "Register `DTCFG` reader"]
pub type R = crate::R<DtcfgSpec>;
#[doc = "Register `DTCFG` writer"]
pub type W = crate::W<DtcfgSpec>;
#[doc = "Field `DTEN` reader - DTI Enable"]
pub type DtenR = crate::BitReader;
#[doc = "Field `DTEN` writer - DTI Enable"]
pub type DtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DTI Automatic Start-up Functionality\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtdas {
    #[doc = "0: No DTI restart on debugger exit"]
    Norestart = 0,
    #[doc = "1: DTI restart on debugger exit"]
    Restart = 1,
}
impl From<Dtdas> for bool {
    #[inline(always)]
    fn from(variant: Dtdas) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTDAS` reader - DTI Automatic Start-up Functionality"]
pub type DtdasR = crate::BitReader<Dtdas>;
impl DtdasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtdas {
        match self.bits {
            false => Dtdas::Norestart,
            true => Dtdas::Restart,
        }
    }
    #[doc = "No DTI restart on debugger exit"]
    #[inline(always)]
    pub fn is_norestart(&self) -> bool {
        *self == Dtdas::Norestart
    }
    #[doc = "DTI restart on debugger exit"]
    #[inline(always)]
    pub fn is_restart(&self) -> bool {
        *self == Dtdas::Restart
    }
}
#[doc = "Field `DTDAS` writer - DTI Automatic Start-up Functionality"]
pub type DtdasW<'a, REG> = crate::BitWriter<'a, REG, Dtdas>;
impl<'a, REG> DtdasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DTI restart on debugger exit"]
    #[inline(always)]
    pub fn norestart(self) -> &'a mut crate::W<REG> {
        self.variant(Dtdas::Norestart)
    }
    #[doc = "DTI restart on debugger exit"]
    #[inline(always)]
    pub fn restart(self) -> &'a mut crate::W<REG> {
        self.variant(Dtdas::Restart)
    }
}
#[doc = "Field `DTAR` reader - DTI Always Run"]
pub type DtarR = crate::BitReader;
#[doc = "Field `DTAR` writer - DTI Always Run"]
pub type DtarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFATS` reader - DTI Fault Action on Timer Stop"]
pub type DtfatsR = crate::BitReader;
#[doc = "Field `DTFATS` writer - DTI Fault Action on Timer Stop"]
pub type DtfatsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTPRSEN` reader - DTI PRS Source Enable"]
pub type DtprsenR = crate::BitReader;
#[doc = "Field `DTPRSEN` writer - DTI PRS Source Enable"]
pub type DtprsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    pub fn dten(&self) -> DtenR {
        DtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    pub fn dtdas(&self) -> DtdasR {
        DtdasR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - DTI Always Run"]
    #[inline(always)]
    pub fn dtar(&self) -> DtarR {
        DtarR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DTI Fault Action on Timer Stop"]
    #[inline(always)]
    pub fn dtfats(&self) -> DtfatsR {
        DtfatsR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DTI PRS Source Enable"]
    #[inline(always)]
    pub fn dtprsen(&self) -> DtprsenR {
        DtprsenR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    pub fn dten(&mut self) -> DtenW<DtcfgSpec> {
        DtenW::new(self, 0)
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    pub fn dtdas(&mut self) -> DtdasW<DtcfgSpec> {
        DtdasW::new(self, 1)
    }
    #[doc = "Bit 9 - DTI Always Run"]
    #[inline(always)]
    pub fn dtar(&mut self) -> DtarW<DtcfgSpec> {
        DtarW::new(self, 9)
    }
    #[doc = "Bit 10 - DTI Fault Action on Timer Stop"]
    #[inline(always)]
    pub fn dtfats(&mut self) -> DtfatsW<DtcfgSpec> {
        DtfatsW::new(self, 10)
    }
    #[doc = "Bit 11 - DTI PRS Source Enable"]
    #[inline(always)]
    pub fn dtprsen(&mut self) -> DtprsenW<DtcfgSpec> {
        DtprsenW::new(self, 11)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dtcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtcfgSpec;
impl crate::RegisterSpec for DtcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtcfg::R`](R) reader structure"]
impl crate::Readable for DtcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dtcfg::W`](W) writer structure"]
impl crate::Writable for DtcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTCFG to value 0"]
impl crate::Resettable for DtcfgSpec {}
