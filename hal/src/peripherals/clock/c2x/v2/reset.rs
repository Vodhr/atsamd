//! This module is intentionally private. Its contents are publicly exported
//! from the `v2` module, which is where the corresponding documentation will
//! appear.

use typenum::U1;

use crate::pac::{ Gclk, Mclk, Nvmctrl, Osc32kctrl, Oscctrl };

use super::*;

/// Collection of low-level PAC structs
///
/// This struct serves to guard access to the low-level PAC structs. It places
/// them behind an `unsafe` barrier.
///
/// Normally, users trade the low-level PAC structs for the higher-level
/// `clock::v2` API. However, in some cases, the `clock::v2` API may not be
/// sufficient. In these cases, users can access the registers directly by
/// calling [`Pac::steal`] to recover the PAC structs.
pub struct Pac {
    oscctrl: Oscctrl,
    osc32kctrl: Osc32kctrl,
    gclk: Gclk,
    mclk: Mclk,
}

impl Pac {
    /// Escape hatch allowing to access low-level PAC structs
    ///
    /// Consume the [`Pac`] and return the low-level PAC structs. This is
    /// useful when the `clock::v2` API does not provide a necessary feature, or
    /// when dealing with the legacy `clock::v1` API. For example, many of the
    /// `clock::v1` functions require access to the [`Mclk`] peripheral.
    ///
    /// # Safety
    ///
    /// Directly configuring clocks through the PAC API can invalidate the
    /// type-level guarantees of the `clock` module API.
    pub unsafe fn steal(self) -> (Oscctrl, Osc32kctrl, Gclk, Mclk) {
        (self.oscctrl, self.osc32kctrl, self.gclk, self.mclk)
    }
}

/// Bus clock objects
///
/// This type is constructed using the [`clock_system_at_reset`] function, which
/// consumes the PAC-level clocking structs and returns the HAL-level clocking
/// structs in their reset state.
///
/// This type contains the [bus clocks](super#bus-clocks), which are a necessary
/// to implement memory safety for the [`AhbClk`]s and [`ApbClk`]s.
///
/// [`AhbClk`]: super::ahb::AhbClk
/// [`ApbClk`]: super::apb::ApbClk
pub struct Buses {
    //pub ahb: ahb::Ahb,
    pub apb: apb::Apb,
}

/// Enabled clocks at power-on reset
///
/// This type is constructed using the [`clock_system_at_reset`] function, which
/// consumes the PAC-level clocking structs and returns the HAL-level clocking
/// structs in their reset state.
///
/// This type represents the clocks as they are configured at power-on reset.
/// The main clock, [`Gclk0`](gclk::Gclk0), runs at 48 MHz using the
/// [`Dfll`](dfll::Dfll) in open-loop mode. The ultra-low power
/// [base oscillator](osculp32k::OscUlp32kBase) is also enabled and running, as
/// it can never be disabled.
///
/// As described in the [top-level](super::super) documentation for the `clock`
/// module, only [`Enabled`] clocks can be used as a [`Source`] for downstream
/// clocks. This struct contains all of the `Enabled` clocks at reset.
///
/// This struct also contains the [`Pac`] wrapper struct, which provides
/// `unsafe` access to the low-level PAC structs.
pub struct Clocks {
    /// Wrapper providing `unsafe` access to low-level PAC structs
    pub pac: Pac,
    /// Enabled AHB clocks
    //pub ahbs: ahb::AhbClks,
    /// Enabled APB clocks
    pub apbs: apb::ApbClks,
    /// Main system clock, driven at 48 MHz by the Osc48m
    pub gclk0: Enabled<gclk::Gclk0<osc48m::Osc48mId>, U1>,
    /// Osc48m in open loop mode
    pub osc48m: Enabled<osc48m::Osc48m, U1>,
    // Always-enabled base oscillator for the [`OscUlp1k`](osculp32k::OscUlp1k)
    // and [`OscUlp32k`](osculp32k::OscUlp32k) clocks.
    // pub osculp32k_base: Enabled<osculp32k::OscUlp32kBase>,
}

