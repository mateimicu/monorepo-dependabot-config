use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
pub struct Cli {
    /// Enable the built-in default detection rules
    #[arg(short, long, default_value_t = false, help = "Enable built-in default rules for common package ecosystems")]
    pub enable_default_rules: bool,

    /// Path to an extra configuration file with custom generators
    #[arg(short = 'c', long, help = "Path to an extra YAML configuration file with custom generators")]
    pub extra_configuration_file: Option<PathBuf>,

    /// Root directory to search for packages
    #[clap(default_value = ".", help = "Root directory to search for packages recursively")]
    pub search_dir: PathBuf,
    // Ideas:
    // *  We could also add a command to check if a generated config matches
    //    the current configuration given the path
    // * Also we could configure if we dump the generated config to stdout
    //   or to a file
}

#[derive(Deserialize, Serialize)]
pub struct Detector {
    #[serde(rename = "type")]
    pub type_: String,
    pub config: serde_yml::Value,
}

#[derive(Deserialize, Serialize)]
pub struct Generator {
    pub detector: Detector,
    // this can be any yaml value
    pub generated_block: serde_yml::Value,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    // The rules to run
    pub generators: Vec<Generator>,
}
