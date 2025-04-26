#[doc = "Register `LESENSE_ROUTEEN` reader"]
pub type R = crate::R<LesenseRouteenSpec>;
#[doc = "Register `LESENSE_ROUTEEN` writer"]
pub type W = crate::W<LesenseRouteenSpec>;
#[doc = "Field `CH0OUTPEN` reader - CH0OUT pin enable control bit"]
pub type Ch0outpenR = crate::BitReader;
#[doc = "Field `CH0OUTPEN` writer - CH0OUT pin enable control bit"]
pub type Ch0outpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1OUTPEN` reader - CH1OUT pin enable control bit"]
pub type Ch1outpenR = crate::BitReader;
#[doc = "Field `CH1OUTPEN` writer - CH1OUT pin enable control bit"]
pub type Ch1outpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2OUTPEN` reader - CH2OUT pin enable control bit"]
pub type Ch2outpenR = crate::BitReader;
#[doc = "Field `CH2OUTPEN` writer - CH2OUT pin enable control bit"]
pub type Ch2outpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3OUTPEN` reader - CH3OUT pin enable control bit"]
pub type Ch3outpenR = crate::BitReader;
#[doc = "Field `CH3OUTPEN` writer - CH3OUT pin enable control bit"]
pub type Ch3outpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4OUTPEN` reader - CH4OUT pin enable control bit"]
pub type Ch4outpenR = crate::BitReader;
#[doc = "Field `CH4OUTPEN` writer - CH4OUT pin enable control bit"]
pub type Ch4outpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5OUTPEN` reader - CH5OUT pin enable control bit"]
pub type Ch5outpenR = crate::BitReader;
#[doc = "Field `CH5OUTPEN` writer - CH5OUT pin enable control bit"]
pub type Ch5outpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6OUTPEN` reader - CH6OUT pin enable control bit"]
pub type Ch6outpenR = crate::BitReader;
#[doc = "Field `CH6OUTPEN` writer - CH6OUT pin enable control bit"]
pub type Ch6outpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7OUTPEN` reader - CH7OUT pin enable control bit"]
pub type Ch7outpenR = crate::BitReader;
#[doc = "Field `CH7OUTPEN` writer - CH7OUT pin enable control bit"]
pub type Ch7outpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH8OUTPEN` reader - CH8OUT pin enable control bit"]
pub type Ch8outpenR = crate::BitReader;
#[doc = "Field `CH8OUTPEN` writer - CH8OUT pin enable control bit"]
pub type Ch8outpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH9OUTPEN` reader - CH9OUT pin enable control bit"]
pub type Ch9outpenR = crate::BitReader;
#[doc = "Field `CH9OUTPEN` writer - CH9OUT pin enable control bit"]
pub type Ch9outpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH10OUTPEN` reader - CH10OUT pin enable control bit"]
pub type Ch10outpenR = crate::BitReader;
#[doc = "Field `CH10OUTPEN` writer - CH10OUT pin enable control bit"]
pub type Ch10outpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH11OUTPEN` reader - CH11OUT pin enable control bit"]
pub type Ch11outpenR = crate::BitReader;
#[doc = "Field `CH11OUTPEN` writer - CH11OUT pin enable control bit"]
pub type Ch11outpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH12OUTPEN` reader - CH12OUT pin enable control bit"]
pub type Ch12outpenR = crate::BitReader;
#[doc = "Field `CH12OUTPEN` writer - CH12OUT pin enable control bit"]
pub type Ch12outpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH13OUTPEN` reader - CH13OUT pin enable control bit"]
pub type Ch13outpenR = crate::BitReader;
#[doc = "Field `CH13OUTPEN` writer - CH13OUT pin enable control bit"]
pub type Ch13outpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH14OUTPEN` reader - CH14OUT pin enable control bit"]
pub type Ch14outpenR = crate::BitReader;
#[doc = "Field `CH14OUTPEN` writer - CH14OUT pin enable control bit"]
pub type Ch14outpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH15OUTPEN` reader - CH15OUT pin enable control bit"]
pub type Ch15outpenR = crate::BitReader;
#[doc = "Field `CH15OUTPEN` writer - CH15OUT pin enable control bit"]
pub type Ch15outpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CH0OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch0outpen(&self) -> Ch0outpenR {
        Ch0outpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch1outpen(&self) -> Ch1outpenR {
        Ch1outpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch2outpen(&self) -> Ch2outpenR {
        Ch2outpenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch3outpen(&self) -> Ch3outpenR {
        Ch3outpenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH4OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch4outpen(&self) -> Ch4outpenR {
        Ch4outpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH5OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch5outpen(&self) -> Ch5outpenR {
        Ch5outpenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH6OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch6outpen(&self) -> Ch6outpenR {
        Ch6outpenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH7OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch7outpen(&self) -> Ch7outpenR {
        Ch7outpenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CH8OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch8outpen(&self) -> Ch8outpenR {
        Ch8outpenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CH9OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch9outpen(&self) -> Ch9outpenR {
        Ch9outpenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CH10OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch10outpen(&self) -> Ch10outpenR {
        Ch10outpenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CH11OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch11outpen(&self) -> Ch11outpenR {
        Ch11outpenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CH12OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch12outpen(&self) -> Ch12outpenR {
        Ch12outpenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CH13OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch13outpen(&self) -> Ch13outpenR {
        Ch13outpenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CH14OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch14outpen(&self) -> Ch14outpenR {
        Ch14outpenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CH15OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch15outpen(&self) -> Ch15outpenR {
        Ch15outpenR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch0outpen(&mut self) -> Ch0outpenW<LesenseRouteenSpec> {
        Ch0outpenW::new(self, 0)
    }
    #[doc = "Bit 1 - CH1OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch1outpen(&mut self) -> Ch1outpenW<LesenseRouteenSpec> {
        Ch1outpenW::new(self, 1)
    }
    #[doc = "Bit 2 - CH2OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch2outpen(&mut self) -> Ch2outpenW<LesenseRouteenSpec> {
        Ch2outpenW::new(self, 2)
    }
    #[doc = "Bit 3 - CH3OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch3outpen(&mut self) -> Ch3outpenW<LesenseRouteenSpec> {
        Ch3outpenW::new(self, 3)
    }
    #[doc = "Bit 4 - CH4OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch4outpen(&mut self) -> Ch4outpenW<LesenseRouteenSpec> {
        Ch4outpenW::new(self, 4)
    }
    #[doc = "Bit 5 - CH5OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch5outpen(&mut self) -> Ch5outpenW<LesenseRouteenSpec> {
        Ch5outpenW::new(self, 5)
    }
    #[doc = "Bit 6 - CH6OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch6outpen(&mut self) -> Ch6outpenW<LesenseRouteenSpec> {
        Ch6outpenW::new(self, 6)
    }
    #[doc = "Bit 7 - CH7OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch7outpen(&mut self) -> Ch7outpenW<LesenseRouteenSpec> {
        Ch7outpenW::new(self, 7)
    }
    #[doc = "Bit 8 - CH8OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch8outpen(&mut self) -> Ch8outpenW<LesenseRouteenSpec> {
        Ch8outpenW::new(self, 8)
    }
    #[doc = "Bit 9 - CH9OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch9outpen(&mut self) -> Ch9outpenW<LesenseRouteenSpec> {
        Ch9outpenW::new(self, 9)
    }
    #[doc = "Bit 10 - CH10OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch10outpen(&mut self) -> Ch10outpenW<LesenseRouteenSpec> {
        Ch10outpenW::new(self, 10)
    }
    #[doc = "Bit 11 - CH11OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch11outpen(&mut self) -> Ch11outpenW<LesenseRouteenSpec> {
        Ch11outpenW::new(self, 11)
    }
    #[doc = "Bit 12 - CH12OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch12outpen(&mut self) -> Ch12outpenW<LesenseRouteenSpec> {
        Ch12outpenW::new(self, 12)
    }
    #[doc = "Bit 13 - CH13OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch13outpen(&mut self) -> Ch13outpenW<LesenseRouteenSpec> {
        Ch13outpenW::new(self, 13)
    }
    #[doc = "Bit 14 - CH14OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch14outpen(&mut self) -> Ch14outpenW<LesenseRouteenSpec> {
        Ch14outpenW::new(self, 14)
    }
    #[doc = "Bit 15 - CH15OUT pin enable control bit"]
    #[inline(always)]
    pub fn ch15outpen(&mut self) -> Ch15outpenW<LesenseRouteenSpec> {
        Ch15outpenW::new(self, 15)
    }
}
#[doc = "LESENSE pin enable\n\nYou can [`read`](crate::Reg::read) this register and get [`lesense_routeen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lesense_routeen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LesenseRouteenSpec;
impl crate::RegisterSpec for LesenseRouteenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lesense_routeen::R`](R) reader structure"]
impl crate::Readable for LesenseRouteenSpec {}
#[doc = "`write(|w| ..)` method takes [`lesense_routeen::W`](W) writer structure"]
impl crate::Writable for LesenseRouteenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LESENSE_ROUTEEN to value 0"]
impl crate::Resettable for LesenseRouteenSpec {}
