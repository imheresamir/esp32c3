#[doc = "Reader of register RTC_I2C_CMD13"]
pub type R = crate::R<u32, super::RTC_I2C_CMD13>;
#[doc = "Writer for register RTC_I2C_CMD13"]
pub type W = crate::W<u32, super::RTC_I2C_CMD13>;
#[doc = "Register RTC_I2C_CMD13 `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_I2C_CMD13 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTC_I2C_COMMAND13_DONE`"]
pub type RTC_I2C_COMMAND13_DONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RTC_I2C_COMMAND13`"]
pub type RTC_I2C_COMMAND13_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RTC_I2C_COMMAND13`"]
pub struct RTC_I2C_COMMAND13_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_I2C_COMMAND13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn rtc_i2c_command13_done(&self) -> RTC_I2C_COMMAND13_DONE_R {
        RTC_I2C_COMMAND13_DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn rtc_i2c_command13(&self) -> RTC_I2C_COMMAND13_R {
        RTC_I2C_COMMAND13_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn rtc_i2c_command13(&mut self) -> RTC_I2C_COMMAND13_W {
        RTC_I2C_COMMAND13_W { w: self }
    }
}
