#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `MBOXIF0` reader - Mailbox Interupt Flag"]
pub type Mboxif0R = crate::BitReader;
#[doc = "Field `MBOXIF0` writer - Mailbox Interupt Flag"]
pub type Mboxif0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBOXIF1` reader - Mailbox Interupt Flag"]
pub type Mboxif1R = crate::BitReader;
#[doc = "Field `MBOXIF1` writer - Mailbox Interupt Flag"]
pub type Mboxif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBOXIF2` reader - Mailbox Interupt Flag"]
pub type Mboxif2R = crate::BitReader;
#[doc = "Field `MBOXIF2` writer - Mailbox Interupt Flag"]
pub type Mboxif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBOXIF3` reader - Mailbox Interupt Flag"]
pub type Mboxif3R = crate::BitReader;
#[doc = "Field `MBOXIF3` writer - Mailbox Interupt Flag"]
pub type Mboxif3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mailbox Interupt Flag"]
    #[inline(always)]
    pub fn mboxif0(&self) -> Mboxif0R {
        Mboxif0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox Interupt Flag"]
    #[inline(always)]
    pub fn mboxif1(&self) -> Mboxif1R {
        Mboxif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mailbox Interupt Flag"]
    #[inline(always)]
    pub fn mboxif2(&self) -> Mboxif2R {
        Mboxif2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mailbox Interupt Flag"]
    #[inline(always)]
    pub fn mboxif3(&self) -> Mboxif3R {
        Mboxif3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mailbox Interupt Flag"]
    #[inline(always)]
    pub fn mboxif0(&mut self) -> Mboxif0W<IfSpec> {
        Mboxif0W::new(self, 0)
    }
    #[doc = "Bit 1 - Mailbox Interupt Flag"]
    #[inline(always)]
    pub fn mboxif1(&mut self) -> Mboxif1W<IfSpec> {
        Mboxif1W::new(self, 1)
    }
    #[doc = "Bit 2 - Mailbox Interupt Flag"]
    #[inline(always)]
    pub fn mboxif2(&mut self) -> Mboxif2W<IfSpec> {
        Mboxif2W::new(self, 2)
    }
    #[doc = "Bit 3 - Mailbox Interupt Flag"]
    #[inline(always)]
    pub fn mboxif3(&mut self) -> Mboxif3W<IfSpec> {
        Mboxif3W::new(self, 3)
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
