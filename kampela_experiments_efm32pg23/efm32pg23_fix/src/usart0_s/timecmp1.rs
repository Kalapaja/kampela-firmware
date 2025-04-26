#[doc = "Register `TIMECMP1` reader"]
pub type R = crate::R<Timecmp1Spec>;
#[doc = "Register `TIMECMP1` writer"]
pub type W = crate::W<Timecmp1Spec>;
#[doc = "Field `TCMPVAL` reader - Timer comparator 1."]
pub type TcmpvalR = crate::FieldReader;
#[doc = "Field `TCMPVAL` writer - Timer comparator 1."]
pub type TcmpvalW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Timer start source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tstart {
    #[doc = "0: Comparator 1 is disabled"]
    Disable = 0,
    #[doc = "1: Comparator 1 and timer are started at TX end of frame"]
    Txeof = 1,
    #[doc = "2: Comparator 1 and timer are started at TX Complete"]
    Txc = 2,
    #[doc = "3: Comparator 1 and timer are started at RX going going Active (default: low)"]
    Rxact = 3,
    #[doc = "4: Comparator 1 and timer are started at RX end of frame"]
    Rxeof = 4,
}
impl From<Tstart> for u8 {
    #[inline(always)]
    fn from(variant: Tstart) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tstart {
    type Ux = u8;
}
impl crate::IsEnum for Tstart {}
#[doc = "Field `TSTART` reader - Timer start source"]
pub type TstartR = crate::FieldReader<Tstart>;
impl TstartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tstart> {
        match self.bits {
            0 => Some(Tstart::Disable),
            1 => Some(Tstart::Txeof),
            2 => Some(Tstart::Txc),
            3 => Some(Tstart::Rxact),
            4 => Some(Tstart::Rxeof),
            _ => None,
        }
    }
    #[doc = "Comparator 1 is disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Tstart::Disable
    }
    #[doc = "Comparator 1 and timer are started at TX end of frame"]
    #[inline(always)]
    pub fn is_txeof(&self) -> bool {
        *self == Tstart::Txeof
    }
    #[doc = "Comparator 1 and timer are started at TX Complete"]
    #[inline(always)]
    pub fn is_txc(&self) -> bool {
        *self == Tstart::Txc
    }
    #[doc = "Comparator 1 and timer are started at RX going going Active (default: low)"]
    #[inline(always)]
    pub fn is_rxact(&self) -> bool {
        *self == Tstart::Rxact
    }
    #[doc = "Comparator 1 and timer are started at RX end of frame"]
    #[inline(always)]
    pub fn is_rxeof(&self) -> bool {
        *self == Tstart::Rxeof
    }
}
#[doc = "Field `TSTART` writer - Timer start source"]
pub type TstartW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tstart>;
impl<'a, REG> TstartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comparator 1 is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::Disable)
    }
    #[doc = "Comparator 1 and timer are started at TX end of frame"]
    #[inline(always)]
    pub fn txeof(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::Txeof)
    }
    #[doc = "Comparator 1 and timer are started at TX Complete"]
    #[inline(always)]
    pub fn txc(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::Txc)
    }
    #[doc = "Comparator 1 and timer are started at RX going going Active (default: low)"]
    #[inline(always)]
    pub fn rxact(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::Rxact)
    }
    #[doc = "Comparator 1 and timer are started at RX end of frame"]
    #[inline(always)]
    pub fn rxeof(self) -> &'a mut crate::W<REG> {
        self.variant(Tstart::Rxeof)
    }
}
#[doc = "Source used to disable comparator 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Tstop {
    #[doc = "0: Comparator 1 is disabled when the counter equals TCMPVAL and triggers a TCMP1 event"]
    Tcmp1 = 0,
    #[doc = "1: Comparator 1 is disabled at TX start TX Engine"]
    Txst = 1,
    #[doc = "2: Comparator 1 is disabled on RX going going Active (default: low)"]
    Rxact = 2,
    #[doc = "3: Comparator 1 is disabled on RX going Inactive"]
    Rxactn = 3,
}
impl From<Tstop> for u8 {
    #[inline(always)]
    fn from(variant: Tstop) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Tstop {
    type Ux = u8;
}
impl crate::IsEnum for Tstop {}
#[doc = "Field `TSTOP` reader - Source used to disable comparator 1"]
pub type TstopR = crate::FieldReader<Tstop>;
impl TstopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Tstop> {
        match self.bits {
            0 => Some(Tstop::Tcmp1),
            1 => Some(Tstop::Txst),
            2 => Some(Tstop::Rxact),
            3 => Some(Tstop::Rxactn),
            _ => None,
        }
    }
    #[doc = "Comparator 1 is disabled when the counter equals TCMPVAL and triggers a TCMP1 event"]
    #[inline(always)]
    pub fn is_tcmp1(&self) -> bool {
        *self == Tstop::Tcmp1
    }
    #[doc = "Comparator 1 is disabled at TX start TX Engine"]
    #[inline(always)]
    pub fn is_txst(&self) -> bool {
        *self == Tstop::Txst
    }
    #[doc = "Comparator 1 is disabled on RX going going Active (default: low)"]
    #[inline(always)]
    pub fn is_rxact(&self) -> bool {
        *self == Tstop::Rxact
    }
    #[doc = "Comparator 1 is disabled on RX going Inactive"]
    #[inline(always)]
    pub fn is_rxactn(&self) -> bool {
        *self == Tstop::Rxactn
    }
}
#[doc = "Field `TSTOP` writer - Source used to disable comparator 1"]
pub type TstopW<'a, REG> = crate::FieldWriter<'a, REG, 3, Tstop>;
impl<'a, REG> TstopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Comparator 1 is disabled when the counter equals TCMPVAL and triggers a TCMP1 event"]
    #[inline(always)]
    pub fn tcmp1(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::Tcmp1)
    }
    #[doc = "Comparator 1 is disabled at TX start TX Engine"]
    #[inline(always)]
    pub fn txst(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::Txst)
    }
    #[doc = "Comparator 1 is disabled on RX going going Active (default: low)"]
    #[inline(always)]
    pub fn rxact(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::Rxact)
    }
    #[doc = "Comparator 1 is disabled on RX going Inactive"]
    #[inline(always)]
    pub fn rxactn(self) -> &'a mut crate::W<REG> {
        self.variant(Tstop::Rxactn)
    }
}
#[doc = "Restart Timer on TCMP1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Restarten {
    #[doc = "0: Disable the timer restarting on TCMP1"]
    Disable = 0,
    #[doc = "1: Enable the timer restarting on TCMP1"]
    Enable = 1,
}
impl From<Restarten> for bool {
    #[inline(always)]
    fn from(variant: Restarten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RESTARTEN` reader - Restart Timer on TCMP1"]
pub type RestartenR = crate::BitReader<Restarten>;
impl RestartenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Restarten {
        match self.bits {
            false => Restarten::Disable,
            true => Restarten::Enable,
        }
    }
    #[doc = "Disable the timer restarting on TCMP1"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Restarten::Disable
    }
    #[doc = "Enable the timer restarting on TCMP1"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Restarten::Enable
    }
}
#[doc = "Field `RESTARTEN` writer - Restart Timer on TCMP1"]
pub type RestartenW<'a, REG> = crate::BitWriter<'a, REG, Restarten>;
impl<'a, REG> RestartenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the timer restarting on TCMP1"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Restarten::Disable)
    }
    #[doc = "Enable the timer restarting on TCMP1"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Restarten::Enable)
    }
}
impl R {
    #[doc = "Bits 0:7 - Timer comparator 1."]
    #[inline(always)]
    pub fn tcmpval(&self) -> TcmpvalR {
        TcmpvalR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - Timer start source"]
    #[inline(always)]
    pub fn tstart(&self) -> TstartR {
        TstartR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Source used to disable comparator 1"]
    #[inline(always)]
    pub fn tstop(&self) -> TstopR {
        TstopR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - Restart Timer on TCMP1"]
    #[inline(always)]
    pub fn restarten(&self) -> RestartenR {
        RestartenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer comparator 1."]
    #[inline(always)]
    pub fn tcmpval(&mut self) -> TcmpvalW<Timecmp1Spec> {
        TcmpvalW::new(self, 0)
    }
    #[doc = "Bits 16:18 - Timer start source"]
    #[inline(always)]
    pub fn tstart(&mut self) -> TstartW<Timecmp1Spec> {
        TstartW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Source used to disable comparator 1"]
    #[inline(always)]
    pub fn tstop(&mut self) -> TstopW<Timecmp1Spec> {
        TstopW::new(self, 20)
    }
    #[doc = "Bit 24 - Restart Timer on TCMP1"]
    #[inline(always)]
    pub fn restarten(&mut self) -> RestartenW<Timecmp1Spec> {
        RestartenW::new(self, 24)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`timecmp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timecmp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timecmp1Spec;
impl crate::RegisterSpec for Timecmp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timecmp1::R`](R) reader structure"]
impl crate::Readable for Timecmp1Spec {}
#[doc = "`write(|w| ..)` method takes [`timecmp1::W`](W) writer structure"]
impl crate::Writable for Timecmp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMECMP1 to value 0"]
impl crate::Resettable for Timecmp1Spec {}
