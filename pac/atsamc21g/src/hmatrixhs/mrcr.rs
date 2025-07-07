#[doc = "Register `MRCR` reader"]
pub type R = crate::R<MrcrSpec>;
#[doc = "Register `MRCR` writer"]
pub type W = crate::W<MrcrSpec>;
#[doc = "Remap Command Bit for Master 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb0select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb0select> for bool {
    #[inline(always)]
    fn from(variant: Rcb0select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB0` reader - Remap Command Bit for Master 0"]
pub type Rcb0R = crate::BitReader<Rcb0select>;
impl Rcb0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb0select {
        match self.bits {
            false => Rcb0select::Dis,
            true => Rcb0select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb0select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb0select::Ena
    }
}
#[doc = "Field `RCB0` writer - Remap Command Bit for Master 0"]
pub type Rcb0W<'a, REG> = crate::BitWriter<'a, REG, Rcb0select>;
impl<'a, REG> Rcb0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb0select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb0select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb1select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb1select> for bool {
    #[inline(always)]
    fn from(variant: Rcb1select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB1` reader - Remap Command Bit for Master 1"]
pub type Rcb1R = crate::BitReader<Rcb1select>;
impl Rcb1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb1select {
        match self.bits {
            false => Rcb1select::Dis,
            true => Rcb1select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb1select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb1select::Ena
    }
}
#[doc = "Field `RCB1` writer - Remap Command Bit for Master 1"]
pub type Rcb1W<'a, REG> = crate::BitWriter<'a, REG, Rcb1select>;
impl<'a, REG> Rcb1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb1select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb1select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb2select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb2select> for bool {
    #[inline(always)]
    fn from(variant: Rcb2select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB2` reader - Remap Command Bit for Master 2"]
pub type Rcb2R = crate::BitReader<Rcb2select>;
impl Rcb2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb2select {
        match self.bits {
            false => Rcb2select::Dis,
            true => Rcb2select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb2select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb2select::Ena
    }
}
#[doc = "Field `RCB2` writer - Remap Command Bit for Master 2"]
pub type Rcb2W<'a, REG> = crate::BitWriter<'a, REG, Rcb2select>;
impl<'a, REG> Rcb2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb2select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb2select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb3select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb3select> for bool {
    #[inline(always)]
    fn from(variant: Rcb3select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB3` reader - Remap Command Bit for Master 3"]
pub type Rcb3R = crate::BitReader<Rcb3select>;
impl Rcb3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb3select {
        match self.bits {
            false => Rcb3select::Dis,
            true => Rcb3select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb3select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb3select::Ena
    }
}
#[doc = "Field `RCB3` writer - Remap Command Bit for Master 3"]
pub type Rcb3W<'a, REG> = crate::BitWriter<'a, REG, Rcb3select>;
impl<'a, REG> Rcb3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb3select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb3select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb4select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb4select> for bool {
    #[inline(always)]
    fn from(variant: Rcb4select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB4` reader - Remap Command Bit for Master 4"]
pub type Rcb4R = crate::BitReader<Rcb4select>;
impl Rcb4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb4select {
        match self.bits {
            false => Rcb4select::Dis,
            true => Rcb4select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb4select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb4select::Ena
    }
}
#[doc = "Field `RCB4` writer - Remap Command Bit for Master 4"]
pub type Rcb4W<'a, REG> = crate::BitWriter<'a, REG, Rcb4select>;
impl<'a, REG> Rcb4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb4select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb4select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb5select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb5select> for bool {
    #[inline(always)]
    fn from(variant: Rcb5select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB5` reader - Remap Command Bit for Master 5"]
pub type Rcb5R = crate::BitReader<Rcb5select>;
impl Rcb5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb5select {
        match self.bits {
            false => Rcb5select::Dis,
            true => Rcb5select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb5select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb5select::Ena
    }
}
#[doc = "Field `RCB5` writer - Remap Command Bit for Master 5"]
pub type Rcb5W<'a, REG> = crate::BitWriter<'a, REG, Rcb5select>;
impl<'a, REG> Rcb5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb5select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb5select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb6select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb6select> for bool {
    #[inline(always)]
    fn from(variant: Rcb6select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB6` reader - Remap Command Bit for Master 6"]
pub type Rcb6R = crate::BitReader<Rcb6select>;
impl Rcb6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb6select {
        match self.bits {
            false => Rcb6select::Dis,
            true => Rcb6select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb6select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb6select::Ena
    }
}
#[doc = "Field `RCB6` writer - Remap Command Bit for Master 6"]
pub type Rcb6W<'a, REG> = crate::BitWriter<'a, REG, Rcb6select>;
impl<'a, REG> Rcb6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb6select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb6select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb7select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb7select> for bool {
    #[inline(always)]
    fn from(variant: Rcb7select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB7` reader - Remap Command Bit for Master 7"]
pub type Rcb7R = crate::BitReader<Rcb7select>;
impl Rcb7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb7select {
        match self.bits {
            false => Rcb7select::Dis,
            true => Rcb7select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb7select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb7select::Ena
    }
}
#[doc = "Field `RCB7` writer - Remap Command Bit for Master 7"]
pub type Rcb7W<'a, REG> = crate::BitWriter<'a, REG, Rcb7select>;
impl<'a, REG> Rcb7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb7select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb7select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb8select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb8select> for bool {
    #[inline(always)]
    fn from(variant: Rcb8select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB8` reader - Remap Command Bit for Master 8"]
pub type Rcb8R = crate::BitReader<Rcb8select>;
impl Rcb8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb8select {
        match self.bits {
            false => Rcb8select::Dis,
            true => Rcb8select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb8select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb8select::Ena
    }
}
#[doc = "Field `RCB8` writer - Remap Command Bit for Master 8"]
pub type Rcb8W<'a, REG> = crate::BitWriter<'a, REG, Rcb8select>;
impl<'a, REG> Rcb8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb8select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb8select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb9select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb9select> for bool {
    #[inline(always)]
    fn from(variant: Rcb9select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB9` reader - Remap Command Bit for Master 9"]
pub type Rcb9R = crate::BitReader<Rcb9select>;
impl Rcb9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb9select {
        match self.bits {
            false => Rcb9select::Dis,
            true => Rcb9select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb9select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb9select::Ena
    }
}
#[doc = "Field `RCB9` writer - Remap Command Bit for Master 9"]
pub type Rcb9W<'a, REG> = crate::BitWriter<'a, REG, Rcb9select>;
impl<'a, REG> Rcb9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb9select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb9select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb10select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb10select> for bool {
    #[inline(always)]
    fn from(variant: Rcb10select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB10` reader - Remap Command Bit for Master 10"]
pub type Rcb10R = crate::BitReader<Rcb10select>;
impl Rcb10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb10select {
        match self.bits {
            false => Rcb10select::Dis,
            true => Rcb10select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb10select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb10select::Ena
    }
}
#[doc = "Field `RCB10` writer - Remap Command Bit for Master 10"]
pub type Rcb10W<'a, REG> = crate::BitWriter<'a, REG, Rcb10select>;
impl<'a, REG> Rcb10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb10select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb10select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb11select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb11select> for bool {
    #[inline(always)]
    fn from(variant: Rcb11select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB11` reader - Remap Command Bit for Master 11"]
pub type Rcb11R = crate::BitReader<Rcb11select>;
impl Rcb11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb11select {
        match self.bits {
            false => Rcb11select::Dis,
            true => Rcb11select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb11select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb11select::Ena
    }
}
#[doc = "Field `RCB11` writer - Remap Command Bit for Master 11"]
pub type Rcb11W<'a, REG> = crate::BitWriter<'a, REG, Rcb11select>;
impl<'a, REG> Rcb11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb11select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb11select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb12select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb12select> for bool {
    #[inline(always)]
    fn from(variant: Rcb12select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB12` reader - Remap Command Bit for Master 12"]
pub type Rcb12R = crate::BitReader<Rcb12select>;
impl Rcb12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb12select {
        match self.bits {
            false => Rcb12select::Dis,
            true => Rcb12select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb12select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb12select::Ena
    }
}
#[doc = "Field `RCB12` writer - Remap Command Bit for Master 12"]
pub type Rcb12W<'a, REG> = crate::BitWriter<'a, REG, Rcb12select>;
impl<'a, REG> Rcb12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb12select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb12select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb13select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb13select> for bool {
    #[inline(always)]
    fn from(variant: Rcb13select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB13` reader - Remap Command Bit for Master 13"]
pub type Rcb13R = crate::BitReader<Rcb13select>;
impl Rcb13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb13select {
        match self.bits {
            false => Rcb13select::Dis,
            true => Rcb13select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb13select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb13select::Ena
    }
}
#[doc = "Field `RCB13` writer - Remap Command Bit for Master 13"]
pub type Rcb13W<'a, REG> = crate::BitWriter<'a, REG, Rcb13select>;
impl<'a, REG> Rcb13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb13select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb13select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb14select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb14select> for bool {
    #[inline(always)]
    fn from(variant: Rcb14select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB14` reader - Remap Command Bit for Master 14"]
pub type Rcb14R = crate::BitReader<Rcb14select>;
impl Rcb14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb14select {
        match self.bits {
            false => Rcb14select::Dis,
            true => Rcb14select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb14select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb14select::Ena
    }
}
#[doc = "Field `RCB14` writer - Remap Command Bit for Master 14"]
pub type Rcb14W<'a, REG> = crate::BitWriter<'a, REG, Rcb14select>;
impl<'a, REG> Rcb14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb14select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb14select::Ena)
    }
}
#[doc = "Remap Command Bit for Master 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rcb15select {
    #[doc = "0: Disable remapped address decoding for master"]
    Dis = 0,
    #[doc = "1: Enable remapped address decoding for master"]
    Ena = 1,
}
impl From<Rcb15select> for bool {
    #[inline(always)]
    fn from(variant: Rcb15select) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RCB15` reader - Remap Command Bit for Master 15"]
pub type Rcb15R = crate::BitReader<Rcb15select>;
impl Rcb15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rcb15select {
        match self.bits {
            false => Rcb15select::Dis,
            true => Rcb15select::Ena,
        }
    }
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Rcb15select::Dis
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn is_ena(&self) -> bool {
        *self == Rcb15select::Ena
    }
}
#[doc = "Field `RCB15` writer - Remap Command Bit for Master 15"]
pub type Rcb15W<'a, REG> = crate::BitWriter<'a, REG, Rcb15select>;
impl<'a, REG> Rcb15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable remapped address decoding for master"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb15select::Dis)
    }
    #[doc = "Enable remapped address decoding for master"]
    #[inline(always)]
    pub fn ena(self) -> &'a mut crate::W<REG> {
        self.variant(Rcb15select::Ena)
    }
}
impl R {
    #[doc = "Bit 0 - Remap Command Bit for Master 0"]
    #[inline(always)]
    pub fn rcb0(&self) -> Rcb0R {
        Rcb0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Remap Command Bit for Master 1"]
    #[inline(always)]
    pub fn rcb1(&self) -> Rcb1R {
        Rcb1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Remap Command Bit for Master 2"]
    #[inline(always)]
    pub fn rcb2(&self) -> Rcb2R {
        Rcb2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Remap Command Bit for Master 3"]
    #[inline(always)]
    pub fn rcb3(&self) -> Rcb3R {
        Rcb3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Remap Command Bit for Master 4"]
    #[inline(always)]
    pub fn rcb4(&self) -> Rcb4R {
        Rcb4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Remap Command Bit for Master 5"]
    #[inline(always)]
    pub fn rcb5(&self) -> Rcb5R {
        Rcb5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Remap Command Bit for Master 6"]
    #[inline(always)]
    pub fn rcb6(&self) -> Rcb6R {
        Rcb6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Remap Command Bit for Master 7"]
    #[inline(always)]
    pub fn rcb7(&self) -> Rcb7R {
        Rcb7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Remap Command Bit for Master 8"]
    #[inline(always)]
    pub fn rcb8(&self) -> Rcb8R {
        Rcb8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Remap Command Bit for Master 9"]
    #[inline(always)]
    pub fn rcb9(&self) -> Rcb9R {
        Rcb9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Remap Command Bit for Master 10"]
    #[inline(always)]
    pub fn rcb10(&self) -> Rcb10R {
        Rcb10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Remap Command Bit for Master 11"]
    #[inline(always)]
    pub fn rcb11(&self) -> Rcb11R {
        Rcb11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Remap Command Bit for Master 12"]
    #[inline(always)]
    pub fn rcb12(&self) -> Rcb12R {
        Rcb12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Remap Command Bit for Master 13"]
    #[inline(always)]
    pub fn rcb13(&self) -> Rcb13R {
        Rcb13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Remap Command Bit for Master 14"]
    #[inline(always)]
    pub fn rcb14(&self) -> Rcb14R {
        Rcb14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Remap Command Bit for Master 15"]
    #[inline(always)]
    pub fn rcb15(&self) -> Rcb15R {
        Rcb15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remap Command Bit for Master 0"]
    #[inline(always)]
    #[must_use]
    pub fn rcb0(&mut self) -> Rcb0W<MrcrSpec> {
        Rcb0W::new(self, 0)
    }
    #[doc = "Bit 1 - Remap Command Bit for Master 1"]
    #[inline(always)]
    #[must_use]
    pub fn rcb1(&mut self) -> Rcb1W<MrcrSpec> {
        Rcb1W::new(self, 1)
    }
    #[doc = "Bit 2 - Remap Command Bit for Master 2"]
    #[inline(always)]
    #[must_use]
    pub fn rcb2(&mut self) -> Rcb2W<MrcrSpec> {
        Rcb2W::new(self, 2)
    }
    #[doc = "Bit 3 - Remap Command Bit for Master 3"]
    #[inline(always)]
    #[must_use]
    pub fn rcb3(&mut self) -> Rcb3W<MrcrSpec> {
        Rcb3W::new(self, 3)
    }
    #[doc = "Bit 4 - Remap Command Bit for Master 4"]
    #[inline(always)]
    #[must_use]
    pub fn rcb4(&mut self) -> Rcb4W<MrcrSpec> {
        Rcb4W::new(self, 4)
    }
    #[doc = "Bit 5 - Remap Command Bit for Master 5"]
    #[inline(always)]
    #[must_use]
    pub fn rcb5(&mut self) -> Rcb5W<MrcrSpec> {
        Rcb5W::new(self, 5)
    }
    #[doc = "Bit 6 - Remap Command Bit for Master 6"]
    #[inline(always)]
    #[must_use]
    pub fn rcb6(&mut self) -> Rcb6W<MrcrSpec> {
        Rcb6W::new(self, 6)
    }
    #[doc = "Bit 7 - Remap Command Bit for Master 7"]
    #[inline(always)]
    #[must_use]
    pub fn rcb7(&mut self) -> Rcb7W<MrcrSpec> {
        Rcb7W::new(self, 7)
    }
    #[doc = "Bit 8 - Remap Command Bit for Master 8"]
    #[inline(always)]
    #[must_use]
    pub fn rcb8(&mut self) -> Rcb8W<MrcrSpec> {
        Rcb8W::new(self, 8)
    }
    #[doc = "Bit 9 - Remap Command Bit for Master 9"]
    #[inline(always)]
    #[must_use]
    pub fn rcb9(&mut self) -> Rcb9W<MrcrSpec> {
        Rcb9W::new(self, 9)
    }
    #[doc = "Bit 10 - Remap Command Bit for Master 10"]
    #[inline(always)]
    #[must_use]
    pub fn rcb10(&mut self) -> Rcb10W<MrcrSpec> {
        Rcb10W::new(self, 10)
    }
    #[doc = "Bit 11 - Remap Command Bit for Master 11"]
    #[inline(always)]
    #[must_use]
    pub fn rcb11(&mut self) -> Rcb11W<MrcrSpec> {
        Rcb11W::new(self, 11)
    }
    #[doc = "Bit 12 - Remap Command Bit for Master 12"]
    #[inline(always)]
    #[must_use]
    pub fn rcb12(&mut self) -> Rcb12W<MrcrSpec> {
        Rcb12W::new(self, 12)
    }
    #[doc = "Bit 13 - Remap Command Bit for Master 13"]
    #[inline(always)]
    #[must_use]
    pub fn rcb13(&mut self) -> Rcb13W<MrcrSpec> {
        Rcb13W::new(self, 13)
    }
    #[doc = "Bit 14 - Remap Command Bit for Master 14"]
    #[inline(always)]
    #[must_use]
    pub fn rcb14(&mut self) -> Rcb14W<MrcrSpec> {
        Rcb14W::new(self, 14)
    }
    #[doc = "Bit 15 - Remap Command Bit for Master 15"]
    #[inline(always)]
    #[must_use]
    pub fn rcb15(&mut self) -> Rcb15W<MrcrSpec> {
        Rcb15W::new(self, 15)
    }
}
#[doc = "Master Remap Control\n\nYou can [`read`](crate::Reg::read) this register and get [`mrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrcrSpec;
impl crate::RegisterSpec for MrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrcr::R`](R) reader structure"]
impl crate::Readable for MrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mrcr::W`](W) writer structure"]
impl crate::Writable for MrcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MRCR to value 0"]
impl crate::Resettable for MrcrSpec {
    const RESET_VALUE: u32 = 0;
}
