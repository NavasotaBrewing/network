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


// #[cfg(feature = "rtu")]
impl Device {
    /// Updates either the device state or the Model state, 
    /// depending on the Mode.
    ///
    /// This is one of the things that needs to be updated when new drivers
    /// are added. I'd like to extract this to a trait within `brewdrivers` so
    /// I don't have to expand this when I write a new driver.
    pub async fn update(device: &mut Device, mode: &Mode) {
        use brewdrivers::{
            omega::CN7500,
            relays::STR1
        };
        // This is a little bit fucked. I'm maintaining two different states becauese
        // it'll save time and space later.
        use brewdrivers::relays::State as BState;
        
        match device.driver {
            Driver::STR1 => {
                // TODO: make an override for the port with an environment variable or file or something
                // We want to panic! here. This will be run by rocket, so if it panics it will just fail with a message
                let mut board = STR1::new(device.controller_addr, "/dev/ttyUSB0", 9600).expect("Couldn't connect to STR1!");
                match mode {
                    Mode::Write => {
                        let new_state = match device.state {
                            State::On => BState::On,
                            State::Off => BState::Off
                        };
                        board.set_relay(device.addr, new_state);
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
                let mut cn7500 = CN7500::new(device.controller_addr, "/dev/ttyUSB0", 19200).await.expect("Couldn't connect to CN7500!");
                match mode {
                    Mode::Write => {
                        cn7500.set_sv(device.sv.unwrap()).await.expect("Couldn't set SV on CN7500");
                        match device.state {
                            State::On => cn7500.run().await.expect("Couldn't start CN7500"),
                            State::Off => cn7500.stop().await.expect("Couldn't stop CN7500"),
                        };
                    },
                    Mode::Read => {
                        // Don't do anything here, we always read new state
                    }
                }

                // Read|Update
                device.pv = cn7500.get_pv().await.ok();
                device.sv = cn7500.get_sv().await.ok();
                // Again, we're ok with panic!ing here.
                if cn7500.is_running().await.expect("Couldn't communicate with CN7500!") {
                    device.state = State::On;
                } else {
                    device.state = State::Off;
                }
            }
        }

    }
}

// /// If you're not on an RTU (ie. there's no hardware to interact with),
// /// then this function does nothing. Enabling the "rtu" feature will enable
// /// the real update() function.
// #[cfg(not(feature = "rtu"))]
// impl Device {
//     pub async fn update(_: &mut Device, _: &Mode) {
//         println!("RTU feature hasn't been enabled, update() does nothing.");
//     }
// }



#[cfg(test)]
mod tests {
    use super::*;
    use brewdrivers::omega::CN7500;
    use serial_test::serial;
    use tokio::test;

    #[test]
    #[serial]
    async fn test_serialize_deserialize_device() {
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

    #[test]
    #[serial]
    async fn test_cn7500_set_pv_from_device() {
        let mut cn = CN7500::new(0x16, "/dev/ttyUSB0", 19200).await.unwrap();
        assert!(cn.set_sv(150.0).await.is_ok());
        assert_eq!(cn.get_sv().await.unwrap(), 150.0);

        let device_data = r#"{
            "name": "CN7500",
            "id": "some id",
            "driver": "Omega",
            "state": "Off",
            "addr": 0,
            "pv": 0.0,
            "sv": 140.5,
            "controller_addr": 22
        }"#;

        let mut device: Device = serde_json::from_str(device_data).unwrap();
        Device::update(&mut device, &Mode::Write).await;
        assert_eq!(cn.get_sv().await.unwrap(), 140.5);
    }
}