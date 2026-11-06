//! Runtime packet impairment shared by every latency relay.
//!
//! The relays delay datagrams from a model resolved once at startup. A scenario that reproduces a
//! transport fault also needs to take a link away for a while and hand it back degraded, and that
//! schedule cannot be expressed in the latency config: it has to change *during* the run. So the
//! schedule lives in a file the orchestrator watches, `<data-dir>/impairment`, which the test
//! rewrites between phases:
//!
//! ```text
//! blackhole on          # drop everything, both directions
//! drop-every 20         # drop one datagram in every 20
//! loss-ppm 50000        # drop each datagram with probability 50000/1_000_000
//! ```
//!
//! Each line is independent; `off` (or `0`) clears it, an absent or unreadable file clears
//! everything. Unknown keys and blank lines are ignored, so a caller can leave a commented
//! schedule in the file. Rewrite it atomically (write a temporary file and rename) — the watcher
//! polls, and a half-written file would be read as a different schedule.
//!
//! Every relay applies the same impairment: it is keyed by nothing, because the link this exists
//! for is the one between a client outside the cluster and the node it dials, and that client has
//! no cluster index to name it by.
//!
//! Counters are exposed in the `status` snapshot, so a test can tell "the impairment never applied"
//! apart from "the connection held" — the difference between a scenario that failed and one that
//! never ran.

use std::sync::{
    Arc, RwLock,
    atomic::{AtomicU64, Ordering},
};

/// What the relays should do with a datagram right now.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Spec {
    /// Drop everything.
    pub blackhole: bool,
    /// Drop one datagram in every `n`; `None` (or `Some(0)`) is off.
    pub drop_every: Option<u32>,
    /// Per-datagram drop probability, in parts per million.
    pub loss_ppm: u32,
}

impl Spec {
    /// Whether this spec would drop anything at all.
    pub fn is_empty(&self) -> bool {
        !self.blackhole && self.drop_every.is_none() && self.loss_ppm == 0
    }
}

/// Which leg of a relay a datagram is on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Peer → node.
    Forward,
    /// Node → peer.
    Return,
}

/// Datagrams seen and dropped since the relays started.
///
/// Split by direction: a scenario that shapes the link between a client and a node has to show the
/// impairment sat on *both* legs, or a run in which only one direction crossed the relays would
/// look like a healthy connection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counters {
    pub forwarded_forward: u64,
    pub forwarded_return: u64,
    pub dropped: u64,
}

/// The shared impairment: one instance drives every relay in the cluster.
#[derive(Debug, Default)]
pub struct Impairment {
    spec: RwLock<Spec>,
    /// Counts drop decisions, so `drop_every` spreads over the datagrams actually offered.
    decisions: AtomicU64,
    forwarded_forward: AtomicU64,
    forwarded_return: AtomicU64,
    dropped: AtomicU64,
}

impl Impairment {
    /// The current schedule.
    pub fn spec(&self) -> Spec {
        self.spec.read().map(|s| *s).unwrap_or_default()
    }

    /// Replace the schedule.
    pub fn set_spec(&self, spec: Spec) {
        if let Ok(mut current) = self.spec.write() {
            *current = spec;
        }
    }

    /// Datagrams seen and dropped so far.
    pub fn counters(&self) -> Counters {
        Counters {
            forwarded_forward: self.forwarded_forward.load(Ordering::Relaxed),
            forwarded_return: self.forwarded_return.load(Ordering::Relaxed),
            dropped: self.dropped.load(Ordering::Relaxed),
        }
    }

    /// Decide one datagram's fate, counting a drop if that is the verdict.
    ///
    /// `true` means forward it. The decision is taken per datagram rather than per flow, so a
    /// blackhole that ends mid-run lets the next datagram through instead of holding a session
    /// that will never be used again.
    ///
    /// Admission is not delivery: a datagram let through here can still be dropped by a full
    /// queue, a full release heap or a failed socket send. [`Self::record_forwarded`] counts the
    /// ones that actually left, so `forwarded_*` measures traffic that crossed the relay rather
    /// than traffic the schedule allowed.
    pub fn admit(&self) -> bool {
        let spec = self.spec();
        let seen = self.decisions.fetch_add(1, Ordering::Relaxed);
        let drop = spec.blackhole
            || spec
                .drop_every
                .is_some_and(|n| n > 0 && seen.is_multiple_of(u64::from(n)))
            || (spec.loss_ppm > 0 && rand::random::<u32>() % 1_000_000 < spec.loss_ppm);
        if drop {
            self.dropped.fetch_add(1, Ordering::Relaxed);
        }
        !drop
    }

