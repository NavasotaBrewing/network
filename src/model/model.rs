use serde::{Serialize, Deserialize};

use crate::model::rtu::RTU;
use crate::model::{Mode, get_rtu_id};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Model {
    pub name: String,
    pub description: Option<String>,
    pub slackChannel: Option<String>,
    pub slackWebhook: Option<String>,
    pub masterAddr: Option<String>,
    pub date: Option<String>,
    pub mode: Mode,
    pub id: u32,
    pub RTUs: Vec<RTU>
}

impl Model {
    pub fn update(source_model: &Self, mode: &Mode) -> Self {
        
        let mut model = source_model.clone();

        for mut rtu in &mut model.RTUs {
            // This is called on the RTU.
            // Only update this RTU.
            if rtu.id == get_rtu_id() {
            	RTU::update(&mut rtu, &mode);
            }
        }
        model
    }
}