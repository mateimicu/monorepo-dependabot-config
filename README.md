# monorepo-dependabot-config

Generate dependabot configuration files

This is a WIP !


Possible input:
```yaml
generators:
- detector:
    type: DIRECOTRY_HAS_FILE_FILE_MATCHING
    config:
      regex: "a-regex"
  generated_block:
    # Here we should support all the keys from https://docs.github.com/en/code-security/dependabot/dependabot-version-updates/configuration-options-for-the-dependabot.yml-file#configuration-options-for-the-dependabotyml-file
    # except directory that gets populated
- detector:
    type: DIRECOTRY_HAS_FILE_FILE_MATCHING
  generated_block:
    # TBD
- detector:
    type: DIRECOTRY_HAS_FILE_FILE_MATCHING
  generated_block:
    # TBD
```

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
