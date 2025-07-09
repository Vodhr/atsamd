use typenum::U0;
use crate::time::Hertz;
use fugit::RateExtU32;
use super::{Enabled, Source, Sealed};
use crate::calibration;

pub struct Osc48mToken (());

impl Osc48mToken {
    #[inline]
    pub(crate) unsafe fn new() -> Self {
        Self(())
    }

    #[inline]
    fn oscctrl(&self) -> &crate::pac::oscctrl::RegisterBlock {
        unsafe { &*crate::pac::Oscctrl::PTR }
    }

    #[inline]
    fn osc48mctrl(&self) -> &crate::pac::oscctrl::Osc48mctrl {
        self.oscctrl().osc48mctrl()
    }

    #[inline]
    fn osc48mdiv(&self) -> &crate::pac::oscctrl::Osc48mdiv {
        self.oscctrl().osc48mdiv()
    }

    #[inline]
    fn osc48mstup(&self) -> &crate::pac::oscctrl::Osc48mstup {
        self.oscctrl().osc48mstup()
    }

    #[inline]
    fn osc48msyncbusy(&self) -> &crate::pac::oscctrl::Osc48msyncbusy {
        self.oscctrl().osc48msyncbusy()
    }

    #[inline]
    fn status(&self) -> &crate::pac::oscctrl::Status {
        self.oscctrl().status()
    }

    #[inline]
    fn cal48m(&self) -> &crate::pac::oscctrl::Cal48m {
        self.oscctrl().cal48m()
    }

    #[inline]
    fn is_ready(&self) -> bool {
        self.status().read().osc48mrdy().bit_is_set()
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

        self.osc48mctrl().modify(|_, w| {
            w.ondemand().bit(settings.on_demand);
            w.runstdby().bit(settings.run_standby)
        });

        self.cal48m().write(|w| unsafe {
            w.bits(calibration::osc48m_5v_cal())
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

        self.wait_sync_busy();
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

pub enum Osc48mId {}

impl Sealed for Osc48mId {}

pub struct Osc48m {
    token: Osc48mToken,
    settings: Settings,
}

impl Osc48m {
    #[inline]
    pub fn new(token: Osc48mToken) -> Self {
        let settings = Settings {
            start_up: StartUp::Delay21333ns,
            on_demand: false,
            run_standby: false,
            divider: Divider::Div4000kHz,
        };

        Self { token, settings }
    }

    #[inline]
    pub fn freq(&self) -> Hertz {
        match self.settings.divider {
            Divider::Div48MHz => { 48.MHz() }
            Divider::Div24MHz => { 24.MHz() }
            Divider::Div16MHz => { 16.MHz() }
            Divider::Div12MHz => { 12.MHz() }
            Divider::Div9600kHz => { 9_600.kHz() }
            Divider::Div8000kHz => { 8_000.kHz() }
            Divider::Div6860kHz => { 6_860.kHz() }
            Divider::Div6000kHz => { 6_000.kHz() }
            Divider::Div5330kHz => { 5_330.kHz() }
            Divider::Div4800kHz => { 4_800.kHz() }
            Divider::Div4360kHz => { 4_360.kHz() }
            Divider::Div4000kHz => { 4_000.kHz() }
            Divider::Div3690kHz => { 3_690.kHz() }
            Divider::Div3430kHz => { 3_430.kHz() }
            Divider::Div3200kHz => { 3_200.kHz() }
            Divider::Div3000kHz => { 3_000.kHz() }
        }
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

impl<N> Enabled<Osc48m, N> {
    #[inline]
    pub fn is_ready(&self) -> bool {
        self.0.token.is_ready()
    }
}
impl<N> Source for Enabled<Osc48m, N> {
    type Id = Osc48mId;

    #[inline]
    fn freq(&self) -> Hertz {
        self.0.freq()
    }
}