/// Type-level tokens for unused clocks at power-on reset
///
/// This type is constructed using the [`clock_system_at_reset`] function, which
/// consumes the PAC-level clocking structs and returns the HAL-level clocking
/// structs in their reset state.
///
/// As described in the [top-level](super::super) documentation for the `clock`
/// module, token types are used to guanrantee the uniqueness of each clock. To
/// configure or enable a clock, you must provide the corresponding token.
pub struct Tokens {
    /// Tokens to create [`apb::ApbClk`]s
    pub apbs: apb::ApbTokens,
    /// Token to create [`dpll::Dpll0`]
    //pub dpll0: dpll::DpllToken<dpll::Dpll0Id>,
    /// Token to create [`dpll::Dpll1`]
    //pub dpll1: dpll::DpllToken<dpll::Dpll1Id>,
    /// Tokens to create [`gclk::Gclk`]
    pub gclks: gclk::GclkTokens,
    /// Tokens to create [`pclk::Pclk`]s
    pub pclks: pclk::PclkTokens,
    /// Tokens to create [`rtcosc::RtcOsc`]
    // pub rtcosc: rtcosc::RtcOscToken,
    /// Tokens [`xosc::Xosc`]
    pub xosc: xosc::XoscToken,
    // Tokens to create [`xosc32k::Xosc32kBase`], [`xosc32k::Xosc1k`] and
    // [`xosc32k::Xosc32k`]
    // pub xosc32k: xosc32k::Xosc32kTokens,
    // Tokens to create [`osculp32k::OscUlp1k`] and [`osculp32k::OscUlp32k`]
    // pub osculp32k: osculp32k::OscUlp32kTokens,
}

/// Consume the PAC clocking structs and return a HAL-level
/// representation of the clocks at power-on reset
///
/// This function consumes the [`Oscctrl`], [`Osc32kctrl`], [`Gclk`] and
/// [`Mclk`] PAC structs and returns the [`Buses`], [`Clocks`] and [`Tokens`].
///
/// See the [module-level documentation](super) for more details.
#[inline]
pub fn clock_system_at_reset(
    oscctrl: Oscctrl,
    osc32kctrl: Osc32kctrl,
    gclk: Gclk,
    mclk: Mclk,
    nvmctrl: &mut Nvmctrl,
) -> (Buses, Clocks, Tokens) {
    // Safety: No bus, clock or token is instantiated more than once
    unsafe {
        let buses = Buses {
            //ahb: ahb::Ahb::new(),
            apb: apb::Apb::new(),
        };
        let pac = Pac {
            oscctrl,
            osc32kctrl,
            gclk,
            mclk,
        };

        // See Pages 441 and 1107 for NVM Read Wait States depending on voltage and CPU clock frequency
        nvmctrl.ctrlb().write(|w| {
            w.rws().dual()
        });

        let osc48m = osc48m::Osc48m::new(osc48m::Osc48mToken::new()).divider(osc48m::Divider::Div48MHz).enable();
        let (gclk0, osc48m) = gclk::Gclk0::from_source(gclk::GclkToken::new(), osc48m);
        let gclk0 = Enabled::new(gclk0);
        let clocks = Clocks {
            pac,
            // ahbs: ahb::AhbClks::new(),
            apbs: apb::ApbClks::new(),
            gclk0,
            osc48m,
            // osculp32k_base: osculp32k::OscUlp32kBase::new(),
        };
        let tokens = Tokens {
            apbs: apb::ApbTokens::new(),
            // dpll0: dpll::DpllToken::new(),
            // dpll1: dpll::DpllToken::new(),
            gclks: gclk::GclkTokens::new(),
            pclks: pclk::PclkTokens::new(),
            // rtcosc: rtcosc::RtcOscToken::new(),
            xosc: xosc::XoscToken::new(),
            // xosc1: xosc::XoscToken::new(),
            // xosc32k: xosc32k::Xosc32kTokens::new(),
            // osculp32k: osculp32k::OscUlp32kTokens::new(),
        };
        (buses, clocks, tokens)
    }
}
