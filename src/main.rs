use clap::Parser;
use env_logger;
use log;
use regex::Regex;
use std::fs::read_dir;
use std::path::PathBuf;
use strucs::{Cli, Config};

mod strucs;

// include string from file
const DEFAULT_RULES: &str = include_str!("default_rules.yaml");

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
pub fn run_detector(
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

pub fn generate_dependabot_config(config: Config, search_dir: PathBuf) -> serde_yaml::Value {
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
    let walker = walkdir::WalkDir::new(search_dir.clone());
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
                        // remove the search_dir from the path
                        serde_yaml::Value::String(
                            path.strip_prefix(search_dir.clone())
                                .unwrap()
                                .to_str()
                                .unwrap()
                                .to_string(),
                        ),
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
    return dependabot_config;
}

pub fn load_configs(
    enable_default_rules: bool,
    extra_configuration_file: Option<PathBuf>,
) -> Config {
    let mut config: Config = Config {
        generators: Vec::new(),
    };
    if enable_default_rules {
        log::debug!("Default rules are enabled");
        config = serde_yaml::from_str(DEFAULT_RULES).unwrap();
    }
    if let Some(extra_configuration_file) = extra_configuration_file {
        let raw_config = std::fs::read_to_string(extra_configuration_file).unwrap();
        let extra_config: Config = serde_yaml::from_str(&raw_config).unwrap();
        // Note that this is a simple overwrite because we only have generators
        config.generators.extend(extra_config.generators);
    } else {
        log::debug!("No extra configuration file defined");
    }
    return config;
}

fn main() {
    let args = Cli::parse();
    log::debug!("Args: {:?}", args);
    env_logger::init();

    let config = load_configs(args.enable_default_rules, args.extra_configuration_file);

    let dependabot_config = generate_dependabot_config(config, args.search_dir);

    println!("{}", serde_yaml::to_string(&dependabot_config).unwrap());
}
