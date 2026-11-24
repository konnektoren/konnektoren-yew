use crate::model::{Excursion, PlaybackStep};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_TOKEN: AtomicU64 = AtomicU64::new(1);

/// Identifies one execution attempt, including across playback restarts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlaybackToken(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackOutcome {
    Complete,
    Pending,
}

/// Executes an instruction in the host application.
///
/// Hosts may deserialize `PlaybackStep::Extension` parameters into their own
/// action types and should return an error for unknown extensions. The host
/// owns cancellation and cleanup of any work it starts, including on stop.
pub trait PlaybackHost {
    type Error;

    /// Return `Pending` for work completed later; pass its token back to the engine.
    fn execute(
        &mut self,
        step: &PlaybackStep,
        token: PlaybackToken,
    ) -> Result<PlaybackOutcome, Self::Error>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Waiting,
    Finished,
    Stopped,
}

/// Sequences an excursion; the host performs each instruction.
#[derive(Clone, Debug, PartialEq)]
pub struct PlaybackEngine {
    excursion: Excursion,
    index: usize,
    cycle: usize,
    state: PlaybackState,
    pending: Option<PlaybackToken>,
}

impl PlaybackEngine {
    pub fn new(excursion: Excursion) -> Self {
        let state = if excursion.steps.is_empty() {
            PlaybackState::Finished
        } else {
            PlaybackState::Playing
        };
        Self {
            excursion,
            index: 0,
            cycle: 0,
            state,
            pending: None,
        }
    }

    pub fn state(&self) -> PlaybackState {
        self.state
    }
    pub fn index(&self) -> usize {
        self.index
    }
    /// Number of completed loops, starting at zero.
    pub fn cycle(&self) -> usize {
        self.cycle
    }
    pub fn total(&self) -> usize {
        self.excursion.steps.len()
    }

    /// Cursor presentation configured by the root excursion.
    pub fn cursor(&self) -> Option<&crate::model::CursorConfig> {
        self.excursion.cursor.as_ref()
    }

    pub fn current(&self) -> Option<&PlaybackStep> {
        if matches!(self.state, PlaybackState::Finished | PlaybackState::Stopped) {
            None
        } else {
            self.excursion.steps.get(self.index)
        }
    }

    /// Executes one step. A failed step remains current so the host may retry.
    pub fn advance<H: PlaybackHost>(&mut self, host: &mut H) -> Result<(), H::Error> {
        if self.state != PlaybackState::Playing {
            return Ok(());
        }
        if let Some(step) = self.current() {
            let token = PlaybackToken(NEXT_TOKEN.fetch_add(1, Ordering::Relaxed));
            match host.execute(step, token)? {
                PlaybackOutcome::Complete => self.finish_step(),
                PlaybackOutcome::Pending => {
                    self.pending = Some(token);
                    self.state = PlaybackState::Waiting;
                }
            }
        }
        Ok(())
    }

    fn finish_step(&mut self) {
        self.index += 1;
        if self.index == self.total() {
            if self.excursion.repeat {
                self.index = 0;
                self.cycle = self.cycle.wrapping_add(1);
            } else {
                self.state = PlaybackState::Finished;
            }
        }
    }

    /// Finish a pending attempt; returns false for an obsolete token.
    pub fn complete_pending(&mut self, token: PlaybackToken) -> bool {
        if self.pending != Some(token)
            || !matches!(self.state, PlaybackState::Waiting | PlaybackState::Paused)
        {
            return false;
        }
        self.pending = None;
        let paused = self.state == PlaybackState::Paused;
        self.state = if paused {
            PlaybackState::Paused
        } else {
            PlaybackState::Playing
        };
        self.finish_step();
        true
    }

    /// Mark a pending attempt as failed so the same step can be retried.
    pub fn fail_pending(&mut self, token: PlaybackToken) -> bool {
        if self.pending != Some(token)
            || !matches!(self.state, PlaybackState::Waiting | PlaybackState::Paused)
        {
            return false;
        }
        self.pending = None;
        if self.state == PlaybackState::Waiting {
            self.state = PlaybackState::Playing;
        }
        true
    }

