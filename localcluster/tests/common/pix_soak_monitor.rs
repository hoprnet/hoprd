//! Closure evidence for the single PIX session owned by the soak test.
//!
//! The REST API lists Entry listeners, not Exit sessions, and the pool's timeout
//! counter stays zero when an SSA handshake fails before deposit tracking starts.
//! Tail this run's logs from immediately before opening its only session instead.

use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead, BufReader, Seek, SeekFrom},
    path::Path,
};

#[derive(Clone, Debug)]
pub struct SessionClosure {
    pub description: String,
    deposit_timeout: bool,
}

impl SessionClosure {
    pub fn is_budget_exhaustion(&self, deposits: u64, funded: u64, budget_refusals: u64) -> bool {
        self.deposit_timeout && deposits == funded && budget_refusals > 0
    }
}

#[derive(Default)]
struct SessionEvents {
    closure: Option<SessionClosure>,
    recent: VecDeque<String>,
}

impl SessionEvents {
    fn observe(&mut self, role: &str, line: &str) {
        let line = strip_ansi(line);
        let manager = line
            .split_once("hopr_transport_session::manager: ")
            .map(|(_, msg)| msg);
        // The PIX supervisor closes the session and the manager logs the reason; only the
        // `DepositTimeout` reason is the budget-exhaustion closure this soak expects. Neither
        // "timeout set" nor "timeout - session not found" proves a closure.
        let deposit_timeout = role == "Exit"
            && manager.is_some_and(|msg| {
                msg.starts_with("pix supervisor closed the session ")
                    && msg.contains("reason=DepositTimeout")
            });
        let closed = deposit_timeout
            || manager.is_some_and(|msg| msg.starts_with("closed session "))
            || line.contains("hopr_utils_session: client session ended ")
            || line.contains("hopr_session_server_forwarder: server bridged session to UDP ended ");
        let relevant = closed
            || manager.is_some_and(|msg| {
                msg.starts_with("generated exit commitments for the SSA batch ")
                    || msg.starts_with("generated client SSA commitment and deposit address ")
                    || msg.starts_with("timeout sending ")
                    || msg.starts_with("failed to send ")
                    || msg.starts_with("failed to process Start protocol message ")
            })
            || line.contains("single deposit flush failed ")
            || line.contains("deposit tracking timed out");
        if relevant {
            // Keep the handshake error and last SSA indices without copying the packet log.
            if self.recent.len() == 8 {
                self.recent.pop_front();
            }
            self.recent
                .push_back(format!("{role}: {}", line.trim_end()));
        }
        if closed && (self.closure.is_none() || deposit_timeout) {
            self.closure = Some(SessionClosure {
                description: format!("{role}: {}", line.trim_end()),
                deposit_timeout,
            });
        }
    }

    fn summary(&self) -> String {
        if self.recent.is_empty() {
            "no session lifecycle events found in the current node logs".to_owned()
        } else {
            self.recent.iter().cloned().collect::<Vec<_>>().join("\n")
        }
    }
}

struct LogTail<R> {
    reader: R,
    partial: String,
}

impl<R: BufRead + Seek> LogTail<R> {
    fn from_now(mut reader: R) -> io::Result<Self> {
        reader.seek(SeekFrom::End(0))?;
        Ok(Self {
            reader,
            partial: String::new(),
        })
    }

    fn poll(&mut self, role: &str, events: &mut SessionEvents) -> io::Result<()> {
        while self.reader.read_line(&mut self.partial)? != 0 {
            if !self.partial.ends_with('\n') {
                // A logger can be halfway through a write. Keep it until the next poll.
                break;
            }
            events.observe(role, &self.partial);
            self.partial.clear();
        }
        Ok(())
    }
}

pub struct SessionMonitor {
    entry: LogTail<BufReader<File>>,
    exit: LogTail<BufReader<File>>,
    events: SessionEvents,
}

impl SessionMonitor {
    pub fn open(entry_log: &Path, exit_log: &Path) -> io::Result<Self> {
        Ok(Self {
            entry: LogTail::from_now(BufReader::new(File::open(entry_log)?))?,
            exit: LogTail::from_now(BufReader::new(File::open(exit_log)?))?,
            events: SessionEvents::default(),
        })
    }

    pub fn poll(&mut self) -> io::Result<Option<SessionClosure>> {
        self.entry.poll("Entry", &mut self.events)?;
        self.exit.poll("Exit", &mut self.events)?;
        Ok(self.events.closure.clone())
    }

    pub fn summary(&self) -> String {
        self.events.summary()
    }
}

