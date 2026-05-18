#!/usr/bin/env python3
"""
Memory leak hunting visualization from jemalloc heap dumps.

Walks a directory of jemalloc heap dumps (jeprof.<PID>.<N>.iN.heap),
samples N snapshots evenly per PID, runs `jeprof --text` on each
against the matching binary, parses Total inuse + per-function bytes,
and plots:

  1. Total inuse memory over time (line plot, one series per PID)
  2. Top-K function inuse over time (stacked area, per PID, separate file)

Outputs PNGs to --out (default: /tmp/jeprof-plots).

Usage on the VM:

  nix-shell -p 'python313.withPackages(ps: [ps.matplotlib ps.numpy])' \\
    --run "python3 ~/hoprd/scripts/jeprof_plot.py \\
           --dumps /tmp/jeprof --binary ~/hoprd/result/bin/hoprd \\
           --samples 100 --out /tmp/jeprof-plots --topk 10"

Then `scp 'nixos-test@orb:/tmp/jeprof-plots/*.png' ./plots/` to view.
"""
from __future__ import annotations

import argparse
import os
import re
import subprocess
import sys
from collections import defaultdict
from dataclasses import dataclass
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt  # noqa: E402

SIZE_UNITS = {"B": 1, "KB": 1024, "MB": 1024**2, "GB": 1024**3, "TB": 1024**4}

TOTAL_RE = re.compile(r"^Total:\s+([\d.]+)\s+(\w+)\s*$")
# ROW_RE captures: self, self_unit, self%, cum%, cumulative, cumulative_unit, cum%, function
ROW_RE = re.compile(
    r"^\s*([\d.]+)\s*([a-zA-Z]+)?\s+([\d.]+)%\s+([\d.]+)%\s+([\d.]+)\s*([a-zA-Z]+)?\s+([\d.]+)%\s+(.+)$"
)
NAME_RE = re.compile(r"^jeprof\.(\d+)\.(\d+)\.i\d+\.heap$")


@dataclass
class Snapshot:
    pid: int
    seq: int                        # interval counter from filename
    path: Path
    total_mb: float = 0.0
    funcs: dict[str, float] = None  # {fn_name -> cum_mb}


def parse_jeprof(out: str, unit_to_mb: bool = True) -> tuple[float, dict[str, float]]:
    """Parse jeprof --text output. Returns (total_MB, {fn: cum_MB})."""
    total = 0.0
    default_unit = "MB"
    funcs: dict[str, float] = {}
    for line in out.splitlines():
        m = TOTAL_RE.match(line)
        if m:
            v = float(m.group(1))
            default_unit = m.group(2)
            total = v * SIZE_UNITS.get(default_unit, 1) / SIZE_UNITS["MB"]
            continue
        m = ROW_RE.match(line)
        if m:
            cum_val = float(m.group(5))
            cum_unit = m.group(6) or default_unit
            cum_mb = cum_val * SIZE_UNITS.get(cum_unit, 1) / SIZE_UNITS["MB"]

            fn = m.group(8).strip()
            # Skip jemalloc's own backtrace metadata; it dominates.
            if fn.startswith("_rjem_je_prof_backtrace"):
                continue
            # Aggregate duplicates (same name, different addr suffix).
            short = re.sub(r"@[0-9a-fA-F]+$", "", fn)
            funcs[short] = funcs.get(short, 0.0) + cum_mb
    return total, funcs


def collect_snapshots(dumps_dir: Path) -> dict[int, list[Snapshot]]:
    by_pid: dict[int, list[Snapshot]] = defaultdict(list)
    for entry in os.scandir(dumps_dir):
        m = NAME_RE.match(entry.name)
        if not m:
            continue
        pid = int(m.group(1))
        seq = int(m.group(2))
        by_pid[pid].append(Snapshot(pid=pid, seq=seq, path=Path(entry.path)))
    for pid in by_pid:
        by_pid[pid].sort(key=lambda s: s.seq)
    return by_pid


def evenly_sample(snaps: list[Snapshot], k: int) -> list[Snapshot]:
    if len(snaps) <= k:
        return list(snaps)
    step = (len(snaps) - 1) / (k - 1)
    return [snaps[round(i * step)] for i in range(k)]


def run_jeprof(binary: Path, heap: Path, mode: str = "inuse_space",
               jeprof: str = "jeprof") -> str:
    cmd = [jeprof, "--text", f"--{mode}", str(binary), str(heap)]
    res = subprocess.run(cmd, capture_output=True, text=True, timeout=120)
    if res.returncode != 0 and not res.stdout:
        raise RuntimeError(f"jeprof failed: {res.stderr[:300]}")
    return res.stdout


