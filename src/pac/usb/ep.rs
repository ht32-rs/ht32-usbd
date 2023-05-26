#[doc = r"Register block"]
#[repr(C)]
pub struct EP {
    #[doc = "0x00 - Endpoint control and status bits"]
    pub csr: CSR,
    #[doc = "0x04 - Endpoint interrupt enable control bits"]
    pub ier: IER,
    #[doc = "0x08 - Endpoint interrupt status"]
    pub isr: ISR,
    #[doc = "0x0c - Endpoint data transfer byte count"]
    pub tcr: TCR,
    #[doc = "0x10 - Endpoint configuration"]
    pub cfgr: CFGR,
}
#[doc = "CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`"]
pub type CSR = crate::pac::generic::Reg<csr::CSR_SPEC>;
#[doc = "Endpoint control and status bits"]
pub mod csr;
#[doc = "IER (rw) register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::pac::generic::Reg<ier::IER_SPEC>;
#[doc = "Endpoint interrupt enable control bits"]
pub mod ier;
#[doc = "ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::pac::generic::Reg<isr::ISR_SPEC>;
#[doc = "Endpoint interrupt status"]
pub mod isr;
#[doc = "TCR (rw) register accessor: an alias for `Reg<TCR_SPEC>`"]
pub type TCR = crate::pac::generic::Reg<tcr::TCR_SPEC>;
#[doc = "Endpoint data transfer byte count"]
pub mod tcr;
#[doc = "CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::pac::generic::Reg<cfgr::CFGR_SPEC>;
#[doc = "Endpoint configuration"]
pub mod cfgr;
