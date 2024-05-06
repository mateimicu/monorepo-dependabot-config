# monorepo-dependabot-config

*Problem:* Dependabot configuration files are hard to maintain in a monorepo. It can't auto discover the different package ecosystems in the monorepo.
*Solution:* A tool that can auto generate dependabot configuration files based on the package ecosystems in the monorepo.

## Features

- [x] Generate dependabot configuration files based on the package ecosystems
- [x] Detect package ecosystems based on the files in the monorepo
- [x] Have a configuration file to specify extra rules
- [~] Have default rules for common package ecosystems

## Quick Star

Just run the `monorepo-dependabot-config .` command in the root of your monorepo.
It will generate a dependabot configuration file based on the package ecosystems it finds in the monorepo.

You can also run this in CI to validate no package/project is left behind.

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

```yaml
generate:
- detector:
  direcotry-has-file-file-matching:
    regex: "aaa"
  generated_block:
    TBD
```
