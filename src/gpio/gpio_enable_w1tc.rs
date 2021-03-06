#[doc = "Writer for register GPIO_ENABLE_W1TC"]
pub type W = crate::W<u32, super::GPIO_ENABLE_W1TC>;
#[doc = "Register GPIO_ENABLE_W1TC `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_ENABLE_W1TC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `GPIO_ENABLE_W1TC`"]
pub struct GPIO_ENABLE_W1TC_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_ENABLE_W1TC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff_ffff) | ((value as u32) & 0x03ff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:25"]
    #[inline(always)]
    pub fn gpio_enable_w1tc(&mut self) -> GPIO_ENABLE_W1TC_W {
        GPIO_ENABLE_W1TC_W { w: self }
    }
}
