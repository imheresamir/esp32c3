#[doc = "Reader of register SPI_W6"]
pub type R = crate::R<u32, super::SPI_W6>;
#[doc = "Writer for register SPI_W6"]
pub type W = crate::W<u32, super::SPI_W6>;
#[doc = "Register SPI_W6 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI_W6 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI_BUF6`"]
pub type SPI_BUF6_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SPI_BUF6`"]
pub struct SPI_BUF6_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_BUF6_W<'a> {
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
    pub fn spi_buf6(&self) -> SPI_BUF6_R {
        SPI_BUF6_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn spi_buf6(&mut self) -> SPI_BUF6_W {
        SPI_BUF6_W { w: self }
    }
}