def plot_total(by_pid: dict[int, list[Snapshot]], out: Path) -> Path:
    fig, ax = plt.subplots(figsize=(12, 6))
    for pid, snaps in sorted(by_pid.items()):
        xs = [s.seq for s in snaps]
        ys = [s.total_mb for s in snaps]
        ax.plot(xs, ys, marker="o", markersize=3, label=f"PID {pid}", alpha=0.85)
    ax.set_xlabel("interval counter (~MB allocated × 2^lg_prof_interval)")
    ax.set_ylabel("inuse (MB)")
    ax.set_title("Total inuse memory over time (per node)")
    ax.legend()
    ax.grid(True, alpha=0.3)
    fig.tight_layout()
    p = out / "total_inuse.png"
    fig.savefig(p, dpi=120)
    plt.close(fig)
    return p


def plot_per_pid(pid: int, snaps: list[Snapshot], topk: int, out: Path) -> Path:
    # union of top-K functions across all snapshots
    score: dict[str, float] = defaultdict(float)
    for s in snaps:
        for fn, mb in (s.funcs or {}).items():
            score[fn] = max(score[fn], mb)
    top_fns = [fn for fn, _ in sorted(score.items(), key=lambda x: -x[1])[:topk]]

    xs = [s.seq for s in snaps]
    series = [[(s.funcs or {}).get(fn, 0.0) for s in snaps] for fn in top_fns]

    fig, ax = plt.subplots(figsize=(14, 7))
    ax.stackplot(xs, *series, labels=[fn[:60] for fn in top_fns], alpha=0.85)
    ax.set_xlabel("interval counter")
    ax.set_ylabel("inuse (MB) — cumulative per fn")
    ax.set_title(f"PID {pid}: top-{topk} functions over time (excluding jeprof backtrace)")
    ax.legend(loc="upper left", fontsize=7, ncol=2)
    ax.grid(True, alpha=0.3)
    fig.tight_layout()
    p = out / f"top_{topk}_pid_{pid}.png"
    fig.savefig(p, dpi=120)
    plt.close(fig)
    return p


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--dumps", required=True, type=Path,
                    help="Directory of jeprof.<PID>.<N>.iN.heap files")
    ap.add_argument("--binary", required=True, type=Path,
                    help="Matching hoprd binary that produced the dumps")
    ap.add_argument("--samples", type=int, default=100,
                    help="Even snapshots per PID to analyze (default: 100)")
    ap.add_argument("--topk", type=int, default=10)
    ap.add_argument("--mode", choices=["inuse_space", "alloc_space"],
                    default="inuse_space")
    ap.add_argument("--out", type=Path, default=Path("/tmp/jeprof-plots"))
    ap.add_argument("--jeprof", default="jeprof")
    ap.add_argument("--pids", nargs="*", type=int, help="Restrict to these PIDs")
    args = ap.parse_args()

    args.out.mkdir(parents=True, exist_ok=True)
    print(f"scanning {args.dumps} ...", file=sys.stderr)
    by_pid = collect_snapshots(args.dumps)
    if args.pids:
        by_pid = {p: by_pid[p] for p in args.pids if p in by_pid}
    if not by_pid:
        print("no dumps found", file=sys.stderr)
        return 2

    print(f"PIDs: {sorted(by_pid)} -- counts: "
          + ", ".join(f"{p}={len(s)}" for p, s in by_pid.items()), file=sys.stderr)

    sampled: dict[int, list[Snapshot]] = {}
    for pid, snaps in by_pid.items():
        sub = evenly_sample(snaps, args.samples)
        print(f"PID {pid}: analyzing {len(sub)} snapshots ...", file=sys.stderr)
        for i, s in enumerate(sub, 1):
            try:
                out = run_jeprof(args.binary, s.path, args.mode, args.jeprof)
                s.total_mb, s.funcs = parse_jeprof(out)
            except Exception as e:
                print(f"  WARN {s.path.name}: {e}", file=sys.stderr)
                s.total_mb, s.funcs = 0.0, {}
            if i % max(1, len(sub) // 10) == 0:
                print(f"    [{i}/{len(sub)}] total={s.total_mb:.1f} MB",
                      file=sys.stderr)
        sampled[pid] = sub

    p1 = plot_total(sampled, args.out)
    print(f"wrote {p1}")
    for pid, snaps in sampled.items():
        p = plot_per_pid(pid, snaps, args.topk, args.out)
        print(f"wrote {p}")

    # CSV dump of total time-series (for spreadsheets / further analysis)
    csv = args.out / "totals.csv"
    with csv.open("w") as f:
        f.write("pid,seq,total_mb\n")
        for pid, snaps in sampled.items():
            for s in snaps:
                f.write(f"{pid},{s.seq},{s.total_mb:.3f}\n")
    print(f"wrote {csv}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
