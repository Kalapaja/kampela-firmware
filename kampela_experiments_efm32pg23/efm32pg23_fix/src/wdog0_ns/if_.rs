#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `TOUT` reader - WDOG Timeout Interrupt Flag"]
pub type ToutR = crate::BitReader;
#[doc = "Field `TOUT` writer - WDOG Timeout Interrupt Flag"]
pub type ToutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WARN` reader - WDOG Warning Timeout Interrupt Flag"]
pub type WarnR = crate::BitReader;
#[doc = "Field `WARN` writer - WDOG Warning Timeout Interrupt Flag"]
pub type WarnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN` reader - WDOG Window Interrupt Flag"]
pub type WinR = crate::BitReader;
#[doc = "Field `WIN` writer - WDOG Window Interrupt Flag"]
pub type WinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEM0` reader - PRS Src0 Event Missing Interrupt Flag"]
pub type Pem0R = crate::BitReader;
#[doc = "Field `PEM0` writer - PRS Src0 Event Missing Interrupt Flag"]
pub type Pem0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PEM1` reader - PRS Src1 Event Missing Interrupt Flag"]
pub type Pem1R = crate::BitReader;
#[doc = "Field `PEM1` writer - PRS Src1 Event Missing Interrupt Flag"]
pub type Pem1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - WDOG Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn tout(&self) -> ToutR {
        ToutR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDOG Warning Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn warn(&self) -> WarnR {
        WarnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDOG Window Interrupt Flag"]
    #[inline(always)]
    pub fn win(&self) -> WinR {
        WinR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PRS Src0 Event Missing Interrupt Flag"]
    #[inline(always)]
    pub fn pem0(&self) -> Pem0R {
        Pem0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PRS Src1 Event Missing Interrupt Flag"]
    #[inline(always)]
    pub fn pem1(&self) -> Pem1R {
        Pem1R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDOG Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn tout(&mut self) -> ToutW<IfSpec> {
        ToutW::new(self, 0)
    }
    #[doc = "Bit 1 - WDOG Warning Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn warn(&mut self) -> WarnW<IfSpec> {
        WarnW::new(self, 1)
    }
    #[doc = "Bit 2 - WDOG Window Interrupt Flag"]
    #[inline(always)]
    pub fn win(&mut self) -> WinW<IfSpec> {
        WinW::new(self, 2)
    }
    #[doc = "Bit 3 - PRS Src0 Event Missing Interrupt Flag"]
    #[inline(always)]
    pub fn pem0(&mut self) -> Pem0W<IfSpec> {
        Pem0W::new(self, 3)
    }
    #[doc = "Bit 4 - PRS Src1 Event Missing Interrupt Flag"]
    #[inline(always)]
    pub fn pem1(&mut self) -> Pem1W<IfSpec> {
        Pem1W::new(self, 4)
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
