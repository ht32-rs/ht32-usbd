#[doc = "Register `CSR` reader"]
pub struct R(crate::pac::generic::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::pac::generic::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::pac::generic::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::pac::generic::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::pac::generic::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::pac::generic::W<CSR_SPEC>;
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
impl From<crate::pac::generic::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::pac::generic::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTGTX` reader - DTGTX"]
pub type DTGTX_R = crate::pac::generic::BitReader<bool>;
#[doc = "Field `DTGTX` writer - DTGTX"]
pub type DTGTX_W<'a, const O: u8> = crate::pac::generic::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `NAKTX` reader - NAKTX"]
pub type NAKTX_R = crate::pac::generic::BitReader<bool>;
#[doc = "Field `NAKTX` writer - NAKTX"]
pub type NAKTX_W<'a, const O: u8> = crate::pac::generic::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `STLTX` reader - STLTX"]
pub type STLTX_R = crate::pac::generic::BitReader<bool>;
#[doc = "Field `STLTX` writer - STLTX"]
pub type STLTX_W<'a, const O: u8> = crate::pac::generic::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `DTGRX` reader - DTGRX"]
pub type DTGRX_R = crate::pac::generic::BitReader<bool>;
#[doc = "Field `DTGRX` writer - DTGRX"]
pub type DTGRX_W<'a, const O: u8> = crate::pac::generic::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `NAKRX` reader - NAKRX"]
pub type NAKRX_R = crate::pac::generic::BitReader<bool>;
#[doc = "Field `NAKRX` writer - NAKRX"]
pub type NAKRX_W<'a, const O: u8> = crate::pac::generic::BitWriter<'a, u32, CSR_SPEC, bool, O>;
#[doc = "Field `STLRX` reader - STLRX"]
pub type STLRX_R = crate::pac::generic::BitReader<bool>;
#[doc = "Field `STLRX` writer - STLRX"]
pub type STLRX_W<'a, const O: u8> = crate::pac::generic::BitWriter<'a, u32, CSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - DTGTX"]
    #[inline(always)]
    pub fn dtgtx(&self) -> DTGTX_R {
        DTGTX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NAKTX"]
    #[inline(always)]
    pub fn naktx(&self) -> NAKTX_R {
        NAKTX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - STLTX"]
    #[inline(always)]
    pub fn stltx(&self) -> STLTX_R {
        STLTX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTGRX"]
    #[inline(always)]
    pub fn dtgrx(&self) -> DTGRX_R {
        DTGRX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAKRX"]
    #[inline(always)]
    pub fn nakrx(&self) -> NAKRX_R {
        NAKRX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - STLRX"]
    #[inline(always)]
    pub fn stlrx(&self) -> STLRX_R {
        STLRX_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTGTX"]
    #[inline(always)]
    #[must_use]
    pub fn dtgtx(&mut self) -> DTGTX_W<0> {
        DTGTX_W::new(self)
    }
    #[doc = "Bit 1 - NAKTX"]
    #[inline(always)]
    #[must_use]
    pub fn naktx(&mut self) -> NAKTX_W<1> {
        NAKTX_W::new(self)
    }
    #[doc = "Bit 2 - STLTX"]
    #[inline(always)]
    #[must_use]
    pub fn stltx(&mut self) -> STLTX_W<2> {
        STLTX_W::new(self)
    }
    #[doc = "Bit 3 - DTGRX"]
    #[inline(always)]
    #[must_use]
    pub fn dtgrx(&mut self) -> DTGRX_W<3> {
        DTGRX_W::new(self)
    }
    #[doc = "Bit 4 - NAKRX"]
    #[inline(always)]
    #[must_use]
    pub fn nakrx(&mut self) -> NAKRX_W<4> {
        NAKRX_W::new(self)
    }
    #[doc = "Bit 5 - STLRX"]
    #[inline(always)]
    #[must_use]
    pub fn stlrx(&mut self) -> STLRX_W<5> {
        STLRX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB_EP0CSR\n\nThis register you can [`read`](crate::pac::generic::Reg::read), [`write_with_zero`](crate::pac::generic::Reg::write_with_zero), [`reset`](crate::pac::generic::Reg::reset), [`write`](crate::pac::generic::Reg::write), [`modify`](crate::pac::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::pac::generic::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::pac::generic::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::pac::generic::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::pac::generic::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
