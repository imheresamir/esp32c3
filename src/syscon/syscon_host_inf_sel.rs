#[doc = "Reader of register SYSCON_HOST_INF_SEL"]
pub type R = crate::R<u32, super::SYSCON_HOST_INF_SEL>;
#[doc = "Writer for register SYSCON_HOST_INF_SEL"]
pub type W = crate::W<u32, super::SYSCON_HOST_INF_SEL>;
#[doc = "Register SYSCON_HOST_INF_SEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCON_HOST_INF_SEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SYSCON_PERI_IO_SWAP`"]
pub type SYSCON_PERI_IO_SWAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SYSCON_PERI_IO_SWAP`"]
pub struct SYSCON_PERI_IO_SWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSCON_PERI_IO_SWAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn syscon_peri_io_swap(&self) -> SYSCON_PERI_IO_SWAP_R {
        SYSCON_PERI_IO_SWAP_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn syscon_peri_io_swap(&mut self) -> SYSCON_PERI_IO_SWAP_W {
        SYSCON_PERI_IO_SWAP_W { w: self }
    }
}
