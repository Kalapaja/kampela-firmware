#[doc = "Register `EXTINFO` reader"]
pub type R = crate::R<ExtinfoSpec>;
#[doc = "Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Type {
    #[doc = "255: NONE"]
    None = 255,
}
impl From<Type> for u8 {
    #[inline(always)]
    fn from(variant: Type) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Type {
    type Ux = u8;
}
impl crate::IsEnum for Type {}
#[doc = "Field `TYPE` reader - Type"]
pub type TypeR = crate::FieldReader<Type>;
impl TypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Type> {
        match self.bits {
            255 => Some(Type::None),
            _ => None,
        }
    }
    #[doc = "NONE"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Type::None
    }
}
#[doc = "Connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Connection {
    #[doc = "0: SPI control interface"]
    Spi = 0,
    #[doc = "255: No interface"]
    None = 255,
}
impl From<Connection> for u8 {
    #[inline(always)]
    fn from(variant: Connection) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Connection {
    type Ux = u8;
}
impl crate::IsEnum for Connection {}
#[doc = "Field `CONNECTION` reader - Connection"]
pub type ConnectionR = crate::FieldReader<Connection>;
impl ConnectionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Connection> {
        match self.bits {
            0 => Some(Connection::Spi),
            255 => Some(Connection::None),
            _ => None,
        }
    }
    #[doc = "SPI control interface"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Connection::Spi
    }
    #[doc = "No interface"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Connection::None
    }
}
#[doc = "Field `REV` reader - Revision"]
pub type RevR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Type"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Connection"]
    #[inline(always)]
    pub fn connection(&self) -> ConnectionR {
        ConnectionR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Revision"]
    #[inline(always)]
    pub fn rev(&self) -> RevR {
        RevR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "External component description\n\nYou can [`read`](crate::Reg::read) this register and get [`extinfo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtinfoSpec;
impl crate::RegisterSpec for ExtinfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extinfo::R`](R) reader structure"]
impl crate::Readable for ExtinfoSpec {}
#[doc = "`reset()` method sets EXTINFO to value 0"]
impl crate::Resettable for ExtinfoSpec {}
