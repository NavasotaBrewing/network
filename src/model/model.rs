#![allow(non_snake_case)]
use serde::{Serialize, Deserialize};

use crate::model::rtu::RTU;
use crate::model::{Mode, get_rtu_id};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub mode: Mode,
    pub id: u32,
    pub RTUs: Vec<RTU>
}

impl Model {
    pub async fn update(source_model: &Self, mode: &Mode) -> Self {
        
        let mut model = source_model.clone();

        // TODO: find() instead of iterating like a dumbass
        for mut rtu in &mut model.RTUs {
            // This is called on the RTU.
            // Only update this RTU.
            if rtu.id == get_rtu_id() {
            	RTU::update(&mut rtu, &mode).await;
            }
        }
        model
    }
}
