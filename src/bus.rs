//! USB peripheral driver.

use core::mem::{self, MaybeUninit};
use cortex_m::interrupt::{self, Mutex};
use usb_device::bus::{PollResult, UsbBusAllocator};
use usb_device::endpoint::{EndpointAddress, EndpointType};
use usb_device::{Result, UsbDirection, UsbError};

use super::endpoint::{Endpoint, NUM_ENDPOINTS};
use super::endpoint_memory::EndpointMemoryAllocator;
use super::registers::UsbRegisters;
use super::UsbPeripheral;

/// USB peripheral driver for HT32 microcontrollers.
pub struct UsbBus<USB> {
    peripheral: USB,
    regs: Mutex<UsbRegisters<USB>>,
    endpoints: [Endpoint<USB>; NUM_ENDPOINTS],
    ep_allocator: EndpointMemoryAllocator<USB>,
}

impl<USB: UsbPeripheral> UsbBus<USB> {
    /// Constructs a new USB peripheral driver.
    pub fn new(peripheral: USB) -> UsbBusAllocator<Self> {
        USB::enable(&peripheral);

        let bus = UsbBus {
            peripheral,
            regs: Mutex::new(UsbRegisters::new()),
            ep_allocator: EndpointMemoryAllocator::new(),
            endpoints: {
                let mut endpoints: [MaybeUninit<Endpoint<USB>>; NUM_ENDPOINTS] =
                    unsafe { MaybeUninit::uninit().assume_init() };

                for (i, endpoint) in endpoints.iter_mut().enumerate() {
                    *endpoint = MaybeUninit::new(Endpoint::new(i as u8));
                }

                unsafe { mem::transmute::<_, [Endpoint<USB>; NUM_ENDPOINTS]>(endpoints) }
            },
        };

        UsbBusAllocator::new(bus)
    }

    pub fn free(self) -> USB {
        self.peripheral
    }

    /// Simulates a disconnect from the USB bus, causing the host to reset and re-enumerate the
    /// device.
    ///
    /// Mostly used for development. By calling this at the start of your program ensures that the
    /// host re-enumerates your device after a new program has been flashed.
    pub fn force_reenumeration(&self) {
        interrupt::free(|cs| {
            let regs = self.regs.borrow(cs);

            let pdwn = regs.usb_csr.read().pdwn().bit_is_set();
            regs.usb_csr.modify(|_, w| w.pdwn().set_bit());
            regs.usb_csr.modify(|_, w| w.pdwn().bit(pdwn));
        });
    }
}

impl<USB: UsbPeripheral> usb_device::bus::UsbBus for UsbBus<USB> {
    fn alloc_ep(
        &mut self,
        ep_dir: UsbDirection,
        ep_addr: Option<EndpointAddress>,
        ep_type: EndpointType,
        max_packet_size: u16,
        _interval: u8,
    ) -> Result<EndpointAddress> {
        if ep_type == EndpointType::Control {
            if let Some(a) = ep_addr {
                if a.index() != 0 {
                    // Only EP0 can be used as control endpoint
                    return Err(UsbError::InvalidEndpoint);
                }
            }
            if !self.endpoints[0].is_buf_set() {
                let buffer = self
                    .ep_allocator
                    .allocate_double_buffer(max_packet_size as usize)?;
                self.endpoints[0].set_double_buf(buffer);
                self.endpoints[0].set_ep_type(ep_type);
                self.endpoints[0].set_ep_dir(ep_dir);
            }

            return Ok(EndpointAddress::from_parts(0, ep_dir));
        }

        let range = if let Some(a) = ep_addr {
            if a.index() == 0 {
                // EP0 is always a control endpoint
                return Err(UsbError::InvalidEndpoint);
            } else {
                a.index()..a.index() + 1
            }
        } else {
            1..8
        };

        for index in range {
            match ep_type {
                EndpointType::Isochronous { .. } if index < 4 => continue,
                _ => (),
            }

            let ep = &mut self.endpoints[index];
            match ep.ep_type() {
                None => {
                    ep.set_ep_type(ep_type);
                }
                Some(t) if t != ep_type => {
                    continue;
                }
                _ => {}
            };
            match ep.ep_dir() {
                None => {
                    ep.set_ep_dir(ep_dir);
                }
                Some(d) if d != ep_dir => {
                    continue;
                }
                _ => {}
            }
            if ep.is_buf_set() {
                continue;
            }

            let buffer = self
                .ep_allocator
                .allocate_buffer(max_packet_size as usize)?;
            ep.set_buf(buffer);

            return Ok(EndpointAddress::from_parts(index, ep_dir));
        }

        Err(match ep_addr {
            Some(_) => UsbError::InvalidEndpoint,
            None => UsbError::EndpointOverflow,
        })
    }

    fn enable(&mut self) {
        interrupt::free(|cs| {
            let regs = self.regs.borrow(cs);

            regs.usb_csr.write(|w| {
                if USB::DP_PULL_UP_FEATURE {
                    w.dppuen().set_bit().dpwken().set_bit();
                }
                w.pdwn().set_bit().lpmode().set_bit()
            });

            if !USB::DP_PULL_UP_FEATURE {
                self.peripheral.dp_pull_up();
            }

            regs.usb_isr.write(|w| unsafe { w.bits(!0) });

            if USB::DP_PULL_UP_FEATURE {
                regs.usb_csr.modify(|_, w| w.dpwken().clear_bit());
            }

            regs.usb_ier.write(|w| {
                w.ugie().set_bit();
                w.urstie().set_bit();
                w.rsmie().set_bit();
                w.suspie().set_bit()
            });
        });
    }

