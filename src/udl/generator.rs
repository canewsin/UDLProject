use crate::udl::{UDL, core::Enum};

pub trait LangGenerator {
    fn gen_enum(&self, enumm: &Enum) -> String;

    fn generate(&self, udl: &UDL) -> String;

    fn format(&self, path: &str);
}
