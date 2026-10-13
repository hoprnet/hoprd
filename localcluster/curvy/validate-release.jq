# Registry runs use public images and verified proving files; archives carry both.
def digest: type == "string" and test("^[a-zA-Z0-9][a-zA-Z0-9._:/-]*@sha256:[a-f0-9]{64}$");
def sha256: type == "string" and test("^[a-f0-9]{64}$");
def absolute_path: type == "string" and test("^/[a-zA-Z0-9_./-]+$") and (contains("..") | not);
def relative_path: type == "string" and test("^[a-zA-Z0-9_][a-zA-Z0-9_./-]*$") and (contains("..") | not);
def image_id: type == "string" and test("^sha256:[a-f0-9]{64}$");
def local_tag: type == "string" and test("^hopr-localcluster/[a-z0-9-]+:[a-zA-Z0-9_.-]+$");
. as $release |
.version == 1 and
((.source // "registry") == "registry" or .source == "archive") and
(.platform == "linux/amd64" or .platform == "linux/arm64") and
([.images.chain, .images.localdb, .images.indexer, .images.relayer,
  .images.batch_prover, .images.gateway] |
  if $release.source == "archive" then all(local_tag) else all(digest) end) and
(.images.runner == null) and
(if .source == "archive" then
  (.images | to_entries | map(select(.value != null)) | all(. as $entry | $release.image_ids[$entry.key] |
    type == "array" and length > 0 and all(image_id)))
 else true end) and
((.database.pgdata // "/var/lib/postgresql/data") |
  . == "/var/lib/postgresql/data" or . == "/var/lib/postgresql/cluster") and
((.database.configure_localnet // false) | type == "boolean") and
(if .images.artifacts != null then
  (.images.artifacts | if $release.source == "archive" then local_tag else digest end) and
  (.artifacts_directory | absolute_path)
 else
  .source != "archive" and
  (.artifacts.base_url | type == "string" and test("^https://github.com/0xCurvy/rs-sdk/releases/download/[a-zA-Z0-9._-]+$")) and
  (.artifacts.files | type == "object" and length > 0 and
    (to_entries | all((.key | test("^[a-zA-Z0-9_][a-zA-Z0-9_.-]*$")) and (.value | sha256)))) and
  .artifacts.files[.pending.graph] == .pending.graph_sha256 and
  .artifacts.files[.pending.zkey] == .pending.zkey_sha256
 end) and
(.pending.graph | relative_path) and (.pending.zkey | relative_path) and
(.pending.graph_sha256 | sha256) and (.pending.zkey_sha256 | sha256)
