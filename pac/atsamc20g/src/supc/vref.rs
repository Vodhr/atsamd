#[doc = "Register `VREF` reader"]
pub type R = crate::R<VrefSpec>;
#[doc = "Register `VREF` writer"]
pub type W = crate::W<VrefSpec>;
#[doc = "Field `TSEN` reader - Temperature Sensor Output Enable"]
pub type TsenR = crate::BitReader;
#[doc = "Field `TSEN` writer - Temperature Sensor Output Enable"]
pub type TsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFOE` reader - Voltage Reference Output Enable"]
pub type VrefoeR = crate::BitReader;
#[doc = "Field `VREFOE` writer - Voltage Reference Output Enable"]
pub type VrefoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUNSTDBY` reader - Run during Standby"]
pub type RunstdbyR = crate::BitReader;
#[doc = "Field `RUNSTDBY` writer - Run during Standby"]
pub type RunstdbyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONDEMAND` reader - On Demand Contrl"]
pub type OndemandR = crate::BitReader;
#[doc = "Field `ONDEMAND` writer - On Demand Contrl"]
pub type OndemandW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Voltage Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Selselect {
    #[doc = "0: 1.024V voltage reference typical value"]
    _1v024 = 0,
    #[doc = "2: 2.048V voltage reference typical value"]
    _2v048 = 2,
    #[doc = "3: 4.096V voltage reference typical value"]
    _4v096 = 3,
}
impl From<Selselect> for u8 {
    #[inline(always)]
    fn from(variant: Selselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Selselect {
    type Ux = u8;
}
impl crate::IsEnum for Selselect {}
#[doc = "Field `SEL` reader - Voltage Reference Selection"]
pub type SelR = crate::FieldReader<Selselect>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Selselect> {
        match self.bits {
            0 => Some(Selselect::_1v024),
            2 => Some(Selselect::_2v048),
            3 => Some(Selselect::_4v096),
            _ => None,
        }
    }
    #[doc = "1.024V voltage reference typical value"]
    #[inline(always)]
    pub fn is_1v024(&self) -> bool {
        *self == Selselect::_1v024
    }
    #[doc = "2.048V voltage reference typical value"]
    #[inline(always)]
    pub fn is_2v048(&self) -> bool {
        *self == Selselect::_2v048
    }
    #[doc = "4.096V voltage reference typical value"]
    #[inline(always)]
    pub fn is_4v096(&self) -> bool {
        *self == Selselect::_4v096
    }
}
#[doc = "Field `SEL` writer - Voltage Reference Selection"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 4, Selselect>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1.024V voltage reference typical value"]
    #[inline(always)]
    pub fn _1v024(self) -> &'a mut crate::W<REG> {
        self.variant(Selselect::_1v024)
    }
    #[doc = "2.048V voltage reference typical value"]
    #[inline(always)]
    pub fn _2v048(self) -> &'a mut crate::W<REG> {
        self.variant(Selselect::_2v048)
    }
    #[doc = "4.096V voltage reference typical value"]
    #[inline(always)]
    pub fn _4v096(self) -> &'a mut crate::W<REG> {
        self.variant(Selselect::_4v096)
    }
}
impl R {
    #[doc = "Bit 1 - Temperature Sensor Output Enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TsenR {
        TsenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage Reference Output Enable"]
    #[inline(always)]
    pub fn vrefoe(&self) -> VrefoeR {
        VrefoeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    pub fn runstdby(&self) -> RunstdbyR {
        RunstdbyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - On Demand Contrl"]
    #[inline(always)]
    pub fn ondemand(&self) -> OndemandR {
        OndemandR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Temperature Sensor Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TsenW<VrefSpec> {
        TsenW::new(self, 1)
    }
    #[doc = "Bit 2 - Voltage Reference Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrefoe(&mut self) -> VrefoeW<VrefSpec> {
        VrefoeW::new(self, 2)
    }
    #[doc = "Bit 6 - Run during Standby"]
    #[inline(always)]
    #[must_use]
    pub fn runstdby(&mut self) -> RunstdbyW<VrefSpec> {
        RunstdbyW::new(self, 6)
    }
    #[doc = "Bit 7 - On Demand Contrl"]
    #[inline(always)]
    #[must_use]
    pub fn ondemand(&mut self) -> OndemandW<VrefSpec> {
        OndemandW::new(self, 7)
    }
    #[doc = "Bits 16:19 - Voltage Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<VrefSpec> {
        SelW::new(self, 16)
    }
}
#[doc = "VREF Control\n\nYou can [`read`](crate::Reg::read) this register and get [`vref::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vref::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VrefSpec;
impl crate::RegisterSpec for VrefSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vref::R`](R) reader structure"]
impl crate::Readable for VrefSpec {}
#[doc = "`write(|w| ..)` method takes [`vref::W`](W) writer structure"]
impl crate::Writable for VrefSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VREF to value 0"]
impl crate::Resettable for VrefSpec {
    const RESET_VALUE: u32 = 0;
}
