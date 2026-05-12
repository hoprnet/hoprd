#!/usr/bin/env bash
# Build a self-contained PDF from FINDINGS.md with images inlined.
# Runs entirely on the OrbStack NixOS VM.
#
# Usage:  scripts/findings_to_pdf.sh
# Out:    /tmp/findings-out/FINDINGS.pdf  (then scp it back)

set -o errexit -o nounset -o pipefail

VM_HOST="${VM_HOST:-nixos-test@orb}"
SRC_DIR="${SRC_DIR:-jeprof-pull-20260507-152514}"
REMOTE_DIR="${REMOTE_DIR:-findings}"
OUT_DIR="${OUT_DIR:-/tmp/findings-out}"

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOCAL_SRC="$REPO_ROOT/$SRC_DIR"

[ -d "$LOCAL_SRC" ] || {
  echo "missing $LOCAL_SRC" >&2
  exit 1
}

echo "==> Pushing artifacts to ${VM_HOST}:~/${REMOTE_DIR}/..."
ssh "$VM_HOST" "rm -rf ~/${REMOTE_DIR} && mkdir -p ~/${REMOTE_DIR}"
scp "$LOCAL_SRC/FINDINGS.md" \
  "$LOCAL_SRC"/plots/total_inuse.png \
  "$LOCAL_SRC"/plots/top_10_pid_*.png \
  "$LOCAL_SRC"/leak/leak_*.pdf \
  "$LOCAL_SRC"/leak/leak_*.txt \
  "$VM_HOST:${REMOTE_DIR}/"

echo "==> Converting leak PDFs to PNGs on VM..."
ssh "$VM_HOST" "cd ${REMOTE_DIR} && for f in leak_*.pdf; do
  pdftoppm -r 144 -png \"\$f\" \"\${f%.pdf}\"
done"

echo "==> Building inlined markdown variant..."
ssh "$VM_HOST" "cd ${REMOTE_DIR} && python3 <<'PY'
import re, pathlib
md = pathlib.Path('FINDINGS.md').read_text()

# Inline images: replace plot/*.png references with bare PNG filenames
md = re.sub(r'\[(\`?[^]]+?\`?)\]\(plots/([^)]+\.png)\)',
            r'![\1](\2)', md)

# Inline leak PDFs as the rendered PNG (page 1) plus a link to the PDF
def repl_leak(m):
    label, pdf = m.group(1), m.group(2)
    png = pdf.replace('.pdf', '-1.png')
    return f'![{label}]({png})\n\n*PDF source: {pdf}*'
md = re.sub(r'\[(leak/leak_\d+\.pdf)\]\(leak/(leak_\d+\.pdf)\)', repl_leak, md)
md = re.sub(r'\[leak/leak_(\d+)\.pdf\]\(leak/(leak_\d+\.pdf)\)', repl_leak, md)

# Strip the table that links to leak files; replace with sectioned per-PID inlining.
# We do this by appending image blocks just before '### Raw time-series CSV' anchor.
inject = '''

### Inlined leak diff renderings

#### PID 284977

![leak diff PID 284977](leak_284977-1.png)

#### PID 284978

![leak diff PID 284978](leak_284978-1.png)

#### PID 284979

![leak diff PID 284979](leak_284979-1.png)

'''
md = md.replace('### Raw time-series CSV', inject + '### Raw time-series CSV')

# Remove links that point inside leak/ since we inlined renderings.
md = re.sub(r'\[(leak/[^)]+)\]\(leak/[^)]+\)', r'\`\1\`', md)
md = re.sub(r'\[plots/([^)]+)\]\(plots/[^)]+\)', r'\`plots/\1\`', md)

pathlib.Path('FINDINGS_inline.md').write_text(md)
print('wrote FINDINGS_inline.md (', len(md), 'bytes)')
PY"

echo "==> Rendering PDF via pandoc + weasyprint..."
ssh "$VM_HOST" "cd ${REMOTE_DIR} && \
  nix-shell -p 'python313.withPackages(ps: [ps.weasyprint])' --run \
  'pandoc FINDINGS_inline.md -o FINDINGS.pdf --pdf-engine=weasyprint --resource-path=. --toc --toc-depth=2 --metadata title=\"hoprd jemalloc leak findings\" -V geometry:margin=2cm' 2>&1 | tail -10"

mkdir -p "$LOCAL_SRC/pdf"
echo "==> Pulling FINDINGS.pdf to $LOCAL_SRC/pdf/..."
scp "$VM_HOST:${REMOTE_DIR}/FINDINGS.pdf" "$LOCAL_SRC/pdf/FINDINGS.pdf"
ls -lh "$LOCAL_SRC/pdf/FINDINGS.pdf"
echo "==> Open with: open '$LOCAL_SRC/pdf/FINDINGS.pdf'"
