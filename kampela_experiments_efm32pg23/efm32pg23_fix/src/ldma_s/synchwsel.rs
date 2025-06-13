#[doc = "Register `SYNCHWSEL` reader"]
pub type R = crate::R<SynchwselSpec>;
#[doc = "Register `SYNCHWSEL` writer"]
pub type W = crate::W<SynchwselSpec>;
#[doc = "Hardware Sync Trigger Set Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Syncsetedge {
    #[doc = "0: Use rising edge detection"]
    Rise = 0,
    #[doc = "1: Use falling edge detection"]
    Fall = 1,
}
impl From<Syncsetedge> for u8 {
    #[inline(always)]
    fn from(variant: Syncsetedge) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Syncsetedge {
    type Ux = u8;
}
impl crate::IsEnum for Syncsetedge {}
#[doc = "Field `SYNCSETEDGE` reader - Hardware Sync Trigger Set Edge Select"]
pub type SyncsetedgeR = crate::FieldReader<Syncsetedge>;
impl SyncsetedgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Syncsetedge> {
        match self.bits {
            0 => Some(Syncsetedge::Rise),
            1 => Some(Syncsetedge::Fall),
            _ => None,
        }
    }
    #[doc = "Use rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Syncsetedge::Rise
    }
    #[doc = "Use falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Syncsetedge::Fall
    }
}
#[doc = "Field `SYNCSETEDGE` writer - Hardware Sync Trigger Set Edge Select"]
pub type SyncsetedgeW<'a, REG> = crate::FieldWriter<'a, REG, 8, Syncsetedge>;
impl<'a, REG> SyncsetedgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Syncsetedge::Rise)
    }
    #[doc = "Use falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Syncsetedge::Fall)
    }
}
#[doc = "Hardware Sync Trigger Clear Edge Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Syncclredge {
    #[doc = "0: Use rising edge detection"]
    Rise = 0,
    #[doc = "1: Use falling edge detection"]
    Fall = 1,
}
impl From<Syncclredge> for u8 {
    #[inline(always)]
    fn from(variant: Syncclredge) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Syncclredge {
    type Ux = u8;
}
impl crate::IsEnum for Syncclredge {}
#[doc = "Field `SYNCCLREDGE` reader - Hardware Sync Trigger Clear Edge Select"]
pub type SyncclredgeR = crate::FieldReader<Syncclredge>;
impl SyncclredgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Syncclredge> {
        match self.bits {
            0 => Some(Syncclredge::Rise),
            1 => Some(Syncclredge::Fall),
            _ => None,
        }
    }
    #[doc = "Use rising edge detection"]
    #[inline(always)]
    pub fn is_rise(&self) -> bool {
        *self == Syncclredge::Rise
    }
    #[doc = "Use falling edge detection"]
    #[inline(always)]
    pub fn is_fall(&self) -> bool {
        *self == Syncclredge::Fall
    }
}
#[doc = "Field `SYNCCLREDGE` writer - Hardware Sync Trigger Clear Edge Select"]
pub type SyncclredgeW<'a, REG> = crate::FieldWriter<'a, REG, 8, Syncclredge>;
impl<'a, REG> SyncclredgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Use rising edge detection"]
    #[inline(always)]
    pub fn rise(self) -> &'a mut crate::W<REG> {
        self.variant(Syncclredge::Rise)
    }
    #[doc = "Use falling edge detection"]
    #[inline(always)]
    pub fn fall(self) -> &'a mut crate::W<REG> {
        self.variant(Syncclredge::Fall)
    }
}
impl R {
    #[doc = "Bits 0:7 - Hardware Sync Trigger Set Edge Select"]
    #[inline(always)]
    pub fn syncsetedge(&self) -> SyncsetedgeR {
        SyncsetedgeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Hardware Sync Trigger Clear Edge Select"]
    #[inline(always)]
    pub fn syncclredge(&self) -> SyncclredgeR {
        SyncclredgeR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Hardware Sync Trigger Set Edge Select"]
    #[inline(always)]
    pub fn syncsetedge(&mut self) -> SyncsetedgeW<SynchwselSpec> {
        SyncsetedgeW::new(self, 0)
    }
    #[doc = "Bits 16:23 - Hardware Sync Trigger Clear Edge Select"]
    #[inline(always)]
    pub fn syncclredge(&mut self) -> SyncclredgeW<SynchwselSpec> {
        SyncclredgeW::new(self, 16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`synchwsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`synchwsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SynchwselSpec;
impl crate::RegisterSpec for SynchwselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`synchwsel::R`](R) reader structure"]
impl crate::Readable for SynchwselSpec {}
#[doc = "`write(|w| ..)` method takes [`synchwsel::W`](W) writer structure"]
impl crate::Writable for SynchwselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNCHWSEL to value 0"]
impl crate::Resettable for SynchwselSpec {}
