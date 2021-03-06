#[doc = "Reader of register EXTMEM_CORE0_DBUS_REJECT_ST"]
pub type R = crate::R<u32, super::EXTMEM_CORE0_DBUS_REJECT_ST>;
#[doc = "Reader of field `EXTMEM_CORE0_DBUS_WORLD`"]
pub type EXTMEM_CORE0_DBUS_WORLD_R = crate::R<bool, bool>;
#[doc = "Reader of field `EXTMEM_CORE0_DBUS_ATTR`"]
pub type EXTMEM_CORE0_DBUS_ATTR_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn extmem_core0_dbus_world(&self) -> EXTMEM_CORE0_DBUS_WORLD_R {
        EXTMEM_CORE0_DBUS_WORLD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn extmem_core0_dbus_attr(&self) -> EXTMEM_CORE0_DBUS_ATTR_R {
        EXTMEM_CORE0_DBUS_ATTR_R::new((self.bits & 0x07) as u8)
    }
}
