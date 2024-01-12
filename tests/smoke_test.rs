use serde_yaml;
use std::fs;

#[test]
fn execute_examples_with_no_default_config() {
    // list all example directories in examples
    // for each directory find run the main file with
    // disabled default config
    for example in fs::read_dir("examples/no-default-config/").unwrap() {
        let example = example.unwrap();
        let example_path = example.path();

        let config_file = example_path.join("config.yml");
        let expected_output_file = example_path.join("expected_generated_config.yml");
        let working_dir = example_path.join("working-dir");

        let process_output = std::process::Command::new("cargo")
            .arg("run")
            .arg("--")
            .arg("--extra-configuration-file")
            .arg(config_file)
            .arg(working_dir)
            .output()
            .expect("failed to execute process");

        let expected_output_raw = fs::read_to_string(expected_output_file).unwrap();
        let output_raw = String::from_utf8(process_output.stdout).unwrap();

        let expected_output: serde_yaml::Value =
            serde_yaml::from_str(&expected_output_raw).unwrap();
        let output: serde_yaml::Value = serde_yaml::from_str(&output_raw).unwrap();

        // write this to a file
        assert_eq!(expected_output, output);
    }
}
