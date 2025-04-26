#[doc = "Register `DTOGEN` reader"]
pub type R = crate::R<DtogenSpec>;
#[doc = "Register `DTOGEN` writer"]
pub type W = crate::W<DtogenSpec>;
#[doc = "Field `DTOGCC0EN` reader - DTI CCn Output Generation Enable"]
pub type Dtogcc0enR = crate::BitReader;
#[doc = "Field `DTOGCC0EN` writer - DTI CCn Output Generation Enable"]
pub type Dtogcc0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOGCC1EN` reader - DTI CCn Output Generation Enable"]
pub type Dtogcc1enR = crate::BitReader;
#[doc = "Field `DTOGCC1EN` writer - DTI CCn Output Generation Enable"]
pub type Dtogcc1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOGCC2EN` reader - DTI CCn Output Generation Enable"]
pub type Dtogcc2enR = crate::BitReader;
#[doc = "Field `DTOGCC2EN` writer - DTI CCn Output Generation Enable"]
pub type Dtogcc2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOGCDTI0EN` reader - DTI CDTIn Output Generation Enable"]
pub type Dtogcdti0enR = crate::BitReader;
#[doc = "Field `DTOGCDTI0EN` writer - DTI CDTIn Output Generation Enable"]
pub type Dtogcdti0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOGCDTI1EN` reader - DTI CDTIn Output Generation Enable"]
pub type Dtogcdti1enR = crate::BitReader;
#[doc = "Field `DTOGCDTI1EN` writer - DTI CDTIn Output Generation Enable"]
pub type Dtogcdti1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTOGCDTI2EN` reader - DTI CDTIn Output Generation Enable"]
pub type Dtogcdti2enR = crate::BitReader;
#[doc = "Field `DTOGCDTI2EN` writer - DTI CDTIn Output Generation Enable"]
pub type Dtogcdti2enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DTI CCn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc0en(&self) -> Dtogcc0enR {
        Dtogcc0enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTI CCn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc1en(&self) -> Dtogcc1enR {
        Dtogcc1enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DTI CCn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc2en(&self) -> Dtogcc2enR {
        Dtogcc2enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTI CDTIn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti0en(&self) -> Dtogcdti0enR {
        Dtogcdti0enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DTI CDTIn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti1en(&self) -> Dtogcdti1enR {
        Dtogcdti1enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DTI CDTIn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti2en(&self) -> Dtogcdti2enR {
        Dtogcdti2enR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTI CCn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc0en(&mut self) -> Dtogcc0enW<DtogenSpec> {
        Dtogcc0enW::new(self, 0)
    }
    #[doc = "Bit 1 - DTI CCn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc1en(&mut self) -> Dtogcc1enW<DtogenSpec> {
        Dtogcc1enW::new(self, 1)
    }
    #[doc = "Bit 2 - DTI CCn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc2en(&mut self) -> Dtogcc2enW<DtogenSpec> {
        Dtogcc2enW::new(self, 2)
    }
    #[doc = "Bit 3 - DTI CDTIn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti0en(&mut self) -> Dtogcdti0enW<DtogenSpec> {
        Dtogcdti0enW::new(self, 3)
    }
    #[doc = "Bit 4 - DTI CDTIn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti1en(&mut self) -> Dtogcdti1enW<DtogenSpec> {
        Dtogcdti1enW::new(self, 4)
    }
    #[doc = "Bit 5 - DTI CDTIn Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti2en(&mut self) -> Dtogcdti2enW<DtogenSpec> {
        Dtogcdti2enW::new(self, 5)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`dtogen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtogen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtogenSpec;
impl crate::RegisterSpec for DtogenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtogen::R`](R) reader structure"]
impl crate::Readable for DtogenSpec {}
#[doc = "`write(|w| ..)` method takes [`dtogen::W`](W) writer structure"]
impl crate::Writable for DtogenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTOGEN to value 0"]
impl crate::Resettable for DtogenSpec {}
