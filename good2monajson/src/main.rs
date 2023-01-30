use std::fs::{self, File};
use std::io::prelude::*;
use std::path::PathBuf;

use nanoserde::{DeJson, SerJson};
mod convert;
mod convert_maps;
mod good;
mod monajson;

fn main() {
    let flags = xflags::parse_or_exit! {
        /// Input path of GOOD JSON file to convert
        required input: PathBuf
        /// Output path of the converted Mona JSON
        optional -o,--output output: PathBuf
    };

    let input_path = flags.input;
    let output_path = {
        let mut path = match flags.output {
            Some(path) => path,
            None => input_path.with_file_name("mona.json"),
        };
        if path.is_dir() {
            path.push(input_path.file_name().expect("Input is not a file."));
        }
        path
    };

    let input_json: good::GOOD = DeJson::deserialize_json(
        fs::read_to_string(input_path)
            .expect("GOOD JSON file cannot be read.")
            .as_str(),
    )
    .unwrap();

    let output_json = convert::convert(input_json);

    // println!("{} from {}", input_json.format, input_json.source,);

    let mut file = File::create(output_path).expect("Unable to create output file");
    file.write_all(SerJson::serialize_json(&output_json).as_bytes())
        .expect("Unable to write output file");
}
