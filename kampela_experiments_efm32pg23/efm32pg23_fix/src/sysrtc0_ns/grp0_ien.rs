#[doc = "Register `GRP0_IEN` reader"]
pub type R = crate::R<Grp0IenSpec>;
#[doc = "Register `GRP0_IEN` writer"]
pub type W = crate::W<Grp0IenSpec>;
#[doc = "Field `OVF` reader - Overflow Interrupt Enable"]
pub type OvfR = crate::BitReader;
#[doc = "Field `OVF` writer - Overflow Interrupt Enable"]
pub type OvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP0` reader - Compare 0 Interrupt Enable"]
pub type Cmp0R = crate::BitReader;
#[doc = "Field `CMP0` writer - Compare 0 Interrupt Enable"]
pub type Cmp0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMP1` reader - Compare 1 Interrupt Enable"]
pub type Cmp1R = crate::BitReader;
#[doc = "Field `CMP1` writer - Compare 1 Interrupt Enable"]
pub type Cmp1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0` reader - Capture 0 Interrupt Enable"]
pub type Cap0R = crate::BitReader;
#[doc = "Field `CAP0` writer - Capture 0 Interrupt Enable"]
pub type Cap0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&self) -> OvfR {
        OvfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cmp0(&self) -> Cmp0R {
        Cmp0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub fn cmp1(&self) -> Cmp1R {
        Cmp1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cap0(&self) -> Cap0R {
        Cap0R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OvfW<Grp0IenSpec> {
        OvfW::new(self, 0)
    }
    #[doc = "Bit 1 - Compare 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cmp0(&mut self) -> Cmp0W<Grp0IenSpec> {
        Cmp0W::new(self, 1)
    }
    #[doc = "Bit 2 - Compare 1 Interrupt Enable"]
    #[inline(always)]
    pub fn cmp1(&mut self) -> Cmp1W<Grp0IenSpec> {
        Cmp1W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cap0(&mut self) -> Cap0W<Grp0IenSpec> {
        Cap0W::new(self, 3)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`grp0_ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp0_ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Grp0IenSpec;
impl crate::RegisterSpec for Grp0IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp0_ien::R`](R) reader structure"]
impl crate::Readable for Grp0IenSpec {}
#[doc = "`write(|w| ..)` method takes [`grp0_ien::W`](W) writer structure"]
impl crate::Writable for Grp0IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP0_IEN to value 0"]
impl crate::Resettable for Grp0IenSpec {}
