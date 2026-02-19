use serde_yml;
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

        let expected_output: serde_yml::Value =
            serde_yml::from_str(&expected_output_raw).unwrap();
        let output: serde_yml::Value = serde_yml::from_str(&output_raw).unwrap();

        println!("Expected output: {:#?}", expected_output);
        println!("Output: {:#?}", output);

        assert_eq!(expected_output["version"], output["version"]);

        // Because we have a list of `updates` the order matters. Depending
        // on the OS we might have different orderings. So we need to compare
        // the sets of updates
        let updates_expected_output = expected_output["updates"].as_sequence().unwrap();
        let updates_output = output["updates"].as_sequence().unwrap();

        let set_expected_output: std::collections::HashSet<_> =
            updates_expected_output.iter().collect();
        let set_output: std::collections::HashSet<_> = updates_output.iter().collect();

        assert_eq!(set_expected_output, set_output);
    }
}
