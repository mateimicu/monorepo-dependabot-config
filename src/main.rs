use env_logger;
use log;
use regex::Regex;
use std::fs::read_dir;
use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
struct Cli {
    // If you want to use the built  in rules
    #[arg(short, long, default_value_t = false)]
    enable_default_rules: bool,

    // Custom cofiguration file to use.
    #[arg(short = 'c', long)]
    extra_configuration_file: Option<PathBuf>,

    // the directory to search pachages in
    #[clap(default_value = ".")]
    search_dir: PathBuf,
    // Ideas:
    // *  We could also add a command to check if a generated config matches
    //    the current configuration given the path
    // * Also we could configure if we dump the generated config to stdout
    //   or to a file
}

#[derive(Deserialize, Serialize)]
struct Detector {
    #[serde(rename = "type")]
    type_: String,
    config: serde_yaml::Value,
}

#[derive(Deserialize, Serialize)]
struct Generator {
    detector: Detector,
    // this can be any yaml value
    generated_block: serde_yaml::Value,
}

#[derive(Deserialize, Serialize)]
struct Config {
    // The rules to run
    generators: Vec<Generator>,
}
fn detector_has_file_matching(dir_path: PathBuf, regex_pattern: String) -> bool {
    let re = Regex::new(&regex_pattern).unwrap();
    // list all files in the directory top level
    let paths = read_dir(dir_path).unwrap();

    log::debug!("Evaluating paths {:?}", paths);
    for path in paths {
        let path = path.unwrap().path();
        // if any file matches the regex return true
        log::debug!("Evaluating {}", path.to_str().unwrap());
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if re.is_match(file_name) {
                log::debug!("Matched on {}", file_name);
                return true;
            }
        }
    }
    return false;
}
fn run_detector(
    detector_type: String,
    detector_config: serde_yaml::Value,
    dir_path: PathBuf,
) -> bool {
    // for now we only have the DIRECOTRY_HAS_FILE_FILE_MATCHING
    log::debug!(
        "Running detector: {} on path {} with config {}",
        detector_type,
        dir_path.to_str().unwrap(),
        serde_yaml::to_string(&detector_config).unwrap()
    );

    match detector_type.as_str() {
        "DIRECOTRY_HAS_FILE_FILE_MATCHING" => {
            let regex_pattern = detector_config["regex"].as_str().unwrap();
            return detector_has_file_matching(dir_path, regex_pattern.to_string());
        }
        _ => {
            log::debug!("Unknown detector type {}", detector_type);
            return false;
        }
    }
}

fn generate_config(config: Config, search_dir: PathBuf) -> String {
    // recursevely search the search_dir
    // for each directory found run the generator
    // for each generator call the appropiate Detector
    // if detector matches then append to the raw yaml
    // a generated block with a directory overwrite
    // and return the file
    let mut dependabot_config = serde_yaml::Value::Mapping(serde_yaml::Mapping::new());
    dependabot_config["version"] = serde_yaml::Value::String("2".to_string());
    dependabot_config["updates"] = serde_yaml::Value::Sequence(serde_yaml::Sequence::new());
    // search recursevely the search_dir
    let walker = walkdir::WalkDir::new(search_dir);
    for entry in walker {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            // for each generator
            for generator in &config.generators {
                // run the detector
                let detector = &generator.detector;
                let detector_type = &detector.type_;
                let detector_config = &detector.config;

                // if detector matches
                if run_detector(
                    detector_type.to_string(),
                    detector_config.clone(),
                    path.to_path_buf(),
                ) {
                    // a generated block with a directory overwrite
                    let generated_block = &generator.generated_block;
                    let generated_block = generated_block.as_mapping().unwrap();
                    let mut generated_block = generated_block.clone();
                    generated_block.insert(
                        serde_yaml::Value::String("directory".to_string()),
                        serde_yaml::Value::String(path.to_str().unwrap().to_string()),
                    );
                    let generated_block = serde_yaml::Value::Mapping(generated_block);
                    dependabot_config["updates"]
                        .as_sequence_mut()
                        .unwrap()
                        .push(generated_block);
                }
            }
        }
    }
    return serde_yaml::to_string(&dependabot_config).unwrap();
}

fn main() {
    let args = Cli::parse();
    env_logger::init();
    // TODO(mmicu): if args.enable_default_rules is defined we
    // should load the default rules from the default location (
    // or ideally from an embedded string)
    // combine it with the read configuration file and then
    // run the rules on the search_dir
    //
    // For now will just use the extra_configuration_file
    // if it is defined
    if let Some(extra_configuration_file) = args.extra_configuration_file {
        let raw_config = std::fs::read_to_string(extra_configuration_file).unwrap();
        let config: Config = serde_yaml::from_str(&raw_config).unwrap();

        let dependabot_config = generate_config(config, args.search_dir);
        println!("{}", dependabot_config);
    } else {
        println!("No extra configuration file defined");
    }
}
