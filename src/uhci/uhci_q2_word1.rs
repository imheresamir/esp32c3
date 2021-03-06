#[doc = "Reader of register UHCI_Q2_WORD1"]
pub type R = crate::R<u32, super::UHCI_Q2_WORD1>;
#[doc = "Writer for register UHCI_Q2_WORD1"]
pub type W = crate::W<u32, super::UHCI_Q2_WORD1>;
#[doc = "Register UHCI_Q2_WORD1 `reset()`'s with value 0"]
impl crate::ResetValue for super::UHCI_Q2_WORD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UHCI_SEND_Q2_WORD1`"]
pub type UHCI_SEND_Q2_WORD1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UHCI_SEND_Q2_WORD1`"]
pub struct UHCI_SEND_Q2_WORD1_W<'a> {
    w: &'a mut W,
}
impl<'a> UHCI_SEND_Q2_WORD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn uhci_send_q2_word1(&self) -> UHCI_SEND_Q2_WORD1_R {
        UHCI_SEND_Q2_WORD1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn uhci_send_q2_word1(&mut self) -> UHCI_SEND_Q2_WORD1_W {
        UHCI_SEND_Q2_WORD1_W { w: self }
    }
}
