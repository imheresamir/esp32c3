#[doc = "Reader of register RMT_CH3STATUS"]
pub type R = crate::R<u32, super::RMT_CH3STATUS>;
#[doc = "Reader of field `RMT_APB_MEM_RD_ERR_CH3`"]
pub type RMT_APB_MEM_RD_ERR_CH3_R = crate::R<bool, bool>;
#[doc = "Reader of field `RMT_MEM_FULL_CH3`"]
pub type RMT_MEM_FULL_CH3_R = crate::R<bool, bool>;
#[doc = "Reader of field `RMT_MEM_OWNER_ERR_CH3`"]
pub type RMT_MEM_OWNER_ERR_CH3_R = crate::R<bool, bool>;
#[doc = "Reader of field `RMT_STATE_CH3`"]
pub type RMT_STATE_CH3_R = crate::R<u8, u8>;
#[doc = "Reader of field `RMT_APB_MEM_RADDR_CH3`"]
pub type RMT_APB_MEM_RADDR_CH3_R = crate::R<u16, u16>;
#[doc = "Reader of field `RMT_MEM_WADDR_EX_CH3`"]
pub type RMT_MEM_WADDR_EX_CH3_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rmt_apb_mem_rd_err_ch3(&self) -> RMT_APB_MEM_RD_ERR_CH3_R {
        RMT_APB_MEM_RD_ERR_CH3_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn rmt_mem_full_ch3(&self) -> RMT_MEM_FULL_CH3_R {
        RMT_MEM_FULL_CH3_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn rmt_mem_owner_err_ch3(&self) -> RMT_MEM_OWNER_ERR_CH3_R {
        RMT_MEM_OWNER_ERR_CH3_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 22:24"]
    #[inline(always)]
    pub fn rmt_state_ch3(&self) -> RMT_STATE_CH3_R {
        RMT_STATE_CH3_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 12:20"]
    #[inline(always)]
    pub fn rmt_apb_mem_raddr_ch3(&self) -> RMT_APB_MEM_RADDR_CH3_R {
        RMT_APB_MEM_RADDR_CH3_R::new(((self.bits >> 12) & 0x01ff) as u16)
    }
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn rmt_mem_waddr_ex_ch3(&self) -> RMT_MEM_WADDR_EX_CH3_R {
        RMT_MEM_WADDR_EX_CH3_R::new((self.bits & 0x01ff) as u16)
    }
}
