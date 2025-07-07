#[doc = "Register `SWEVT` writer"]
pub type W = crate::W<SwevtSpec>;
#[doc = "Field `CHANNEL0` writer - Channel 0 Software Selection"]
pub type Channel0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL1` writer - Channel 1 Software Selection"]
pub type Channel1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL2` writer - Channel 2 Software Selection"]
pub type Channel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL3` writer - Channel 3 Software Selection"]
pub type Channel3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL4` writer - Channel 4 Software Selection"]
pub type Channel4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHANNEL5` writer - Channel 5 Software Selection"]
pub type Channel5W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel 0 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel0(&mut self) -> Channel0W<SwevtSpec> {
        Channel0W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel1(&mut self) -> Channel1W<SwevtSpec> {
        Channel1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel 2 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel2(&mut self) -> Channel2W<SwevtSpec> {
        Channel2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel 3 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel3(&mut self) -> Channel3W<SwevtSpec> {
        Channel3W::new(self, 3)
    }
    #[doc = "Bit 4 - Channel 4 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel4(&mut self) -> Channel4W<SwevtSpec> {
        Channel4W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 5 Software Selection"]
    #[inline(always)]
    #[must_use]
    pub fn channel5(&mut self) -> Channel5W<SwevtSpec> {
        Channel5W::new(self, 5)
    }
}
#[doc = "Software Event\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swevt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwevtSpec;
impl crate::RegisterSpec for SwevtSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swevt::W`](W) writer structure"]
impl crate::Writable for SwevtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWEVT to value 0"]
impl crate::Resettable for SwevtSpec {
    const RESET_VALUE: u32 = 0;
}
