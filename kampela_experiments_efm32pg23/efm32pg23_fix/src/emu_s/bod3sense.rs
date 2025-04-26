#[doc = "Register `BOD3SENSE` reader"]
pub type R = crate::R<Bod3senseSpec>;
#[doc = "Register `BOD3SENSE` writer"]
pub type W = crate::W<Bod3senseSpec>;
#[doc = "Field `AVDDBODEN` reader - AVDD BOD enable"]
pub type AvddbodenR = crate::BitReader;
#[doc = "Field `AVDDBODEN` writer - AVDD BOD enable"]
pub type AvddbodenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDIO0BODEN` reader - VDDIO0 BOD enable"]
pub type Vddio0bodenR = crate::BitReader;
#[doc = "Field `VDDIO0BODEN` writer - VDDIO0 BOD enable"]
pub type Vddio0bodenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VDDIO1BODEN` reader - VDDIO1 BOD enable"]
pub type Vddio1bodenR = crate::BitReader;
#[doc = "Field `VDDIO1BODEN` writer - VDDIO1 BOD enable"]
pub type Vddio1bodenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AVDD BOD enable"]
    #[inline(always)]
    pub fn avddboden(&self) -> AvddbodenR {
        AvddbodenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VDDIO0 BOD enable"]
    #[inline(always)]
    pub fn vddio0boden(&self) -> Vddio0bodenR {
        Vddio0bodenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VDDIO1 BOD enable"]
    #[inline(always)]
    pub fn vddio1boden(&self) -> Vddio1bodenR {
        Vddio1bodenR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AVDD BOD enable"]
    #[inline(always)]
    pub fn avddboden(&mut self) -> AvddbodenW<Bod3senseSpec> {
        AvddbodenW::new(self, 0)
    }
    #[doc = "Bit 1 - VDDIO0 BOD enable"]
    #[inline(always)]
    pub fn vddio0boden(&mut self) -> Vddio0bodenW<Bod3senseSpec> {
        Vddio0bodenW::new(self, 1)
    }
    #[doc = "Bit 2 - VDDIO1 BOD enable"]
    #[inline(always)]
    pub fn vddio1boden(&mut self) -> Vddio1bodenW<Bod3senseSpec> {
        Vddio1bodenW::new(self, 2)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`bod3sense::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bod3sense::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bod3senseSpec;
impl crate::RegisterSpec for Bod3senseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bod3sense::R`](R) reader structure"]
impl crate::Readable for Bod3senseSpec {}
#[doc = "`write(|w| ..)` method takes [`bod3sense::W`](W) writer structure"]
impl crate::Writable for Bod3senseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOD3SENSE to value 0"]
impl crate::Resettable for Bod3senseSpec {}
