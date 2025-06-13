#[doc = "Register `VREGVDDCMPCTRL` reader"]
pub type R = crate::R<VregvddcmpctrlSpec>;
#[doc = "Register `VREGVDDCMPCTRL` writer"]
pub type W = crate::W<VregvddcmpctrlSpec>;
#[doc = "Field `VREGINCMPEN` reader - VREGVDD comparator enable"]
pub type VregincmpenR = crate::BitReader;
#[doc = "Field `VREGINCMPEN` writer - VREGVDD comparator enable"]
pub type VregincmpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THRESSEL` reader - VREGVDD comparator threshold programming"]
pub type ThresselR = crate::FieldReader;
#[doc = "Field `THRESSEL` writer - VREGVDD comparator threshold programming"]
pub type ThresselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - VREGVDD comparator enable"]
    #[inline(always)]
    pub fn vregincmpen(&self) -> VregincmpenR {
        VregincmpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - VREGVDD comparator threshold programming"]
    #[inline(always)]
    pub fn thressel(&self) -> ThresselR {
        ThresselR::new(((self.bits >> 1) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - VREGVDD comparator enable"]
    #[inline(always)]
    pub fn vregincmpen(&mut self) -> VregincmpenW<VregvddcmpctrlSpec> {
        VregincmpenW::new(self, 0)
    }
    #[doc = "Bits 1:2 - VREGVDD comparator threshold programming"]
    #[inline(always)]
    pub fn thressel(&mut self) -> ThresselW<VregvddcmpctrlSpec> {
        ThresselW::new(self, 1)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`vregvddcmpctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vregvddcmpctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VregvddcmpctrlSpec;
impl crate::RegisterSpec for VregvddcmpctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vregvddcmpctrl::R`](R) reader structure"]
impl crate::Readable for VregvddcmpctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`vregvddcmpctrl::W`](W) writer structure"]
impl crate::Writable for VregvddcmpctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VREGVDDCMPCTRL to value 0x06"]
impl crate::Resettable for VregvddcmpctrlSpec {
    const RESET_VALUE: u32 = 0x06;
}
