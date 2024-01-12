# monorepo-dependabot-config

Generate dependabot configuration files

 This is a Work in Progress !

## How to install

*Install from crates.io*

```bash
cargo install monorepo-dependabot-config
```


*Build from source*

After cloning the repository  run:
```bash
cargo run
```


## How to Run

1. Generate a config file for a new repository
```
monorepo-dependabot-config --enable-default-rules .
```
Will generate all the possible dependabot rules it can find.

2. Configure specific rules
```
monorepo-dependabot-config --extra-configuration-file=extra-config.yml .
```

where the configuration is
```yaml
# extra-config.yml
generators:
- detector:
    type: DIRECOTRY_HAS_FILE_FILE_MATCHING
    config:
      regex: ".*.tf"
  generated_block:
    package-ecosystem: terraform
    schedule:
      interval: daily
- detector:
    type: DIRECOTRY_HAS_FILE_FILE_MATCHING
    config:
      regex: ".*.hcl"
  generated_block:
    package-ecosystem: terraform
    schedule:
      interval: daily
```


## To Do

* [ ] Add integration tests (at least a few examples)
* [ ] Populate the default rules with some common values (py, js, ts, terraform etc)
  * [ ] Start a page with supported selectors and rules by default
* [ ] Add docs about how to run locally and in CI
* [ ] Add tests on common examples
* [ ] Add a cli option to check if all the packages are monitored. It can be a simple generate new config 
      deep compare with what is in the repository
* [ ] Consider adding extra detector types
  * [ ] HAS_FILE
  * [ ] HAS_DIRECOTRY_REGEX -> can be used to detect if we are in a git repo or have a .github directory
  * [ ] HAS_FILE_WITH_CONTENT_MATCHING
  * [ ] HAS_FILE_DEEP_PATH -
* [ ] Question: should we remove `type` and rely on a name only ?

### Other detectors

We could also have some detectors detectors:
```yaml
generators:
- detector:
    type: HAS_FILE
    config:
      file_names:
      match_type: ALL | ANY

```

```yaml
generators:
- detector:
    type: HAS_FILE_WITH_CONTENT_MATCHING
    config:
      regex: "a-regex"

```

### Alternative usage

The config file could have
```
generate:
- detector:
  direcotry-has-file-file-matching:
    regex: "aaa"
  generated_block:
    TBD
```