fn strip_ansi(line: &str) -> String {
    let mut plain = String::with_capacity(line.len());
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' && chars.peek() == Some(&'[') {
            chars.next();
            for code in chars.by_ref() {
                if ('@'..='~').contains(&code) {
                    break;
                }
            }
        } else {
            plain.push(ch);
        }
    }
    plain
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use super::*;

    const MANAGER: &str = "INFO hopr_transport_session::manager: ";
    const TIMEOUT: &str = "pix session deposit timeout session_id=abc ssa_index=5";

    #[test]
    fn handshake_failure_closes_without_a_pool_timeout() {
        let mut events = SessionEvents::default();
        events.observe("Exit", &format!("{MANAGER}generated exit commitments for the SSA batch session_id=abc first_ssa_index=5"));
        events.observe(
            "Entry",
            &format!("{MANAGER}timeout sending client SSA commitment message"),
        );
        events.observe("Entry", &format!("{MANAGER}failed to process Start protocol message error=session operation timed out"));
        assert!(events.closure.is_none());
        events.observe("Exit", &format!("{MANAGER}{TIMEOUT}"));
        events.observe("Exit", "INFO hopr_session_server_forwarder: server bridged session to UDP ended session_id=abc");
        let closure = events.closure.as_ref().unwrap();
        assert!(closure.description.contains("ssa_index=5"));
        assert!(!closure.is_budget_exhaustion(4, 10, 0));
        assert!(
            events
                .summary()
                .contains("timeout sending client SSA commitment message")
        );
    }

    #[test]
    fn expected_end_requires_actual_timeout_and_all_deposits_and_budget_refusal() {
        let mut events = SessionEvents::default();
        events.observe("Exit", &format!("{MANAGER}{TIMEOUT}"));
        let closure = events.closure.unwrap();
        assert!(closure.is_budget_exhaustion(10, 10, 1));
        assert!(!closure.is_budget_exhaustion(4, 10, 1));
        assert!(!closure.is_budget_exhaustion(10, 10, 0));
        assert!(!closure.is_budget_exhaustion(11, 10, 1));
    }

    #[test]
    fn arming_or_missing_a_session_is_not_closure() {
        let mut events = SessionEvents::default();
        for message in [
            "pix session deposit timeout set session_id=abc batch_size=1",
            "pix session deposit timeout - session not found session_id=abc",
        ] {
            events.observe("Exit", &format!("{MANAGER}{message}"));
        }
        events.observe(
            "Exit",
            "ERROR hopr_strategy::pix::strategy: deposit tracking timed out",
        );
        assert!(events.closure.is_none());
    }

    #[test]
    fn kill_switch_reason_can_arrive_after_bridge_closure() {
        let mut events = SessionEvents::default();
        events.observe("Exit", "INFO hopr_session_server_forwarder: server bridged session to UDP ended session_id=abc");
        assert!(
            !events
                .closure
                .as_ref()
                .unwrap()
                .is_budget_exhaustion(10, 10, 1)
        );
        events.observe("Exit", &format!("{MANAGER}{TIMEOUT}"));
        assert!(events.closure.unwrap().is_budget_exhaustion(10, 10, 1));
    }

    #[test]
    fn generic_closure_is_detected_but_cannot_pass_as_budget_exhaustion() {
        for line in [
            "INFO hopr_utils_session: client session ended session_id=abc",
            "INFO hopr_session_server_forwarder: server bridged session to UDP ended session_id=abc",
            "ERROR hopr_transport_session::manager: closed session due to too many unverifiable shares session_id=abc",
        ] {
            let mut events = SessionEvents::default();
            events.observe("Exit", line);
            assert!(!events.closure.unwrap().is_budget_exhaustion(10, 10, 1));
        }
    }

    #[test]
    fn tail_skips_old_events_and_handles_partial_colored_appends() {
        let old = format!("{MANAGER}{TIMEOUT}\n").into_bytes();
        let mut tail = LogTail::from_now(Cursor::new(old)).unwrap();
        let mut events = SessionEvents::default();
        tail.poll("Exit", &mut events).unwrap();
        assert!(events.closure.is_none());

        tail.reader.get_mut().extend_from_slice(b"\x1b[31mERROR\x1b[0m hopr_transport_session::manager: pix session deposit timeout session_");
        tail.poll("Exit", &mut events).unwrap();
        assert!(events.closure.is_none());
        tail.reader
            .get_mut()
            .extend_from_slice(b"id=abc ssa_index=5\n");
        tail.poll("Exit", &mut events).unwrap();
        assert!(events.closure.as_ref().unwrap().deposit_timeout);
        assert!(!events.summary().contains('\u{1b}'));
        let summary = events.summary();
        tail.poll("Exit", &mut events).unwrap();
        assert_eq!(events.summary(), summary);
    }
}
