#[doc = "Register `SINGLEFIFOCFG` reader"]
pub type R = crate::R<SinglefifocfgSpec>;
#[doc = "Register `SINGLEFIFOCFG` writer"]
pub type W = crate::W<SinglefifocfgSpec>;
#[doc = "Alignment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Alignment {
    #[doc = "0: ID\\[7:0\\], SIGN_EXT, DATA\\[11:0\\]"]
    Right12 = 0,
    #[doc = "1: ID\\[7:0\\], SIGN_EXT, DATA\\[15:0\\]"]
    Right16 = 1,
    #[doc = "2: ID\\[7:0\\], SIGN_EXT, DATA\\[19:0\\]"]
    Right20 = 2,
    #[doc = "3: DATA\\[11:0\\], 000000000000, ID\\[7:0\\]"]
    Left12 = 3,
    #[doc = "4: DATA\\[15:0\\], 00000000, ID\\[7:0\\]"]
    Left16 = 4,
    #[doc = "5: DATA\\[19:0\\], 0000, ID\\[7:0\\]"]
    Left20 = 5,
}
impl From<Alignment> for u8 {
    #[inline(always)]
    fn from(variant: Alignment) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Alignment {
    type Ux = u8;
}
impl crate::IsEnum for Alignment {}
#[doc = "Field `ALIGNMENT` reader - Alignment"]
pub type AlignmentR = crate::FieldReader<Alignment>;
impl AlignmentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Alignment> {
        match self.bits {
            0 => Some(Alignment::Right12),
            1 => Some(Alignment::Right16),
            2 => Some(Alignment::Right20),
            3 => Some(Alignment::Left12),
            4 => Some(Alignment::Left16),
            5 => Some(Alignment::Left20),
            _ => None,
        }
    }
    #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[11:0\\]"]
    #[inline(always)]
    pub fn is_right12(&self) -> bool {
        *self == Alignment::Right12
    }
    #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[15:0\\]"]
    #[inline(always)]
    pub fn is_right16(&self) -> bool {
        *self == Alignment::Right16
    }
    #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[19:0\\]"]
    #[inline(always)]
    pub fn is_right20(&self) -> bool {
        *self == Alignment::Right20
    }
    #[doc = "DATA\\[11:0\\], 000000000000, ID\\[7:0\\]"]
    #[inline(always)]
    pub fn is_left12(&self) -> bool {
        *self == Alignment::Left12
    }
    #[doc = "DATA\\[15:0\\], 00000000, ID\\[7:0\\]"]
    #[inline(always)]
    pub fn is_left16(&self) -> bool {
        *self == Alignment::Left16
    }
    #[doc = "DATA\\[19:0\\], 0000, ID\\[7:0\\]"]
    #[inline(always)]
    pub fn is_left20(&self) -> bool {
        *self == Alignment::Left20
    }
}
#[doc = "Field `ALIGNMENT` writer - Alignment"]
pub type AlignmentW<'a, REG> = crate::FieldWriter<'a, REG, 3, Alignment>;
impl<'a, REG> AlignmentW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[11:0\\]"]
    #[inline(always)]
    pub fn right12(self) -> &'a mut crate::W<REG> {
        self.variant(Alignment::Right12)
    }
    #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[15:0\\]"]
    #[inline(always)]
    pub fn right16(self) -> &'a mut crate::W<REG> {
        self.variant(Alignment::Right16)
    }
    #[doc = "ID\\[7:0\\], SIGN_EXT, DATA\\[19:0\\]"]
    #[inline(always)]
    pub fn right20(self) -> &'a mut crate::W<REG> {
        self.variant(Alignment::Right20)
    }
    #[doc = "DATA\\[11:0\\], 000000000000, ID\\[7:0\\]"]
    #[inline(always)]
    pub fn left12(self) -> &'a mut crate::W<REG> {
        self.variant(Alignment::Left12)
    }
    #[doc = "DATA\\[15:0\\], 00000000, ID\\[7:0\\]"]
    #[inline(always)]
    pub fn left16(self) -> &'a mut crate::W<REG> {
        self.variant(Alignment::Left16)
    }
    #[doc = "DATA\\[19:0\\], 0000, ID\\[7:0\\]"]
    #[inline(always)]
    pub fn left20(self) -> &'a mut crate::W<REG> {
        self.variant(Alignment::Left20)
    }
}
#[doc = "Field `SHOWID` reader - Show ID"]
pub type ShowidR = crate::BitReader;
#[doc = "Field `SHOWID` writer - Show ID"]
pub type ShowidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Data Valid Level\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dvl {
    #[doc = "0: When 1 entry in the single FIFO is valid, set the SINGLEFIFODVL interrupt and request DMA."]
    Valid1 = 0,
    #[doc = "1: When 2 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    Valid2 = 1,
    #[doc = "2: When 3 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    Valid3 = 2,
    #[doc = "3: When 4 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    Valid4 = 3,
    #[doc = "4: When 5 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    Valid5 = 4,
    #[doc = "5: When 6 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    Valid6 = 5,
    #[doc = "6: When 7 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    Valid7 = 6,
    #[doc = "7: When 8 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    Valid8 = 7,
}
impl From<Dvl> for u8 {
    #[inline(always)]
    fn from(variant: Dvl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dvl {
    type Ux = u8;
}
impl crate::IsEnum for Dvl {}
#[doc = "Field `DVL` reader - Data Valid Level"]
pub type DvlR = crate::FieldReader<Dvl>;
impl DvlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dvl {
        match self.bits {
            0 => Dvl::Valid1,
            1 => Dvl::Valid2,
            2 => Dvl::Valid3,
            3 => Dvl::Valid4,
            4 => Dvl::Valid5,
            5 => Dvl::Valid6,
            6 => Dvl::Valid7,
            7 => Dvl::Valid8,
            _ => unreachable!(),
        }
    }
    #[doc = "When 1 entry in the single FIFO is valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn is_valid1(&self) -> bool {
        *self == Dvl::Valid1
    }
    #[doc = "When 2 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn is_valid2(&self) -> bool {
        *self == Dvl::Valid2
    }
    #[doc = "When 3 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn is_valid3(&self) -> bool {
        *self == Dvl::Valid3
    }
    #[doc = "When 4 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn is_valid4(&self) -> bool {
        *self == Dvl::Valid4
    }
    #[doc = "When 5 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn is_valid5(&self) -> bool {
        *self == Dvl::Valid5
    }
    #[doc = "When 6 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn is_valid6(&self) -> bool {
        *self == Dvl::Valid6
    }
    #[doc = "When 7 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn is_valid7(&self) -> bool {
        *self == Dvl::Valid7
    }
    #[doc = "When 8 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn is_valid8(&self) -> bool {
        *self == Dvl::Valid8
    }
}
#[doc = "Field `DVL` writer - Data Valid Level"]
pub type DvlW<'a, REG> = crate::FieldWriter<'a, REG, 3, Dvl, crate::Safe>;
impl<'a, REG> DvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "When 1 entry in the single FIFO is valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid1(self) -> &'a mut crate::W<REG> {
        self.variant(Dvl::Valid1)
    }
    #[doc = "When 2 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid2(self) -> &'a mut crate::W<REG> {
        self.variant(Dvl::Valid2)
    }
    #[doc = "When 3 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid3(self) -> &'a mut crate::W<REG> {
        self.variant(Dvl::Valid3)
    }
    #[doc = "When 4 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid4(self) -> &'a mut crate::W<REG> {
        self.variant(Dvl::Valid4)
    }
    #[doc = "When 5 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid5(self) -> &'a mut crate::W<REG> {
        self.variant(Dvl::Valid5)
    }
    #[doc = "When 6 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid6(self) -> &'a mut crate::W<REG> {
        self.variant(Dvl::Valid6)
    }
    #[doc = "When 7 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid7(self) -> &'a mut crate::W<REG> {
        self.variant(Dvl::Valid7)
    }
    #[doc = "When 8 entries in the single FIFO are valid, set the SINGLEFIFODVL interrupt and request DMA."]
    #[inline(always)]
    pub fn valid8(self) -> &'a mut crate::W<REG> {
        self.variant(Dvl::Valid8)
    }
}
#[doc = "Single FIFO DMA wakeup.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmawufifosingle {
    #[doc = "0: While in EM2 or EM3, the DMA controller will not be requested."]
    Disabled = 0,
    #[doc = "1: While in EM2 or EM3, the DMA controller will be requested when the single FIFO reaches its Data Valid Level. \\[DVL must be set to 0 (VALID1).\\]"]
    Enabled = 1,
}
impl From<Dmawufifosingle> for bool {
    #[inline(always)]
    fn from(variant: Dmawufifosingle) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAWUFIFOSINGLE` reader - Single FIFO DMA wakeup."]
pub type DmawufifosingleR = crate::BitReader<Dmawufifosingle>;
impl DmawufifosingleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmawufifosingle {
        match self.bits {
            false => Dmawufifosingle::Disabled,
            true => Dmawufifosingle::Enabled,
        }
    }
    #[doc = "While in EM2 or EM3, the DMA controller will not be requested."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dmawufifosingle::Disabled
    }
    #[doc = "While in EM2 or EM3, the DMA controller will be requested when the single FIFO reaches its Data Valid Level. \\[DVL must be set to 0 (VALID1).\\]"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dmawufifosingle::Enabled
    }
}
#[doc = "Field `DMAWUFIFOSINGLE` writer - Single FIFO DMA wakeup."]
pub type DmawufifosingleW<'a, REG> = crate::BitWriter<'a, REG, Dmawufifosingle>;
impl<'a, REG> DmawufifosingleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "While in EM2 or EM3, the DMA controller will not be requested."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmawufifosingle::Disabled)
    }
    #[doc = "While in EM2 or EM3, the DMA controller will be requested when the single FIFO reaches its Data Valid Level. \\[DVL must be set to 0 (VALID1).\\]"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dmawufifosingle::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:2 - Alignment"]
    #[inline(always)]
    pub fn alignment(&self) -> AlignmentR {
        AlignmentR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Show ID"]
    #[inline(always)]
    pub fn showid(&self) -> ShowidR {
        ShowidR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Data Valid Level"]
    #[inline(always)]
    pub fn dvl(&self) -> DvlR {
        DvlR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Single FIFO DMA wakeup."]
    #[inline(always)]
    pub fn dmawufifosingle(&self) -> DmawufifosingleR {
        DmawufifosingleR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Alignment"]
    #[inline(always)]
    pub fn alignment(&mut self) -> AlignmentW<SinglefifocfgSpec> {
        AlignmentW::new(self, 0)
    }
    #[doc = "Bit 3 - Show ID"]
    #[inline(always)]
    pub fn showid(&mut self) -> ShowidW<SinglefifocfgSpec> {
        ShowidW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Data Valid Level"]
    #[inline(always)]
    pub fn dvl(&mut self) -> DvlW<SinglefifocfgSpec> {
        DvlW::new(self, 4)
    }
    #[doc = "Bit 8 - Single FIFO DMA wakeup."]
    #[inline(always)]
    pub fn dmawufifosingle(&mut self) -> DmawufifosingleW<SinglefifocfgSpec> {
        DmawufifosingleW::new(self, 8)
    }
}
#[doc = "Single FIFO Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`singlefifocfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlefifocfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SinglefifocfgSpec;
impl crate::RegisterSpec for SinglefifocfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`singlefifocfg::R`](R) reader structure"]
impl crate::Readable for SinglefifocfgSpec {}
#[doc = "`write(|w| ..)` method takes [`singlefifocfg::W`](W) writer structure"]
impl crate::Writable for SinglefifocfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SINGLEFIFOCFG to value 0x30"]
impl crate::Resettable for SinglefifocfgSpec {
    const RESET_VALUE: u32 = 0x30;
}
