use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub id: String,
    pub description: Option<String>,
    pub immutable: Option<bool>,
    pub error: Option<String>,
    pub properties: HashMap<String, Property>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Property {
    Type(String),
    Map(HashMap<PropertyKey, String>),
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PropertyKey {
    Description,
    Type,
    Format,
    Length,
    Min,
    Max,
    Private,
    Default,
}
