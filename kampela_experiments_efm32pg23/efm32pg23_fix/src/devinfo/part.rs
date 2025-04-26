#[doc = "Register `PART` reader"]
pub type R = crate::R<PartSpec>;
#[doc = "Field `DEVICENUM` reader - Device Number"]
pub type DevicenumR = crate::FieldReader<u16>;
#[doc = "Field `FAMILYNUM` reader - Device Family"]
pub type FamilynumR = crate::FieldReader;
#[doc = "Device Family\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Family {
    #[doc = "0: Flex Gecko"]
    Fg = 0,
    #[doc = "3: Z-Wave Gecko"]
    Zg = 3,
    #[doc = "5: Pearl Gecko"]
    Pg = 5,
    #[doc = "8: Sidewalk Gecko"]
    Sg = 8,
}
impl From<Family> for u8 {
    #[inline(always)]
    fn from(variant: Family) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Family {
    type Ux = u8;
}
impl crate::IsEnum for Family {}
#[doc = "Field `FAMILY` reader - Device Family"]
pub type FamilyR = crate::FieldReader<Family>;
impl FamilyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Family> {
        match self.bits {
            0 => Some(Family::Fg),
            3 => Some(Family::Zg),
            5 => Some(Family::Pg),
            8 => Some(Family::Sg),
            _ => None,
        }
    }
    #[doc = "Flex Gecko"]
    #[inline(always)]
    pub fn is_fg(&self) -> bool {
        *self == Family::Fg
    }
    #[doc = "Z-Wave Gecko"]
    #[inline(always)]
    pub fn is_zg(&self) -> bool {
        *self == Family::Zg
    }
    #[doc = "Pearl Gecko"]
    #[inline(always)]
    pub fn is_pg(&self) -> bool {
        *self == Family::Pg
    }
    #[doc = "Sidewalk Gecko"]
    #[inline(always)]
    pub fn is_sg(&self) -> bool {
        *self == Family::Sg
    }
}
impl R {
    #[doc = "Bits 0:15 - Device Number"]
    #[inline(always)]
    pub fn devicenum(&self) -> DevicenumR {
        DevicenumR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - Device Family"]
    #[inline(always)]
    pub fn familynum(&self) -> FamilynumR {
        FamilynumR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Device Family"]
    #[inline(always)]
    pub fn family(&self) -> FamilyR {
        FamilyR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[doc = "Part description\n\nYou can [`read`](crate::Reg::read) this register and get [`part::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PartSpec;
impl crate::RegisterSpec for PartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`part::R`](R) reader structure"]
impl crate::Readable for PartSpec {}
#[doc = "`reset()` method sets PART to value 0"]
impl crate::Resettable for PartSpec {}
