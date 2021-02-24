//! This represents one RTU in the BCS Model.
use std::net::SocketAddrV4;

use serde::{Serialize, Deserialize};

use crate::model::device::Device;
use crate::model::Mode;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RTU {
    pub name: String,
    pub location: String,
    pub id: String,
    pub ipv4: SocketAddrV4,
    pub devices: Vec<Device>
}


impl RTU {
    /// This calls update on each device contained in it 
    pub async fn update(rtu: &mut RTU, mode: &Mode) {
        for mut device in &mut rtu.devices {
            Device::update(&mut device, &mode).await;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_deserialize_rtu() {
        let raw_rtu = r#"
        {
            "name": "RTU Name",
            "location": "some location",
            "id": "some id",
            "ipv4": "192.168.43.38:3012",
            "devices": [
                {
                    "name": "Some Relay",
                    "id": "some id",
                    "driver": "STR1",
                    "state": "Off",
                    "addr": 0,
                    "controller_addr": 0
                }
            ]
        }
        "#;

        let rtu: RTU = serde_json::from_str(raw_rtu).unwrap();
        assert_eq!(rtu.name, "RTU Name");
        assert!(rtu.devices.len() == 1);
    }
}