extern crate indicatif;
extern crate metaheuristics;
extern crate nanoserde;
extern crate rand;
extern crate time;

use indicatif::{ProgressBar, ProgressStyle};
use nanoserde::{DeJson, SerJson};
use std::path::{Path, PathBuf};
use time::Duration;

mod traveler;

#[derive(DeJson, SerJson, PartialEq, Clone)]
struct Coordinate {
    description: String,
    name: String,
    position: [f64; 3],
}

fn read_coordinates(folder: &Path) -> Vec<Coordinate> {
    let mut coordinates = Vec::new();

    for entry in std::fs::read_dir(folder)
        .unwrap()
        .filter_map(Result::ok)
        .filter(|d| {
            if let Some(e) = d.path().extension() {
                e == "json"
            } else {
                false
            }
        })
    {
        let path = entry.path();
        let file_contents = std::fs::read_to_string(path.clone()).unwrap();
        let mut coordinate: Coordinate = DeJson::deserialize_json(&file_contents).unwrap();
        coordinate.name = format!(
            "{}-{}",
            path.parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_string_lossy(),
            coordinate.name
        );
        coordinates.push(coordinate);
    }

    coordinates
}

fn coordinates_to_tuples(coordinates: &[Coordinate]) -> Vec<(f64, f64, f64)> {
    coordinates
        .iter()
        .map(|c| (c.position[0], c.position[1], c.position[2]))
        .collect()
}

fn sort_and_rename_files(
    mut coordinates: Vec<Coordinate>,
    duration: Duration,
    output_folder: &Path,
) {
    let tuples = coordinates_to_tuples(&coordinates);
    let tour = traveler::solve(&tuples, duration);

    for (i, index) in tour.route.iter().enumerate() {
        let mut coordinate = &mut coordinates[*index];
        coordinate.name = format!("{}-{}", i + 1, coordinate.name.clone());

        let new_contents = SerJson::serialize_json(coordinate);

        let new_file_path = output_folder.join(coordinate.name.clone() + ".json");

        std::fs::write(new_file_path, new_contents).unwrap();
    }
}

fn main() {
    // let opts = MyOptions::parse_args_default_or_exit();
    let opts = xflags::parse_or_exit! {
        ///path to the folder containing the JSON files
        repeated input: PathBuf

        ///path to the folder output the sorted JSON files
        optional -o,--output output: PathBuf

        ///duration in seconds to solve the traveler problem
        optional -t,--time time: i64
    };

    let input_folders: Vec<PathBuf> = opts.input;
    let duration = Duration::seconds(opts.time.unwrap_or(15));

    let output_folder = &opts
        .output
        .unwrap_or(std::env::current_dir().expect("Current directory cannot be accessed."));
    std::fs::create_dir_all(&output_folder).unwrap();

    let mut all_coordinates = Vec::new();

    let pb = ProgressBar::new((input_folders.len()) as u64).with_style(
        ProgressStyle::with_template("{bar:16.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap(),
    );

    for folder in input_folders {
        pb.set_message(format!("{}", folder.file_name().unwrap().to_string_lossy()));

        let coordinates = read_coordinates(&folder);
        all_coordinates.extend(coordinates);

        pb.inc(1);
    }

    println!("{}", all_coordinates.len());

    sort_and_rename_files(all_coordinates, duration, &output_folder);
}
