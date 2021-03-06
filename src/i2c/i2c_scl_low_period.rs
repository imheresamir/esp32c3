#[doc = "Reader of register I2C_SCL_LOW_PERIOD"]
pub type R = crate::R<u32, super::I2C_SCL_LOW_PERIOD>;
#[doc = "Writer for register I2C_SCL_LOW_PERIOD"]
pub type W = crate::W<u32, super::I2C_SCL_LOW_PERIOD>;
#[doc = "Register I2C_SCL_LOW_PERIOD `reset()`'s with value 0"]
impl crate::ResetValue for super::I2C_SCL_LOW_PERIOD {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C_SCL_LOW_PERIOD`"]
pub type I2C_SCL_LOW_PERIOD_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `I2C_SCL_LOW_PERIOD`"]
pub struct I2C_SCL_LOW_PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_SCL_LOW_PERIOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn i2c_scl_low_period(&self) -> I2C_SCL_LOW_PERIOD_R {
        I2C_SCL_LOW_PERIOD_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn i2c_scl_low_period(&mut self) -> I2C_SCL_LOW_PERIOD_W {
        I2C_SCL_LOW_PERIOD_W { w: self }
    }
}
