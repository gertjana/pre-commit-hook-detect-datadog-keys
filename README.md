# pre-commit-hook-detect-datadog-keys

[Pre-commit](https://pre-commit.com) hook to detect Datadog API Keys

Finds case-insensitive strings that look like Datadog API/APP keys in your codebase.

to be more precise it looks for the following regex: `^.*ap[ip]_key[\ ]*=[\ ]*  \"[a-f0-9]+\".*$`

## usage

Add the following to your `.pre-commit-config.yaml` file:

```yaml
- repo: https://github.com/gertjana/pre-commit-hook-detect-datadog-secrets
  rev: 1.0.0
  hooks:
    - id: detect-datadog-secrets
```
