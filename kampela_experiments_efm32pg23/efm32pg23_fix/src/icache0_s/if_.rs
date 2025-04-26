#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `HITOF` reader - Hit Overflow Interrupt Flag"]
pub type HitofR = crate::BitReader;
#[doc = "Field `HITOF` writer - Hit Overflow Interrupt Flag"]
pub type HitofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISSOF` reader - Miss Overflow Interrupt Flag"]
pub type MissofR = crate::BitReader;
#[doc = "Field `MISSOF` writer - Miss Overflow Interrupt Flag"]
pub type MissofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHITOF` reader - Advanced Hit Overflow Interrupt Flag"]
pub type AhitofR = crate::BitReader;
#[doc = "Field `AHITOF` writer - Advanced Hit Overflow Interrupt Flag"]
pub type AhitofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERROR` reader - RAM error Interrupt Flag"]
pub type RamerrorR = crate::BitReader;
#[doc = "Field `RAMERROR` writer - RAM error Interrupt Flag"]
pub type RamerrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Hit Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn hitof(&self) -> HitofR {
        HitofR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Miss Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn missof(&self) -> MissofR {
        MissofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Advanced Hit Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ahitof(&self) -> AhitofR {
        AhitofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - RAM error Interrupt Flag"]
    #[inline(always)]
    pub fn ramerror(&self) -> RamerrorR {
        RamerrorR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hit Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn hitof(&mut self) -> HitofW<IfSpec> {
        HitofW::new(self, 0)
    }
    #[doc = "Bit 1 - Miss Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn missof(&mut self) -> MissofW<IfSpec> {
        MissofW::new(self, 1)
    }
    #[doc = "Bit 2 - Advanced Hit Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ahitof(&mut self) -> AhitofW<IfSpec> {
        AhitofW::new(self, 2)
    }
    #[doc = "Bit 8 - RAM error Interrupt Flag"]
    #[inline(always)]
    pub fn ramerror(&mut self) -> RamerrorW<IfSpec> {
        RamerrorW::new(self, 8)
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
