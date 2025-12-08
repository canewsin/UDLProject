use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub id: String,
    pub description: Option<String>,
    pub variants: Vec<EnumKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum EnumKind {
    Simple(String),
    Complex(EnumVariant),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumVariant {
    pub id: String,
    pub description: Option<String>,
    pub value: EnumVariantValue,
    pub target: Option<String>,
    pub target_field: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum EnumVariantValue {
    Single(String),
    Multiple(HashMap<String, String>),
}
