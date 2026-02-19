use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
pub struct Cli {
    /// Enable the built-in default detection rules
    #[arg(
        short,
        long,
        default_value_t = false,
        help = "Enable built-in default rules for common package ecosystems"
    )]
    pub enable_default_rules: bool,

    /// Path to an extra configuration file with custom generators
    #[arg(
        short = 'c',
        long,
        help = "Path to an extra YAML configuration file with custom generators"
    )]
    pub extra_configuration_file: Option<PathBuf>,

    /// Root directory to search for packages
    #[clap(
        default_value = ".",
        help = "Root directory to search for packages recursively"
    )]
    pub search_dir: PathBuf,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum DetectorType {
    #[serde(rename = "DIRECTORY_HAS_FILE_MATCHING")]
    DirectoryHasFileMatching,
}

#[derive(Deserialize, Serialize)]
pub struct Detector {
    #[serde(rename = "type")]
    pub type_: DetectorType,
    pub config: serde_yml::Value,
}

#[derive(Deserialize, Serialize)]
pub struct Generator {
    pub detector: Detector,
    pub generated_block: serde_yml::Value,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub generators: Vec<Generator>,
}
