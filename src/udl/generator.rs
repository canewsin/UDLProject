use crate::udl::{UDL, core::Enum};

pub trait LangGenerator {
    fn gen_enum(&self, enumm: &Enum) -> String;

    fn gen_struct(&self, class: &UDL) -> String;

    fn generate(&self, udl: &UDL, format: bool) -> String;

    fn format(&self, code: &str) -> String;
}
