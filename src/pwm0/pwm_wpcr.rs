#[doc = "Writer for register PWM_WPCR"]
pub type W = crate::W<u32, super::PWM_WPCR>;
#[doc = "Register PWM_WPCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PWM_WPCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write Protection Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPCMD_AW {
    #[doc = "0: Disables the software write protection of the register groups of which the bit WPRGx is at '1'."]
    DISABLE_SW_PROT,
    #[doc = "1: Enables the software write protection of the register groups of which the bit WPRGx is at '1'."]
    ENABLE_SW_PROT,
    #[doc = "2: Enables the hardware write protection of the register groups of which the bit WPRGx is at '1'. Only a hardware reset of the PWM controller can disable the hardware write protection. Moreover, to meet security requirements, the PIO lines associated with the PWM can not be configured through the PIO interface."]
    ENABLE_HW_PROT,
}
impl From<WPCMD_AW> for u8 {
    #[inline(always)]
    fn from(variant: WPCMD_AW) -> Self {
        match variant {
            WPCMD_AW::DISABLE_SW_PROT => 0,
            WPCMD_AW::ENABLE_SW_PROT => 1,
            WPCMD_AW::ENABLE_HW_PROT => 2,
        }
    }
}
#[doc = "Write proxy for field `WPCMD`"]
pub struct WPCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> WPCMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPCMD_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Disables the software write protection of the register groups of which the bit WPRGx is at '1'."]
    #[inline(always)]
    pub fn disable_sw_prot(self) -> &'a mut W {
        self.variant(WPCMD_AW::DISABLE_SW_PROT)
    }
    #[doc = "Enables the software write protection of the register groups of which the bit WPRGx is at '1'."]
    #[inline(always)]
    pub fn enable_sw_prot(self) -> &'a mut W {
        self.variant(WPCMD_AW::ENABLE_SW_PROT)
    }
    #[doc = "Enables the hardware write protection of the register groups of which the bit WPRGx is at '1'. Only a hardware reset of the PWM controller can disable the hardware write protection. Moreover, to meet security requirements, the PIO lines associated with the PWM can not be configured through the PIO interface."]
    #[inline(always)]
    pub fn enable_hw_prot(self) -> &'a mut W {
        self.variant(WPCMD_AW::ENABLE_HW_PROT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Write proxy for field `WPRG0`"]
pub struct WPRG0_W<'a> {
    w: &'a mut W,
}
impl<'a> WPRG0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Write proxy for field `WPRG1`"]
pub struct WPRG1_W<'a> {
    w: &'a mut W,
}
impl<'a> WPRG1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Write proxy for field `WPRG2`"]
pub struct WPRG2_W<'a> {
    w: &'a mut W,
}
impl<'a> WPRG2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `WPRG3`"]
pub struct WPRG3_W<'a> {
    w: &'a mut W,
}
impl<'a> WPRG3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Write proxy for field `WPRG4`"]
pub struct WPRG4_W<'a> {
    w: &'a mut W,
}
impl<'a> WPRG4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Write proxy for field `WPRG5`"]
pub struct WPRG5_W<'a> {
    w: &'a mut W,
}
impl<'a> WPRG5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Write Protection Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WPKEY_AW {
    #[doc = "5265229: Writing any other value in this field aborts the write operation of the WPCMD field.Always reads as 0"]
    PASSWD,
}
impl From<WPKEY_AW> for u32 {
    #[inline(always)]
    fn from(variant: WPKEY_AW) -> Self {
        match variant {
            WPKEY_AW::PASSWD => 5265229,
        }
    }
}
#[doc = "Write proxy for field `WPKEY`"]
pub struct WPKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> WPKEY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WPKEY_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Writing any other value in this field aborts the write operation of the WPCMD field.Always reads as 0"]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(WPKEY_AW::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x00ff_ffff << 8)) | (((value as u32) & 0x00ff_ffff) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:1 - Write Protection Command"]
    #[inline(always)]
    pub fn wpcmd(&mut self) -> WPCMD_W {
        WPCMD_W { w: self }
    }
    #[doc = "Bit 2 - Write Protection Register Group 0"]
    #[inline(always)]
    pub fn wprg0(&mut self) -> WPRG0_W {
        WPRG0_W { w: self }
    }
    #[doc = "Bit 3 - Write Protection Register Group 1"]
    #[inline(always)]
    pub fn wprg1(&mut self) -> WPRG1_W {
        WPRG1_W { w: self }
    }
    #[doc = "Bit 4 - Write Protection Register Group 2"]
    #[inline(always)]
    pub fn wprg2(&mut self) -> WPRG2_W {
        WPRG2_W { w: self }
    }
    #[doc = "Bit 5 - Write Protection Register Group 3"]
    #[inline(always)]
    pub fn wprg3(&mut self) -> WPRG3_W {
        WPRG3_W { w: self }
    }
    #[doc = "Bit 6 - Write Protection Register Group 4"]
    #[inline(always)]
    pub fn wprg4(&mut self) -> WPRG4_W {
        WPRG4_W { w: self }
    }
    #[doc = "Bit 7 - Write Protection Register Group 5"]
    #[inline(always)]
    pub fn wprg5(&mut self) -> WPRG5_W {
        WPRG5_W { w: self }
    }
    #[doc = "Bits 8:31 - Write Protection Key"]
    #[inline(always)]
    pub fn wpkey(&mut self) -> WPKEY_W {
        WPKEY_W { w: self }
    }
}
