//! # Peripheral Channel Clocks
//!
//! ## Overview
//!
//! Peripheral channel clocks, or [`Pclk`]s, connect generic clock controllers
//! ([`Gclk`]s) to various peripherals within the chip. Each [`Pclk`] maps 1:1
//! with a corresponding peripheral.
//!
//! The 48 possible [`Pclk`]s are distinguished by their corresponding
//! [`PclkId`] types. Ideally, each [`PclkId`] type would be a relevant type
//! from a corresponding HAL module. For example, each of the eight different
//! [`Sercom`] types implements [`PclkId`]. However, the HAL does not yet
//! support all peripherals, nor have all existing HAL peripherals been
//! integrated with `clock::v2`. In those cases, a dummy type is defined in the
//! [`clock::v2::types`] module.
//!
//! [`Pclk`]s are typically leaves in the clock tree. The only exceptions are
//! [`Pclk`]s used for the [`DFLL`] or [`DPLL`] peripherals. In those cases, the
//! [`Pclk`] acts as a branch clock.
//!
//! Each [`Pclk`] powers only a single peripheral; they do not act as general
//! purpose clock [`Source`]s for other clocks in the tree. As a result, they do
//! not need to be wrapped with [`Enabled`].
//!
//! [`Pclk`]s also do not have any meaningful configuration beyond identifying
//! which [`EnabledGclk`] is its [`Source`]. Consequently, [`PclkToken`]s can be
//! directly converted into enabled [`Pclk`]s with [`Pclk::enable`].
//!
//! See the [`clock` module documentation] for a more thorough explanation of
//! the various concepts discussed above.
//!
//! ## Example
//!
//! The following example shows how to enable the [`Pclk`] for [`Sercom0`]. It
//! derives the [`Sercom0`] clock from [`EnabledGclk0`], which is already
//! running at power-on reset. In doing so, the [`EnabledGclk0`] counter is
//! [`Increment`]ed.
//!
//! ```no_run
//! use atsamd_hal::{
//!     clock::v2::{clock_system_at_reset, pclk::Pclk},
//!     pac::Peripherals,
//! };
//! let mut pac = Peripherals::take().unwrap();
//! let (buses, clocks, tokens) = clock_system_at_reset(
//!     pac.oscctrl,
//!     pac.osc32kctrl,
//!     pac.gclk,
//!     pac.mclk,
//!     &mut pac.nvmctrl,
//! );
//! let (pclk_sercom0, gclk0) = Pclk::enable(tokens.pclks.sercom0, clocks.gclk0);
//! ```
//!
//! [`Gclk`]: super::gclk::Gclk
//! [`DFLL`]: super::dfll
//! [`DPLL`]: super::dpll
//! [`Enabled`]: super::Enabled
//! [`EnabledGclk`]: super::gclk::EnabledGclk
//! [`EnabledGclk0`]: super::gclk::EnabledGclk0
//! [`clock` module documentation]: super
//! [`clock::v2::types`]: super::types
//! [`Sercom`]: crate::sercom::Sercom

use atsamd_hal_macros::hal_macro_helper;

use core::marker::PhantomData;

use paste::paste;
use seq_macro::seq;

use crate::pac;
use crate::pac::gclk::pchctrl::Genselect;

use crate::time::Hertz;
use crate::typelevel::{Decrement, Increment, Sealed};

use super::gclk::{DynGclkId, GclkId};
use super::Source;

//==============================================================================
// PclkToken
//==============================================================================

/// Singleton token that can be exchanged for a [`Pclk`]
///
/// As explained in the [`clock` module documentation](super), instances of
/// various `Token` types can be exchanged for actual clock types. They
/// typically represent clocks that are disabled at power-on reset.
///
/// [`PclkToken`]s are no different. All [`Pclk`]s are disabled at power-on
/// reset. To use a [`Pclk`], you must first exchange the token for an actual
/// clock with the [`Pclk::enable`] function.
///
/// [`PclkToken`] is generic over the [`PclkId`], where each token represents a
/// corresponding peripheral clock channel.
pub struct PclkToken<P: PclkId> {
    pclk: PhantomData<P>,
}

impl<P: PclkId> PclkToken<P> {
    /// Create a new instance of [`PclkToken`]
    ///
    /// # Safety
    ///
    /// Each `PclkToken`s is a singleton. There must never be two simulatenous
    /// instances with the same [`PclkId`]. See the notes on `Token` types and
    /// memory safety in the root of the `clock` module for more details.
    #[inline]
    pub(super) unsafe fn new() -> Self {
        PclkToken { pclk: PhantomData }
    }

