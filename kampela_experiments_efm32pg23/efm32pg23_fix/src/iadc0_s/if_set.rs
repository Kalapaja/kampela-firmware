#[doc = "Register `IF_SET` reader"]
pub type R = crate::R<IfSetSpec>;
#[doc = "Register `IF_SET` writer"]
pub type W = crate::W<IfSetSpec>;
#[doc = "Field `SINGLEFIFODVL` reader - Single FIFO Data Valid Level"]
pub type SinglefifodvlR = crate::BitReader;
#[doc = "Field `SINGLEFIFODVL` writer - Single FIFO Data Valid Level"]
pub type SinglefifodvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANFIFODVL` reader - Scan FIFO Data Valid Level"]
pub type ScanfifodvlR = crate::BitReader;
#[doc = "Field `SCANFIFODVL` writer - Scan FIFO Data Valid Level"]
pub type ScanfifodvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLECMP` reader - Single Result Window Compare"]
pub type SinglecmpR = crate::BitReader;
#[doc = "Field `SINGLECMP` writer - Single Result Window Compare"]
pub type SinglecmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANCMP` reader - Scan Result Window Compare"]
pub type ScancmpR = crate::BitReader;
#[doc = "Field `SCANCMP` writer - Scan Result Window Compare"]
pub type ScancmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANENTRYDONE` reader - Scan Entry Done"]
pub type ScanentrydoneR = crate::BitReader;
#[doc = "Field `SCANENTRYDONE` writer - Scan Entry Done"]
pub type ScanentrydoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANTABLEDONE` reader - Scan Table Done"]
pub type ScantabledoneR = crate::BitReader;
#[doc = "Field `SCANTABLEDONE` writer - Scan Table Done"]
pub type ScantabledoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEDONE` reader - Single Conversion Done"]
pub type SingledoneR = crate::BitReader;
#[doc = "Field `SINGLEDONE` writer - Single Conversion Done"]
pub type SingledoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLARITYERR` reader - Polarity Error"]
pub type PolarityerrR = crate::BitReader;
#[doc = "Field `POLARITYERR` writer - Polarity Error"]
pub type PolarityerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTALLOCERR` reader - Port Allocation Error"]
pub type PortallocerrR = crate::BitReader;
#[doc = "Field `PORTALLOCERR` writer - Port Allocation Error"]
pub type PortallocerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEFIFOOF` reader - Single FIFO Overflow"]
pub type SinglefifoofR = crate::BitReader;
#[doc = "Field `SINGLEFIFOOF` writer - Single FIFO Overflow"]
pub type SinglefifoofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANFIFOOF` reader - Scan FIFO Overflow"]
pub type ScanfifoofR = crate::BitReader;
#[doc = "Field `SCANFIFOOF` writer - Scan FIFO Overflow"]
pub type ScanfifoofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEFIFOUF` reader - Single FIFO Underflow"]
pub type SinglefifoufR = crate::BitReader;
#[doc = "Field `SINGLEFIFOUF` writer - Single FIFO Underflow"]
pub type SinglefifoufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANFIFOUF` reader - Scan FIFO Underflow"]
pub type ScanfifoufR = crate::BitReader;
#[doc = "Field `SCANFIFOUF` writer - Scan FIFO Underflow"]
pub type ScanfifoufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23ABORTERROR` reader - EM2/3 Abort Error"]
pub type Em23aborterrorR = crate::BitReader;
#[doc = "Field `EM23ABORTERROR` writer - EM2/3 Abort Error"]
pub type Em23aborterrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Single FIFO Data Valid Level"]
    #[inline(always)]
    pub fn singlefifodvl(&self) -> SinglefifodvlR {
        SinglefifodvlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan FIFO Data Valid Level"]
    #[inline(always)]
    pub fn scanfifodvl(&self) -> ScanfifodvlR {
        ScanfifodvlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Single Result Window Compare"]
    #[inline(always)]
    pub fn singlecmp(&self) -> SinglecmpR {
        SinglecmpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Scan Result Window Compare"]
    #[inline(always)]
    pub fn scancmp(&self) -> ScancmpR {
        ScancmpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Scan Entry Done"]
    #[inline(always)]
    pub fn scanentrydone(&self) -> ScanentrydoneR {
        ScanentrydoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan Table Done"]
    #[inline(always)]
    pub fn scantabledone(&self) -> ScantabledoneR {
        ScantabledoneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Single Conversion Done"]
    #[inline(always)]
    pub fn singledone(&self) -> SingledoneR {
        SingledoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Polarity Error"]
    #[inline(always)]
    pub fn polarityerr(&self) -> PolarityerrR {
        PolarityerrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Allocation Error"]
    #[inline(always)]
    pub fn portallocerr(&self) -> PortallocerrR {
        PortallocerrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Single FIFO Overflow"]
    #[inline(always)]
    pub fn singlefifoof(&self) -> SinglefifoofR {
        SinglefifoofR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Scan FIFO Overflow"]
    #[inline(always)]
    pub fn scanfifoof(&self) -> ScanfifoofR {
        ScanfifoofR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Single FIFO Underflow"]
    #[inline(always)]
    pub fn singlefifouf(&self) -> SinglefifoufR {
        SinglefifoufR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Scan FIFO Underflow"]
    #[inline(always)]
    pub fn scanfifouf(&self) -> ScanfifoufR {
        ScanfifoufR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - EM2/3 Abort Error"]
    #[inline(always)]
    pub fn em23aborterror(&self) -> Em23aborterrorR {
        Em23aborterrorR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single FIFO Data Valid Level"]
    #[inline(always)]
    pub fn singlefifodvl(&mut self) -> SinglefifodvlW<IfSetSpec> {
        SinglefifodvlW::new(self, 0)
    }
    #[doc = "Bit 1 - Scan FIFO Data Valid Level"]
    #[inline(always)]
    pub fn scanfifodvl(&mut self) -> ScanfifodvlW<IfSetSpec> {
        ScanfifodvlW::new(self, 1)
    }
    #[doc = "Bit 2 - Single Result Window Compare"]
    #[inline(always)]
    pub fn singlecmp(&mut self) -> SinglecmpW<IfSetSpec> {
        SinglecmpW::new(self, 2)
    }
    #[doc = "Bit 3 - Scan Result Window Compare"]
    #[inline(always)]
    pub fn scancmp(&mut self) -> ScancmpW<IfSetSpec> {
        ScancmpW::new(self, 3)
    }
    #[doc = "Bit 7 - Scan Entry Done"]
    #[inline(always)]
    pub fn scanentrydone(&mut self) -> ScanentrydoneW<IfSetSpec> {
        ScanentrydoneW::new(self, 7)
    }
    #[doc = "Bit 8 - Scan Table Done"]
    #[inline(always)]
    pub fn scantabledone(&mut self) -> ScantabledoneW<IfSetSpec> {
        ScantabledoneW::new(self, 8)
    }
    #[doc = "Bit 9 - Single Conversion Done"]
    #[inline(always)]
    pub fn singledone(&mut self) -> SingledoneW<IfSetSpec> {
        SingledoneW::new(self, 9)
    }
    #[doc = "Bit 12 - Polarity Error"]
    #[inline(always)]
    pub fn polarityerr(&mut self) -> PolarityerrW<IfSetSpec> {
        PolarityerrW::new(self, 12)
    }
    #[doc = "Bit 13 - Port Allocation Error"]
    #[inline(always)]
    pub fn portallocerr(&mut self) -> PortallocerrW<IfSetSpec> {
        PortallocerrW::new(self, 13)
    }
    #[doc = "Bit 16 - Single FIFO Overflow"]
    #[inline(always)]
    pub fn singlefifoof(&mut self) -> SinglefifoofW<IfSetSpec> {
        SinglefifoofW::new(self, 16)
    }
    #[doc = "Bit 17 - Scan FIFO Overflow"]
    #[inline(always)]
    pub fn scanfifoof(&mut self) -> ScanfifoofW<IfSetSpec> {
        ScanfifoofW::new(self, 17)
    }
    #[doc = "Bit 18 - Single FIFO Underflow"]
    #[inline(always)]
    pub fn singlefifouf(&mut self) -> SinglefifoufW<IfSetSpec> {
        SinglefifoufW::new(self, 18)
    }
    #[doc = "Bit 19 - Scan FIFO Underflow"]
    #[inline(always)]
    pub fn scanfifouf(&mut self) -> ScanfifoufW<IfSetSpec> {
        ScanfifoufW::new(self, 19)
    }
    #[doc = "Bit 31 - EM2/3 Abort Error"]
    #[inline(always)]
    pub fn em23aborterror(&mut self) -> Em23aborterrorW<IfSetSpec> {
        Em23aborterrorW::new(self, 31)
    }
}
#[doc = "Interrupt Flag\n\nYou can [`read`](crate::Reg::read) this register and get [`if_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`if_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfSetSpec;
impl crate::RegisterSpec for IfSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`if_set::R`](R) reader structure"]
impl crate::Readable for IfSetSpec {}
#[doc = "`write(|w| ..)` method takes [`if_set::W`](W) writer structure"]
impl crate::Writable for IfSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IF_SET to value 0"]
impl crate::Resettable for IfSetSpec {}
