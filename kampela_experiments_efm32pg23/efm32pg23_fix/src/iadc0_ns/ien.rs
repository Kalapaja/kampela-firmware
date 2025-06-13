#[doc = "Register `IEN` reader"]
pub type R = crate::R<IenSpec>;
#[doc = "Register `IEN` writer"]
pub type W = crate::W<IenSpec>;
#[doc = "Field `SINGLEFIFODVL` reader - Single FIFO Data Valid Level Enable"]
pub type SinglefifodvlR = crate::BitReader;
#[doc = "Field `SINGLEFIFODVL` writer - Single FIFO Data Valid Level Enable"]
pub type SinglefifodvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANFIFODVL` reader - Scan FIFO Data Valid Level Enable"]
pub type ScanfifodvlR = crate::BitReader;
#[doc = "Field `SCANFIFODVL` writer - Scan FIFO Data Valid Level Enable"]
pub type ScanfifodvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLECMP` reader - Single Result Window Compare Enable"]
pub type SinglecmpR = crate::BitReader;
#[doc = "Field `SINGLECMP` writer - Single Result Window Compare Enable"]
pub type SinglecmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANCMP` reader - Scan Result Window Compare Enable"]
pub type ScancmpR = crate::BitReader;
#[doc = "Field `SCANCMP` writer - Scan Result Window Compare Enable"]
pub type ScancmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANENTRYDONE` reader - Scan Entry Done Enable"]
pub type ScanentrydoneR = crate::BitReader;
#[doc = "Field `SCANENTRYDONE` writer - Scan Entry Done Enable"]
pub type ScanentrydoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANTABLEDONE` reader - Scan Table Done Enable"]
pub type ScantabledoneR = crate::BitReader;
#[doc = "Field `SCANTABLEDONE` writer - Scan Table Done Enable"]
pub type ScantabledoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEDONE` reader - Single Conversion Done Enable"]
pub type SingledoneR = crate::BitReader;
#[doc = "Field `SINGLEDONE` writer - Single Conversion Done Enable"]
pub type SingledoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLARITYERR` reader - Polarity Error Enable"]
pub type PolarityerrR = crate::BitReader;
#[doc = "Field `POLARITYERR` writer - Polarity Error Enable"]
pub type PolarityerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORTALLOCERR` reader - Port Allocation Error Enable"]
pub type PortallocerrR = crate::BitReader;
#[doc = "Field `PORTALLOCERR` writer - Port Allocation Error Enable"]
pub type PortallocerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEFIFOOF` reader - Single FIFO Overflow Enable"]
pub type SinglefifoofR = crate::BitReader;
#[doc = "Field `SINGLEFIFOOF` writer - Single FIFO Overflow Enable"]
pub type SinglefifoofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANFIFOOF` reader - Scan FIFO Overflow Enable"]
pub type ScanfifoofR = crate::BitReader;
#[doc = "Field `SCANFIFOOF` writer - Scan FIFO Overflow Enable"]
pub type ScanfifoofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SINGLEFIFOUF` reader - Single FIFO Underflow Enable"]
pub type SinglefifoufR = crate::BitReader;
#[doc = "Field `SINGLEFIFOUF` writer - Single FIFO Underflow Enable"]
pub type SinglefifoufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANFIFOUF` reader - Scan FIFO Underflow Enable"]
pub type ScanfifoufR = crate::BitReader;
#[doc = "Field `SCANFIFOUF` writer - Scan FIFO Underflow Enable"]
pub type ScanfifoufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM23ABORTERROR` reader - EM2/3 Abort Error Enable"]
pub type Em23aborterrorR = crate::BitReader;
#[doc = "Field `EM23ABORTERROR` writer - EM2/3 Abort Error Enable"]
pub type Em23aborterrorW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Single FIFO Data Valid Level Enable"]
    #[inline(always)]
    pub fn singlefifodvl(&self) -> SinglefifodvlR {
        SinglefifodvlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan FIFO Data Valid Level Enable"]
    #[inline(always)]
    pub fn scanfifodvl(&self) -> ScanfifodvlR {
        ScanfifodvlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Single Result Window Compare Enable"]
    #[inline(always)]
    pub fn singlecmp(&self) -> SinglecmpR {
        SinglecmpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Scan Result Window Compare Enable"]
    #[inline(always)]
    pub fn scancmp(&self) -> ScancmpR {
        ScancmpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Scan Entry Done Enable"]
    #[inline(always)]
    pub fn scanentrydone(&self) -> ScanentrydoneR {
        ScanentrydoneR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Scan Table Done Enable"]
    #[inline(always)]
    pub fn scantabledone(&self) -> ScantabledoneR {
        ScantabledoneR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Single Conversion Done Enable"]
    #[inline(always)]
    pub fn singledone(&self) -> SingledoneR {
        SingledoneR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Polarity Error Enable"]
    #[inline(always)]
    pub fn polarityerr(&self) -> PolarityerrR {
        PolarityerrR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Allocation Error Enable"]
    #[inline(always)]
    pub fn portallocerr(&self) -> PortallocerrR {
        PortallocerrR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Single FIFO Overflow Enable"]
    #[inline(always)]
    pub fn singlefifoof(&self) -> SinglefifoofR {
        SinglefifoofR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Scan FIFO Overflow Enable"]
    #[inline(always)]
    pub fn scanfifoof(&self) -> ScanfifoofR {
        ScanfifoofR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Single FIFO Underflow Enable"]
    #[inline(always)]
    pub fn singlefifouf(&self) -> SinglefifoufR {
        SinglefifoufR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Scan FIFO Underflow Enable"]
    #[inline(always)]
    pub fn scanfifouf(&self) -> ScanfifoufR {
        ScanfifoufR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - EM2/3 Abort Error Enable"]
    #[inline(always)]
    pub fn em23aborterror(&self) -> Em23aborterrorR {
        Em23aborterrorR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Single FIFO Data Valid Level Enable"]
    #[inline(always)]
    pub fn singlefifodvl(&mut self) -> SinglefifodvlW<IenSpec> {
        SinglefifodvlW::new(self, 0)
    }
    #[doc = "Bit 1 - Scan FIFO Data Valid Level Enable"]
    #[inline(always)]
    pub fn scanfifodvl(&mut self) -> ScanfifodvlW<IenSpec> {
        ScanfifodvlW::new(self, 1)
    }
    #[doc = "Bit 2 - Single Result Window Compare Enable"]
    #[inline(always)]
    pub fn singlecmp(&mut self) -> SinglecmpW<IenSpec> {
        SinglecmpW::new(self, 2)
    }
    #[doc = "Bit 3 - Scan Result Window Compare Enable"]
    #[inline(always)]
    pub fn scancmp(&mut self) -> ScancmpW<IenSpec> {
        ScancmpW::new(self, 3)
    }
    #[doc = "Bit 7 - Scan Entry Done Enable"]
    #[inline(always)]
    pub fn scanentrydone(&mut self) -> ScanentrydoneW<IenSpec> {
        ScanentrydoneW::new(self, 7)
    }
    #[doc = "Bit 8 - Scan Table Done Enable"]
    #[inline(always)]
    pub fn scantabledone(&mut self) -> ScantabledoneW<IenSpec> {
        ScantabledoneW::new(self, 8)
    }
    #[doc = "Bit 9 - Single Conversion Done Enable"]
    #[inline(always)]
    pub fn singledone(&mut self) -> SingledoneW<IenSpec> {
        SingledoneW::new(self, 9)
    }
    #[doc = "Bit 12 - Polarity Error Enable"]
    #[inline(always)]
    pub fn polarityerr(&mut self) -> PolarityerrW<IenSpec> {
        PolarityerrW::new(self, 12)
    }
    #[doc = "Bit 13 - Port Allocation Error Enable"]
    #[inline(always)]
    pub fn portallocerr(&mut self) -> PortallocerrW<IenSpec> {
        PortallocerrW::new(self, 13)
    }
    #[doc = "Bit 16 - Single FIFO Overflow Enable"]
    #[inline(always)]
    pub fn singlefifoof(&mut self) -> SinglefifoofW<IenSpec> {
        SinglefifoofW::new(self, 16)
    }
    #[doc = "Bit 17 - Scan FIFO Overflow Enable"]
    #[inline(always)]
    pub fn scanfifoof(&mut self) -> ScanfifoofW<IenSpec> {
        ScanfifoofW::new(self, 17)
    }
    #[doc = "Bit 18 - Single FIFO Underflow Enable"]
    #[inline(always)]
    pub fn singlefifouf(&mut self) -> SinglefifoufW<IenSpec> {
        SinglefifoufW::new(self, 18)
    }
    #[doc = "Bit 19 - Scan FIFO Underflow Enable"]
    #[inline(always)]
    pub fn scanfifouf(&mut self) -> ScanfifoufW<IenSpec> {
        ScanfifoufW::new(self, 19)
    }
    #[doc = "Bit 31 - EM2/3 Abort Error Enable"]
    #[inline(always)]
    pub fn em23aborterror(&mut self) -> Em23aborterrorW<IenSpec> {
        Em23aborterrorW::new(self, 31)
    }
}
#[doc = "Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
