#[doc = "Register `IF` reader"]
pub type R = crate::R<IfSpec>;
#[doc = "Register `IF` writer"]
pub type W = crate::W<IfSpec>;
#[doc = "Field `CH0CD` reader - CH0 Conversion Done Interrupt Flag"]
pub type Ch0cdR = crate::BitReader;
#[doc = "Field `CH0CD` writer - CH0 Conversion Done Interrupt Flag"]
pub type Ch0cdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1CD` reader - CH1 Conversion Done Interrupt Flag"]
pub type Ch1cdR = crate::BitReader;
#[doc = "Field `CH1CD` writer - CH1 Conversion Done Interrupt Flag"]
pub type Ch1cdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0OF` reader - CH0 Data Overflow Interrupt Flag"]
pub type Ch0ofR = crate::BitReader;
#[doc = "Field `CH0OF` writer - CH0 Data Overflow Interrupt Flag"]
pub type Ch0ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1OF` reader - CH1 Data Overflow Interrupt Flag"]
pub type Ch1ofR = crate::BitReader;
#[doc = "Field `CH1OF` writer - CH1 Data Overflow Interrupt Flag"]
pub type Ch1ofW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0UF` reader - CH0 Data Underflow Interrupt Flag"]
pub type Ch0ufR = crate::BitReader;
#[doc = "Field `CH0UF` writer - CH0 Data Underflow Interrupt Flag"]
pub type Ch0ufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1UF` reader - CH1 Data Underflow Interrupt Flag"]
pub type Ch1ufR = crate::BitReader;
#[doc = "Field `CH1UF` writer - CH1 Data Underflow Interrupt Flag"]
pub type Ch1ufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABUSALLOCERR` reader - ABUS Port Allocation Error Flag"]
pub type AbusallocerrR = crate::BitReader;
#[doc = "Field `ABUSALLOCERR` writer - ABUS Port Allocation Error Flag"]
pub type AbusallocerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0DVL` reader - CH0 Data Valid Level Interrupt Flag"]
pub type Ch0dvlR = crate::BitReader;
#[doc = "Field `CH0DVL` writer - CH0 Data Valid Level Interrupt Flag"]
pub type Ch0dvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1DVL` reader - CH1 Data Valid Level Interrupt Flag"]
pub type Ch1dvlR = crate::BitReader;
#[doc = "Field `CH1DVL` writer - CH1 Data Valid Level Interrupt Flag"]
pub type Ch1dvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABUSINPUTCONFLICT` reader - ABUS Input Conflict Error Flag"]
pub type AbusinputconflictR = crate::BitReader;
#[doc = "Field `ABUSINPUTCONFLICT` writer - ABUS Input Conflict Error Flag"]
pub type AbusinputconflictW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CH0 Conversion Done Interrupt Flag"]
    #[inline(always)]
    pub fn ch0cd(&self) -> Ch0cdR {
        Ch0cdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1 Conversion Done Interrupt Flag"]
    #[inline(always)]
    pub fn ch1cd(&self) -> Ch1cdR {
        Ch1cdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - CH0 Data Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch0of(&self) -> Ch0ofR {
        Ch0ofR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH1 Data Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch1of(&self) -> Ch1ofR {
        Ch1ofR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CH0 Data Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch0uf(&self) -> Ch0ufR {
        Ch0ufR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CH1 Data Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch1uf(&self) -> Ch1ufR {
        Ch1ufR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 18 - ABUS Port Allocation Error Flag"]
    #[inline(always)]
    pub fn abusallocerr(&self) -> AbusallocerrR {
        AbusallocerrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - CH0 Data Valid Level Interrupt Flag"]
    #[inline(always)]
    pub fn ch0dvl(&self) -> Ch0dvlR {
        Ch0dvlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CH1 Data Valid Level Interrupt Flag"]
    #[inline(always)]
    pub fn ch1dvl(&self) -> Ch1dvlR {
        Ch1dvlR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - ABUS Input Conflict Error Flag"]
    #[inline(always)]
    pub fn abusinputconflict(&self) -> AbusinputconflictR {
        AbusinputconflictR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0 Conversion Done Interrupt Flag"]
    #[inline(always)]
    pub fn ch0cd(&mut self) -> Ch0cdW<IfSpec> {
        Ch0cdW::new(self, 0)
    }
    #[doc = "Bit 1 - CH1 Conversion Done Interrupt Flag"]
    #[inline(always)]
    pub fn ch1cd(&mut self) -> Ch1cdW<IfSpec> {
        Ch1cdW::new(self, 1)
    }
    #[doc = "Bit 4 - CH0 Data Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch0of(&mut self) -> Ch0ofW<IfSpec> {
        Ch0ofW::new(self, 4)
    }
    #[doc = "Bit 5 - CH1 Data Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch1of(&mut self) -> Ch1ofW<IfSpec> {
        Ch1ofW::new(self, 5)
    }
    #[doc = "Bit 8 - CH0 Data Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch0uf(&mut self) -> Ch0ufW<IfSpec> {
        Ch0ufW::new(self, 8)
    }
    #[doc = "Bit 9 - CH1 Data Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch1uf(&mut self) -> Ch1ufW<IfSpec> {
        Ch1ufW::new(self, 9)
    }
    #[doc = "Bit 18 - ABUS Port Allocation Error Flag"]
    #[inline(always)]
    pub fn abusallocerr(&mut self) -> AbusallocerrW<IfSpec> {
        AbusallocerrW::new(self, 18)
    }
    #[doc = "Bit 20 - CH0 Data Valid Level Interrupt Flag"]
    #[inline(always)]
    pub fn ch0dvl(&mut self) -> Ch0dvlW<IfSpec> {
        Ch0dvlW::new(self, 20)
    }
    #[doc = "Bit 21 - CH1 Data Valid Level Interrupt Flag"]
    #[inline(always)]
    pub fn ch1dvl(&mut self) -> Ch1dvlW<IfSpec> {
        Ch1dvlW::new(self, 21)
    }
    #[doc = "Bit 26 - ABUS Input Conflict Error Flag"]
    #[inline(always)]
    pub fn abusinputconflict(&mut self) -> AbusinputconflictW<IfSpec> {
        AbusinputconflictW::new(self, 26)
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
