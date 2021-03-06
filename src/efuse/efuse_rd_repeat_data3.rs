#[doc = "Reader of register EFUSE_RD_REPEAT_DATA3"]
pub type R = crate::R<u32, super::EFUSE_RD_REPEAT_DATA3>;
#[doc = "Reader of field `EFUSE_RPT4_RESERVED1`"]
pub type EFUSE_RPT4_RESERVED1_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_SECURE_VERSION`"]
pub type EFUSE_SECURE_VERSION_R = crate::R<u16, u16>;
#[doc = "Reader of field `EFUSE_FORCE_SEND_RESUME`"]
pub type EFUSE_FORCE_SEND_RESUME_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_FLASH_ECC_EN`"]
pub type EFUSE_FLASH_ECC_EN_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_FLASH_PAGE_SIZE`"]
pub type EFUSE_FLASH_PAGE_SIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_FLASH_TYPE`"]
pub type EFUSE_FLASH_TYPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_PIN_POWER_SELECTION`"]
pub type EFUSE_PIN_POWER_SELECTION_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_UART_PRINT_CONTROL`"]
pub type EFUSE_UART_PRINT_CONTROL_R = crate::R<u8, u8>;
#[doc = "Reader of field `EFUSE_ENABLE_SECURITY_DOWNLOAD`"]
pub type EFUSE_ENABLE_SECURITY_DOWNLOAD_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_DIS_USB_DOWNLOAD_MODE`"]
pub type EFUSE_DIS_USB_DOWNLOAD_MODE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_FLASH_ECC_MODE`"]
pub type EFUSE_FLASH_ECC_MODE_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_UART_PRINT_CHANNEL`"]
pub type EFUSE_UART_PRINT_CHANNEL_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_DIS_LEGACY_SPI_BOOT`"]
pub type EFUSE_DIS_LEGACY_SPI_BOOT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EFUSE_DIS_DOWNLOAD_MODE`"]
pub type EFUSE_DIS_DOWNLOAD_MODE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 30:31"]
    #[inline(always)]
    pub fn efuse_rpt4_reserved1(&self) -> EFUSE_RPT4_RESERVED1_R {
        EFUSE_RPT4_RESERVED1_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 14:29"]
    #[inline(always)]
    pub fn efuse_secure_version(&self) -> EFUSE_SECURE_VERSION_R {
        EFUSE_SECURE_VERSION_R::new(((self.bits >> 14) & 0xffff) as u16)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn efuse_force_send_resume(&self) -> EFUSE_FORCE_SEND_RESUME_R {
        EFUSE_FORCE_SEND_RESUME_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn efuse_flash_ecc_en(&self) -> EFUSE_FLASH_ECC_EN_R {
        EFUSE_FLASH_ECC_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn efuse_flash_page_size(&self) -> EFUSE_FLASH_PAGE_SIZE_R {
        EFUSE_FLASH_PAGE_SIZE_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn efuse_flash_type(&self) -> EFUSE_FLASH_TYPE_R {
        EFUSE_FLASH_TYPE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn efuse_pin_power_selection(&self) -> EFUSE_PIN_POWER_SELECTION_R {
        EFUSE_PIN_POWER_SELECTION_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn efuse_uart_print_control(&self) -> EFUSE_UART_PRINT_CONTROL_R {
        EFUSE_UART_PRINT_CONTROL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn efuse_enable_security_download(&self) -> EFUSE_ENABLE_SECURITY_DOWNLOAD_R {
        EFUSE_ENABLE_SECURITY_DOWNLOAD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn efuse_dis_usb_download_mode(&self) -> EFUSE_DIS_USB_DOWNLOAD_MODE_R {
        EFUSE_DIS_USB_DOWNLOAD_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn efuse_flash_ecc_mode(&self) -> EFUSE_FLASH_ECC_MODE_R {
        EFUSE_FLASH_ECC_MODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn efuse_uart_print_channel(&self) -> EFUSE_UART_PRINT_CHANNEL_R {
        EFUSE_UART_PRINT_CHANNEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn efuse_dis_legacy_spi_boot(&self) -> EFUSE_DIS_LEGACY_SPI_BOOT_R {
        EFUSE_DIS_LEGACY_SPI_BOOT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn efuse_dis_download_mode(&self) -> EFUSE_DIS_DOWNLOAD_MODE_R {
        EFUSE_DIS_DOWNLOAD_MODE_R::new((self.bits & 0x01) != 0)
    }
}
