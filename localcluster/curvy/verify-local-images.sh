#!/usr/bin/env bash
# Read-only preflight for docker-save bundles. Tags survive save/load; registry
# RepoDigests need not. Accept the saved manifest or config digest so that
# bundles can move between Docker's containerd and classic image stores.
set -euo pipefail
release=${1:?release manifest required}
platform=$(jq -er .platform "$release")
while IFS=$'\t' read -r name tag expected; do
  actual=$(docker image inspect "$tag" --format '{{json .}}') || {
    echo "missing bundled image $tag; load images.tar.gz first" >&2
    exit 1
  }
  if ! jq -e --arg platform "$platform" --argjson ids "$expected" '
    (.Os + "/" + .Architecture) == $platform and
    (. as $image | any($ids[];
      . == $image.Id or . == $image.Descriptor.annotations["config.digest"]))
  ' <<<"$actual" >/dev/null; then
    echo "bundled image $name does not match the saved ID/platform" >&2
    exit 1
  fi
done < <(jq -r '. as $r | .images | to_entries[] | select(.value != null) | [.key, .value, ($r.image_ids[.key] | tojson)] | @tsv' "$release")
