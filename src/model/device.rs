//! This is the lowest level of a Model. It represents one discrete device
//! in the BCS. Every device has some state and a driver, corresponding to a
//! device driver in `brewdrivers`.

use serde::{Serialize, Deserialize};

use crate::model::{Driver, Mode};
use crate::model::State;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Device {
    pub driver: Driver,
    pub name: String,
    pub id: String,
    pub state: State,
    pub addr: u8,
    pub controller_addr: u8,
    pub pv: Option<f64>,
    pub sv: Option<f64>,
}


#[cfg(feature = "rtu")]
impl Device {
    /// Updates either the device state or the Model state, 
    /// depending on the Mode.
    ///
    /// This is one of the things that needs to be updates when new drivers
    /// are added. I'd like to extract this to a trait within `brewdrivers` so
    /// I don't have to expand this when I write a new driver.
    pub fn update(device: &mut Device, mode: &Mode) {
        use brewdrivers::RTU::{
            omega::CN7500,
            relays::{STR1, Board}
        };
        // This is a little bit fucked. I'm maintaining two different states becauese
        // it'll save time and space later.
        use brewdrivers::RTU::relays::State as BState;
        
        match device.driver {
            Driver::STR1 => {
                let mut board = STR1::with_address(device.controller_addr);
                match mode {
                    Mode::Write => {
                        let new_state = match device.state {
                            State::On => BState::On,
                            State::Off => BState::Off
                        };
                        board.set_relay(device.addr, new_state)
                    },
                    Mode::Read => {
                        // Don't do anything here, we always read new state
                    }
                }
                // Read|Update
                device.state = match board.get_relay(device.addr) {
                    BState::On => State::On,
                    BState::Off => State::Off
                }
            },
            Driver::Omega => {
                let cn7500 = CN7500::new(device.controller_addr, "/dev/ttyAMA0", 19200);
                match mode {
                    Mode::Write => {
                        cn7500.set_sv(device.sv.unwrap());
                        match device.state {
                            State::On => cn7500.run(),
                            State::Off => cn7500.stop(),
                        }
                    },
                    Mode::Read => {
                        // Don't do anything here, we always read new state
                    }
                }

                // Read|Update
                device.pv = Some(cn7500.get_pv());
                device.sv = Some(cn7500.get_sv());
                if cn7500.is_running() {
                    device.state = State::On;
                } else {
                    device.state = State::Off;
                }
            }
        }

    }
}

/// If you're not on an RTU (ie. there's no hardware to interact with),
/// then this function does nothing. Enabling the "rtu" feature will enable
/// the real update() function.
#[cfg(not(feature = "rtu"))]
impl Device {
    pub fn update(_: &mut Device, _: &Mode) {
        let data = "It worked! rtu feature is disabled";
        std::fs::write("rtu_test", data).expect("Unable to write file");
        // println!("RTU feature hasn't been enabled, update() does nothing.");
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize_device() {
        let raw = r#"
            {
                "name": "Some Relay",
                "id": "some id",
                "driver": "STR1",
                "state": "Off",
                "addr": 0,
                "controller_addr": 0
            }
        "#;

        let device: Device = serde_json::from_str(raw).unwrap();
        assert_eq!(device.name, "Some Relay");
        assert_eq!(device.state, crate::model::State::Off);
        assert_eq!(device.pv, None);
    }
}