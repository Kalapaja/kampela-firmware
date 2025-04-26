#[doc = "Register `GRP0_CMP1VALUE` reader"]
pub type R = crate::R<Grp0Cmp1valueSpec>;
#[doc = "Register `GRP0_CMP1VALUE` writer"]
pub type W = crate::W<Grp0Cmp1valueSpec>;
#[doc = "Field `CMP1VALUE` reader - Compare 1 Value"]
pub type Cmp1valueR = crate::FieldReader<u32>;
#[doc = "Field `CMP1VALUE` writer - Compare 1 Value"]
pub type Cmp1valueW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Compare 1 Value"]
    #[inline(always)]
    pub fn cmp1value(&self) -> Cmp1valueR {
        Cmp1valueR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Compare 1 Value"]
    #[inline(always)]
    pub fn cmp1value(&mut self) -> Cmp1valueW<Grp0Cmp1valueSpec> {
        Cmp1valueW::new(self, 0)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`grp0_cmp1value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grp0_cmp1value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Grp0Cmp1valueSpec;
impl crate::RegisterSpec for Grp0Cmp1valueSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grp0_cmp1value::R`](R) reader structure"]
impl crate::Readable for Grp0Cmp1valueSpec {}
#[doc = "`write(|w| ..)` method takes [`grp0_cmp1value::W`](W) writer structure"]
impl crate::Writable for Grp0Cmp1valueSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GRP0_CMP1VALUE to value 0"]
impl crate::Resettable for Grp0Cmp1valueSpec {}