    pub fn pause(&mut self) {
        if matches!(self.state, PlaybackState::Playing | PlaybackState::Waiting) {
            self.state = PlaybackState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.state == PlaybackState::Paused {
            self.state = if self.pending.is_some() {
                PlaybackState::Waiting
            } else {
                PlaybackState::Playing
            };
        }
    }

    pub fn stop(&mut self) {
        if self.state != PlaybackState::Finished {
            self.state = PlaybackState::Stopped;
            self.pending = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Host {
        steps: Vec<PlaybackStep>,
        fail: bool,
    }
    impl PlaybackHost for Host {
        type Error = &'static str;
        fn execute(
            &mut self,
            step: &PlaybackStep,
            _: PlaybackToken,
        ) -> Result<PlaybackOutcome, Self::Error> {
            if self.fail {
                return Err("failed");
            }
            self.steps.push(step.clone());
            Ok(PlaybackOutcome::Complete)
        }
    }

    #[test]
    fn sequences_steps_and_keeps_failed_step_for_retry() {
        let excursion = Excursion {
            version: 1,
            id: "demo".into(),
            title: "Demo".into(),
            repeat: false,
            cursor: None,
            steps: vec![
                PlaybackStep::Say { say: "Hi".into() },
                PlaybackStep::Zoom { zoom: 1.2 },
            ],
        };
        let mut engine = PlaybackEngine::new(excursion);
        let mut host = Host::default();
        engine.pause();
        assert!(engine.advance(&mut host).is_ok());
        assert_eq!(engine.index(), 0);
        engine.resume();
        host.fail = true;
        assert!(engine.advance(&mut host).is_err());
        assert_eq!(engine.index(), 0);
        host.fail = false;
        assert!(engine.advance(&mut host).is_ok());
        assert!(engine.advance(&mut host).is_ok());
        assert_eq!(engine.state(), PlaybackState::Finished);
        assert_eq!(host.steps.len(), 2);
    }

    #[test]
    fn empty_and_stopped_excursions_do_not_execute() {
        let excursion = Excursion {
            version: 1,
            id: "empty".into(),
            title: "Empty".into(),
            repeat: false,
            cursor: None,
            steps: vec![],
        };
        assert_eq!(
            PlaybackEngine::new(excursion).state(),
            PlaybackState::Finished
        );
        let excursion = Excursion {
            version: 1,
            id: "demo".into(),
            title: "Demo".into(),
            repeat: false,
            cursor: None,
            steps: vec![PlaybackStep::Say { say: "Hi".into() }],
        };
        let mut engine = PlaybackEngine::new(excursion);
        engine.stop();
        engine.resume();
        let mut host = Host::default();
        assert!(engine.advance(&mut host).is_ok());
        assert!(host.steps.is_empty());
        assert_eq!(engine.state(), PlaybackState::Stopped);
    }

    #[test]
    fn pending_steps_wait_retry_and_ignore_stale_completions() {
        #[derive(Default)]
        struct PendingHost(Option<PlaybackToken>);
        impl PlaybackHost for PendingHost {
            type Error = ();
            fn execute(
                &mut self,
                _: &PlaybackStep,
                token: PlaybackToken,
            ) -> Result<PlaybackOutcome, Self::Error> {
                self.0 = Some(token);
                Ok(PlaybackOutcome::Pending)
            }
        }

        let excursion = Excursion {
            version: 1,
            id: "demo".into(),
            title: "Demo".into(),
            repeat: false,
            cursor: None,
            steps: vec![
                PlaybackStep::Say { say: "Hi".into() },
                PlaybackStep::Zoom { zoom: 1.0 },
            ],
        };
        let mut engine = PlaybackEngine::new(excursion.clone());
        let mut host = PendingHost::default();
        assert!(engine.advance(&mut host).is_ok());
        let first = host.0.expect("pending token");
        assert_eq!(engine.state(), PlaybackState::Waiting);
        assert!(engine.advance(&mut host).is_ok());
        assert_eq!(engine.index(), 0);
        assert!(engine.fail_pending(first));
        assert!(engine.advance(&mut host).is_ok());
        let second = host.0.expect("retry token");
        assert!(!engine.complete_pending(first));
        engine.pause();
        assert!(engine.complete_pending(second));
        assert_eq!(engine.state(), PlaybackState::Paused);
        assert_eq!(engine.index(), 1);
        engine.resume();
        assert!(engine.advance(&mut host).is_ok());
        let third = host.0.expect("next token");
        engine.stop();
        assert!(!engine.complete_pending(third));
        let mut restarted = PlaybackEngine::new(excursion);
        assert!(restarted.advance(&mut host).is_ok());
        assert!(!restarted.complete_pending(third));
        assert!(restarted.complete_pending(host.0.expect("new token")));
    }

    #[test]
    fn repeats_completed_steps_and_keeps_empty_excursions_finished() {
        let mut excursion = Excursion {
            version: 1,
            id: "loop".into(),
            title: "Loop".into(),
            repeat: true,
            cursor: None,
            steps: vec![PlaybackStep::Say { say: "Hi".into() }],
        };
        let mut engine = PlaybackEngine::new(excursion.clone());
        let mut host = Host::default();
        assert!(engine.advance(&mut host).is_ok());
        assert_eq!(
            (engine.index(), engine.cycle(), engine.state()),
            (0, 1, PlaybackState::Playing)
        );
        assert!(engine.advance(&mut host).is_ok());
        assert_eq!(engine.cycle(), 2);
        excursion.steps.clear();
        assert_eq!(
            PlaybackEngine::new(excursion).state(),
            PlaybackState::Finished
        );
    }

    #[test]
    fn pending_last_step_repeats_without_resuming_a_paused_engine() {
        struct PendingHost(Option<PlaybackToken>);
        impl PlaybackHost for PendingHost {
            type Error = ();
            fn execute(
                &mut self,
                _: &PlaybackStep,
                token: PlaybackToken,
            ) -> Result<PlaybackOutcome, Self::Error> {
                self.0 = Some(token);
                Ok(PlaybackOutcome::Pending)
            }
        }
        let excursion = Excursion {
            version: 1,
            id: "loop".into(),
            title: "Loop".into(),
            repeat: true,
            cursor: None,
            steps: vec![PlaybackStep::Say { say: "Hi".into() }],
        };
        let mut engine = PlaybackEngine::new(excursion);
        let mut host = PendingHost(None);
        assert!(engine.advance(&mut host).is_ok());
        engine.pause();
        assert!(engine.complete_pending(host.0.expect("pending token")));
        assert_eq!(
            (engine.index(), engine.cycle(), engine.state()),
            (0, 1, PlaybackState::Paused)
        );
        engine.resume();
        assert_eq!(engine.state(), PlaybackState::Playing);
    }
}
