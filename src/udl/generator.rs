use std::collections::HashSet;

use crate::udl::{UDL, class::Class, enums::Enum};

pub trait LangGenerator {
    fn extension(&self) -> &str;

    fn gen_enum(&self, enumm: &Enum) -> String;

    fn gen_class(&self, class: &Class, error_enum: Option<&Enum>) -> (String, HashSet<String>);

    fn generate(&self, udl: &UDL) -> String;

    fn format(&self, path: &str);
}
