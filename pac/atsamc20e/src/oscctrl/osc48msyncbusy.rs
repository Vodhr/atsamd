#[doc = "Register `OSC48MSYNCBUSY` reader"]
pub type R = crate::R<Osc48msyncbusySpec>;
#[doc = "Field `OSC48MDIV` reader - OSC48MDIV Synchronization Status"]
pub type Osc48mdivR = crate::BitReader;
impl R {
    #[doc = "Bit 2 - OSC48MDIV Synchronization Status"]
    #[inline(always)]
    pub fn osc48mdiv(&self) -> Osc48mdivR {
        Osc48mdivR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "OSC48M Synchronization Busy\n\nYou can [`read`](crate::Reg::read) this register and get [`osc48msyncbusy::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Osc48msyncbusySpec;
impl crate::RegisterSpec for Osc48msyncbusySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osc48msyncbusy::R`](R) reader structure"]
impl crate::Readable for Osc48msyncbusySpec {}
#[doc = "`reset()` method sets OSC48MSYNCBUSY to value 0"]
impl crate::Resettable for Osc48msyncbusySpec {
    const RESET_VALUE: u32 = 0;
}
