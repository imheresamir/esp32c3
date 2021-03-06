#[doc = "Reader of register LEDC_LSCH1_DUTY"]
pub type R = crate::R<u32, super::LEDC_LSCH1_DUTY>;
#[doc = "Writer for register LEDC_LSCH1_DUTY"]
pub type W = crate::W<u32, super::LEDC_LSCH1_DUTY>;
#[doc = "Register LEDC_LSCH1_DUTY `reset()`'s with value 0"]
impl crate::ResetValue for super::LEDC_LSCH1_DUTY {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LEDC_DUTY_LSCH1`"]
pub type LEDC_DUTY_LSCH1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `LEDC_DUTY_LSCH1`"]
pub struct LEDC_DUTY_LSCH1_W<'a> {
    w: &'a mut W,
}
impl<'a> LEDC_DUTY_LSCH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0007_ffff) | ((value as u32) & 0x0007_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn ledc_duty_lsch1(&self) -> LEDC_DUTY_LSCH1_R {
        LEDC_DUTY_LSCH1_R::new((self.bits & 0x0007_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:18"]
    #[inline(always)]
    pub fn ledc_duty_lsch1(&mut self) -> LEDC_DUTY_LSCH1_W {
        LEDC_DUTY_LSCH1_W { w: self }
    }
}
