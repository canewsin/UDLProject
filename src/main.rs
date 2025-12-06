mod rust;
mod udl;

use std::io::Write;

use udl::*;

use crate::rust::RustGenerator;

const EXAMPLES_DIR: &str = "examples";
const OUTPUT_DIR: &str = "gen";
const OUTPUT_FILE: &str = "output.rs";

fn main() {
    let file = std::fs::File::open(format!("{EXAMPLES_DIR}/udl-example.udl")).unwrap();
    let str = std::io::read_to_string(file).unwrap();
    let udl: UDL = serde_yaml::from_str(&str).unwrap();
    // println!("{:#?}", udl);
    let generator = RustGenerator();
    let code = generator.generate(&udl);

    let _ = std::fs::create_dir_all(OUTPUT_DIR);
    let output_file = std::fs::File::create(format!("{OUTPUT_DIR}/{OUTPUT_FILE}")).unwrap();
    let mut output = std::io::BufWriter::new(output_file);
    output.write_all(code.as_bytes()).unwrap();
    output.flush().unwrap();

    RustGenerator::format(&generator, &format!("{OUTPUT_DIR}/{OUTPUT_FILE}"));
}
