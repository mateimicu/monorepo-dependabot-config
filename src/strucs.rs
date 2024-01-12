use std::path::PathBuf;

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
pub struct Cli {
    // If you want to use the built  in rules
    #[arg(short, long, default_value_t = false)]
    pub enable_default_rules: bool,

    // Custom cofiguration file to use.
    #[arg(short = 'c', long)]
    pub extra_configuration_file: Option<PathBuf>,

    // the directory to search pachages in
    #[clap(default_value = ".")]
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
    pub config: serde_yaml::Value,
}

#[derive(Deserialize, Serialize)]
pub struct Generator {
    pub detector: Detector,
    // this can be any yaml value
    pub generated_block: serde_yaml::Value,
}

#[derive(Deserialize, Serialize)]
pub struct Config {
    // The rules to run
    pub generators: Vec<Generator>,
}
