#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR2 {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `MMS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMSR {# [ doc = "the UG bit from the TIMx_EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset." ] RESET , # [ doc = "the Counter Enable signal CNT_EN is used as trigger output (TRGO)." ] ENABLE , # [ doc = "The update event is selected as trigger output (TRGO)." ] UPDATE , # [ doc = "The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred.(TRGO)." ] COMPAREPULSE , # [ doc = "OC1REF signal is used as trigger output (TRGO)" ] COMPAREOC1REF , # [ doc = "OC2REF signal is used as trigger output (TRGO)" ] COMPAREOC2REF , # [ doc = "OC3REF signal is used as trigger output (TRGO)" ] COMPAREOC3REF , # [ doc = "OC4REF signal is used as trigger output (TRGO)" ] COMPAREOC4REF ,}
impl MMSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MMSR::RESET => 0,
            MMSR::ENABLE => 1,
            MMSR::UPDATE => 2,
            MMSR::COMPAREPULSE => 3,
            MMSR::COMPAREOC1REF => 4,
            MMSR::COMPAREOC2REF => 5,
            MMSR::COMPAREOC3REF => 6,
            MMSR::COMPAREOC4REF => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MMSR {
        match value {
            0 => MMSR::RESET,
            1 => MMSR::ENABLE,
            2 => MMSR::UPDATE,
            3 => MMSR::COMPAREPULSE,
            4 => MMSR::COMPAREOC1REF,
            5 => MMSR::COMPAREOC2REF,
            6 => MMSR::COMPAREOC3REF,
            7 => MMSR::COMPAREOC4REF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline]
    pub fn is_reset(&self) -> bool {
        *self == MMSR::RESET
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline]
    pub fn is_enable(&self) -> bool {
        *self == MMSR::ENABLE
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline]
    pub fn is_update(&self) -> bool {
        *self == MMSR::UPDATE
    }
    #[doc = "Checks if the value of the field is `COMPAREPULSE`"]
    #[inline]
    pub fn is_compare_pulse(&self) -> bool {
        *self == MMSR::COMPAREPULSE
    }
    #[doc = "Checks if the value of the field is `COMPAREOC1REF`"]
    #[inline]
    pub fn is_compare_oc1ref(&self) -> bool {
        *self == MMSR::COMPAREOC1REF
    }
    #[doc = "Checks if the value of the field is `COMPAREOC2REF`"]
    #[inline]
    pub fn is_compare_oc2ref(&self) -> bool {
        *self == MMSR::COMPAREOC2REF
    }
    #[doc = "Checks if the value of the field is `COMPAREOC3REF`"]
    #[inline]
    pub fn is_compare_oc3ref(&self) -> bool {
        *self == MMSR::COMPAREOC3REF
    }
    #[doc = "Checks if the value of the field is `COMPAREOC4REF`"]
    #[inline]
    pub fn is_compare_oc4ref(&self) -> bool {
        *self == MMSR::COMPAREOC4REF
    }
}
#[doc = "Values that can be written to the field `MMS`"]
pub enum MMSW {# [ doc = "the UG bit from the TIMx_EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset." ] RESET , # [ doc = "the Counter Enable signal CNT_EN is used as trigger output (TRGO)." ] ENABLE , # [ doc = "The update event is selected as trigger output (TRGO)." ] UPDATE , # [ doc = "The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred.(TRGO)." ] COMPAREPULSE , # [ doc = "OC1REF signal is used as trigger output (TRGO)" ] COMPAREOC1REF , # [ doc = "OC2REF signal is used as trigger output (TRGO)" ] COMPAREOC2REF , # [ doc = "OC3REF signal is used as trigger output (TRGO)" ] COMPAREOC3REF , # [ doc = "OC4REF signal is used as trigger output (TRGO)" ] COMPAREOC4REF ,}
impl MMSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MMSW::RESET => 0,
            MMSW::ENABLE => 1,
            MMSW::UPDATE => 2,
            MMSW::COMPAREPULSE => 3,
            MMSW::COMPAREOC1REF => 4,
            MMSW::COMPAREOC2REF => 5,
            MMSW::COMPAREOC3REF => 6,
            MMSW::COMPAREOC4REF => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MMSW<'a> {
    w: &'a mut W,
}
impl<'a> _MMSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MMSW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    # [ doc = "the UG bit from the TIMx_EGR register is used as trigger output (TRGO). If the reset is generated by the trigger input (slave mode controller configured in reset mode) then the signal on TRGO is delayed compared to the actual reset." ] # [ inline ]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMSW::RESET)
    }
    #[doc = "the Counter Enable signal CNT_EN is used as trigger output (TRGO)."]
    #[inline]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMSW::ENABLE)
    }
    #[doc = "The update event is selected as trigger output (TRGO)."]
    #[inline]
    pub fn update(self) -> &'a mut W {
        self.variant(MMSW::UPDATE)
    }
    # [ doc = "The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred.(TRGO)." ] # [ inline ]
    pub fn compare_pulse(self) -> &'a mut W {
        self.variant(MMSW::COMPAREPULSE)
    }
    #[doc = "OC1REF signal is used as trigger output (TRGO)"]
    #[inline]
    pub fn compare_oc1ref(self) -> &'a mut W {
        self.variant(MMSW::COMPAREOC1REF)
    }
    #[doc = "OC2REF signal is used as trigger output (TRGO)"]
    #[inline]
    pub fn compare_oc2ref(self) -> &'a mut W {
        self.variant(MMSW::COMPAREOC2REF)
    }
    #[doc = "OC3REF signal is used as trigger output (TRGO)"]
    #[inline]
    pub fn compare_oc3ref(self) -> &'a mut W {
        self.variant(MMSW::COMPAREOC3REF)
    }
    #[doc = "OC4REF signal is used as trigger output (TRGO)"]
    #[inline]
    pub fn compare_oc4ref(self) -> &'a mut W {
        self.variant(MMSW::COMPAREOC4REF)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline]
    pub fn mms(&self) -> MMSR {
        MMSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline]
    pub fn mms(&mut self) -> _MMSW {
        _MMSW { w: self }
    }
}
