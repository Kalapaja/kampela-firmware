#[doc = "Register `LPMODE` reader"]
pub type R = crate::R<LpmodeSpec>;
#[doc = "Register `LPMODE` writer"]
pub type W = crate::W<LpmodeSpec>;
#[doc = "Low Power Level\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lplevel {
    #[doc = "0: Base instruction cache functionality"]
    Basic = 0,
    #[doc = "1: Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory"]
    Advanced = 1,
    #[doc = "3: Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    Minactivity = 3,
}
impl From<Lplevel> for u8 {
    #[inline(always)]
    fn from(variant: Lplevel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lplevel {
    type Ux = u8;
}
impl crate::IsEnum for Lplevel {}
#[doc = "Field `LPLEVEL` reader - Low Power Level"]
pub type LplevelR = crate::FieldReader<Lplevel>;
impl LplevelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lplevel> {
        match self.bits {
            0 => Some(Lplevel::Basic),
            1 => Some(Lplevel::Advanced),
            3 => Some(Lplevel::Minactivity),
            _ => None,
        }
    }
    #[doc = "Base instruction cache functionality"]
    #[inline(always)]
    pub fn is_basic(&self) -> bool {
        *self == Lplevel::Basic
    }
    #[doc = "Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory"]
    #[inline(always)]
    pub fn is_advanced(&self) -> bool {
        *self == Lplevel::Advanced
    }
    #[doc = "Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    #[inline(always)]
    pub fn is_minactivity(&self) -> bool {
        *self == Lplevel::Minactivity
    }
}
#[doc = "Field `LPLEVEL` writer - Low Power Level"]
pub type LplevelW<'a, REG> = crate::FieldWriter<'a, REG, 2, Lplevel>;
impl<'a, REG> LplevelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Base instruction cache functionality"]
    #[inline(always)]
    pub fn basic(self) -> &'a mut crate::W<REG> {
        self.variant(Lplevel::Basic)
    }
    #[doc = "Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory"]
    #[inline(always)]
    pub fn advanced(self) -> &'a mut crate::W<REG> {
        self.variant(Lplevel::Advanced)
    }
    #[doc = "Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    #[inline(always)]
    pub fn minactivity(self) -> &'a mut crate::W<REG> {
        self.variant(Lplevel::Minactivity)
    }
}
#[doc = "Field `NESTFACTOR` reader - Low Power Nest Factor"]
pub type NestfactorR = crate::FieldReader;
#[doc = "Field `NESTFACTOR` writer - Low Power Nest Factor"]
pub type NestfactorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Low Power Level"]
    #[inline(always)]
    pub fn lplevel(&self) -> LplevelR {
        LplevelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - Low Power Nest Factor"]
    #[inline(always)]
    pub fn nestfactor(&self) -> NestfactorR {
        NestfactorR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low Power Level"]
    #[inline(always)]
    pub fn lplevel(&mut self) -> LplevelW<LpmodeSpec> {
        LplevelW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Low Power Nest Factor"]
    #[inline(always)]
    pub fn nestfactor(&mut self) -> NestfactorW<LpmodeSpec> {
        NestfactorW::new(self, 4)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`lpmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpmodeSpec;
impl crate::RegisterSpec for LpmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpmode::R`](R) reader structure"]
impl crate::Readable for LpmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`lpmode::W`](W) writer structure"]
impl crate::Writable for LpmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPMODE to value 0x23"]
impl crate::Resettable for LpmodeSpec {
    const RESET_VALUE: u32 = 0x23;
}
