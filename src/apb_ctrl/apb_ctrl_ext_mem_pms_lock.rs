#[doc = "Reader of register APB_CTRL_EXT_MEM_PMS_LOCK"]
pub type R = crate::R<u32, super::APB_CTRL_EXT_MEM_PMS_LOCK>;
#[doc = "Writer for register APB_CTRL_EXT_MEM_PMS_LOCK"]
pub type W = crate::W<u32, super::APB_CTRL_EXT_MEM_PMS_LOCK>;
#[doc = "Register APB_CTRL_EXT_MEM_PMS_LOCK `reset()`'s with value 0"]
impl crate::ResetValue for super::APB_CTRL_EXT_MEM_PMS_LOCK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_CTRL_EXT_MEM_PMS_LOCK`"]
pub type APB_CTRL_EXT_MEM_PMS_LOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APB_CTRL_EXT_MEM_PMS_LOCK`"]
pub struct APB_CTRL_EXT_MEM_PMS_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_CTRL_EXT_MEM_PMS_LOCK_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn apb_ctrl_ext_mem_pms_lock(&self) -> APB_CTRL_EXT_MEM_PMS_LOCK_R {
        APB_CTRL_EXT_MEM_PMS_LOCK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn apb_ctrl_ext_mem_pms_lock(&mut self) -> APB_CTRL_EXT_MEM_PMS_LOCK_W {
        APB_CTRL_EXT_MEM_PMS_LOCK_W { w: self }
    }
}
