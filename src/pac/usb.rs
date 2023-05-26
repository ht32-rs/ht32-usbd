use crate::pac::generic::Reg;

#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - USB control bits and USB data line status"]
    pub usb_csr: USB_CSR,
    #[doc = "0x04 - USB interrupt enable control"]
    pub usb_ier: USB_IER,
    #[doc = "0x08 - USB interrupt status"]
    pub usb_isr: USB_ISR,
    #[doc = "0x0c - Lost Start-of-Frame number and the USB frame count"]
    pub usb_fcr: USB_FCR,
    #[doc = "0x10 - USB device address"]
    pub usb_devar: USB_DEVAR,
    #[doc = "0x14..0xb4 - USB endpoints"]
    pub ep: [EP; 8],
}

#[doc = "USB_CSR (rw) register accessor: an alias for `Reg<USB_CSR_SPEC>`"]
pub type USB_CSR = Reg<usb_csr::USB_CSR_SPEC>;
#[doc = "USB control bits and USB data line status"]
pub mod usb_csr;
#[doc = "USB_IER (rw) register accessor: an alias for `Reg<USB_IER_SPEC>`"]
pub type USB_IER = Reg<usb_ier::USB_IER_SPEC>;
#[doc = "USB interrupt enable control"]
pub mod usb_ier;
#[doc = "USB_ISR (rw) register accessor: an alias for `Reg<USB_ISR_SPEC>`"]
pub type USB_ISR = Reg<usb_isr::USB_ISR_SPEC>;
#[doc = "USB interrupt status"]
pub mod usb_isr;
#[doc = "USB_FCR (rw) register accessor: an alias for `Reg<USB_FCR_SPEC>`"]
pub type USB_FCR = Reg<usb_fcr::USB_FCR_SPEC>;
#[doc = "Lost Start-of-Frame number and the USB frame count"]
pub mod usb_fcr;
#[doc = "USB_DEVAR (rw) register accessor: an alias for `Reg<USB_DEVAR_SPEC>`"]
pub type USB_DEVAR = Reg<usb_devar::USB_DEVAR_SPEC>;
#[doc = "USB device address"]
pub mod usb_devar;
#[doc = "USB control endpoint"]
pub use self::ep::EP;
#[doc = "USB endpoints"]
pub mod ep;
