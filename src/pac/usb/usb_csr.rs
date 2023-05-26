#[doc = "Register `USB_CSR` reader"]
pub struct R(crate::pac::generic::R<USB_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::pac::generic::R<USB_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::pac::generic::R<USB_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::pac::generic::R<USB_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB_CSR` writer"]
pub struct W(crate::pac::generic::W<USB_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::pac::generic::W<USB_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::pac::generic::W<USB_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::pac::generic::W<USB_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRES` reader - FRES"]
pub type FRES_R = crate::pac::generic::BitReader<bool>;
#[doc = "Field `FRES` writer - FRES"]
pub type FRES_W<'a, const O: u8> = crate::pac::generic::BitWriter<'a, u32, USB_CSR_SPEC, bool, O>;
#[doc = "Field `PDWN` reader - PDWN"]
pub type PDWN_R = crate::pac::generic::BitReader<bool>;
#[doc = "Field `PDWN` writer - PDWN"]
pub type PDWN_W<'a, const O: u8> = crate::pac::generic::BitWriter<'a, u32, USB_CSR_SPEC, bool, O>;
#[doc = "Field `LPMODE` reader - LPMODE"]
pub type LPMODE_R = crate::pac::generic::BitReader<bool>;
#[doc = "Field `LPMODE` writer - LPMODE"]
pub type LPMODE_W<'a, const O: u8> = crate::pac::generic::BitWriter<'a, u32, USB_CSR_SPEC, bool, O>;
#[doc = "Field `GENRSM` reader - GENRSM"]
pub type GENRSM_R = crate::pac::generic::BitReader<bool>;
#[doc = "Field `GENRSM` writer - GENRSM"]
pub type GENRSM_W<'a, const O: u8> = crate::pac::generic::BitWriter<'a, u32, USB_CSR_SPEC, bool, O>;
#[doc = "Field `RXDP` reader - RXDP"]
pub type RXDP_R = crate::pac::generic::BitReader<bool>;
#[doc = "Field `RXDP` writer - RXDP"]
pub type RXDP_W<'a, const O: u8> = crate::pac::generic::BitWriter<'a, u32, USB_CSR_SPEC, bool, O>;
#[doc = "Field `RXDM` reader - RXDM"]
pub type RXDM_R = crate::pac::generic::BitReader<bool>;
#[doc = "Field `RXDM` writer - RXDM"]
pub type RXDM_W<'a, const O: u8> = crate::pac::generic::BitWriter<'a, u32, USB_CSR_SPEC, bool, O>;
#[doc = "Field `ADRSET` reader - ADRSET"]
pub type ADRSET_R = crate::pac::generic::BitReader<bool>;
#[doc = "Field `ADRSET` writer - ADRSET"]
pub type ADRSET_W<'a, const O: u8> = crate::pac::generic::BitWriter<'a, u32, USB_CSR_SPEC, bool, O>;
#[doc = "Field `SRAMRSTC` reader - SRAMRSTC"]
pub type SRAMRSTC_R = crate::pac::generic::BitReader<bool>;
#[doc = "Field `SRAMRSTC` writer - SRAMRSTC"]
pub type SRAMRSTC_W<'a, const O: u8> = crate::pac::generic::BitWriter<'a, u32, USB_CSR_SPEC, bool, O>;
#[doc = "Field `DPPUEN` reader - DPPUEN"]
pub type DPPUEN_R = crate::pac::generic::BitReader<bool>;
#[doc = "Field `DPPUEN` writer - DPPUEN"]
pub type DPPUEN_W<'a, const O: u8> = crate::pac::generic::BitWriter<'a, u32, USB_CSR_SPEC, bool, O>;
#[doc = "Field `DPWKEN` reader - DPWKEN"]
pub type DPWKEN_R = crate::pac::generic::BitReader<bool>;
#[doc = "Field `DPWKEN` writer - DPWKEN"]
pub type DPWKEN_W<'a, const O: u8> = crate::pac::generic::BitWriter<'a, u32, USB_CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - FRES"]
    #[inline(always)]
    pub fn fres(&self) -> FRES_R {
        FRES_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDWN"]
    #[inline(always)]
    pub fn pdwn(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPMODE"]
    #[inline(always)]
    pub fn lpmode(&self) -> LPMODE_R {
        LPMODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - GENRSM"]
    #[inline(always)]
    pub fn genrsm(&self) -> GENRSM_R {
        GENRSM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXDP"]
    #[inline(always)]
    pub fn rxdp(&self) -> RXDP_R {
        RXDP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RXDM"]
    #[inline(always)]
    pub fn rxdm(&self) -> RXDM_R {
        RXDM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADRSET"]
    #[inline(always)]
    pub fn adrset(&self) -> ADRSET_R {
        ADRSET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAMRSTC"]
    #[inline(always)]
    pub fn sramrstc(&self) -> SRAMRSTC_R {
        SRAMRSTC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DPPUEN"]
    #[inline(always)]
    pub fn dppuen(&self) -> DPPUEN_R {
        DPPUEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DPWKEN"]
    #[inline(always)]
    pub fn dpwken(&self) -> DPWKEN_R {
        DPWKEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FRES"]
    #[inline(always)]
    #[must_use]
    pub fn fres(&mut self) -> FRES_W<1> {
        FRES_W::new(self)
    }
    #[doc = "Bit 2 - PDWN"]
    #[inline(always)]
    #[must_use]
    pub fn pdwn(&mut self) -> PDWN_W<2> {
        PDWN_W::new(self)
    }
    #[doc = "Bit 3 - LPMODE"]
    #[inline(always)]
    #[must_use]
    pub fn lpmode(&mut self) -> LPMODE_W<3> {
        LPMODE_W::new(self)
    }
    #[doc = "Bit 5 - GENRSM"]
    #[inline(always)]
    #[must_use]
    pub fn genrsm(&mut self) -> GENRSM_W<5> {
        GENRSM_W::new(self)
    }
    #[doc = "Bit 6 - RXDP"]
    #[inline(always)]
    #[must_use]
    pub fn rxdp(&mut self) -> RXDP_W<6> {
        RXDP_W::new(self)
    }
    #[doc = "Bit 7 - RXDM"]
    #[inline(always)]
    #[must_use]
    pub fn rxdm(&mut self) -> RXDM_W<7> {
        RXDM_W::new(self)
    }
    #[doc = "Bit 8 - ADRSET"]
    #[inline(always)]
    #[must_use]
    pub fn adrset(&mut self) -> ADRSET_W<8> {
        ADRSET_W::new(self)
    }
    #[doc = "Bit 9 - SRAMRSTC"]
    #[inline(always)]
    #[must_use]
    pub fn sramrstc(&mut self) -> SRAMRSTC_W<9> {
        SRAMRSTC_W::new(self)
    }
    #[doc = "Bit 10 - DPPUEN"]
    #[inline(always)]
    #[must_use]
    pub fn dppuen(&mut self) -> DPPUEN_W<10> {
        DPPUEN_W::new(self)
    }
    #[doc = "Bit 11 - DPWKEN"]
    #[inline(always)]
    #[must_use]
    pub fn dpwken(&mut self) -> DPWKEN_W<11> {
        DPWKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB control bits and USB data line status\n\nThis register you can [`read`](Reg::read), [`write_with_zero`](Reg::write_with_zero), [`reset`](Reg::reset), [`write`](Reg::write), [`modify`](Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb_csr](index.html) module"]
pub struct USB_CSR_SPEC;
impl crate::pac::generic::RegisterSpec for USB_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb_csr::R](R) reader structure"]
impl crate::pac::generic::Readable for USB_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb_csr::W](W) writer structure"]
impl crate::pac::generic::Writable for USB_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_CSR to value 0"]
impl crate::pac::generic::Resettable for USB_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
