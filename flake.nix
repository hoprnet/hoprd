{
  description = "hoprd — HOPR node daemon and REST API";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/release-25.11";
    nixpkgs-unstable.url = "github:NixOS/nixpkgs/master";
    rust-overlay.url = "github:oxalica/rust-overlay/master";
    crane.url = "github:ipetkov/crane/v0.23.0";
    nix-lib.url = "github:hoprnet/nix-lib/v1.1.0";
    foundry.url = "github:hoprnet/foundry.nix/tb/202505-add-xz";
    pre-commit.url = "github:cachix/git-hooks.nix";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    flake-root.url = "github:srid/flake-root";

    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";
    foundry.inputs.flake-utils.follows = "flake-utils";
    foundry.inputs.nixpkgs.follows = "nixpkgs";
    nix-lib.inputs.nixpkgs.follows = "nixpkgs";
    nix-lib.inputs.flake-utils.follows = "flake-utils";
    nix-lib.inputs.crane.follows = "crane";
    nix-lib.inputs.flake-parts.follows = "flake-parts";
    nix-lib.inputs.rust-overlay.follows = "rust-overlay";
    nix-lib.inputs.treefmt-nix.follows = "treefmt-nix";
    nix-lib.inputs.nixpkgs-unstable.follows = "nixpkgs-unstable";
    pre-commit.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    {
      self,
      nixpkgs,
      nixpkgs-unstable,
      flake-utils,
      flake-parts,
      rust-overlay,
      crane,
      nix-lib,
      foundry,
      pre-commit,
      ...
    }@inputs:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.treefmt-nix.flakeModule
        inputs.flake-root.flakeModule
      ];
      perSystem =
        {
          config,
          lib,
          system,
          ...
        }:
        let
          rev = toString (self.shortRev or self.dirtyShortRev);
          fs = lib.fileset;
          localSystem = system;
          overlays = [
            (import rust-overlay)
            foundry.overlay
          ];
          pkgs = import nixpkgs { inherit localSystem overlays; };
          pkgs-unstable = import nixpkgs-unstable { inherit localSystem overlays; };
          buildPlatform = pkgs.stdenv.buildPlatform;

          nixLib = nix-lib.lib.${system};

          nightlyToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain-nightly.toml;

          rustfmtWrapper = pkgs.writeShellScriptBin "rustfmt" ''
            export DYLD_LIBRARY_PATH="${nightlyToolchain}/lib:$DYLD_LIBRARY_PATH"
            exec "${nightlyToolchain}/bin/rustfmt" "$@"
          '';

          craneLib = (crane.mkLib pkgs).overrideToolchain (p: p.rust-bin.stable.latest.default);
          hoprdCrateInfoOriginal = craneLib.crateNameFromCargoToml {
            cargoToml = ./hoprd/Cargo.toml;
          };
          hoprdCrateInfo = {
            pname = "hoprd";
            version = pkgs.lib.strings.concatStringsSep "." (
              pkgs.lib.lists.take 3 (builtins.splitVersion hoprdCrateInfoOriginal.version)
            );
          };

          depsSrc = nixLib.mkDepsSrc {
            root = ./.;
            inherit fs;
          };
          src = nixLib.mkSrc {
            root = ./.;
            inherit fs;
            extraFiles = [
              ./deploy/compose/hoprd/conf/hoprd.cfg.yaml
            ];
          };
          testSrc = nixLib.mkTestSrc {
            root = ./.;
            inherit fs;
            extraFiles = [
              ./deploy/compose/hoprd/conf/hoprd.cfg.yaml
              (fs.fileFilter (file: file.hasExt "snap") ./.)
            ];
          };

          builders = nixLib.mkRustBuilders {
            inherit localSystem;
            rustToolchainFile = ./rust-toolchain.toml;
          };

          rust-builder-local = builders.local;
          rust-builder-x86_64-linux = builders.x86_64-linux;
          rust-builder-x86_64-darwin = builders.x86_64-darwin;
          rust-builder-aarch64-linux = builders.aarch64-linux;
          rust-builder-aarch64-darwin = builders.aarch64-darwin;

          rust-builder-local-nightly = nixLib.mkRustBuilder {
            inherit localSystem;
            rustToolchainFile = ./rust-toolchain-nightly.toml;
          };
          rust-builder-local-coverage = builders.localCoverage;

          projectBuildArgs = {
            inherit src depsSrc rev;
            cargoExtraArgs = "-p hoprd -p hoprd-api";
            cargoToml = ./hoprd/Cargo.toml;
          };
          localclusterBuildArgs = {
            inherit src depsSrc rev;
            cargoExtraArgs = "-p hoprd-localcluster";
            cargoToml = ./localcluster/Cargo.toml;
          };

          fixUtoipaEmbedPaths =
            drv:
            drv.overrideAttrs (old: {
              preBuild = ''
                find target -name 'embed.rs' -path '*/utoipa-swagger-ui*/out/*' \
                  -exec sed -i "s|/nix/var/nix/builds/[^/]*/source|$(pwd)|g" {} \;
                if find target -name 'embed.rs' -path '*/utoipa-swagger-ui*/out/*' \
                     -exec grep -l '/nix/var/nix/builds/' {} \; | grep -q .; then
                  echo "error: stale /nix/var/nix/builds/ paths remain in utoipa-swagger-ui embed.rs after substitution" >&2
                  exit 1
                fi
              ''
              + (old.preBuild or "");
            });

          hoprdPackages = {
            binary-hoprd = rust-builder-local.callPackage nixLib.mkRustPackage projectBuildArgs;
            binary-hoprd-localcluster = rust-builder-local.callPackage nixLib.mkRustPackage localclusterBuildArgs;
            binary-hoprd-x86_64-linux = rust-builder-x86_64-linux.callPackage nixLib.mkRustPackage projectBuildArgs;
            binary-hoprd-localcluster-x86_64-linux = rust-builder-x86_64-linux.callPackage nixLib.mkRustPackage localclusterBuildArgs;
            binary-hoprd-dev-x86_64-linux = rust-builder-x86_64-linux.callPackage nixLib.mkRustPackage (
              projectBuildArgs
              // {
                CARGO_PROFILE = "dev";
                cargoExtraArgs = "-p hoprd -p hoprd-api -F capture";
              }
            );
            binary-hoprd-aarch64-linux = rust-builder-aarch64-linux.callPackage nixLib.mkRustPackage projectBuildArgs;
            binary-hoprd-x86_64-darwin = rust-builder-x86_64-darwin.callPackage nixLib.mkRustPackage projectBuildArgs;
            binary-hoprd-aarch64-darwin = rust-builder-aarch64-darwin.callPackage nixLib.mkRustPackage projectBuildArgs;

            binary-hoprd-api-schema-x86_64-linux = rust-builder-x86_64-linux.callPackage nixLib.mkRustPackage {
              inherit src depsSrc rev;
              cargoExtraArgs = "-p hoprd-api --bin hoprd-api-schema";
              cargoToml = ./rest-api/Cargo.toml;
            };
            binary-hoprd-api-schema-aarch64-linux =
              rust-builder-aarch64-linux.callPackage nixLib.mkRustPackage
                {
                  inherit src depsSrc rev;
                  cargoExtraArgs = "-p hoprd-api --bin hoprd-api-schema";
                  cargoToml = ./rest-api/Cargo.toml;
                };

            binary-hoprd-cfg-x86_64-linux = rust-builder-x86_64-linux.callPackage nixLib.mkRustPackage {
              inherit src depsSrc rev;
              cargoExtraArgs = "-p hoprd --bin hoprd-cfg";
              cargoToml = ./hoprd/Cargo.toml;
            };
            binary-hoprd-cfg-aarch64-linux = rust-builder-aarch64-linux.callPackage nixLib.mkRustPackage {
              inherit src depsSrc rev;
              cargoExtraArgs = "-p hoprd --bin hoprd-cfg";
              cargoToml = ./hoprd/Cargo.toml;
            };

            test-unit =
              (fixUtoipaEmbedPaths (
                rust-builder-local.callPackage nixLib.mkRustPackage (
                  projectBuildArgs
                  // {
                    src = testSrc;
                    cargoExtraArgs = "-p hoprd -p hoprd-api";
                    runTests = true;
                    prependPackageName = false;
                    cargoTestExtraArgs = "--lib";
                    extraNativeBuildInputs = [ pkgs.cargo-nextest ];
                  }
                )
              )).overrideAttrs
                (_: {
                  checkPhase = ''
                    runHook preCheck
                    cargo nextest run ''${CARGO_PROFILE:+--cargo-profile $CARGO_PROFILE} --lib
                    runHook postCheck
                  '';
                });

            test-nightly =
              (fixUtoipaEmbedPaths (
                rust-builder-local-nightly.callPackage nixLib.mkRustPackage (
                  projectBuildArgs
                  // {
                    src = testSrc;
                    cargoExtraArgs = "-p hoprd -p hoprd-api -Z panic-abort-tests";
                    runTests = true;
                    prependPackageName = false;
                    cargoTestExtraArgs = "--lib";
                    extraNativeBuildInputs = [ pkgs.cargo-nextest ];
                  }
                )
              )).overrideAttrs
                (_: {
                  checkPhase = ''
                    runHook preCheck
                    cargo nextest run ''${CARGO_PROFILE:+--cargo-profile $CARGO_PROFILE} -Z panic-abort-tests --lib
                    runHook postCheck
                  '';
                });

            coverage-unit =
              (fixUtoipaEmbedPaths (
                rust-builder-local-coverage.callPackage nixLib.mkRustPackage (
                  projectBuildArgs
                  // {
                    src = testSrc;
                    cargoExtraArgs = "-p hoprd -p hoprd-api";
                    runCoverage = true;
                    prependPackageName = false;
                    cargoLlvmCovExtraArgs = "--lcov --output-path $out --lib";
                    extraNativeBuildInputs = [ pkgs.cargo-nextest ];
                  }
                )
              )).overrideAttrs
                (_: {
                  buildPhase = ''
                    runHook preBuild
                    cargo llvm-cov nextest --lcov --output-path $out --lib \
                      ''${CARGO_PROFILE:+--cargo-profile $CARGO_PROFILE} \
                      -p hoprd -p hoprd-api
                    runHook postBuild
                  '';
                });

            hoprd-clippy = rust-builder-local.callPackage nixLib.mkRustPackage (
              projectBuildArgs
              // {
                runClippy = true;
                cargoExtraArgs = "-p hoprd -p hoprd-api --no-default-features -F runtime-tokio,telemetry,transport-quic";
              }
            );
            binary-hoprd-dev = rust-builder-local.callPackage nixLib.mkRustPackage (
              projectBuildArgs
              // {
                CARGO_PROFILE = "dev";
                cargoExtraArgs = "-p hoprd -p hoprd-api -F capture";
              }
            );
          };

          mkHoprdCandidate =
            cargoExtraArgs:
            if buildPlatform.isLinux && buildPlatform.isx86_64 then
              rust-builder-x86_64-linux.callPackage nixLib.mkRustPackage (
                projectBuildArgs
                // {
                  inherit cargoExtraArgs;
                  CARGO_PROFILE = "candidate";
                }
              )
            else
              rust-builder-local.callPackage nixLib.mkRustPackage (
                projectBuildArgs
                // {
                  inherit cargoExtraArgs;
                  CARGO_PROFILE = "candidate";
                }
              );

          dockerHoprdEntrypoint = pkgs.writeShellScriptBin "docker-entrypoint.sh" ''
            set -euo pipefail

            ssl_cert_file="${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
            if [ -f "$ssl_cert_file" ]; then
              export SSL_CERT_FILE="$ssl_cert_file"
              export NIX_SSL_CERT_FILE="$ssl_cert_file"
            fi

            mkdir -p "$TMPDIR"

            listen_host="''${HOPRD_DEFAULT_SESSION_LISTEN_HOST:-}"
            case "''${listen_host}" in
              *:*)
                listen_host_preset_ip="''${listen_host%%:*}"
                listen_host_preset_port="''${listen_host#*:}"
                ;;
              *)
                listen_host_preset_ip="''${listen_host}"
                listen_host_preset_port=""
                ;;
            esac

            if [ -z "''${listen_host_preset_ip:-}" ]; then
              listen_host_ip="$(hostname -i | awk '{print $1}')"

              if [ -z "''${listen_host_preset_port:-}" ]; then
                listen_host="''${listen_host_ip}:0"
              else
                listen_host="''${listen_host_ip}:''${listen_host_preset_port}"
              fi
            fi

            export HOPRD_DEFAULT_SESSION_LISTEN_HOST="''${listen_host}"

            if [ -n "''${1:-}" ] && [ -f "/bin/''${1:-}" ] && [ -x "/bin/''${1:-}" ]; then
              exec "$@"
            else
              exec /bin/hoprd "$@"
            fi
          '';

          hoprd-man = nixLib.mkManPage {
            pname = "hoprd";
            binary = hoprdPackages.binary-hoprd-dev;
            description = "HOPR node executable";
          };

          hoprdDocker = {
            docker-hoprd-x86_64-linux = nixLib.mkDockerImage {
              name = "hoprd";
              extraContents = [
                dockerHoprdEntrypoint
                hoprdPackages.binary-hoprd-x86_64-linux
                pkgs.cacert
                pkgs.curl
              ];
              Entrypoint = [ "/bin/docker-entrypoint.sh" ];
              Cmd = [ "hoprd" ];
              env = [ "TMPDIR=/app/.tmp" ];
            };
            docker-hoprd-dev-x86_64-linux = nixLib.mkDockerImage {
              name = "hoprd";
              extraContents = [
                dockerHoprdEntrypoint
                hoprdPackages.binary-hoprd-dev-x86_64-linux
                pkgs.cacert
                pkgs.curl
              ];
              Entrypoint = [ "/bin/docker-entrypoint.sh" ];
              Cmd = [ "hoprd" ];
              env = [ "TMPDIR=/app/.tmp" ];
            };
            docker-hoprd-aarch64-linux = nixLib.mkDockerImage {
              name = "hoprd";
              extraContents = [
                dockerHoprdEntrypoint
                hoprdPackages.binary-hoprd-aarch64-linux
                pkgs.cacert
                pkgs.curl
              ];
              Entrypoint = [ "/bin/docker-entrypoint.sh" ];
              Cmd = [ "hoprd" ];
              env = [ "TMPDIR=/app/.tmp" ];
            };
          };

          docs =
            (rust-builder-local-nightly.callPackage nixLib.mkRustPackage (
              projectBuildArgs
              // {
                buildDocs = true;
                # Drop jemalloc default feature for docs: native lib fails to link in the docs sandbox.
                # Must be applied here (not just in buildPhase) so cargoArtifacts/deps step also skips it.
                cargoExtraArgs = "-p hoprd -p hoprd-api --no-default-features -F runtime-tokio,telemetry,transport-quic";
              }
            )).overrideAttrs
              (_: {
                buildPhase = ''
                  runHook preBuild
                  cargo doc -p hoprd -p hoprd-api --no-default-features \
                    -F runtime-tokio,telemetry,transport-quic \
                    --no-deps --document-private-items
                  runHook postBuild
                '';
              });

          pre-commit-lightweight = pkgs.pre-commit.overridePythonAttrs {
            nativeCheckInputs = [ ];
            doCheck = false;
            doInstallCheck = false;
            dontUsePytestCheck = true;
            preCheck = "";
            postCheck = "";
          };

          pre-commit-check = pre-commit.lib.${system}.run {
            src = ./.;
            package = pre-commit-lightweight;
            hooks = {
              check-executables-have-shebangs.enable = true;
              check-shebang-scripts-are-executable.enable = true;
              check-case-conflicts.enable = true;
              check-symlinks.enable = true;
              check-merge-conflicts.enable = true;
              check-added-large-files.enable = true;
              commitizen.enable = true;
            };
          };

          devShell = nixLib.mkDevShell {
            rustToolchainFile = ./rust-toolchain.toml;
            shellName = "hoprd Development";
            treefmtWrapper = config.treefmt.build.wrapper;
            treefmtPrograms = pkgs.lib.attrValues config.treefmt.build.programs;
            extraPackages = with pkgs; [
              pkgs-unstable.cargo-audit
              cargo-machete
              cargo-shear
              cargo-insta
              cargo-nextest
              foundry-bin
              nfpm
              envsubst
            ];
            shellHook = ''
              ${pre-commit-check.shellHook}
            '';
          };

          ciShell = nixLib.mkDevShell {
            rustToolchainFile = ./rust-toolchain.toml;
            shellName = "hoprd CI";
            treefmtWrapper = config.treefmt.build.wrapper;
            treefmtPrograms = pkgs.lib.attrValues config.treefmt.build.programs;
            extraPackages = with pkgs; [
              act
              gh
              google-cloud-sdk
              pkgs-unstable.cargo-audit
              cargo-machete
              cargo-shear
              swagger-codegen3
              vacuum-go
              zizmor
              gnupg
              perl
            ];
          };

          testShell = nixLib.mkDevShell {
            rustToolchainFile = ./rust-toolchain.toml;
            shellName = "hoprd Testing";
            treefmtWrapper = config.treefmt.build.wrapper;
            treefmtPrograms = pkgs.lib.attrValues config.treefmt.build.programs;
            extraPackages = with pkgs; [
              foundry-bin
              cargo-nextest
            ];
          };

          run-check = nixLib.mkCheckApp { inherit system; };
          run-audit = nixLib.mkAuditApp {
            rustToolchainFile = ./rust-toolchain.toml;
          };
        in
        {
          treefmt = {
            inherit (config.flake-root) projectRootFile;

            settings.global.excludes = [
              "**/*.id"
              "**/.cargo-ok"
              "**/.gitignore"
              ".actrc"
              ".dockerignore"
              ".editorconfig"
              ".gcloudignore"
              ".gitattributes"
              ".yamlfmt"
              "LICENSE"
              "Makefile"
              "rest-api-client/src/codegen/*"
              "deploy/compose/grafana/config.monitoring"
              "deploy/nfpm/nfpm.yaml"
              "target/*"
            ];

            programs.shfmt.enable = true;
            settings.formatter.shfmt.includes = [
              "*.sh"
              "deploy/compose/.env.sample"
              "deploy/compose/.env-secrets.sample"
            ];

            programs.yamlfmt.enable = true;
            settings.formatter.yamlfmt.includes = [
              ".github/labeler.yml"
              ".github/workflows/*.yaml"
            ];
            settings.formatter.yamlfmt.settings = {
              formatter.type = "basic";
              formatter.max_line_length = 120;
              formatter.trim_trailing_whitespace = true;
              formatter.scan_folded_as_literal = true;
              formatter.include_document_start = true;
            };

            programs.prettier.enable = true;
            settings.formatter.prettier.includes = [
              "*.md"
              "*.json"
            ];
            settings.formatter.prettier.excludes = [
              "*.yml"
              "*.yaml"
            ];
            programs.rustfmt.enable = true;
            programs.nixfmt.enable = true;
            programs.taplo.enable = true;

            settings.formatter.rustfmt = {
              command = "${rustfmtWrapper}/bin/rustfmt";
            };
          };

          checks = { inherit (hoprdPackages) hoprd-clippy; };

          apps = {
            check = run-check;
            audit = run-audit;
          };

          packages =
            hoprdPackages
            // hoprdDocker
            // {
              inherit docs;
              inherit pre-commit-check;
              inherit hoprd-man;
              default = hoprdPackages.binary-hoprd;
              hoprd-candidate = (mkHoprdCandidate "-p hoprd -p hoprd-api");
            };

          devShells.default = devShell;
          devShells.ci = ciShell;
          devShells.test = testShell;

          formatter = config.treefmt.build.wrapper;
        };
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
        "x86_64-darwin"
      ];
    };
}
