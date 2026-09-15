# Run Curvy services for the PIX soak

Image builds and schema migrations belong to the Curvy publishing pipeline.
This directory only configures and runs prebuilt services. HOPR and its soak
executable are built on the Linux dev box.

The launcher starts a fresh chain, PostgreSQL, indexer, relayer, batch prover,
metadata service and gateway, then runs the four-node PIX soak. The Entry shields
directly from its Safe. Nodes submit through the shared relayer; the batch prover
has a separate funded signer. The ten-deposit assertions remain unchanged.

## Linux prerequisites

Use Docker Engine with Compose v2, Bash 4+, curl, jq, GNU coreutils and util-linux
(`setsid`). The dashboard also needs bc and procps. Over SSH, use tmux.

In the updated HOPR checkout, enter `nix develop`, then build on Linux:

```bash
cargo build --locked --release -p hoprd --bin hoprd --features strategy-pix-curvy
cargo test --locked --release -p hoprd-localcluster --test session_pix_soak \
  --no-run --message-format=json > /tmp/pix-soak-build.jsonl
export HOPRD_BIN="$PWD/target/release/hoprd"
export HOPRD_PIX_SOAK_BIN="$(jq -r 'select(.reason == "compiler-artifact" and .target.name == "session_pix_soak" and .executable != null) | .executable' /tmp/pix-soak-build.jsonl | tail -n 1)"
```

## Images and launch

The launcher defaults to `release.json` and pulls every image before starting
containers. It pins the public ECR PostgreSQL, indexer, relayer and batch-prover
images, the HOPR chain fixture, and public Nginx by digest. No AWS login is needed.
The metadata and proving-artifact entries still need their public image digests
from the publisher; until supplied, the launcher reports those missing entries
before doing any work. The current chain fixture requires Linux AMD64.

After those entries are published and pinned, run from the repository root:

```bash
unset HOPRD_CHAIN_URL
PIX_DEMO_RATE=1000 ./localcluster/scripts/curvy-localcluster.sh
```

Use `--release /path/to/release.json` to select another matching release. The
registry flow requires no transferred image archive or local image tags.

For the already prepared offline bundle, transfer and load `images.tar.gz` with
`docker load --input images.tar.gz`, then pass its `release.json` with `--offline`.
The launcher verifies the saved image IDs and starts Compose with `--pull never`
and `--no-build`. Image archives and export tooling are maintained outside this
repository. No Docker HOPR runner image is used.

The PostgreSQL image contains all schemas. Its manifest specifies the actual
`database.pgdata` directory (`/var/lib/postgresql/cluster` for the published
image). With `database.configure_localnet=true`, the launcher configures its own
fresh volume for chain 31337 using the paired chain's contract addresses and the
release's circuit metadata. It checks that configuration before starting workers;
no schema migrations or image builds run at startup.

The artifacts image supplies the pinned HOPR proving keys at
`artifacts_directory`. Pending-note graph and zkey hashes must match the release
and database; the paired verifier uses batch size 5 and tree depth 30.

The launcher refuses to replace an existing `hopr-chain`. Ports 8080 and 3000
must be available on loopback, along with the node ports. Each run owns its
containers and database volume and removes them on exit. Logs remain in the
printed `/tmp/hopr-curvy.XXXXXX` directory, `/tmp/pix-demo/test.log`, and
`/tmp/pix-soak-logs`. Use `--no-dashboard` to stream test output.

## Configuration checks

```bash
bash -n localcluster/scripts/curvy-localcluster.sh localcluster/scripts/pix-demo.sh
sh -n localcluster/curvy/run-soak.sh
```

These checks do not build HOPR or run the soak test.
