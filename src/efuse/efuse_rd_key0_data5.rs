#[doc = "Reader of register EFUSE_RD_KEY0_DATA5"]
pub type R = crate::R<u32, super::EFUSE_RD_KEY0_DATA5>;
#[doc = "Reader of field `EFUSE_KEY0_DATA5`"]
pub type EFUSE_KEY0_DATA5_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn efuse_key0_data5(&self) -> EFUSE_KEY0_DATA5_R {
        EFUSE_KEY0_DATA5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