    /// Access the corresponding `PCHCTRL` register
    #[inline]
    fn pchctrl(&self) -> &pac::gclk::Pchctrl {
        // Safety: Each `PclkToken` only has access to a mutually exclusive set
        // of registers for the corresponding `PclkId`, and we use a shared
        // reference to the register block. See the notes on `Token` types and
        // memory safety in the root of the `clock` module for more details.
        unsafe { (*pac::Gclk::PTR).pchctrl(P::DYN as usize) }
    }

    /// Set the [`Pclk`] source
    #[inline]
    fn set_source(&mut self, source: DynPclkSourceId) {
        self.pchctrl().modify(|_, w| w.r#gen().variant(source.into()));
    }

    /// Enable the [`Pclk`]
    #[inline]
    fn enable(&mut self) {
        self.pchctrl().modify(|_, w| w.chen().set_bit());
    }

    /// Disable the [`Pclk`]
    #[inline]
    fn disable(&mut self) {
        self.pchctrl().modify(|_, w| w.chen().clear_bit());
    }
}

//==============================================================================
// PclkId types
//==============================================================================

/// Module containing only the types implementing [`PclkId`]
///
/// Because there are so many types that implement [`PclkId`], it is helpful to
/// have them defined in a separate module, so that you can import all of them
/// using a wildcard (`*`) without importing anything else, i.e.
///
/// ```
/// use atsamd_hal::clock::v2::pclk::ids::*;
/// ```
pub mod ids {
    use atsamd_hal_macros::hal_cfg;

    // todo
    #[hal_cfg("sercom0")]
    pub use crate::sercom::{Sercom0, Sercom1, Sercom2, Sercom3, Sercom4, Sercom5};

    #[hal_cfg("sercom6")]
    pub use crate::sercom::Sercom6;
    #[hal_cfg("sercom7")]
    pub use crate::sercom::Sercom7;

    pub use super::super::osc48m::Osc48mId;
    // todo
    // pub use super::super::dpll::{Dpll0Id, Dpll1Id};

    pub use super::super::types::{
        Ac, Adc0, Adc1, CM4Trace, Ccl, Dac, Eic, EvSys0, EvSys1, EvSys10, EvSys11, EvSys2, EvSys3,
        EvSys4, EvSys5, EvSys6, EvSys7, EvSys8, EvSys9, FreqMMeasure, FreqMReference, PDec, Sdhc0,
        SlowClk, Tc0Tc1, Tc2Tc3, Tc4, Tcc0Tcc1, Tcc2, Usb,
    };

