mod dart;
mod rust;
mod udl;

use std::io::Write;

use udl::*;

use crate::{dart::DartGenerator, rust::RustGenerator};

const EXAMPLES_DIR: &str = "examples";
const OUTPUT_DIR: &str = "gen";
const OUTPUT_FILE: &str = "output";

fn main() {
    let file = std::fs::File::open(format!("{EXAMPLES_DIR}/billing_app.yaml")).unwrap();
    let str = std::io::read_to_string(file).unwrap();
    let udl: UDL = serde_yaml::from_str(&str).unwrap();
    // println!("{:#?}", udl);
    let generators: Vec<Box<dyn LangGenerator>> =
        vec![Box::new(DartGenerator()), Box::new(RustGenerator())];
    for generator in generators {
        let code = generator.generate(&udl);

        let _ = std::fs::create_dir_all(OUTPUT_DIR);
        let output_file = std::fs::File::create(format!(
            "{OUTPUT_DIR}/{OUTPUT_FILE}.{}",
            generator.extension()
        ))
        .unwrap();
        let mut output = std::io::BufWriter::new(output_file);
        output.write_all(code.as_bytes()).unwrap();
        output.flush().unwrap();

        generator.format(&format!(
            "{OUTPUT_DIR}/{OUTPUT_FILE}.{}",
            generator.extension()
        ));
    }
}
