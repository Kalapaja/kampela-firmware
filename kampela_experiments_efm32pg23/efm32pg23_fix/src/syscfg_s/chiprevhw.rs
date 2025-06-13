#[doc = "Register `CHIPREVHW` reader"]
pub type R = crate::R<ChiprevhwSpec>;
#[doc = "Register `CHIPREVHW` writer"]
pub type W = crate::W<ChiprevhwSpec>;
#[doc = "Field `MAJOR` reader - Hardwired Chip Revision Major value"]
pub type MajorR = crate::FieldReader;
#[doc = "Field `MAJOR` writer - Hardwired Chip Revision Major value"]
pub type MajorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FAMILY` reader - Hardwired Chip Family value"]
pub type FamilyR = crate::FieldReader;
#[doc = "Field `FAMILY` writer - Hardwired Chip Family value"]
pub type FamilyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MINOR` reader - Hardwired Chip Revision Minor value"]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MINOR` writer - Hardwired Chip Revision Minor value"]
pub type MinorW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5 - Hardwired Chip Revision Major value"]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Hardwired Chip Family value"]
    #[inline(always)]
    pub fn family(&self) -> FamilyR {
        FamilyR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:19 - Hardwired Chip Revision Minor value"]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Hardwired Chip Revision Major value"]
    #[inline(always)]
    pub fn major(&mut self) -> MajorW<ChiprevhwSpec> {
        MajorW::new(self, 0)
    }
    #[doc = "Bits 6:11 - Hardwired Chip Family value"]
    #[inline(always)]
    pub fn family(&mut self) -> FamilyW<ChiprevhwSpec> {
        FamilyW::new(self, 6)
    }
    #[doc = "Bits 12:19 - Hardwired Chip Revision Minor value"]
    #[inline(always)]
    pub fn minor(&mut self) -> MinorW<ChiprevhwSpec> {
        MinorW::new(self, 12)
    }
}
#[doc = "Read to get the hard-wired chip revision.\n\nYou can [`read`](crate::Reg::read) this register and get [`chiprevhw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chiprevhw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChiprevhwSpec;
impl crate::RegisterSpec for ChiprevhwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chiprevhw::R`](R) reader structure"]
impl crate::Readable for ChiprevhwSpec {}
#[doc = "`write(|w| ..)` method takes [`chiprevhw::W`](W) writer structure"]
impl crate::Writable for ChiprevhwSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHIPREVHW to value 0x0e01"]
impl crate::Resettable for ChiprevhwSpec {
    const RESET_VALUE: u32 = 0x0e01;
}