    #[hal_cfg("can0")]
    pub use super::super::types::Can0;
    #[hal_cfg("can1")]
    pub use super::super::types::Can1;
    #[hal_cfg("sdhc1")]
    pub use super::super::types::Sdhc1;
    #[hal_cfg(all("tc4", "tc5"))]
    pub use super::super::types::Tc4Tc5;
    #[hal_cfg(all("tc6", "tc7"))]
    pub use super::super::types::Tc6Tc7;
    #[hal_cfg("tcc4")]
    pub use super::super::types::Tcc4;
    #[hal_cfg("i2s")]
    pub use super::super::types::{I2S0, I2S1};
}

use ids::*;

/// Append the list of all [`PclkId`] types and `snake_case` id names to the
/// arguments of a macro call
///
/// This macro will perform the embedded macro call with a list of tuples
/// appended to the arguments. Each tuple contains a type implementing
/// [`PclkId`], its corresponding `PCHCTRL` register index, and the `snake_case`
/// name of the corresponding token in the [`PclkTokens`] struct.
///
/// **Note:** The entries within [`DynPclkId`] do not match the type names.
/// Rather, they match the `snake_case` names converted to `CamelCase`.
///
/// An optional attribute is added just before each tuple. These are mainly used
/// to declare the conditions under which the corresponding peripheral exists.
///
/// The example below shows the pattern that should be used to match against the
/// appended tokens.
///
/// ```ignore
/// macro_rules! some_macro {
///     (
///         $first_arg:tt,
///         $second_arg:tt
///         $(
///             $( #[$cfg:meta] )?
///             ($Type:ident = $N:literal, $Id:ident)
///         )+
///     ) =>
///     {
///         // implementation here ...
///     }
/// }
///
/// with_pclk_types_ids!(some_macro!(first, second));
/// ```
///
/// todo: missing PCHCTRL mappings
#[hal_macro_helper]
macro_rules! with_pclk_types_ids {
    ( $some_macro:ident ! ( $( $args:tt )* ) ) => {
        $some_macro!(
            $( $args )*
            (Eic = 2, eic)
            (FreqMMeasure = 3, freq_m_measure)
            (FreqMReference = 4, freq_m_reference)

            (EvSys0 = 6, ev_sys0)
            (EvSys1 = 7, ev_sys1)
            (EvSys2 = 8, ev_sys2)
            (EvSys3 = 9, ev_sys3)
            (EvSys4 = 10, ev_sys4)
            (EvSys5 = 11, ev_sys5)
            (EvSys6 = 12, ev_sys6)
            (EvSys7 = 13, ev_sys7)
            (EvSys8 = 14, ev_sys8)
            (EvSys9 = 15, ev_sys9)
            (EvSys10 = 16, ev_sys10)
            (EvSys11 = 17, ev_sys11)

            (Sercom0 = 19, sercom0)
            (Sercom1 = 20, sercom1)
            (Sercom2 = 21, sercom2)
            (Sercom3 = 22, sercom3)
            (Sercom4 = 23, sercom4)

            (Sercom5 = 25, sercom5)

            #[hal_cfg("can0")]
            (Can0 = 26, can0)
            #[hal_cfg("can1")]
            (Can1 = 27, can1)

            (Tcc0Tcc1 = 28, tcc0_tcc1)
            (Tcc2 = 29, tcc2)

            (Tc0Tc1 = 30, tc0_tc1)
            (Tc2Tc3 = 31, tc2_tc3)

            #[hal_cfg("tc4")]
            (Tc4 = 32, tc4)

            (Adc0 = 33, adc0)
            (Adc1 = 34, adc1)

            (Dac = 36, dac)

            (Ccl = 38, ccl)

            (Ac = 40, ac)

            #[hal_cfg("sercom6")]
            (Sercom6 = 41, sercom6)
            #[hal_cfg("sercom7")]
            (Sercom7 = 42, sercom7)

            #[hal_cfg("tc5")]
            (Tc5 = 43, tc5)
            #[hal_cfg("tc6")]
            (Tc6 = 44, tc6)
            #[hal_cfg("tc7")]
            (Tc7 = 45, tc7)
        );
    };
}

//==============================================================================
// DynPclkId
//==============================================================================

macro_rules! dyn_pclk_id {
    (
        $(
            $( #[$cfg:meta] )?
            ($Type:ident = $N:literal, $id:ident)
        )+
    ) => {
        paste! {
            /// Value-level enum identifying one of the 48 possible [`Pclk`]s
            ///
            /// The variants of this enum identify one of the 48 possible
            /// peripheral channel clocks. When cast to a `u8`, each variant
            /// maps to its corresponding `PCHCTRL` index.
            ///
            /// `DynPclkId` is the value-level equivalent of [`PclkId`].
            #[repr(u8)]
            pub enum DynPclkId {
                $(
                    $( #[$cfg] )?
                    [<$id:camel>] = $N,
                )+
            }

            $(
                $( #[$cfg] )?
                impl PclkId for $Type {
                    const DYN: DynPclkId = DynPclkId::[<$id:camel>];
                }
            )+
        }
    };
}

with_pclk_types_ids!(dyn_pclk_id!());

//==============================================================================
// PclkId
//==============================================================================

/// Type-level enum identifying one of the 48 possible [`Pclk`]s
///
/// The types implementing this trait, e.g. [`Sercom0`] or [`DfllId`], are
/// type-level variants of `PclkId`, and they identify one of the 48 possible
/// peripheral channel clocks.
///
/// `PclkId` is the type-level equivalent of [`DynPclkId`]. See the
/// documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub trait PclkId: Sealed {
    /// Corresponding variant of [`DynPclkId`]
    const DYN: DynPclkId;
}

//==============================================================================
// DynPclkSourceId
//==============================================================================

/// Value-level enum of possible clock sources for a [`Pclk`]
///
/// The variants of this enum identify the [`Gclk`] used as a clock source for
/// a given [`Pclk`]. Because the variants are identical to [`DynGclkId`], we
/// simply define it as a type alias.
///
/// `DynPclkSourceId` is the value-level equivalent of [`PclkSourceId`].
///
/// [`Gclk`]: super::gclk::Gclk
pub type DynPclkSourceId = DynGclkId;

/// Convert from [`DynPclkSourceId`] to the equivalent [PAC](crate::pac) type
impl From<DynPclkSourceId> for Genselect {
    fn from(source: DynPclkSourceId) -> Self {
        seq!(N in 0..=8 {
            match source {
                #(
                    DynGclkId::Gclk~N => Genselect::Gclk~N,
                )*
            }
        })
    }
}

//==============================================================================
// PclkSourceId
//==============================================================================

/// Type-level enum of possible clock [`Source`]s for a [`Pclk`]
///
/// The types implementing this trait are type-level variants of `PclkSourceId`,
/// and they identify the [`Gclk`] acting as a clock [`Source`] for a given
/// [`Pclk`]. Accordingly, all implementers of this trait are [`GclkId`] types,
/// and this trait is simply a trait alias for [`GclkId`]. `Id` types in general
/// are described in more detail in the [`clock` module documentation](super).
///
/// `PclkSourceId` is the type-level equivalent of [`DynPclkSourceId`]. See the
/// documentation on [type-level programming] and specifically
/// [type-level enums] for more details.
///
/// [`Gclk`]: super::gclk::Gclk
/// [type-level programming]: crate::typelevel
/// [type-level enums]: crate::typelevel#type-level-enums
pub trait PclkSourceId: GclkId {}

impl<G: GclkId> PclkSourceId for G {}

//==============================================================================
// Pclk
//==============================================================================

/// Peripheral channel clock for a given peripheral
///
/// Peripheral channel clocks connect generic clock generators ([`Gclk`]s) to
/// various peripherals. `Pclk`s usually act as leaves in the clock tree, except
/// when they feed the [`DFLL`] and [`DPLL`] peripherals.
///
/// The type parameter `P` is a [`PclkId`] that determines which of the 48
/// peripherals this [`Pclk`] feeds. The type parameter `I` represents the `Id`
/// type for the [`EnabledGclk`] acting as the `Pclk`'s [`Source`]. It must be
/// one of the valid [`PclkSourceId`]s, which is simply a trait alias for
/// [`GclkId`]. See the [`clock` module documentation](super) for more detail on
/// `Id` types.
///
/// `Pclk`s cannot act as general purpose clock [`Source`]s; rather, they map
/// 1:1 with corresponding peripherals. Thus, enabled `Pclk`s do not need a
/// compile-time counter of consumer clocks, so they are not wrapped with
/// [`Enabled`]. Enabled `Pclk`s are created directly from [`PclkToken`]s with
/// [`Pclk::enable`].
///
/// See the [module-level documentation](self) for an example.
///
/// [`Enabled`]: super::Enabled
/// [`Gclk`]: super::gclk::Gclk
/// [`EnabledGclk`]: super::gclk::EnabledGclk
/// [`DFLL`]: super::dfll
/// [`DPLL`]: super::dpll
pub struct Pclk<P, I>
where
    P: PclkId,
    I: PclkSourceId,
{
    token: PclkToken<P>,
    src: PhantomData<I>,
    freq: Hertz,
}

impl<P, I> Pclk<P, I>
where
    P: PclkId,
    I: PclkSourceId,
{
    pub(super) fn new(token: PclkToken<P>, freq: Hertz) -> Self {
        Self {
            token,
            src: PhantomData,
            freq,
        }
    }

    /// Create and enable a [`Pclk`]
    ///
    /// Creating a [`Pclk`] immediately enables the corresponding peripheral
    /// channel clock. It also [`Increment`]s the [`Source`]'s [`Enabled`]
    /// counter.
    ///
    /// Note that the [`Source`] will always be an [`EnabledGclk`].
    ///
    /// [`Enabled`]: super::Enabled
    /// [`EnabledGclk`]: super::gclk::EnabledGclk
    #[inline]
    pub fn enable<S>(mut token: PclkToken<P>, gclk: S) -> (Self, S::Inc)
    where
        S: Source<Id = I> + Increment,
    {
        let freq = gclk.freq();
        token.set_source(I::DYN);
        token.enable();
        let pclk = Pclk::new(token, freq);
        (pclk, gclk.inc())
    }

    /// Disable and destroy a [`Pclk`]
    ///
    /// Consume the [`Pclk`], release the [`PclkToken`], and [`Decrement`] the
    /// [`EnabledGclk`]'s counter
    ///
    /// [`Enabled`]: super::Enabled
    /// [`EnabledGclk`]: super::gclk::EnabledGclk
    #[inline]
    pub fn disable<S>(mut self, gclk: S) -> (PclkToken<P>, S::Dec)
    where
        S: Source<Id = I> + Decrement,
    {
        self.token.disable();
        (self.token, gclk.dec())
    }

    /// Return the [`Pclk`] frequency
    #[inline]
    pub fn freq(&self) -> Hertz {
        self.freq
    }
}

impl<P, I> Sealed for Pclk<P, I>
where
    P: PclkId,
    I: PclkSourceId,
{
}

//==============================================================================
// PclkTokens
//==============================================================================

macro_rules! define_pclk_tokens_struct {
    (
        $(
            $( #[$cfg:meta] )?
            ($Type:ident = $_:literal, $id:ident)
        )+
    ) =>
    {
        /// Set of [`PclkToken`]s representing the disabled [`Pclk`]s at
        /// power-on reset
        pub struct PclkTokens {
            $(
                $( #[$cfg] )?
                pub $id: PclkToken<$Type>,
            )+
        }

        impl PclkTokens {
            /// Create the set of [`PclkToken`]s
            ///
            /// # Safety
            ///
            /// All invariants required by `PclkToken::new` must be upheld here
            #[inline]
            pub(super) fn new() -> Self {
                unsafe {
                    Self {
                        $(
                            $( #[$cfg] )?
                            $id: PclkToken::new(),
                        )+
                    }
                }
            }
        }
    };
}

with_pclk_types_ids!(define_pclk_tokens_struct!());
