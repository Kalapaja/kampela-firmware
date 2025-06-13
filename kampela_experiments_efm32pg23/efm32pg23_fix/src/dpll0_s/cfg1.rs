#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Field `M` reader - Factor M"]
pub type MR = crate::FieldReader<u16>;
#[doc = "Field `M` writer - Factor M"]
pub type MW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `N` reader - Factor N"]
pub type NR = crate::FieldReader<u16>;
#[doc = "Field `N` writer - Factor N"]
pub type NW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Factor M"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Factor N"]
    #[inline(always)]
    pub fn n(&self) -> NR {
        NR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Factor M"]
    #[inline(always)]
    pub fn m(&mut self) -> MW<Cfg1Spec> {
        MW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Factor N"]
    #[inline(always)]
    pub fn n(&mut self) -> NW<Cfg1Spec> {
        NW::new(self, 16)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for Cfg1Spec {}
