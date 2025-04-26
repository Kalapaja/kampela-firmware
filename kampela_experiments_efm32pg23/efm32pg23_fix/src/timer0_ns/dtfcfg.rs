#[doc = "Register `DTFCFG` reader"]
pub type R = crate::R<DtfcfgSpec>;
#[doc = "Register `DTFCFG` writer"]
pub type W = crate::W<DtfcfgSpec>;
#[doc = "DTI Fault Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dtfa {
    #[doc = "0: No action on fault"]
    None = 0,
    #[doc = "1: Set outputs inactive"]
    Inactive = 1,
    #[doc = "2: Clear outputs"]
    Clear = 2,
    #[doc = "3: Tristate outputs"]
    Tristate = 3,
}
impl From<Dtfa> for u8 {
    #[inline(always)]
    fn from(variant: Dtfa) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dtfa {
    type Ux = u8;
}
impl crate::IsEnum for Dtfa {}
#[doc = "Field `DTFA` reader - DTI Fault Action"]
pub type DtfaR = crate::FieldReader<Dtfa>;
impl DtfaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtfa {
        match self.bits {
            0 => Dtfa::None,
            1 => Dtfa::Inactive,
            2 => Dtfa::Clear,
            3 => Dtfa::Tristate,
            _ => unreachable!(),
        }
    }
    #[doc = "No action on fault"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Dtfa::None
    }
    #[doc = "Set outputs inactive"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Dtfa::Inactive
    }
    #[doc = "Clear outputs"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == Dtfa::Clear
    }
    #[doc = "Tristate outputs"]
    #[inline(always)]
    pub fn is_tristate(&self) -> bool {
        *self == Dtfa::Tristate
    }
}
#[doc = "Field `DTFA` writer - DTI Fault Action"]
pub type DtfaW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dtfa, crate::Safe>;
impl<'a, REG> DtfaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No action on fault"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Dtfa::None)
    }
    #[doc = "Set outputs inactive"]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Dtfa::Inactive)
    }
    #[doc = "Clear outputs"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(Dtfa::Clear)
    }
    #[doc = "Tristate outputs"]
    #[inline(always)]
    pub fn tristate(self) -> &'a mut crate::W<REG> {
        self.variant(Dtfa::Tristate)
    }
}
#[doc = "Field `DTPRS0FEN` reader - DTI PRS 0 Fault Enable"]
pub type Dtprs0fenR = crate::BitReader;
#[doc = "Field `DTPRS0FEN` writer - DTI PRS 0 Fault Enable"]
pub type Dtprs0fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTPRS1FEN` reader - DTI PRS 1 Fault Enable"]
pub type Dtprs1fenR = crate::BitReader;
#[doc = "Field `DTPRS1FEN` writer - DTI PRS 1 Fault Enable"]
pub type Dtprs1fenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTDBGFEN` reader - DTI Debugger Fault Enable"]
pub type DtdbgfenR = crate::BitReader;
#[doc = "Field `DTDBGFEN` writer - DTI Debugger Fault Enable"]
pub type DtdbgfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTLOCKUPFEN` reader - DTI Lockup Fault Enable"]
pub type DtlockupfenR = crate::BitReader;
#[doc = "Field `DTLOCKUPFEN` writer - DTI Lockup Fault Enable"]
pub type DtlockupfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTEM23FEN` reader - DTI EM23 Fault Enable"]
pub type Dtem23fenR = crate::BitReader;
#[doc = "Field `DTEM23FEN` writer - DTI EM23 Fault Enable"]
pub type Dtem23fenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline(always)]
    pub fn dtfa(&self) -> DtfaR {
        DtfaR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline(always)]
    pub fn dtprs0fen(&self) -> Dtprs0fenR {
        Dtprs0fenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline(always)]
    pub fn dtprs1fen(&self) -> Dtprs1fenR {
        Dtprs1fenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline(always)]
    pub fn dtdbgfen(&self) -> DtdbgfenR {
        DtdbgfenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline(always)]
    pub fn dtlockupfen(&self) -> DtlockupfenR {
        DtlockupfenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DTI EM23 Fault Enable"]
    #[inline(always)]
    pub fn dtem23fen(&self) -> Dtem23fenR {
        Dtem23fenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:17 - DTI Fault Action"]
    #[inline(always)]
    pub fn dtfa(&mut self) -> DtfaW<DtfcfgSpec> {
        DtfaW::new(self, 16)
    }
    #[doc = "Bit 24 - DTI PRS 0 Fault Enable"]
    #[inline(always)]
    pub fn dtprs0fen(&mut self) -> Dtprs0fenW<DtfcfgSpec> {
        Dtprs0fenW::new(self, 24)
    }
    #[doc = "Bit 25 - DTI PRS 1 Fault Enable"]
    #[inline(always)]
    pub fn dtprs1fen(&mut self) -> Dtprs1fenW<DtfcfgSpec> {
        Dtprs1fenW::new(self, 25)
    }
    #[doc = "Bit 26 - DTI Debugger Fault Enable"]
    #[inline(always)]
    pub fn dtdbgfen(&mut self) -> DtdbgfenW<DtfcfgSpec> {
        DtdbgfenW::new(self, 26)
    }
    #[doc = "Bit 27 - DTI Lockup Fault Enable"]
    #[inline(always)]
    pub fn dtlockupfen(&mut self) -> DtlockupfenW<DtfcfgSpec> {
        DtlockupfenW::new(self, 27)
    }
    #[doc = "Bit 28 - DTI EM23 Fault Enable"]
    #[inline(always)]
    pub fn dtem23fen(&mut self) -> Dtem23fenW<DtfcfgSpec> {
        Dtem23fenW::new(self, 28)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dtfcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtfcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtfcfgSpec;
impl crate::RegisterSpec for DtfcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtfcfg::R`](R) reader structure"]
impl crate::Readable for DtfcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dtfcfg::W`](W) writer structure"]
impl crate::Writable for DtfcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTFCFG to value 0"]
impl crate::Resettable for DtfcfgSpec {}
