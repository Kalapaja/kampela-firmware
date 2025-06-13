#[doc = "Register `TRIGGER` reader"]
pub type R = crate::R<TriggerSpec>;
#[doc = "Register `TRIGGER` writer"]
pub type W = crate::W<TriggerSpec>;
#[doc = "Scan Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Scantrigsel {
    #[doc = "0: Immediate triggering. The scan queue will be disabled once all conversions in the scan table are complete, unless TRIGGERACTION is set to continuous."]
    Immediate = 0,
    #[doc = "1: Triggers when the local timer count reaches zero."]
    Timer = 1,
    #[doc = "2: Triggers on PRS0 from a timer module that is using the same clock group as the ADC and has been programmed to use the same clock source as the ADC. The prescale may be different between the ADC and the timer module."]
    Prsclkgrp = 2,
    #[doc = "3: Triggers on asynchronous PRS0 positive edge. Requires PRS0 to go low for 3 ADC_CLKs before another positive edge can be detected. Generates an additional delay of 1 to 2 CLK_SRC_ADC cycles for synchronization."]
    Prspos = 3,
    #[doc = "4: Triggers on asynchronous PRS0 negative edge. Requires PRS0 to go high for 3 ADC_CLKs before another negative edge can be detected. Generates an additional delay of 1 to 2 CLK_SRC_ADC cycles for synchronization. PRSNEG should only be used when the trigger source is from a module that remains powered during EM23. For modules (ie: TIMER) that power down during EM23, PRSPOS should be used for an asynchronous trigger, and PRSCLKGRP should be used for a synchronous trigger."]
    Prsneg = 4,
    #[doc = "5: Triggers on LESENSE convert request. When using the LESENSE for the SCAN Table, only one entry is converted per LESENSE convert request."]
    Lesense = 5,
}
impl From<Scantrigsel> for u8 {
    #[inline(always)]
    fn from(variant: Scantrigsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Scantrigsel {
    type Ux = u8;
}
impl crate::IsEnum for Scantrigsel {}
#[doc = "Field `SCANTRIGSEL` reader - Scan Trigger Select"]
pub type ScantrigselR = crate::FieldReader<Scantrigsel>;
impl ScantrigselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Scantrigsel> {
        match self.bits {
            0 => Some(Scantrigsel::Immediate),
            1 => Some(Scantrigsel::Timer),
            2 => Some(Scantrigsel::Prsclkgrp),
            3 => Some(Scantrigsel::Prspos),
            4 => Some(Scantrigsel::Prsneg),
            5 => Some(Scantrigsel::Lesense),
            _ => None,
        }
    }
    #[doc = "Immediate triggering. The scan queue will be disabled once all conversions in the scan table are complete, unless TRIGGERACTION is set to continuous."]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == Scantrigsel::Immediate
    }
    #[doc = "Triggers when the local timer count reaches zero."]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == Scantrigsel::Timer
    }
    #[doc = "Triggers on PRS0 from a timer module that is using the same clock group as the ADC and has been programmed to use the same clock source as the ADC. The prescale may be different between the ADC and the timer module."]
    #[inline(always)]
    pub fn is_prsclkgrp(&self) -> bool {
        *self == Scantrigsel::Prsclkgrp
    }
    #[doc = "Triggers on asynchronous PRS0 positive edge. Requires PRS0 to go low for 3 ADC_CLKs before another positive edge can be detected. Generates an additional delay of 1 to 2 CLK_SRC_ADC cycles for synchronization."]
    #[inline(always)]
    pub fn is_prspos(&self) -> bool {
        *self == Scantrigsel::Prspos
    }
    #[doc = "Triggers on asynchronous PRS0 negative edge. Requires PRS0 to go high for 3 ADC_CLKs before another negative edge can be detected. Generates an additional delay of 1 to 2 CLK_SRC_ADC cycles for synchronization. PRSNEG should only be used when the trigger source is from a module that remains powered during EM23. For modules (ie: TIMER) that power down during EM23, PRSPOS should be used for an asynchronous trigger, and PRSCLKGRP should be used for a synchronous trigger."]
    #[inline(always)]
    pub fn is_prsneg(&self) -> bool {
        *self == Scantrigsel::Prsneg
    }
    #[doc = "Triggers on LESENSE convert request. When using the LESENSE for the SCAN Table, only one entry is converted per LESENSE convert request."]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == Scantrigsel::Lesense
    }
}
#[doc = "Field `SCANTRIGSEL` writer - Scan Trigger Select"]
pub type ScantrigselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Scantrigsel>;
impl<'a, REG> ScantrigselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate triggering. The scan queue will be disabled once all conversions in the scan table are complete, unless TRIGGERACTION is set to continuous."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(Scantrigsel::Immediate)
    }
    #[doc = "Triggers when the local timer count reaches zero."]
    #[inline(always)]
    pub fn timer(self) -> &'a mut crate::W<REG> {
        self.variant(Scantrigsel::Timer)
    }
    #[doc = "Triggers on PRS0 from a timer module that is using the same clock group as the ADC and has been programmed to use the same clock source as the ADC. The prescale may be different between the ADC and the timer module."]
    #[inline(always)]
    pub fn prsclkgrp(self) -> &'a mut crate::W<REG> {
        self.variant(Scantrigsel::Prsclkgrp)
    }
    #[doc = "Triggers on asynchronous PRS0 positive edge. Requires PRS0 to go low for 3 ADC_CLKs before another positive edge can be detected. Generates an additional delay of 1 to 2 CLK_SRC_ADC cycles for synchronization."]
    #[inline(always)]
    pub fn prspos(self) -> &'a mut crate::W<REG> {
        self.variant(Scantrigsel::Prspos)
    }
    #[doc = "Triggers on asynchronous PRS0 negative edge. Requires PRS0 to go high for 3 ADC_CLKs before another negative edge can be detected. Generates an additional delay of 1 to 2 CLK_SRC_ADC cycles for synchronization. PRSNEG should only be used when the trigger source is from a module that remains powered during EM23. For modules (ie: TIMER) that power down during EM23, PRSPOS should be used for an asynchronous trigger, and PRSCLKGRP should be used for a synchronous trigger."]
    #[inline(always)]
    pub fn prsneg(self) -> &'a mut crate::W<REG> {
        self.variant(Scantrigsel::Prsneg)
    }
    #[doc = "Triggers on LESENSE convert request. When using the LESENSE for the SCAN Table, only one entry is converted per LESENSE convert request."]
    #[inline(always)]
    pub fn lesense(self) -> &'a mut crate::W<REG> {
        self.variant(Scantrigsel::Lesense)
    }
}
#[doc = "Scan Trigger Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scantrigaction {
    #[doc = "0: For TRIGSEL=IMMEDIATE, goes through the scan table once and disables queue. For TRIGSEL = TIMER, PRSCLKGRP, PRSPOS, PRSNEG, goes through the scan table once per trigger."]
    Once = 0,
    #[doc = "1: Goes through the scan table, converts each entry with a mask bit set, and puts it back into the scan queue to repeat again continuously. The queues are first come first serve. If both queues are triggered, the single queue will get to convert after each scan table completes. The scan queue will get to convert after each single conversion completes."]
    Continuous = 1,
}
impl From<Scantrigaction> for bool {
    #[inline(always)]
    fn from(variant: Scantrigaction) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCANTRIGACTION` reader - Scan Trigger Action"]
pub type ScantrigactionR = crate::BitReader<Scantrigaction>;
impl ScantrigactionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Scantrigaction {
        match self.bits {
            false => Scantrigaction::Once,
            true => Scantrigaction::Continuous,
        }
    }
    #[doc = "For TRIGSEL=IMMEDIATE, goes through the scan table once and disables queue. For TRIGSEL = TIMER, PRSCLKGRP, PRSPOS, PRSNEG, goes through the scan table once per trigger."]
    #[inline(always)]
    pub fn is_once(&self) -> bool {
        *self == Scantrigaction::Once
    }
    #[doc = "Goes through the scan table, converts each entry with a mask bit set, and puts it back into the scan queue to repeat again continuously. The queues are first come first serve. If both queues are triggered, the single queue will get to convert after each scan table completes. The scan queue will get to convert after each single conversion completes."]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Scantrigaction::Continuous
    }
}
#[doc = "Field `SCANTRIGACTION` writer - Scan Trigger Action"]
pub type ScantrigactionW<'a, REG> = crate::BitWriter<'a, REG, Scantrigaction>;
impl<'a, REG> ScantrigactionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "For TRIGSEL=IMMEDIATE, goes through the scan table once and disables queue. For TRIGSEL = TIMER, PRSCLKGRP, PRSPOS, PRSNEG, goes through the scan table once per trigger."]
    #[inline(always)]
    pub fn once(self) -> &'a mut crate::W<REG> {
        self.variant(Scantrigaction::Once)
    }
    #[doc = "Goes through the scan table, converts each entry with a mask bit set, and puts it back into the scan queue to repeat again continuously. The queues are first come first serve. If both queues are triggered, the single queue will get to convert after each scan table completes. The scan queue will get to convert after each single conversion completes."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Scantrigaction::Continuous)
    }
}
#[doc = "Single Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Singletrigsel {
    #[doc = "0: Immediate triggering. The single queue will be disabled once the conversion is complete, unless TRIGGERACTION is set to continuous."]
    Immediate = 0,
    #[doc = "1: Triggers when the local timer count reaches zero."]
    Timer = 1,
    #[doc = "2: Triggers on PRS1 from a timer module that is using the same clock group as the ADC and has been programmed to use the same clock source as the ADC. The prescale may be different between the ADC and the timer module."]
    Prsclkgrp = 2,
    #[doc = "3: Triggers on asynchronous PRS1 positive edge. Requires PRS1 to go low for 3 ADC_CLKs before another positive edge can be detected. Generates an additional delay of 1 to 2 CLK_SRC_ADC cycles for synchronization."]
    Prspos = 3,
    #[doc = "4: Triggers on asynchronous PRS1 negative edge. Requires PRS1 to go high for 3 ADC_CLKs before another negative edge can be detected. Generates an additional delay of 1 to 2 CLK_SRC_ADC cycles for synchronization. PRSNEG should only be used when the trigger source is from a module that remains powered during EM23. For modules (ie: TIMER) that power down during EM23, PRSPOS should be used for an asynchronous trigger, and PRSCLKGRP should be used for a synchronous trigger."]
    Prsneg = 4,
}
impl From<Singletrigsel> for u8 {
    #[inline(always)]
    fn from(variant: Singletrigsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Singletrigsel {
    type Ux = u8;
}
impl crate::IsEnum for Singletrigsel {}
#[doc = "Field `SINGLETRIGSEL` reader - Single Trigger Select"]
pub type SingletrigselR = crate::FieldReader<Singletrigsel>;
impl SingletrigselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Singletrigsel> {
        match self.bits {
            0 => Some(Singletrigsel::Immediate),
            1 => Some(Singletrigsel::Timer),
            2 => Some(Singletrigsel::Prsclkgrp),
            3 => Some(Singletrigsel::Prspos),
            4 => Some(Singletrigsel::Prsneg),
            _ => None,
        }
    }
    #[doc = "Immediate triggering. The single queue will be disabled once the conversion is complete, unless TRIGGERACTION is set to continuous."]
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == Singletrigsel::Immediate
    }
    #[doc = "Triggers when the local timer count reaches zero."]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == Singletrigsel::Timer
    }
    #[doc = "Triggers on PRS1 from a timer module that is using the same clock group as the ADC and has been programmed to use the same clock source as the ADC. The prescale may be different between the ADC and the timer module."]
    #[inline(always)]
    pub fn is_prsclkgrp(&self) -> bool {
        *self == Singletrigsel::Prsclkgrp
    }
    #[doc = "Triggers on asynchronous PRS1 positive edge. Requires PRS1 to go low for 3 ADC_CLKs before another positive edge can be detected. Generates an additional delay of 1 to 2 CLK_SRC_ADC cycles for synchronization."]
    #[inline(always)]
    pub fn is_prspos(&self) -> bool {
        *self == Singletrigsel::Prspos
    }
    #[doc = "Triggers on asynchronous PRS1 negative edge. Requires PRS1 to go high for 3 ADC_CLKs before another negative edge can be detected. Generates an additional delay of 1 to 2 CLK_SRC_ADC cycles for synchronization. PRSNEG should only be used when the trigger source is from a module that remains powered during EM23. For modules (ie: TIMER) that power down during EM23, PRSPOS should be used for an asynchronous trigger, and PRSCLKGRP should be used for a synchronous trigger."]
    #[inline(always)]
    pub fn is_prsneg(&self) -> bool {
        *self == Singletrigsel::Prsneg
    }
}
#[doc = "Field `SINGLETRIGSEL` writer - Single Trigger Select"]
pub type SingletrigselW<'a, REG> = crate::FieldWriter<'a, REG, 3, Singletrigsel>;
impl<'a, REG> SingletrigselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Immediate triggering. The single queue will be disabled once the conversion is complete, unless TRIGGERACTION is set to continuous."]
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(Singletrigsel::Immediate)
    }
    #[doc = "Triggers when the local timer count reaches zero."]
    #[inline(always)]
    pub fn timer(self) -> &'a mut crate::W<REG> {
        self.variant(Singletrigsel::Timer)
    }
    #[doc = "Triggers on PRS1 from a timer module that is using the same clock group as the ADC and has been programmed to use the same clock source as the ADC. The prescale may be different between the ADC and the timer module."]
    #[inline(always)]
    pub fn prsclkgrp(self) -> &'a mut crate::W<REG> {
        self.variant(Singletrigsel::Prsclkgrp)
    }
    #[doc = "Triggers on asynchronous PRS1 positive edge. Requires PRS1 to go low for 3 ADC_CLKs before another positive edge can be detected. Generates an additional delay of 1 to 2 CLK_SRC_ADC cycles for synchronization."]
    #[inline(always)]
    pub fn prspos(self) -> &'a mut crate::W<REG> {
        self.variant(Singletrigsel::Prspos)
    }
    #[doc = "Triggers on asynchronous PRS1 negative edge. Requires PRS1 to go high for 3 ADC_CLKs before another negative edge can be detected. Generates an additional delay of 1 to 2 CLK_SRC_ADC cycles for synchronization. PRSNEG should only be used when the trigger source is from a module that remains powered during EM23. For modules (ie: TIMER) that power down during EM23, PRSPOS should be used for an asynchronous trigger, and PRSCLKGRP should be used for a synchronous trigger."]
    #[inline(always)]
    pub fn prsneg(self) -> &'a mut crate::W<REG> {
        self.variant(Singletrigsel::Prsneg)
    }
}
#[doc = "Single Trigger Action\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Singletrigaction {
    #[doc = "0: For TRIGSEL=IMMEDIATE, converts the single queue once and disables queue. For TRIGSEL = TIMER, PRSCLKGRP, PRSPOS, PRSNEG, converts the single queue once per trigger."]
    Once = 0,
    #[doc = "1: Converts the single queue, then checks for a pending scan queue before converting the single queue again continuously. The queues are first come first serve. If both queues are continuous, the IADC alternates between them."]
    Continuous = 1,
}
impl From<Singletrigaction> for bool {
    #[inline(always)]
    fn from(variant: Singletrigaction) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINGLETRIGACTION` reader - Single Trigger Action"]
pub type SingletrigactionR = crate::BitReader<Singletrigaction>;
impl SingletrigactionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Singletrigaction {
        match self.bits {
            false => Singletrigaction::Once,
            true => Singletrigaction::Continuous,
        }
    }
    #[doc = "For TRIGSEL=IMMEDIATE, converts the single queue once and disables queue. For TRIGSEL = TIMER, PRSCLKGRP, PRSPOS, PRSNEG, converts the single queue once per trigger."]
    #[inline(always)]
    pub fn is_once(&self) -> bool {
        *self == Singletrigaction::Once
    }
    #[doc = "Converts the single queue, then checks for a pending scan queue before converting the single queue again continuously. The queues are first come first serve. If both queues are continuous, the IADC alternates between them."]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Singletrigaction::Continuous
    }
}
#[doc = "Field `SINGLETRIGACTION` writer - Single Trigger Action"]
pub type SingletrigactionW<'a, REG> = crate::BitWriter<'a, REG, Singletrigaction>;
impl<'a, REG> SingletrigactionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "For TRIGSEL=IMMEDIATE, converts the single queue once and disables queue. For TRIGSEL = TIMER, PRSCLKGRP, PRSPOS, PRSNEG, converts the single queue once per trigger."]
    #[inline(always)]
    pub fn once(self) -> &'a mut crate::W<REG> {
        self.variant(Singletrigaction::Once)
    }
    #[doc = "Converts the single queue, then checks for a pending scan queue before converting the single queue again continuously. The queues are first come first serve. If both queues are continuous, the IADC alternates between them."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Singletrigaction::Continuous)
    }
}
#[doc = "Single Tailgate Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Singletailgate {
    #[doc = "0: The single queue is ready to start warming up and converting once the trigger had been detected."]
    Tailgateoff = 0,
    #[doc = "1: After the single queue's trigger is detected, it must wait until the end of a scan operation before the Single queue can be converted."]
    Tailgateon = 1,
}
impl From<Singletailgate> for bool {
    #[inline(always)]
    fn from(variant: Singletailgate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SINGLETAILGATE` reader - Single Tailgate Enable"]
pub type SingletailgateR = crate::BitReader<Singletailgate>;
impl SingletailgateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Singletailgate {
        match self.bits {
            false => Singletailgate::Tailgateoff,
            true => Singletailgate::Tailgateon,
        }
    }
    #[doc = "The single queue is ready to start warming up and converting once the trigger had been detected."]
    #[inline(always)]
    pub fn is_tailgateoff(&self) -> bool {
        *self == Singletailgate::Tailgateoff
    }
    #[doc = "After the single queue's trigger is detected, it must wait until the end of a scan operation before the Single queue can be converted."]
    #[inline(always)]
    pub fn is_tailgateon(&self) -> bool {
        *self == Singletailgate::Tailgateon
    }
}
#[doc = "Field `SINGLETAILGATE` writer - Single Tailgate Enable"]
pub type SingletailgateW<'a, REG> = crate::BitWriter<'a, REG, Singletailgate>;
impl<'a, REG> SingletailgateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The single queue is ready to start warming up and converting once the trigger had been detected."]
    #[inline(always)]
    pub fn tailgateoff(self) -> &'a mut crate::W<REG> {
        self.variant(Singletailgate::Tailgateoff)
    }
    #[doc = "After the single queue's trigger is detected, it must wait until the end of a scan operation before the Single queue can be converted."]
    #[inline(always)]
    pub fn tailgateon(self) -> &'a mut crate::W<REG> {
        self.variant(Singletailgate::Tailgateon)
    }
}
impl R {
    #[doc = "Bits 0:2 - Scan Trigger Select"]
    #[inline(always)]
    pub fn scantrigsel(&self) -> ScantrigselR {
        ScantrigselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Scan Trigger Action"]
    #[inline(always)]
    pub fn scantrigaction(&self) -> ScantrigactionR {
        ScantrigactionR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Single Trigger Select"]
    #[inline(always)]
    pub fn singletrigsel(&self) -> SingletrigselR {
        SingletrigselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 12 - Single Trigger Action"]
    #[inline(always)]
    pub fn singletrigaction(&self) -> SingletrigactionR {
        SingletrigactionR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Single Tailgate Enable"]
    #[inline(always)]
    pub fn singletailgate(&self) -> SingletailgateR {
        SingletailgateR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Scan Trigger Select"]
    #[inline(always)]
    pub fn scantrigsel(&mut self) -> ScantrigselW<TriggerSpec> {
        ScantrigselW::new(self, 0)
    }
    #[doc = "Bit 4 - Scan Trigger Action"]
    #[inline(always)]
    pub fn scantrigaction(&mut self) -> ScantrigactionW<TriggerSpec> {
        ScantrigactionW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Single Trigger Select"]
    #[inline(always)]
    pub fn singletrigsel(&mut self) -> SingletrigselW<TriggerSpec> {
        SingletrigselW::new(self, 8)
    }
    #[doc = "Bit 12 - Single Trigger Action"]
    #[inline(always)]
    pub fn singletrigaction(&mut self) -> SingletrigactionW<TriggerSpec> {
        SingletrigactionW::new(self, 12)
    }
    #[doc = "Bit 16 - Single Tailgate Enable"]
    #[inline(always)]
    pub fn singletailgate(&mut self) -> SingletailgateW<TriggerSpec> {
        SingletailgateW::new(self, 16)
    }
}
#[doc = "Trigger\n\nYou can [`read`](crate::Reg::read) this register and get [`trigger::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trigger::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TriggerSpec;
impl crate::RegisterSpec for TriggerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigger::R`](R) reader structure"]
impl crate::Readable for TriggerSpec {}
#[doc = "`write(|w| ..)` method takes [`trigger::W`](W) writer structure"]
impl crate::Writable for TriggerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TRIGGER to value 0"]
impl crate::Resettable for TriggerSpec {}
