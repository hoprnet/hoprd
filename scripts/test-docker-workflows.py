#!/usr/bin/env python3
"""Check Docker publishing contracts with ast-grep.

Run from the repository root: python3 scripts/test-docker-workflows.py
"""

import argparse
import json
import subprocess
import tarfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
WORKFLOWS = ("pr", "merge", "release")
ARCHITECTURES = {"x86_64-linux", "aarch64-linux"}
FLAKE = f"git+file://{ROOT}"


def matches(pattern, source):
    result = subprocess.run(
        [
            "ast-grep",
            "run",
            "--lang",
            "yaml",
            "--pattern",
            pattern,
            "--stdin",
            "--json",
        ],
        input=source,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode not in (0, 1):
        result.check_returncode()
    return json.loads(result.stdout)


def field(source, key):
    found = matches(f"{key}: $VALUE", source)
    if len(found) != 1:
        raise AssertionError(f"Expected one {key}, found {len(found)}")
    return found[0]["metaVariables"]["single"]["VALUE"]["text"]


def docker_jobs(workflow):
    source = (ROOT / ".github/workflows" / f"{workflow}.yaml").read_text()
    return {
        match["metaVariables"]["single"]["ID"]["text"]: match["lines"]
        for match in matches("$ID: $BODY", source)
        if match["range"]["start"]["column"] == 2
        and match["metaVariables"]["single"]["ID"]["text"].startswith("build-docker")
    }


def matrix(job):
    return json.loads(field(job, "build_matrix").removeprefix(">-"))


def run_nix(*args):
    result = subprocess.run(["nix", *args], capture_output=True, text=True, check=False)
    if result.returncode:
        raise AssertionError(result.stderr)
    return result.stdout


class DockerWorkflowUnitTests(unittest.TestCase):
    def test_image_names_select_the_right_pool_on_every_publish_path(self):
        for workflow in WORKFLOWS:
            jobs = docker_jobs(workflow)
            images = [field(job, "docker_image_name") for job in jobs.values()]
            with self.subTest(workflow=workflow):
                self.assertEqual(
                    len(images), len(set(images)), "Two jobs publish the same image"
                )
                self.assertIn("hoprd", images)

                self.assertNotIn("hoprd-pix-curvy", images)
            for job in jobs.values():
                image = field(job, "docker_image_name")
                if image not in ("hoprd", "hoprd-pix-test"):
                    continue
                for entry in matrix(job):
                    with self.subTest(workflow=workflow, image=image, entry=entry):
                        target = f"docker-{image}-{entry['architecture']}"
                        self.assertEqual(
                            entry["build_command"], f"nix build -L .#{target}"
                        )

    def test_images_keep_linux_architectures_and_distinct_concurrency(self):
        for workflow in WORKFLOWS:
            jobs = docker_jobs(workflow)
            suffixes = [
                field(job, "concurrency_group_suffix")
                if matches("concurrency_group_suffix: $VALUE", job)
                else ""
                for job in jobs.values()
            ]
            with self.subTest(workflow=workflow):
                self.assertEqual(len(suffixes), len(set(suffixes)))
                self.assertEqual(field(jobs["build-docker"], "name"), "Docker")
                self.assertEqual(
                    field(jobs["build-docker"], "docker_image_name"), "hoprd"
                )
            for job in jobs.values():
                if field(job, "docker_image_name") in ("hoprd", "hoprd-pix-test"):
                    with self.subTest(workflow=workflow, job=job):
                        self.assertEqual(
                            {entry["architecture"] for entry in matrix(job)},
                            ARCHITECTURES,
                        )

    def test_release_waits_for_curvy_and_publishes_to_docker_hub(self):
        source = (ROOT / ".github/workflows/release.yaml").read_text()
        release = matches("release: $BODY", source)
        release = next(
            match["lines"]
            for match in release
            if match["range"]["start"]["column"] == 2
        )
        needs = field(release, "needs")
        self.assertIn("build-docker\n", needs + "\n")

        for job in docker_jobs("release").values():
            self.assertTrue(matches("docker_hub_username: $VALUE", job))
            self.assertTrue(matches("docker_hub_token: $VALUE", job))


class DockerWorkflowIntegrationTests(unittest.TestCase):
    def test_curvy_builds_embed_proving_artifacts_in_both_build_stages(self):
        expression = f'''
          let
            f = builtins.getFlake "{FLAKE}";
          in builtins.listToAttrs (map (arch:
            let
              p = f.packages.${{arch}};
              binary = p."binary-hoprd-pix-curvy-${{arch}}";
            in {{
              name = arch;
              value = {{
                command = binary.drvAttrs.buildPhase;
                dependencyCommand = binary.cargoArtifacts.drvAttrs.buildPhase;
                keys = binary.CURVY_ZK_KEYS_DIR_DEFAULT;
                dependencyKeys = binary.cargoArtifacts.CURVY_ZK_KEYS_DIR_DEFAULT;
                target = binary.CARGO_BUILD_TARGET;
                dockerPackages = builtins.filter
                  (name: f.inputs.nixpkgs.lib.hasPrefix "docker-" name)
                  (builtins.attrNames p);
              }};
            }}) [ "x86_64-linux" "aarch64-linux" ])
        '''
        result = run_nix("eval", "--json", "--impure", "--expr", expression)
        for arch, build in json.loads(result).items():
            with self.subTest(architecture=arch):
                self.assertIn("-F strategy-pix-curvy", build["command"])
                self.assertIn("-F strategy-pix-curvy", build["dependencyCommand"])
                self.assertEqual(build["keys"], build["dependencyKeys"])
                self.assertIn("curvy-zk-artifacts", build["keys"])

                self.assertEqual(
                    build["target"], arch.removesuffix("-linux") + "-unknown-linux-musl"
                )
                self.assertEqual(
                    set(build["dockerPackages"]),
                    {
                        f"docker-{image}-{architecture}"
                        for image in ("hoprd", "hoprd-pix-test", "hoprd-profile")
                        for architecture in ARCHITECTURES
                    }
                    | {"docker-hoprd-localcluster-x86_64-linux"},
                )


class DockerWorkflowEndToEndTests(unittest.TestCase):
    def test_workflow_targets_produce_the_expected_image_contents(self):
        for job in docker_jobs("pr").values():
            image = field(job, "docker_image_name")
            if image != "hoprd":
                continue
            entry = next(
                entry
                for entry in matrix(job)
                if entry["architecture"] == "x86_64-linux"
            )
            target = entry["build_command"].split(".#", 1)[1]
            result = run_nix(
                "build", "--no-link", "--print-out-paths", f"{FLAKE}#{target}"
            )
            with self.subTest(image=image), tarfile.open(result.strip()) as archive:
                manifest = json.load(archive.extractfile("manifest.json"))[0]
                self.assertEqual(manifest["RepoTags"], [f"{image}:latest"])
                config = json.load(archive.extractfile(manifest["Config"]))
                self.assertEqual(config["architecture"], "amd64")
                self.assertEqual(config["config"]["Cmd"], ["hoprd"])
                history = "\n".join(row.get("comment", "") for row in config["history"])
                self.assertIn("curvy-zk-artifacts", history)
                self.assertIn("hoprd-x86_64-unknown-linux-musl", history)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "suite", choices=("unit", "integration", "e2e"), default="unit", nargs="?"
    )
    args = parser.parse_args()
    test_class = {
        "unit": DockerWorkflowUnitTests,
        "integration": DockerWorkflowIntegrationTests,
        "e2e": DockerWorkflowEndToEndTests,
    }[args.suite]
    result = unittest.TextTestRunner().run(
        unittest.defaultTestLoader.loadTestsFromTestCase(test_class)
    )
    raise SystemExit(not result.wasSuccessful())
