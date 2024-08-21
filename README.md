# Pre-commit sort

[![CI](https://github.com/nim65s/pre-commit-sort/actions/workflows/ci.yml/badge.svg)](https://github.com/nim65s/pre-commit-sort/actions/workflows/ci.yml)
[![Release](https://github.com/nim65s/pre-commit-sort/actions/workflows/release.yml/badge.svg)](https://github.com/nim65s/pre-commit-sort/actions/workflows/release.yml)
[![pre-commit.ci status](https://results.pre-commit.ci/badge/github/nim65s/pre-commit-sort/main.svg)](https://results.pre-commit.ci/latest/github/nim65s/pre-commit-sort/main)

Rust validation, sort, deduplication and formatting for `.pre-commit-config.yaml` and `.pre-commit-hooks.yaml` files.

## Standalone validation

`pre-commit-sort`

## Use as pre-commit hook

Add this to the `repo` section of your `.pre-commit-config.yaml`:

```yaml
- repo: https://github.com/nim65s/pre-commit-sort
  rev: v0.3.0
  hooks:
  - id: pre-commit-sort
```

(This addition can be automated with `pre-commit-sort -i`)

## Installation

pre-commit will install this project automatically for you if you only want to use the hook.
But if you want a standalone version:

```
cargo binstall pre-commit-sort
```
