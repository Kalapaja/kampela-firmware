#[doc = "Register `CFGNSTCALIB` reader"]
pub type R = crate::R<CfgnstcalibSpec>;
#[doc = "Register `CFGNSTCALIB` writer"]
pub type W = crate::W<CfgnstcalibSpec>;
#[doc = "Field `TENMS` reader - Ten Milliseconds"]
pub type TenmsR = crate::FieldReader<u32>;
#[doc = "Field `TENMS` writer - Ten Milliseconds"]
pub type TenmsW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `SKEW` reader - Skew"]
pub type SkewR = crate::BitReader;
#[doc = "Field `SKEW` writer - Skew"]
pub type SkewW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "No Reference\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Noref {
    #[doc = "0: Reference clock is implemented"]
    Ref = 0,
    #[doc = "1: Reference clock is not implemented"]
    Noref = 1,
}
impl From<Noref> for bool {
    #[inline(always)]
    fn from(variant: Noref) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOREF` reader - No Reference"]
pub type NorefR = crate::BitReader<Noref>;
impl NorefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Noref {
        match self.bits {
            false => Noref::Ref,
            true => Noref::Noref,
        }
    }
    #[doc = "Reference clock is implemented"]
    #[inline(always)]
    pub fn is_ref(&self) -> bool {
        *self == Noref::Ref
    }
    #[doc = "Reference clock is not implemented"]
    #[inline(always)]
    pub fn is_noref(&self) -> bool {
        *self == Noref::Noref
    }
}
#[doc = "Field `NOREF` writer - No Reference"]
pub type NorefW<'a, REG> = crate::BitWriter<'a, REG, Noref>;
impl<'a, REG> NorefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reference clock is implemented"]
    #[inline(always)]
    pub fn ref_(self) -> &'a mut crate::W<REG> {
        self.variant(Noref::Ref)
    }
    #[doc = "Reference clock is not implemented"]
    #[inline(always)]
    pub fn noref(self) -> &'a mut crate::W<REG> {
        self.variant(Noref::Noref)
    }
}
impl R {
    #[doc = "Bits 0:23 - Ten Milliseconds"]
    #[inline(always)]
    pub fn tenms(&self) -> TenmsR {
        TenmsR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Skew"]
    #[inline(always)]
    pub fn skew(&self) -> SkewR {
        SkewR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - No Reference"]
    #[inline(always)]
    pub fn noref(&self) -> NorefR {
        NorefR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Ten Milliseconds"]
    #[inline(always)]
    pub fn tenms(&mut self) -> TenmsW<CfgnstcalibSpec> {
        TenmsW::new(self, 0)
    }
    #[doc = "Bit 24 - Skew"]
    #[inline(always)]
    pub fn skew(&mut self) -> SkewW<CfgnstcalibSpec> {
        SkewW::new(self, 24)
    }
    #[doc = "Bit 25 - No Reference"]
    #[inline(always)]
    pub fn noref(&mut self) -> NorefW<CfgnstcalibSpec> {
        NorefW::new(self, 25)
    }
}
#[doc = "Configure to define the system tick for the M33.\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgnstcalib::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgnstcalib::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgnstcalibSpec;
impl crate::RegisterSpec for CfgnstcalibSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgnstcalib::R`](R) reader structure"]
impl crate::Readable for CfgnstcalibSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgnstcalib::W`](W) writer structure"]
impl crate::Writable for CfgnstcalibSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGNSTCALIB to value 0x0100_4a37"]
impl crate::Resettable for CfgnstcalibSpec {
    const RESET_VALUE: u32 = 0x0100_4a37;
}
