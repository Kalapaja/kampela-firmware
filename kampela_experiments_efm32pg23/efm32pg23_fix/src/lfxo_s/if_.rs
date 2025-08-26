#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `RDY` reader - LFXO Ready Interrupt Flag"]
pub type RdyR = crate::BitReader;
#[doc = "Field `RDY` writer - LFXO Ready Interrupt Flag"]
pub type RdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POSEDGE` reader - Rising Edge Interrupt Flag"]
pub type PosedgeR = crate::BitReader;
#[doc = "Field `POSEDGE` writer - Rising Edge Interrupt Flag"]
pub type PosedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NEGEDGE` reader - Falling Edge Interrupt Flag"]
pub type NegedgeR = crate::BitReader;
#[doc = "Field `NEGEDGE` writer - Falling Edge Interrupt Flag"]
pub type NegedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAIL` reader - LFXO Failure Interrupt Flag"]
pub type FailR = crate::BitReader;
#[doc = "Field `FAIL` writer - LFXO Failure Interrupt Flag"]
pub type FailW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LFXO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rdy(&self) -> RdyR {
        RdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising Edge Interrupt Flag"]
    #[inline(always)]
    pub fn posedge(&self) -> PosedgeR {
        PosedgeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling Edge Interrupt Flag"]
    #[inline(always)]
    pub fn negedge(&self) -> NegedgeR {
        NegedgeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LFXO Failure Interrupt Flag"]
    #[inline(always)]
    pub fn fail(&self) -> FailR {
        FailR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LFXO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn rdy(&mut self) -> RdyW<IfSpec> {
        RdyW::new(self, 0)
    }
    #[doc = "Bit 1 - Rising Edge Interrupt Flag"]
    #[inline(always)]
    pub fn posedge(&mut self) -> PosedgeW<IfSpec> {
        PosedgeW::new(self, 1)
    }
    #[doc = "Bit 2 - Falling Edge Interrupt Flag"]
    #[inline(always)]
    pub fn negedge(&mut self) -> NegedgeW<IfSpec> {
        NegedgeW::new(self, 2)
    }
    #[doc = "Bit 3 - LFXO Failure Interrupt Flag"]
    #[inline(always)]
    pub fn fail(&mut self) -> FailW<IfSpec> {
        FailW::new(self, 3)
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
