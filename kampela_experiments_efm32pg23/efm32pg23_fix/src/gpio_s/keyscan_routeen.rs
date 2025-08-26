#[doc = "Register `KEYSCAN_ROUTEEN` reader"]
pub type R = crate::R<KeyscanRouteenSpec>;
#[doc = "Register `KEYSCAN_ROUTEEN` writer"]
pub type W = crate::W<KeyscanRouteenSpec>;
#[doc = "Field `COLOUT0PEN` reader - COLOUT0 pin enable control bit"]
pub type Colout0penR = crate::BitReader;
#[doc = "Field `COLOUT0PEN` writer - COLOUT0 pin enable control bit"]
pub type Colout0penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLOUT1PEN` reader - COLOUT1 pin enable control bit"]
pub type Colout1penR = crate::BitReader;
#[doc = "Field `COLOUT1PEN` writer - COLOUT1 pin enable control bit"]
pub type Colout1penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLOUT2PEN` reader - COLOUT2 pin enable control bit"]
pub type Colout2penR = crate::BitReader;
#[doc = "Field `COLOUT2PEN` writer - COLOUT2 pin enable control bit"]
pub type Colout2penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLOUT3PEN` reader - COLOUT3 pin enable control bit"]
pub type Colout3penR = crate::BitReader;
#[doc = "Field `COLOUT3PEN` writer - COLOUT3 pin enable control bit"]
pub type Colout3penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLOUT4PEN` reader - COLOUT4 pin enable control bit"]
pub type Colout4penR = crate::BitReader;
#[doc = "Field `COLOUT4PEN` writer - COLOUT4 pin enable control bit"]
pub type Colout4penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLOUT5PEN` reader - COLOUT5 pin enable control bit"]
pub type Colout5penR = crate::BitReader;
#[doc = "Field `COLOUT5PEN` writer - COLOUT5 pin enable control bit"]
pub type Colout5penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLOUT6PEN` reader - COLOUT6 pin enable control bit"]
pub type Colout6penR = crate::BitReader;
#[doc = "Field `COLOUT6PEN` writer - COLOUT6 pin enable control bit"]
pub type Colout6penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLOUT7PEN` reader - COLOUT7 pin enable control bit"]
pub type Colout7penR = crate::BitReader;
#[doc = "Field `COLOUT7PEN` writer - COLOUT7 pin enable control bit"]
pub type Colout7penW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - COLOUT0 pin enable control bit"]
    #[inline(always)]
    pub fn colout0pen(&self) -> Colout0penR {
        Colout0penR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - COLOUT1 pin enable control bit"]
    #[inline(always)]
    pub fn colout1pen(&self) -> Colout1penR {
        Colout1penR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - COLOUT2 pin enable control bit"]
    #[inline(always)]
    pub fn colout2pen(&self) -> Colout2penR {
        Colout2penR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COLOUT3 pin enable control bit"]
    #[inline(always)]
    pub fn colout3pen(&self) -> Colout3penR {
        Colout3penR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - COLOUT4 pin enable control bit"]
    #[inline(always)]
    pub fn colout4pen(&self) -> Colout4penR {
        Colout4penR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COLOUT5 pin enable control bit"]
    #[inline(always)]
    pub fn colout5pen(&self) -> Colout5penR {
        Colout5penR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - COLOUT6 pin enable control bit"]
    #[inline(always)]
    pub fn colout6pen(&self) -> Colout6penR {
        Colout6penR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - COLOUT7 pin enable control bit"]
    #[inline(always)]
    pub fn colout7pen(&self) -> Colout7penR {
        Colout7penR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COLOUT0 pin enable control bit"]
    #[inline(always)]
    pub fn colout0pen(&mut self) -> Colout0penW<KeyscanRouteenSpec> {
        Colout0penW::new(self, 0)
    }
    #[doc = "Bit 1 - COLOUT1 pin enable control bit"]
    #[inline(always)]
    pub fn colout1pen(&mut self) -> Colout1penW<KeyscanRouteenSpec> {
        Colout1penW::new(self, 1)
    }
    #[doc = "Bit 2 - COLOUT2 pin enable control bit"]
    #[inline(always)]
    pub fn colout2pen(&mut self) -> Colout2penW<KeyscanRouteenSpec> {
        Colout2penW::new(self, 2)
    }
    #[doc = "Bit 3 - COLOUT3 pin enable control bit"]
    #[inline(always)]
    pub fn colout3pen(&mut self) -> Colout3penW<KeyscanRouteenSpec> {
        Colout3penW::new(self, 3)
    }
    #[doc = "Bit 4 - COLOUT4 pin enable control bit"]
    #[inline(always)]
    pub fn colout4pen(&mut self) -> Colout4penW<KeyscanRouteenSpec> {
        Colout4penW::new(self, 4)
    }
    #[doc = "Bit 5 - COLOUT5 pin enable control bit"]
    #[inline(always)]
    pub fn colout5pen(&mut self) -> Colout5penW<KeyscanRouteenSpec> {
        Colout5penW::new(self, 5)
    }
    #[doc = "Bit 6 - COLOUT6 pin enable control bit"]
    #[inline(always)]
    pub fn colout6pen(&mut self) -> Colout6penW<KeyscanRouteenSpec> {
        Colout6penW::new(self, 6)
    }
    #[doc = "Bit 7 - COLOUT7 pin enable control bit"]
    #[inline(always)]
    pub fn colout7pen(&mut self) -> Colout7penW<KeyscanRouteenSpec> {
        Colout7penW::new(self, 7)
    }
}
#[doc = "KEYSCAN pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`keyscan_routeen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyscan_routeen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KeyscanRouteenSpec;
impl crate::RegisterSpec for KeyscanRouteenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyscan_routeen::R`](R) reader structure"]
impl crate::Readable for KeyscanRouteenSpec {}
#[doc = "`write(|w| ..)` method takes [`keyscan_routeen::W`](W) writer structure"]
impl crate::Writable for KeyscanRouteenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYSCAN_ROUTEEN to value 0"]
impl crate::Resettable for KeyscanRouteenSpec {}
