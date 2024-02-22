# pre-commit-hook-detect-datadog-keys

Pre commit hook to detect Datadog API Keys

Finds case insensitive strings that look like Datadog API/APP keys in your codebase.

## usage

Add the following to your `.pre-commit-config.yaml` file:

```yaml
-   repo: https://github.com/gertjana/pre-commit-hook-detect-datadog-secrets
    rev: 1.0.0
    hooks:
    -   id: detect-datadog-secrets
```
