#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `HITOF` reader - Hit Overflow Interrupt Enable"]
pub type HitofR = crate::BitReader;
#[doc = "Field `HITOF` writer - Hit Overflow Interrupt Enable"]
pub type HitofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISSOF` reader - Miss Overflow Interrupt Enable"]
pub type MissofR = crate::BitReader;
#[doc = "Field `MISSOF` writer - Miss Overflow Interrupt Enable"]
pub type MissofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHITOF` reader - Advanced Hit Overflow Interrupt Enable"]
pub type AhitofR = crate::BitReader;
#[doc = "Field `AHITOF` writer - Advanced Hit Overflow Interrupt Enable"]
pub type AhitofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAMERROR` reader - RAM error Interrupt Enable"]
pub type RamerrorR = crate::BitReader;
#[doc = "Field `RAMERROR` writer - RAM error Interrupt Enable"]
pub type RamerrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Hit Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn hitof(&self) -> HitofR {
        HitofR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Miss Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn missof(&self) -> MissofR {
        MissofR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Advanced Hit Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ahitof(&self) -> AhitofR {
        AhitofR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - RAM error Interrupt Enable"]
    #[inline(always)]
    pub fn ramerror(&self) -> RamerrorR {
        RamerrorR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hit Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn hitof(&mut self) -> HitofW<IenSpec> {
        HitofW::new(self, 0)
    }
    #[doc = "Bit 1 - Miss Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn missof(&mut self) -> MissofW<IenSpec> {
        MissofW::new(self, 1)
    }
    #[doc = "Bit 2 - Advanced Hit Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ahitof(&mut self) -> AhitofW<IenSpec> {
        AhitofW::new(self, 2)
    }
    #[doc = "Bit 8 - RAM error Interrupt Enable"]
    #[inline(always)]
    pub fn ramerror(&mut self) -> RamerrorW<IenSpec> {
        RamerrorW::new(self, 8)
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
