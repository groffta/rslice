use std::path::Path;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::time::{Duration, Instant};
use clap::{Arg, App, SubCommand};


fn main() {
    pretty_env_logger::init();
    let matches = App::new("RSlice")
        .version("0.0.1")
        .author("Todd Groff <groffta@ornl.gov>")
        .about("High performance STL slicing engine for additive manufacturing")
        .arg(
            Arg::with_name("INPUT_FILE")
                .help("Input STL to process")
                .required(true)
                .index(1)
        )
        .get_matches();

    let load_start = Instant::now();
    let input_file_path = Path::new(matches.value_of("INPUT_FILE").unwrap());
    let input_fh = match OpenOptions::new()
        .read(true)
        .open(input_file_path)
    {
        Ok(fh) => fh,
        Err(e) => {
            log::error!("Could not open input file {:?}: {:?}", input_file_path, e);
            return;
        }
    };

    let mut input_reader = BufReader::new(input_fh);
    
    let mesh = match nom_stl::parse_stl(&mut input_reader) {
        Ok(mesh) => mesh,
        Err(e) => {
            log::error!("Error parsing mesh: {:?}", e);
            return;
        }
    };
    let load_duration = Instant::now() - load_start;
    let triangle_count = mesh.triangles().len();
    println!("Parsed mesh with {} triangles in {}ms", triangle_count, load_duration.as_millis())
}
