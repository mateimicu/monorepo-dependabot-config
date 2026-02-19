use anyhow::{bail, Context};
use clap::Parser;
use regex::Regex;
use std::fs::read_dir;
use std::path::PathBuf;
use strucs::{Cli, Config};

mod strucs;

const DEFAULT_RULES: &str = include_str!("default_rules.yaml");

fn detector_has_file_matching(dir_path: PathBuf, regex_pattern: String) -> anyhow::Result<bool> {
    let re = Regex::new(&regex_pattern).context("Failed to compile regex pattern")?;
    let paths = read_dir(&dir_path)
        .with_context(|| format!("Failed to read directory '{}'", dir_path.display()))?;

    log::debug!("Evaluating paths {:?}", paths);
    for path in paths {
        let path = path
            .context("Failed to read directory entry")?
            .path();
        log::debug!(
            "Evaluating {}",
            path.to_str().context("Path contains invalid UTF-8")?
        );
        if path.is_file() {
            let file_name = path
                .file_name()
                .context("Failed to get file name")?
                .to_str()
                .context("File name contains invalid UTF-8")?;
            if re.is_match(file_name) {
                log::debug!("Matched on {}", file_name);
                return Ok(true);
            }
        }
    }
    Ok(false)
}

pub fn run_detector(
    detector_type: String,
    detector_config: serde_yaml::Value,
    dir_path: PathBuf,
) -> anyhow::Result<bool> {
    log::debug!(
        "Running detector: {} on path {} with config {}",
        detector_type,
        dir_path
            .to_str()
            .context("Path contains invalid UTF-8")?,
        serde_yaml::to_string(&detector_config).context("Failed to serialize detector config")?
    );

    match detector_type.as_str() {
        "DIRECOTRY_HAS_FILE_FILE_MATCHING" => {
            let regex_pattern = detector_config["regex"]
                .as_str()
                .context("Detector config missing 'regex' field")?;
            detector_has_file_matching(dir_path, regex_pattern.to_string())
        }
        _ => {
            bail!("Unknown detector type '{}'", detector_type);
        }
    }
}

pub fn generate_dependabot_config(
    config: Config,
    search_dir: PathBuf,
) -> anyhow::Result<serde_yaml::Value> {
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
        let entry = entry.context("Failed to read directory entry during walk")?;
        let path = entry.path();
        if path.is_dir() {
            for generator in &config.generators {
                let detector = &generator.detector;
                let detector_type = &detector.type_;
                let detector_config = &detector.config;

                if run_detector(
                    detector_type.to_string(),
                    detector_config.clone(),
                    path.to_path_buf(),
                )? {
                    let generated_block = &generator.generated_block;
                    let generated_block = generated_block
                        .as_mapping()
                        .context("Generated block is not a YAML mapping")?;
                    let mut generated_block = generated_block.clone();
                    generated_block.insert(
                        serde_yaml::Value::String("directory".to_string()),
                        serde_yaml::Value::String(
                            path.strip_prefix(search_dir.clone())
                                .context("Failed to strip search directory prefix from path")?
                                .to_str()
                                .context("Path contains invalid UTF-8")?
                                .to_string(),
                        ),
                    );
                    let generated_block = serde_yaml::Value::Mapping(generated_block);
                    dependabot_config["updates"]
                        .as_sequence_mut()
                        .context("Updates field is not a YAML sequence")?
                        .push(generated_block);
                }
            }
        }
    }
    Ok(dependabot_config)
}

pub fn load_configs(
    enable_default_rules: bool,
    extra_configuration_file: Option<PathBuf>,
) -> anyhow::Result<Config> {
    let mut config: Config = Config {
        generators: Vec::new(),
    };
    if enable_default_rules {
        log::debug!("Default rules are enabled");
        config = serde_yaml::from_str(DEFAULT_RULES).context("Failed to parse default rules")?;
    }
    if let Some(extra_configuration_file) = extra_configuration_file {
        let raw_config = std::fs::read_to_string(&extra_configuration_file).with_context(|| {
            format!(
                "Failed to read configuration file '{}'",
                extra_configuration_file.display()
            )
        })?;
        let extra_config: Config =
            serde_yaml::from_str(&raw_config).context("Failed to parse extra configuration")?;
        config.generators.extend(extra_config.generators);
    } else {
        log::debug!("No extra configuration file defined");
    }
    Ok(config)
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Cli::parse();
    log::debug!("Args: {:?}", args);

    let config = load_configs(args.enable_default_rules, args.extra_configuration_file)?;

    let dependabot_config = generate_dependabot_config(config, args.search_dir)?;

    println!(
        "{}",
        serde_yaml::to_string(&dependabot_config).context("Failed to serialize output YAML")?
    );
    Ok(())
}
