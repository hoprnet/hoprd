# Run Curvy services for the PIX soak

Image builds and schema migrations belong to the Curvy publishing pipeline.
This directory only configures and runs prebuilt services. HOPR and its soak
executable are built on the Linux dev box.

The launcher starts a fresh chain, PostgreSQL, indexer, relayer, batch prover,
and gateway, then runs the four-node PIX soak. The Entry shields
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
Proving files are downloaded from the pinned public rs-sdk GitHub release and
verified by SHA-256. The current chain fixture requires Linux AMD64.

Run from the repository root:

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

The registry manifest pins the same proving files as `curvyZkArtifacts` in
`flake.nix`. The launcher caches them in `${XDG_CACHE_HOME:-$HOME/.cache}/hopr/curvy-zk`,
verifies them before reuse, and supplies them to both the Linux node and batch
prover. Existing offline bundles can still supply these files from their artifact
image. Pending-note graph and zkey hashes must match the release and database;
the paired verifier uses batch size 5 and tree depth 30.

No metadata container is needed. The Rust relayer adapter calls `/protocol` for
the fee collector's public spend, view and BabyJubjub keys when constructing an
aggregation proof. The gateway serves `protocol.json`, containing Curvy's fixed
localnet public identity from `packages/services/common/src/fee-collector.ts`.
Startup checks the BabyJubjub key against the aggregator's `feeNotePublicKey`.
The spend/view keys are not stored on-chain. This fixture preserves protocol
fees and direct shielding; it is only for the paired local deployment. The local
relayer has no paymaster, so the adapter does not need metadata's `/networks`
endpoint for gas-fee pricing.

The launcher refuses to replace an existing `hopr-chain`. Ports 8080 and 3000
must be available on loopback, along with the node ports. Each run owns its
containers and database volume and removes them on exit. Logs remain in the
printed `/tmp/hopr-curvy.XXXXXX` directory, `/tmp/pix-demo/test.log`, and
`/tmp/pix-soak-logs`. Use `--no-dashboard` to stream test output.

The soak harness watches the current node logs for session closure, including SSA
handshake failures that never start deposit tracking. An early closure fails with
the deposit count and recent handshake events. Success requires all funded
deposits, a budget refusal, and the Exit's deposit-timeout closure; the existing
payout and traffic checks still apply.

## Companion dashboard

In another terminal on the same Linux box, run:

```bash
./localcluster/scripts/curvy-demo.sh
```

The companion reads successful aggregation, commitment and withdrawal transactions
from RPC receipts, including relayer and batch-prover submissions. Direct shielding
is identified by the Entry Safe's token transfer into the vault. It shows scan
coverage and whether readings are live, cached or unavailable. Transaction counts
can differ from deposit counts because transactions may contain batches.
The launcher resets the run timestamp before pulling images, so the companion
does not combine a new chain with the previous run's cached node observations.

The session pane's “Exit observed” means the deposit was detected by the Exit;
recovering the SSA key still requires return-traffic shares. The companion's note
correlation count does not mean that key recovery has happened.
The SSA quota includes the packet-payload byte factor. Funded quota, node-wide
packet totals and verified application echo volume are shown separately; packet
counts cannot be converted directly into delivered application bytes. Application
volume uses the latest test report, rounded down to whole decimal MB. View tag 0
is valid (including for padding); it is not evidence of ownership or a failed note.

If the companion was opened during the run, the launcher takes a final snapshot
before removing the chain. Cached readings remain available afterward. Use
`curvy-demo.sh --ledger` for the transfer and gas report, or `--rpc` for transaction
details. The note list displays at most 1,000 records per category; RPC transaction
counts are independent of that limit. Transfer linkability analysis is confined
to `--ledger`, keeping the live pane focused on the run's operational readings.
That analysis describes visible paths, not a proof of anonymity on this small
local chain.

## Configuration checks

```bash
bash -n localcluster/scripts/curvy-localcluster.sh localcluster/scripts/pix-demo.sh
sh -n localcluster/curvy/run-soak.sh
bash localcluster/scripts/tests/curvy-demo.sh
```

These checks do not build HOPR or run the soak test.
