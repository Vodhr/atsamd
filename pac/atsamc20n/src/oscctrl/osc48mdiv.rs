#[doc = "Register `OSC48MDIV` reader"]
pub type R = crate::R<Osc48mdivSpec>;
#[doc = "Register `OSC48MDIV` writer"]
pub type W = crate::W<Osc48mdivSpec>;
#[doc = "OSC48M Division Factor\n\nValue on reset: 11"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Divselect {
    #[doc = "0: 48 MHz"]
    Div1 = 0,
    #[doc = "1: 24 MHz"]
    Div2 = 1,
    #[doc = "2: 16 MHz"]
    Div3 = 2,
    #[doc = "3: 12 MHz"]
    Div4 = 3,
    #[doc = "4: 9.6 MHz"]
    Div5 = 4,
    #[doc = "5: 8 MHz"]
    Div6 = 5,
    #[doc = "6: 6.86 MHz"]
    Div7 = 6,
    #[doc = "7: 6 MHz"]
    Div8 = 7,
    #[doc = "8: 5.33 MHz"]
    Div9 = 8,
    #[doc = "9: 4.8 MHz"]
    Div10 = 9,
    #[doc = "10: 4.36 MHz"]
    Div11 = 10,
    #[doc = "11: 4 MHz"]
    Div12 = 11,
    #[doc = "12: 3.69 MHz"]
    Div13 = 12,
    #[doc = "13: 3.43 MHz"]
    Div14 = 13,
    #[doc = "14: 3.2 MHz"]
    Div15 = 14,
    #[doc = "15: 3 MHz"]
    Div16 = 15,
}
impl From<Divselect> for u8 {
    #[inline(always)]
    fn from(variant: Divselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Divselect {
    type Ux = u8;
}
impl crate::IsEnum for Divselect {}
#[doc = "Field `DIV` reader - OSC48M Division Factor"]
pub type DivR = crate::FieldReader<Divselect>;
impl DivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Divselect {
        match self.bits {
            0 => Divselect::Div1,
            1 => Divselect::Div2,
            2 => Divselect::Div3,
            3 => Divselect::Div4,
            4 => Divselect::Div5,
            5 => Divselect::Div6,
            6 => Divselect::Div7,
            7 => Divselect::Div8,
            8 => Divselect::Div9,
            9 => Divselect::Div10,
            10 => Divselect::Div11,
            11 => Divselect::Div12,
            12 => Divselect::Div13,
            13 => Divselect::Div14,
            14 => Divselect::Div15,
            15 => Divselect::Div16,
            _ => unreachable!(),
        }
    }
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Divselect::Div1
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Divselect::Div2
    }
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == Divselect::Div3
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Divselect::Div4
    }
    #[doc = "9.6 MHz"]
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == Divselect::Div5
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == Divselect::Div6
    }
    #[doc = "6.86 MHz"]
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == Divselect::Div7
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Divselect::Div8
    }
    #[doc = "5.33 MHz"]
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == Divselect::Div9
    }
    #[doc = "4.8 MHz"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == Divselect::Div10
    }
    #[doc = "4.36 MHz"]
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == Divselect::Div11
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == Divselect::Div12
    }
    #[doc = "3.69 MHz"]
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == Divselect::Div13
    }
    #[doc = "3.43 MHz"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == Divselect::Div14
    }
    #[doc = "3.2 MHz"]
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == Divselect::Div15
    }
    #[doc = "3 MHz"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Divselect::Div16
    }
}
#[doc = "Field `DIV` writer - OSC48M Division Factor"]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 4, Divselect, crate::Safe>;
impl<'a, REG> DivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "48 MHz"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div1)
    }
    #[doc = "24 MHz"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div2)
    }
    #[doc = "16 MHz"]
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div3)
    }
    #[doc = "12 MHz"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div4)
    }
    #[doc = "9.6 MHz"]
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div5)
    }
    #[doc = "8 MHz"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div6)
    }
    #[doc = "6.86 MHz"]
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div7)
    }
    #[doc = "6 MHz"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div8)
    }
    #[doc = "5.33 MHz"]
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div9)
    }
    #[doc = "4.8 MHz"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div10)
    }
    #[doc = "4.36 MHz"]
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div11)
    }
    #[doc = "4 MHz"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div12)
    }
    #[doc = "3.69 MHz"]
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div13)
    }
    #[doc = "3.43 MHz"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div14)
    }
    #[doc = "3.2 MHz"]
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div15)
    }
    #[doc = "3 MHz"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Divselect::Div16)
    }
}
impl R {
    #[doc = "Bits 0:3 - OSC48M Division Factor"]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - OSC48M Division Factor"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<Osc48mdivSpec> {
        DivW::new(self, 0)
    }
}
#[doc = "OSC48M Divider\n\nYou can [`read`](crate::Reg::read) this register and get [`osc48mdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osc48mdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osc48mdivSpec;
impl crate::RegisterSpec for Osc48mdivSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`osc48mdiv::R`](R) reader structure"]
impl crate::Readable for Osc48mdivSpec {}
#[doc = "`write(|w| ..)` method takes [`osc48mdiv::W`](W) writer structure"]
impl crate::Writable for Osc48mdivSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OSC48MDIV to value 0x0b"]
impl crate::Resettable for Osc48mdivSpec {
    const RESET_VALUE: u8 = 0x0b;
}
