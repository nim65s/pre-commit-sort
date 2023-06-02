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
    assert_eq!(serde_yaml::to_string(&example).unwrap(), yaml);
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
    assert_ne!(serde_yaml::to_string(&example).unwrap(), yaml);
    example.sort();
    assert_eq!(serde_yaml::to_string(&example).unwrap(), yaml);
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
    assert_eq!(example, serde_yaml::from_str(yaml).unwrap());
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
    assert_ne!(serde_yaml::to_string(&example).unwrap(), yaml);
    example.sort();
    assert_eq!(serde_yaml::to_string(&example).unwrap(), yaml);
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
    assert_eq!(serde_yaml::to_string(&example).unwrap(), yaml);
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
    assert_ne!(serde_yaml::to_string(&example).unwrap(), yaml);
    example.sort();
    assert_eq!(serde_yaml::to_string(&example).unwrap(), yaml);
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

    assert_eq!(serde_yaml::to_string(&example).unwrap(), yaml);
    assert_eq!(example, serde_yaml::from_str(yaml).unwrap());
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

    assert_eq!(serde_yaml::to_string(&example).unwrap(), yaml);
    assert_eq!(example, serde_yaml::from_str(yaml).unwrap());
}
