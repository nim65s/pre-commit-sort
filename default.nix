{
  lib,
  rustPlatform,
}:

rustPlatform.buildRustPackage {
  pname = "pre-commit-sort";
  version = "0.3.0";

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./Cargo.lock
      ./Cargo.toml
      ./src
      ./tests
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  meta = {
    description = "Sort .pre-commit-config.yaml & .pre-commit-hooks.yaml";
    homepage = "https://github.com/nim65s/pre-commit-sort";
    changelog = "https://github.com/nim65s/pre-commit-sort/blob/main/CHANGELOG.md";
    license = with lib.licenses; [
      asl20
      mit
    ];
    maintainers = with lib.maintainers; [ nim65s ];
    mainProgram = "pre-commit-sort";
  };
}
