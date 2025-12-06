use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub id: String,
    pub description: Option<String>,
    pub immutable: Option<bool>,
    pub properties: HashMap<String, Property>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Property {
    Type(String),
    Map(HashMap<String, String>),
}
