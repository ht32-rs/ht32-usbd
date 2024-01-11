use core::marker::PhantomData;
use cortex_m::interrupt::{self, CriticalSection, Mutex};
use usb_device::endpoint::EndpointType;
use usb_device::{Result, UsbDirection, UsbError};

use super::endpoint_memory::{EndpointBuffer, EndpointDoubleBuffer, EndpointMemoryAllocator};
use super::registers::UsbRegisters;
use super::UsbPeripheral;
use crate::pac::usb;

pub const NUM_ENDPOINTS: usize = 8;

pub struct Endpoint<USB> {
    buf0: Option<Mutex<EndpointBuffer<USB>>>,
    buf1: Option<Mutex<EndpointBuffer<USB>>>,
    ep_dir: Option<UsbDirection>,
    ep_type: Option<EndpointType>,
    index: u8,
    _marker: PhantomData<USB>,
}

impl<USB: UsbPeripheral> Endpoint<USB> {
    pub fn new(index: u8) -> Self {
        Self {
            buf0: None,
            buf1: None,
            ep_dir: None,
            ep_type: None,
            index,
            _marker: PhantomData,
        }
    }

    pub fn ep_dir(&self) -> Option<UsbDirection> {
        self.ep_dir
    }

    pub fn set_ep_dir(&mut self, ep_dir: UsbDirection) {
        self.ep_dir = Some(ep_dir);
    }

    pub fn ep_type(&self) -> Option<EndpointType> {
        self.ep_type
    }

    pub fn set_ep_type(&mut self, ep_type: EndpointType) {
        self.ep_type = Some(ep_type);
    }

    pub fn is_buf_set(&self) -> bool {
        // TODO: is this check enough?
        self.buf0.is_some()
    }

    /// Use single buffered mode.
    pub fn set_buf(&mut self, buffer: EndpointBuffer<USB>) {
        let offset = buffer.offset();
        let size = buffer.capacity();
        self.buf0 = Some(Mutex::new(buffer));

        self.regs().cfgr.modify(|_, w| {
            w.epbufa().variant(offset);
            w.eplen().variant(size as u16);
            w.sdbs().clear_bit()
        });
    }

    /// Use double buffered mode, or set IN + OUT buffers in case of EP0
    pub fn set_double_buf(&mut self, buffer: EndpointDoubleBuffer<USB>) {
        if self.index > 0 && self.index < 4 {
            return;
        }

        let offset = buffer.0.offset();
        let size = buffer.0.capacity();
        self.buf0 = Some(Mutex::new(buffer.0));
        self.buf1 = Some(Mutex::new(buffer.1));

        self.regs().cfgr.modify(|_, w| {
            if self.index >= 4 {
                w.sdbs().set_bit();
            }
            w.epbufa().variant(offset);
            w.eplen().variant(size as u16)
        });
    }

    pub fn configure(&self, _cs: &CriticalSection) -> bool {
        let ep_type = match self.ep_type {
            Some(t) => t,
            None => return false,
        };

        let ep_dir = match self.ep_dir {
            Some(d) => d,
            None => return false,
        };

        self.regs().cfgr.modify(|_, w| {
            if self.index != 0 {
                w.epdir().bit(ep_dir == UsbDirection::In);
            }
            if self.index >= 4 {
                match ep_type {
                    EndpointType::Isochronous { .. } => w.eptype().set_bit(),
                    _ => w.eptype().clear_bit(),
                };
            }
            w.epadr().variant(self.index);
            w.epen().set_bit()
        });

        self.regs().ier.write(|w| {
            if self.index == 0 {
                w.sdrxie().set_bit();
            }
            w.odrxie().set_bit();
            w.idtxie().set_bit()
        });

        true
    }

    pub fn write(&self, buf: &[u8]) -> Result<usize> {
        interrupt::free(|cs| {
            // TODO: for double buffered endpoint, this needs more logic
            let in_buf = self.buf0.as_ref().unwrap().borrow(cs);

            if buf.len() > in_buf.capacity() {
                return Err(UsbError::BufferOverflow);
            }

            if self.regs().tcr.read().tcnt0().bits() != 0 {
                return Err(UsbError::WouldBlock);
            }

            in_buf.write(buf);
            // Set the number of bytes to be transmitted
            self.regs()
                .tcr
                .modify(|_, w| unsafe { w.tcnt0().bits(buf.len() as u16) });
            // Clear NAKTX
            if self.regs().csr.read().naktx().bit_is_set() {
                self.regs().csr.write(|w| w.naktx().set_bit());
            }

            Ok(buf.len())
        })
    }

    pub fn read(&self, buf: &mut [u8]) -> Result<usize> {
        interrupt::free(|cs| {
            let istr = self.regs().isr.read();
            let result = if self.index == 0 && istr.sdrxif().bit_is_set() {
                self.regs().isr.write(|w| w.sdrxif().set_bit());
                // Read SETUP packet in dedicated section of EP-SRAM
                let setup_buf = EndpointMemoryAllocator::<USB>::setup_buffer();
                if 8 > buf.len() {
                    Err(UsbError::BufferOverflow)
                } else {
                    setup_buf.read(&mut buf[0..8]);
                    Ok(8)
                }
            } else if istr.odrxif().bit_is_set() {
                self.regs().isr.write(|w| w.odrxif().set_bit());
                let (out_buf, count) = if self.index == 0 {
                    let buf = self.buf1.as_ref().unwrap().borrow(cs);
                    let count = self.regs().tcr.read().tcnt1().bits() as usize;
                    (buf, count)
                } else {
                    // TODO: for double buffered endpoint, this needs more logic
                    let buf = self.buf0.as_ref().unwrap().borrow(cs);
                    let count = self.regs().tcr.read().tcnt0().bits() as usize;
                    (buf, count)
                };

                if count > buf.len() {
                    Err(UsbError::BufferOverflow)
                } else {
                    out_buf.read(&mut buf[0..count]);
                    Ok(count)
                }
            } else {
                Err(UsbError::WouldBlock)
            };

            // Clear NAKRX
            if self.regs().csr.read().nakrx().bit_is_set() {
                self.regs().csr.write(|w| w.nakrx().set_bit());
            }

            result
        })
    }

    /// Toggle the STALL bit on the endpoint
    pub fn toggle_stalled(&self, dir: UsbDirection) {
        if dir == UsbDirection::Out {
            self.regs().csr.write(|w| w.stlrx().set_bit());
        } else {
            self.regs().csr.write(|w| w.stltx().set_bit());
        }
    }

    /// Get stalled status of the endpoint
    pub fn is_stalled(&self, dir: UsbDirection) -> bool {
        if dir == UsbDirection::Out {
            self.regs().csr.read().stlrx().bit_is_set()
        } else {
            self.regs().csr.read().stltx().bit_is_set()
        }
    }

    #[inline(always)]
    fn regs(&self) -> &usb::EP {
        UsbRegisters::<USB>::ep_register(self.index)
    }
}
