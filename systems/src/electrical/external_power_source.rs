use crate::simulator::{
    SimulatorElement, SimulatorElementVisitable, SimulatorElementVisitor, SimulatorReadState,
    UpdateContext,
};

use super::{Current, ElectricPowerSource, ElectricSource};

pub struct ExternalPowerSource {
    pub is_connected: bool,
}
impl ExternalPowerSource {
    pub fn new() -> ExternalPowerSource {
        ExternalPowerSource {
            is_connected: false,
        }
    }

    pub fn update(&mut self, _: &UpdateContext) {}
}
impl ElectricSource for ExternalPowerSource {
    fn output(&self) -> Current {
        if self.is_connected {
            Current::some(ElectricPowerSource::External)
        } else {
            Current::none()
        }
    }
}
impl SimulatorElementVisitable for ExternalPowerSource {
    fn accept(&mut self, visitor: &mut Box<&mut dyn SimulatorElementVisitor>) {
        visitor.visit(&mut Box::new(self));
    }
}
impl SimulatorElement for ExternalPowerSource {
    fn read(&mut self, state: &SimulatorReadState) {
        self.is_connected = state.electrical.external_power_available;
    }
}

#[cfg(test)]
mod external_power_source_tests {
    use super::*;

    #[test]
    fn starts_without_output() {
        assert!(external_power_source().is_unpowered());
    }

    #[test]
    fn when_plugged_in_provides_output() {
        let mut ext_pwr = external_power_source();
        ext_pwr.is_connected = true;

        assert!(ext_pwr.is_powered());
    }

    #[test]
    fn when_not_plugged_in_provides_no_output() {
        let mut ext_pwr = external_power_source();
        ext_pwr.is_connected = false;

        assert!(ext_pwr.is_unpowered());
    }

    fn external_power_source() -> ExternalPowerSource {
        ExternalPowerSource::new()
    }
}