#![allow(non_snake_case)]
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


#[cfg(test)]
mod tests {
    use super::*;

    fn testing_model() -> Model {
        Model {
            name: String::from("Testing Model"),
            description: Some(String::from("description")),
            slackChannel: Some(String::from("@luke")),
            slackWebhook: Some(String::from("webhook.goes.here")),
            masterAddr: Some(String::from("192.168.0.1")),
            date: Some(String::from("2022/10/10")),
            mode: Mode::Read,
            id: 1,
            RTUs: vec![],
        }
    }

    #[test]
    fn test_to_json() {
        let model: Model = testing_model();
        let res = serde_json::to_string(&model);
        assert!(res.is_ok());

        let expected = r#"{"name":"Testing Model","description":"description","slackChannel":"@luke","slackWebhook":"webhook.goes.here","masterAddr":"192.168.0.1","date":"2022/10/10","mode":"Read","id":1,"RTUs":[]}"#;
        assert_eq!(res.unwrap(), expected);
    }

}