    /// Count a datagram the relay sent, in the direction it was sent.
    pub fn record_forwarded(&self, direction: Direction) {
        let counter = match direction {
            Direction::Forward => &self.forwarded_forward,
            Direction::Return => &self.forwarded_return,
        };
        counter.fetch_add(1, Ordering::Relaxed);
    }
}

/// Parse an impairment file.
///
/// Unparseable values leave that line's setting off rather than failing the watcher: this file is
/// written by a test mid-run, and a typo should not silently keep the *previous* schedule — the
/// absent-file default (nothing dropped) is the honest reading of a file that says nothing usable.
pub fn parse(contents: &str) -> Spec {
    let mut spec = Spec::default();
    for line in contents.lines() {
        let line = line.split('#').next().unwrap_or_default().trim();
        let mut parts = line.split_whitespace();
        let (Some(key), Some(value)) = (parts.next(), parts.next()) else {
            continue;
        };
        match (key, value) {
            ("blackhole", v) => spec.blackhole = matches!(v, "on" | "true" | "1"),
            ("drop-every", v) => {
                spec.drop_every = match v {
                    "off" | "none" => None,
                    _ => v.parse::<u32>().ok().filter(|n| *n > 0),
                }
            }
            ("loss-ppm", v) => spec.loss_ppm = v.parse::<u32>().unwrap_or(0).min(1_000_000),
            _ => {}
        }
    }
    spec
}

/// Keep `impairment` in sync with the file at `path`, polling it every `POLL` (100 ms).
///
/// A missing file means "no impairment", which is also the state a finished test leaves behind.
pub fn watch(path: std::path::PathBuf) -> Arc<Impairment> {
    /// How often the file is re-read. Short enough that a four-second blackhole is a blackhole,
    /// long enough that a long run does not spend its time in `read_to_string`.
    const POLL: std::time::Duration = std::time::Duration::from_millis(100);

    let impairment = Arc::new(Impairment::default());
    let shared = impairment.clone();
    tokio::spawn(async move {
        loop {
            // Read through tokio: this loop shares a runtime with the relays, and a blocking
            // read on a worker thread would stall datagrams for as long as the file takes.
            let spec = tokio::fs::read_to_string(&path)
                .await
                .map(|contents| parse(&contents))
                .unwrap_or_default();
            if shared.spec() != spec {
                tracing::info!(?spec, path = %path.display(), "relay impairment changed");
                shared.set_spec(spec);
            }
            tokio::time::sleep(POLL).await;
        }
    });
    impairment
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_absent_schedule_impairs_nothing() {
        assert!(parse("").is_empty());
        assert_eq!(parse(""), Spec::default());
    }

    #[test]
    fn the_three_knobs_parse() {
        assert_eq!(
            parse("blackhole on\ndrop-every 20\nloss-ppm 50000\n"),
            Spec {
                blackhole: true,
                drop_every: Some(20),
                loss_ppm: 50_000,
            }
        );
    }

    #[test]
    fn off_clears_a_knob_and_junk_is_ignored() {
        // A cleared line must read as off rather than keeping the previous schedule.
        let spec = parse("blackhole off\ndrop-every off\nloss-ppm 0\nnonsense here\n");
        assert!(spec.is_empty());
        // Comments and blank lines are skipped, and an unparseable count is off, not "keep".
        assert_eq!(parse("# a note\n\ndrop-every later\n").drop_every, None);
        // A rate above 1 can only mean "drop everything".
        assert_eq!(parse("loss-ppm 9999999\n").loss_ppm, 1_000_000);
    }

    #[test]
    fn a_blackhole_drops_everything_and_counts_it() {
        let impairment = Impairment::default();
        impairment.set_spec(parse("blackhole on\n"));
        for _ in 0..5 {
            assert!(!impairment.admit());
        }
        assert_eq!(
            impairment.counters(),
            Counters {
                forwarded_forward: 0,
                forwarded_return: 0,
                dropped: 5
            }
        );
    }

    #[test]
    fn drop_every_spreads_over_the_datagrams_offered() {
        let impairment = Impairment::default();
        impairment.set_spec(parse("drop-every 4\n"));
        let forwarded = (0..8).filter(|_| impairment.admit()).count();
        // One in four, starting with the first: 2 of 8.
        assert_eq!(forwarded, 6);
        assert_eq!(impairment.counters().dropped, 2);
    }

    #[test]
    fn clearing_the_schedule_stops_dropping_but_keeps_the_counts() {
        let impairment = Impairment::default();
        impairment.set_spec(parse("blackhole on\n"));
        assert!(!impairment.admit());
        impairment.set_spec(parse("blackhole off\n"));
        assert!(impairment.admit());
        impairment.record_forwarded(Direction::Return);
        assert_eq!(
            impairment.counters(),
            Counters {
                forwarded_forward: 0,
                forwarded_return: 1,
                dropped: 1
            }
        );
    }
}