    fn reset(&self) {
        interrupt::free(|cs| {
            let regs = self.regs.borrow(cs);

            regs.usb_csr.modify(|r, w| {
                unsafe { w.bits(0) };
                if USB::DP_PULL_UP_FEATURE && r.dppuen().bit_is_set() {
                    w.dppuen().set_bit();
                }
                w
            });

            regs.usb_ier.write(|w| {
                w.ugie().set_bit();
                w.urstie().set_bit();
                w.rsmie().set_bit();
                w.suspie().set_bit()
            });

            for (i, ep) in self.endpoints.iter().enumerate() {
                if ep.configure(cs) {
                    regs.usb_ier.modify(|_, w| match i {
                        0 => w.ep0ie().set_bit(),
                        1 => w.ep1ie().set_bit(),
                        2 => w.ep2ie().set_bit(),
                        3 => w.ep3ie().set_bit(),
                        4 => w.ep4ie().set_bit(),
                        5 => w.ep5ie().set_bit(),
                        6 => w.ep6ie().set_bit(),
                        7 => w.ep7ie().set_bit(),
                        _ => w,
                    });
                }
            }
        });
    }

    fn set_device_address(&self, addr: u8) {
        interrupt::free(|cs| {
            let regs = self.regs.borrow(cs);
            // We don't set `USBCSR::ADRSET`, since usb-device will only set the address
            // after accepting the corresponding control transfer.
            regs.usb_devar.modify(|_, w| w.deva().variant(addr));
        });
    }

    fn poll(&self) -> PollResult {
        interrupt::free(|cs| {
            let regs = self.regs.borrow(cs);
            let istr = regs.usb_isr.read();

            if istr.rsmif().bit_is_set() {
                regs.usb_isr
                    .write(|w| w.esofif().set_bit().rsmif().set_bit());
                PollResult::Resume
            } else if istr.urstif().bit_is_set() {
                regs.usb_isr
                    .write(|w| w.esofif().set_bit().urstif().set_bit());
                PollResult::Reset
            } else if istr.suspif().bit_is_set() {
                regs.usb_isr
                    .write(|w| w.esofif().set_bit().suspif().set_bit());
                PollResult::Suspend
            } else if istr.bits() & 0xFF00 != 0 {
                let mut ep_out = 0;
                let mut ep_in_complete = 0;
                let mut ep_setup = 0;

                let ep_ifs = [
                    istr.ep0if(),
                    istr.ep1if(),
                    istr.ep2if(),
                    istr.ep3if(),
                    istr.ep4if(),
                    istr.ep5if(),
                    istr.ep6if(),
                    istr.ep7if(),
                ];

                for (index, ep_if) in ep_ifs.iter().enumerate() {
                    if ep_if.bit_is_set() {
                        let ep_istr = regs.ep[index].isr.read();
                        if index == 0 && ep_istr.sdrxif().bit_is_set() {
                            ep_setup |= 1 << index;
                        }
                        if ep_istr.odrxif().bit_is_set() {
                            ep_out |= 1 << index;
                        }
                        if ep_istr.idtxif().bit_is_set() {
                            ep_in_complete |= 1 << index;
                            regs.ep[index].isr.write(|w| w.idtxif().set_bit());
                        }
                    }
                }

                regs.usb_isr.write(|w| unsafe {
                    // Try to clear all endpoint interrupt flags
                    w.bits(0xFF00);
                    // For ESOFIF a `1` needs to written in order to have no effect
                    w.esofif().set_bit()
                });

                PollResult::Data {
                    ep_out,
                    ep_in_complete,
                    ep_setup,
                }
            } else {
                PollResult::None
            }
        })
    }

    fn write(&self, ep_addr: EndpointAddress, buf: &[u8]) -> Result<usize> {
        if !ep_addr.is_in() {
            return Err(UsbError::InvalidEndpoint);
        }

        self.endpoints[ep_addr.index()].write(buf)
    }

    fn read(&self, ep_addr: EndpointAddress, buf: &mut [u8]) -> Result<usize> {
        if !ep_addr.is_out() {
            return Err(UsbError::InvalidEndpoint);
        }

        let result = self.endpoints[ep_addr.index()].read(buf);
        // At this point we should be able to clear one of the endpoint interrupt flags.
        // To make it easy for ourselves, we try to clear all of them.
        interrupt::free(|cs| {
            let regs = self.regs.borrow(cs);
            regs.usb_isr.write(|w| unsafe {
                w.bits(0xFF00);
                w.esofif().set_bit()
            });
        });

        result
    }

    fn set_stalled(&self, ep_addr: EndpointAddress, stalled: bool) {
        interrupt::free(|_| {
            if self.is_stalled(ep_addr) == stalled {
                return;
            }

            self.endpoints[ep_addr.index()].toggle_stalled(ep_addr.direction())
        });
    }

    fn is_stalled(&self, ep_addr: EndpointAddress) -> bool {
        self.endpoints[ep_addr.index()].is_stalled(ep_addr.direction())
    }

    fn suspend(&self) {
        interrupt::free(|cs| {
            self.regs
                .borrow(cs)
                .usb_csr
                .modify(|_, w| w.lpmode().set_bit());
        });
    }

    fn resume(&self) {
        interrupt::free(|cs| {
            self.regs
                .borrow(cs)
                .usb_csr
                .modify(|_, w| w.lpmode().clear_bit());
        });
    }
}
