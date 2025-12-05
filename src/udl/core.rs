use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UDL {
    #[serde(rename = "udl_version")]
    pub version: String,
    pub project: ProjectMeta,
    pub enums: Vec<Enum>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMeta {
    pub name: String,
    pub version: String,
    pub description: String,
    pub authors: Vec<ProjectAuthor>,
    pub license: String,
    pub namespace: String,
    pub models_only: bool,
    pub target_platforms: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectAuthor {
    name: String,
    email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub id: String,
    pub description: Option<String>,
    pub variants: Vec<EnumKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnumKind {
    Simple(String),
    Complex(EnumVariant),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    pub id: String,
    pub description: Option<String>,
    pub value: EnumVariantValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EnumVariantValue {
    Single(String),
    Multiple(HashMap<String, String>),
}
