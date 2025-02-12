use typenum::U0;
// use crate::async_hal::interrupts::InterruptExt;
use crate::time::Hertz;
use super::{Enabled, Source, Sealed};

pub struct Osc48mToken (());

impl Osc48mToken {
    #[inline]
    pub(crate) unsafe fn new() -> Self {
        Self(())
    }

    #[inline]
    fn oscctrl(&self) -> &crate::pac::oscctrl::RegisterBlock {
        unsafe { &*crate::pac::OSCCTRL::PTR }
    }

    #[inline]
    fn osc48mctrl(&self) -> &crate::pac::oscctrl::OSC48MCTRL {
        &self.oscctrl().osc48mctrl
    }

    #[inline]
    fn osc48mdiv(&self) -> &crate::pac::oscctrl::OSC48MDIV {
        &self.oscctrl().osc48mdiv
    }

    #[inline]
    fn osc48mstup(&self) -> &crate::pac::oscctrl::OSC48MSTUP {
        &self.oscctrl().osc48mstup
    }

    #[inline]
    fn osc48msyncbusy(&self) -> &crate::pac::oscctrl::OSC48MSYNCBUSY {
        &self.oscctrl().osc48msyncbusy
    }

    fn is_ready(&self) -> bool {
        self.osc48msyncbusy().read().osc48mdiv().bit_is_clear()
    }

    #[inline]
    fn wait_sync_busy(&self) {
        while self.osc48msyncbusy().read().osc48mdiv().bit_is_set() {}
    }

    #[inline]
    fn reset(&self) {
        self.osc48mctrl().reset();
        self.osc48mdiv().reset();
    }

    #[inline]
    fn enable(&self) {
        self.osc48mctrl().modify(|_, w| w.enable().set_bit());
    }

    #[inline]
    fn disable(&self) {
        self.osc48mctrl().modify(|_, w| w.enable().clear_bit());
    }

    #[inline]
    fn set_registers(&mut self, settings: Settings) {
        self.osc48mctrl().modify(|_, w| {
            w.ondemand().bit(settings.on_demand);
            w.runstdby().bit(settings.run_standby)
        });

        self.osc48mdiv().modify(|_, w| {
            match settings.divider {
                Divider::Div48MHz => w.div().div1(),
                Divider::Div24MHz => w.div().div2(),
                Divider::Div16MHz => w.div().div3(),
                Divider::Div12MHz => w.div().div4(),
                Divider::Div9600kHz => w.div().div5(),
                Divider::Div8000kHz => w.div().div6(),
                Divider::Div6860kHz => w.div().div7(),
                Divider::Div6000kHz => w.div().div8(),
                Divider::Div5330kHz => w.div().div9(),
                Divider::Div4800kHz => w.div().div10(),
                Divider::Div4360kHz => w.div().div11(),
                Divider::Div4000kHz => w.div().div12(),
                Divider::Div3690kHz => w.div().div13(),
                Divider::Div3430kHz => w.div().div14(),
                Divider::Div3200kHz => w.div().div15(),
                Divider::Div3000kHz => w.div().div16(),
            }
        });

        // Todo: move to Into
        self.osc48mstup().modify(|_, w| {
            match settings.start_up {
                StartUp::Delay166ns => w.startup().cycle8(),
                StartUp::Delay333ns => w.startup().cycle16(),
                StartUp::Delay667ns => w.startup().cycle32(),
                StartUp::Delay1333ns => w.startup().cycle64(),
                StartUp::Delay2667ns => w.startup().cycle128(),
                StartUp::Delay5333ns => w.startup().cycle256(),
                StartUp::Delay10667ns => w.startup().cycle512(),
                StartUp::Delay21333ns => w.startup().cycle1024(),
            }
        });
    }
}

#[derive(Clone, Copy)]
pub struct Settings {
    start_up: StartUp,
    on_demand: bool,
    run_standby: bool,
    divider: Divider,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Divider {
    Div48MHz,
    Div24MHz,
    Div16MHz,
    Div12MHz,
    Div9600kHz,
    Div8000kHz,
    Div6860kHz,
    Div6000kHz,
    Div5330kHz,
    Div4800kHz,
    Div4360kHz,
    Div4000kHz,
    Div3690kHz,
    Div3430kHz,
    Div3200kHz,
    Div3000kHz,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum StartUp {
    Delay166ns,
    Delay333ns,
    Delay667ns,
    Delay1333ns,
    Delay2667ns,
    Delay5333ns,
    Delay10667ns,
    Delay21333ns,
}


pub struct Osc48m {
    token: Osc48mToken,
    settings: Settings,
    freq: Hertz,
}

pub enum Osc48mId {}

impl Sealed for Osc48mId {}

impl Osc48m {
    #[inline]
    pub fn new(token: Osc48mToken, freq: Hertz) -> Self {
        let settings = Settings {
            start_up: StartUp::Delay21333ns,
            on_demand: true,
            run_standby: false,
            divider: Divider::Div4000kHz,
        };

        Self { token, settings, freq }
    }

    #[inline]
    pub fn freq(&self) -> Hertz {
        self.freq
    }

    #[inline]
    pub fn free(self) -> Osc48mToken {
        self.token
    }

    #[inline]
    pub fn start_up_delay(mut self, delay: StartUp) -> Self {
        self.settings.start_up = delay;
        self
    }

    #[inline]
    pub fn on_demand(mut self, on_demand: bool) -> Self {
        self.settings.on_demand = on_demand;
        self
    }

    #[inline]
    pub fn run_standby(mut self, run_standby: bool) -> Self {
        self.settings.run_standby = run_standby;
        self
    }

    #[inline]
    pub fn divider(mut self, divider: Divider) -> Self {
        self.settings.divider = divider;
        self
    }

    #[inline]
    pub fn enable(mut self) -> EnabledOsc48m {
        self.token.reset();
        self.token.set_registers(self.settings);
        self.token.enable();
        super::Enabled::new(self)
    }
}

pub type EnabledOsc48m<N = U0> = Enabled<Osc48m, N>;

impl EnabledOsc48m {
    #[inline]
    pub fn is_ready(&self) -> bool {
        self.0.token.is_ready()
    }
}
impl Source for EnabledOsc48m {
    type Id = Osc48mId;

    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq
    }
}