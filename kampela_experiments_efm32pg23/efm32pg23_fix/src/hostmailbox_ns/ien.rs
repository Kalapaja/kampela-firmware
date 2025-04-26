#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `MBOXIEN0` reader - Mailbox Interrupt Enable"]
pub type Mboxien0R = crate::BitReader;
#[doc = "Field `MBOXIEN0` writer - Mailbox Interrupt Enable"]
pub type Mboxien0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBOXIEN1` reader - Mailbox Interrupt Enable"]
pub type Mboxien1R = crate::BitReader;
#[doc = "Field `MBOXIEN1` writer - Mailbox Interrupt Enable"]
pub type Mboxien1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBOXIEN2` reader - Mailbox Interrupt Enable"]
pub type Mboxien2R = crate::BitReader;
#[doc = "Field `MBOXIEN2` writer - Mailbox Interrupt Enable"]
pub type Mboxien2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBOXIEN3` reader - Mailbox Interrupt Enable"]
pub type Mboxien3R = crate::BitReader;
#[doc = "Field `MBOXIEN3` writer - Mailbox Interrupt Enable"]
pub type Mboxien3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Mailbox Interrupt Enable"]
    #[inline(always)]
    pub fn mboxien0(&self) -> Mboxien0R {
        Mboxien0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mailbox Interrupt Enable"]
    #[inline(always)]
    pub fn mboxien1(&self) -> Mboxien1R {
        Mboxien1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mailbox Interrupt Enable"]
    #[inline(always)]
    pub fn mboxien2(&self) -> Mboxien2R {
        Mboxien2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Mailbox Interrupt Enable"]
    #[inline(always)]
    pub fn mboxien3(&self) -> Mboxien3R {
        Mboxien3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Mailbox Interrupt Enable"]
    #[inline(always)]
    pub fn mboxien0(&mut self) -> Mboxien0W<IenSpec> {
        Mboxien0W::new(self, 0)
    }
    #[doc = "Bit 1 - Mailbox Interrupt Enable"]
    #[inline(always)]
    pub fn mboxien1(&mut self) -> Mboxien1W<IenSpec> {
        Mboxien1W::new(self, 1)
    }
    #[doc = "Bit 2 - Mailbox Interrupt Enable"]
    #[inline(always)]
    pub fn mboxien2(&mut self) -> Mboxien2W<IenSpec> {
        Mboxien2W::new(self, 2)
    }
    #[doc = "Bit 3 - Mailbox Interrupt Enable"]
    #[inline(always)]
    pub fn mboxien3(&mut self) -> Mboxien3W<IenSpec> {
        Mboxien3W::new(self, 3)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IenSpec;
impl crate::RegisterSpec for IenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ien::R`](R) reader structure"]
impl crate::Readable for IenSpec {}
#[doc = "`write(|w| ..)` method takes [`ien::W`](W) writer structure"]
impl crate::Writable for IenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IenSpec {}
