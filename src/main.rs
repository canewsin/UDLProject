mod rust;
mod udl;

use std::io::Write;

use udl::*;

use crate::rust::RustGenerator;

fn main() {
    let file = std::fs::File::open("examples/udl-example.udl").unwrap();
    let str = std::io::read_to_string(file).unwrap();
    let udl: UDL = serde_yaml::from_str(&str).unwrap();
    // println!("{:#?}", udl);
    let generator = RustGenerator();
    let code = generator.generate(&udl);

    let output_file = std::fs::File::create("gen/output.rs").unwrap();
    let mut output = std::io::BufWriter::new(output_file);
    output.write_all(code.as_bytes()).unwrap();
    output.flush().unwrap();

    RustGenerator::format(&generator, "gen/output.rs");
}
