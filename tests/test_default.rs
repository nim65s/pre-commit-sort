use indoc::{formatdoc, indoc};
use pre_commit_sort::{ConfigHook, DeclareHook, Local, Meta, PreCommitConfig, Remote};

#[test]
fn test_serialize() {
    // Just test the serialization with an example .pre-commit-config.yaml
    let mut example = PreCommitConfig::new();

    let mut pre_commit = Remote::new(
        "https://github.com/pre-commit/pre-commit-hooks".to_string(),
        "v2.3.0".to_string(),
    );
    for hook in ["check-yaml", "end-of-file-fixer", "trailing-whitespaces"] {
        pre_commit.add_hook(ConfigHook::new(hook.to_string()));
    }
    example.add_remote(pre_commit);

    let mut black = Remote::new(
        "https://github.com/psf/black".to_string(),
        "22.10.0".to_string(),
    );
    black.add_hook(ConfigHook::new("black".to_string()));
    example.add_remote(black);

    let yaml = indoc! {"
        repos:
        - repo: https://github.com/pre-commit/pre-commit-hooks
          rev: v2.3.0
          hooks:
          - id: check-yaml
          - id: end-of-file-fixer
          - id: trailing-whitespaces
        - repo: https://github.com/psf/black
          rev: 22.10.0
          hooks:
          - id: black
        "};
    assert_eq!(yaml_serde::to_string(&example).unwrap(), yaml);
}

#[test]
fn test_sort() {
    let mut example = PreCommitConfig::new();

    // psf/black is added before pre-commit/pre-commit, but it should be put at the end on .sort()
    let mut black = Remote::new(
        "https://github.com/psf/black".to_string(),
        "22.10.0".to_string(),
    );
    black.add_hook(ConfigHook::new("black".to_string()));
    example.add_remote(black);

    let mut pre_commit = Remote::new(
        "https://github.com/pre-commit/pre-commit-hooks".to_string(),
        "v2.3.0".to_string(),
    );
    for hook in ["end-of-file-fixer", "check-yaml", "trailing-whitespaces"] {
        pre_commit.add_hook(ConfigHook::new(hook.to_string()));
    }
    example.add_remote(pre_commit);

    let yaml = indoc! {"
        repos:
        - repo: https://github.com/pre-commit/pre-commit-hooks
          rev: v2.3.0
          hooks:
          - id: check-yaml
          - id: end-of-file-fixer
          - id: trailing-whitespaces
        - repo: https://github.com/psf/black
          rev: 22.10.0
          hooks:
          - id: black
        "};
    assert_ne!(yaml_serde::to_string(&example).unwrap(), yaml);
    example.sort();
    assert_eq!(yaml_serde::to_string(&example).unwrap(), yaml);
}

#[test]
fn test_deserialize() {
    // Just test the deserialization with an example .pre-commit-config.yaml
    let mut example = PreCommitConfig::new();

    let mut pre_commit = Remote::new(
        "https://github.com/pre-commit/pre-commit-hooks".to_string(),
        "v2.3.0".to_string(),
    );
    for hook in ["check-yaml", "end-of-file-fixer", "trailing-whitespaces"] {
        pre_commit.add_hook(ConfigHook::new(hook.to_string()));
    }
    example.add_remote(pre_commit);

    let mut black = Remote::new(
        "https://github.com/psf/black".to_string(),
        "22.10.0".to_string(),
    );
    black.add_hook(ConfigHook::new("black".to_string()));
    example.add_remote(black);

    let yaml = indoc! {"
        repos:
        - repo: https://github.com/pre-commit/pre-commit-hooks
          rev: v2.3.0
          hooks:
          - id: check-yaml
          - id: end-of-file-fixer
          - id: trailing-whitespaces
        - repo: https://github.com/psf/black
          rev: 22.10.0
          hooks:
          - id: black
        "};
    assert_eq!(example, yaml_serde::from_str(yaml).unwrap());
}

#[test]
fn test_dedup() {
    let mut example = PreCommitConfig::new();

    let mut pre_commit = Remote::new(
        "https://github.com/pre-commit/pre-commit-hooks".to_string(),
        "v2.3.0".to_string(),
    );
    for hook in ["check-yaml", "end-of-file-fixer", "trailing-whitespaces"] {
        pre_commit.add_hook(ConfigHook::new(hook.to_string()));
    }
    example.add_remote(pre_commit);

    let mut black = Remote::new(
        "https://github.com/psf/black".to_string(),
        "22.10.0".to_string(),
    );
    black.add_hook(ConfigHook::new("black".to_string()));
    // Add black twice. The second should be removed after .sort()
    example.add_remote(black.clone());
    example.add_remote(black);

    let yaml = indoc! {"
        repos:
        - repo: https://github.com/pre-commit/pre-commit-hooks
          rev: v2.3.0
          hooks:
          - id: check-yaml
          - id: end-of-file-fixer
          - id: trailing-whitespaces
        - repo: https://github.com/psf/black
          rev: 22.10.0
          hooks:
          - id: black
        "};
    assert_ne!(yaml_serde::to_string(&example).unwrap(), yaml);
    example.sort();
    assert_eq!(yaml_serde::to_string(&example).unwrap(), yaml);
}

#[test]
fn test_install() {
    let mut example = PreCommitConfig::new();

    let mut pre_commit = Remote::new(
        "https://github.com/pre-commit/pre-commit-hooks".to_string(),
        "v2.3.0".to_string(),
    );
    for hook in ["check-yaml", "end-of-file-fixer", "trailing-whitespaces"] {
        pre_commit.add_hook(ConfigHook::new(hook.to_string()));
    }
    example.add_remote(pre_commit);

    let mut black = Remote::new(
        "https://github.com/psf/black".to_string(),
        "22.10.0".to_string(),
    );
    black.add_hook(ConfigHook::new("black".to_string()));
    example.add_remote(black);

    // This should add pre-commit-sort to the repos list
    example.install();

    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const REPO: &str = env!("CARGO_PKG_REPOSITORY");
    const NAME: &str = env!("CARGO_PKG_NAME");
    let yaml = formatdoc! {"
        repos:
        - repo: https://github.com/pre-commit/pre-commit-hooks
          rev: v2.3.0
          hooks:
          - id: check-yaml
          - id: end-of-file-fixer
          - id: trailing-whitespaces
        - repo: https://github.com/psf/black
          rev: 22.10.0
          hooks:
          - id: black
        - repo: {REPO}
          rev: v{VERSION}
          hooks:
          - id: {NAME}
        "};
    assert_eq!(yaml_serde::to_string(&example).unwrap(), yaml);
}

#[test]
fn test_dedup_rev() {
    let mut example = PreCommitConfig::new();

    let mut pre_commit = Remote::new(
        "https://github.com/pre-commit/pre-commit-hooks".to_string(),
        "v2.3.0".to_string(),
    );
    for hook in ["check-yaml", "end-of-file-fixer", "trailing-whitespaces"] {
        pre_commit.add_hook(ConfigHook::new(hook.to_string()));
    }
    example.add_remote(pre_commit);

    let mut black = Remote::new(
        "https://github.com/psf/black".to_string(),
        "22.10.0".to_string(),
    );
    black.add_hook(ConfigHook::new("black".to_string()));
    example.add_remote(black);

    // Add another black, but on a older version
    // This one should be cleaned out after the .sort() call
    let mut black = Remote::new(
        "https://github.com/psf/black".to_string(),
        "20.1.0".to_string(),
    );
    black.add_hook(ConfigHook::new("black".to_string()));
    example.add_remote(black);

    let yaml = indoc! {"
        repos:
        - repo: https://github.com/pre-commit/pre-commit-hooks
          rev: v2.3.0
          hooks:
          - id: check-yaml
          - id: end-of-file-fixer
          - id: trailing-whitespaces
        - repo: https://github.com/psf/black
          rev: 22.10.0
          hooks:
          - id: black
        "};
    assert_ne!(yaml_serde::to_string(&example).unwrap(), yaml);
    example.sort();
    assert_eq!(yaml_serde::to_string(&example).unwrap(), yaml);
}

#[test]
fn test_local() {
    // local repos don't have rev
    let yaml = indoc! {"
        repos:
        - repo: https://github.com/pre-commit/pre-commit-hooks
          rev: v2.3.0
          hooks:
          - id: check-yaml
          - id: end-of-file-fixer
          - id: trailing-whitespaces
        - repo: https://github.com/psf/black
          rev: 22.10.0
          hooks:
          - id: black
        - repo: local
          hooks:
          - id: some-id
            name: some-name
            entry: some entry
            language: rust
        "};
    let mut example = PreCommitConfig::new();

    let mut pre_commit = Remote::new(
        "https://github.com/pre-commit/pre-commit-hooks".to_string(),
        "v2.3.0".to_string(),
    );
    for hook in ["check-yaml", "end-of-file-fixer", "trailing-whitespaces"] {
        pre_commit.add_hook(ConfigHook::new(hook.to_string()));
    }
    example.add_remote(pre_commit);

    let mut black = Remote::new(
        "https://github.com/psf/black".to_string(),
        "22.10.0".to_string(),
    );
    black.add_hook(ConfigHook::new("black".to_string()));
    example.add_remote(black);

    let mut local = Local::new();
    let hook = DeclareHook::new(
        "some-id".to_string(),
        "some-name".to_string(),
        "some entry".to_string(),
        "rust".to_string(),
    );
    local.add_hook(hook);
    example.add_local(local);

    assert_eq!(yaml_serde::to_string(&example).unwrap(), yaml);
    assert_eq!(example, yaml_serde::from_str(yaml).unwrap());
}

#[test]
fn test_meta() {
    // meta repos don't have rev
    let yaml = indoc! {"
        repos:
        - repo: https://github.com/pre-commit/pre-commit-hooks
          rev: v2.3.0
          hooks:
          - id: check-yaml
          - id: end-of-file-fixer
          - id: trailing-whitespaces
        - repo: https://github.com/psf/black
          rev: 22.10.0
          hooks:
          - id: black
        - repo: local
          hooks:
          - id: some-id
            name: some-name
            entry: some entry
            language: rust
        - repo: meta
          hooks:
          - id: check-useless-excludes
        "};
    let mut example = PreCommitConfig::new();

    let mut pre_commit = Remote::new(
        "https://github.com/pre-commit/pre-commit-hooks".to_string(),
        "v2.3.0".to_string(),
    );
    for hook in ["check-yaml", "end-of-file-fixer", "trailing-whitespaces"] {
        pre_commit.add_hook(ConfigHook::new(hook.to_string()));
    }
    example.add_remote(pre_commit);

    let mut black = Remote::new(
        "https://github.com/psf/black".to_string(),
        "22.10.0".to_string(),
    );
    black.add_hook(ConfigHook::new("black".to_string()));
    example.add_remote(black);

    let mut local = Local::new();
    let hook = DeclareHook::new(
        "some-id".to_string(),
        "some-name".to_string(),
        "some entry".to_string(),
        "rust".to_string(),
    );
    local.add_hook(hook);
    example.add_local(local);

    let mut meta = Meta::new();
    let hook = ConfigHook::new("check-useless-excludes".to_string());
    meta.add_hook(hook);
    example.add_meta(meta);

    assert_eq!(yaml_serde::to_string(&example).unwrap(), yaml);
    assert_eq!(example, yaml_serde::from_str(yaml).unwrap());
}

#[test]
fn test_stages_array_parsing_flow() {
    // This test verifies that the `stages` field parses correctly
    // when using flow sequence (inline array) syntax.
    //
    // See: https://pre-commit.com/#pre-commit-configyaml---hooks
    // The `stages` field should accept an array of stage names.
    let yaml = indoc! {"
        repos:
        - repo: https://github.com/crate-ci/committed
          rev: v1.1.8
          hooks:
          - id: committed
            stages: [commit-msg]
        "};

    let result: Result<PreCommitConfig, _> = yaml_serde::from_str(yaml);
    assert!(
        result.is_ok(),
        "Failed to parse valid pre-commit config with stages array (flow): {:?}",
        result.err()
    );
}

#[test]
fn test_stages_array_parsing_block() {
    // This test verifies that the `stages` field parses correctly
    // when using block sequence (multi-line) syntax.
    // This is the format commonly used in real pre-commit config files.
    let yaml = indoc! {"
        repos:
        - repo: https://github.com/crate-ci/committed
          rev: v1.1.8
          hooks:
          - id: committed
            stages:
            - commit-msg
        "};

    let result: Result<PreCommitConfig, _> = yaml_serde::from_str(yaml);
    assert!(
        result.is_ok(),
        "Failed to parse valid pre-commit config with stages array (block): {:?}",
        result.err()
    );
}

#[test]
fn test_types_array_parsing_flow() {
    // This test demonstrates a bug where the `types` field fails to parse
    // when using flow sequence (inline array) syntax.
    // The `types` field in ConfigHook is typed as Option<String> but should
    // be Option<Vec<String>> to handle array values like `types: [python]`.
    //
    // See: https://pre-commit.com/#pre-commit-configyaml---hooks
    // The `types` field should accept an array of file types.
    let yaml = indoc! {"
        repos:
        - repo: https://github.com/psf/black
          rev: 23.1.0
          hooks:
          - id: black
            types: [python]
        "};

    // This should parse successfully, but currently fails because
    // `types` is typed as Option<String> instead of Option<Vec<String>>
    let result: Result<PreCommitConfig, _> = yaml_serde::from_str(yaml);
    assert!(
        result.is_ok(),
        "Failed to parse valid pre-commit config with types array (flow): {:?}",
        result.err()
    );
}

#[test]
fn test_types_array_parsing_block() {
    // This test uses block sequence (multi-line) syntax for the `types` field.
    // This is the format commonly used in real pre-commit config files.
    let yaml = indoc! {"
        repos:
        - repo: https://github.com/psf/black
          rev: 23.1.0
          hooks:
          - id: black
            types:
            - python
        "};

    let result: Result<PreCommitConfig, _> = yaml_serde::from_str(yaml);
    assert!(
        result.is_ok(),
        "Failed to parse valid pre-commit config with types array (block): {:?}",
        result.err()
    );
}

#[test]
fn test_types_or_array_parsing_flow() {
    // This test demonstrates a bug where the `types_or` field fails to parse
    // when using flow sequence (inline array) syntax.
    // The `types_or` field in ConfigHook is typed as Option<String> but should
    // be Option<Vec<String>> to handle array values like `types_or: [python, pyi]`.
    //
    // See: https://pre-commit.com/#pre-commit-configyaml---hooks
    // The `types_or` field should accept an array of file types (OR logic).
    let yaml = indoc! {"
        repos:
        - repo: https://github.com/psf/black
          rev: 23.1.0
          hooks:
          - id: black
            types_or: [python, pyi]
        "};

    // This should parse successfully, but currently fails because
    // `types_or` is typed as Option<String> instead of Option<Vec<String>>
    let result: Result<PreCommitConfig, _> = yaml_serde::from_str(yaml);
    assert!(
        result.is_ok(),
        "Failed to parse valid pre-commit config with types_or array (flow): {:?}",
        result.err()
    );
}

#[test]
fn test_types_or_array_parsing_block() {
    // This test uses block sequence (multi-line) syntax for the `types_or` field.
    let yaml = indoc! {"
        repos:
        - repo: https://github.com/psf/black
          rev: 23.1.0
          hooks:
          - id: black
            types_or:
            - python
            - pyi
        "};

    let result: Result<PreCommitConfig, _> = yaml_serde::from_str(yaml);
    assert!(
        result.is_ok(),
        "Failed to parse valid pre-commit config with types_or array (block): {:?}",
        result.err()
    );
}

#[test]
fn test_exclude_types_array_parsing_flow() {
    // This test demonstrates a bug where the `exclude_types` field fails to parse
    // when using flow sequence (inline array) syntax.
    // The `exclude_types` field in ConfigHook is typed as Option<String> but should
    // be Option<Vec<String>> to handle array values like `exclude_types: [markdown]`.
    //
    // See: https://pre-commit.com/#pre-commit-configyaml---hooks
    // The `exclude_types` field should accept an array of file types to exclude.
    let yaml = indoc! {"
        repos:
        - repo: https://github.com/pre-commit/pre-commit-hooks
          rev: v4.4.0
          hooks:
          - id: trailing-whitespace
            exclude_types: [markdown, rst]
        "};

    // This should parse successfully, but currently fails because
    // `exclude_types` is typed as Option<String> instead of Option<Vec<String>>
    let result: Result<PreCommitConfig, _> = yaml_serde::from_str(yaml);
    assert!(
        result.is_ok(),
        "Failed to parse valid pre-commit config with exclude_types array (flow): {:?}",
        result.err()
    );
}

#[test]
fn test_exclude_types_array_parsing_block() {
    // This test uses block sequence (multi-line) syntax for the `exclude_types` field.
    let yaml = indoc! {"
        repos:
        - repo: https://github.com/pre-commit/pre-commit-hooks
          rev: v4.4.0
          hooks:
          - id: trailing-whitespace
            exclude_types:
            - markdown
            - rst
        "};

    let result: Result<PreCommitConfig, _> = yaml_serde::from_str(yaml);
    assert!(
        result.is_ok(),
        "Failed to parse valid pre-commit config with exclude_types array (block): {:?}",
        result.err()
    );
}
