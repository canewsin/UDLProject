use serde::{Deserialize, Serialize};

use crate::udl::{class::Class, enums::Enum};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct UDL {
    #[serde(rename = "udl_version")]
    pub version: String,
    pub project: ProjectMeta,
    pub enums: Vec<Enum>,
    pub models: Vec<Class>,
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
