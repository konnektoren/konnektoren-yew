use crate::model::{CursorConfig, Excursion, NoExtension, Step};
use std::rc::Rc;
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_TOKEN: AtomicU64 = AtomicU64::new(1);

/// Identifies one execution attempt, including across playback restarts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlaybackToken(u64);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackOutcome {
    Complete,
    Pending,
}

/// Executes a step synchronously, for hosts without timers (tests, previews).
///
/// Browser applications use [`crate::use_playback`] instead, which drives the
/// engine with [`PlaybackEngine::start`] and [`PlaybackEngine::complete_pending`].
pub trait PlaybackHost<X = NoExtension> {
    type Error;

    /// Return `Pending` for work completed later; pass its token back to the engine.
    fn execute(
        &mut self,
        step: &Step<X>,
        token: PlaybackToken,
    ) -> Result<PlaybackOutcome, Self::Error>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackState {
    /// Ready to start the current step.
    Playing,
    /// Paused; a pending attempt may still complete.
    Paused,
    /// The current step was started and awaits its token.
    Waiting,
    Finished,
    Stopped,
}

/// Sequences an excursion. It knows nothing about timing or the DOM; a host
/// starts each step, does the work, then completes or fails its token.
#[derive(Debug)]
pub struct PlaybackEngine<X = NoExtension> {
    excursion: Rc<Excursion<X>>,
    index: usize,
    cycle: usize,
    state: PlaybackState,
    pending: Option<PlaybackToken>,
}

impl<X> Clone for PlaybackEngine<X> {
    fn clone(&self) -> Self {
        Self {
            excursion: self.excursion.clone(),
            index: self.index,
            cycle: self.cycle,
            state: self.state,
            pending: self.pending,
        }
    }
}

impl<X: PartialEq> PartialEq for PlaybackEngine<X> {
    fn eq(&self, other: &Self) -> bool {
        (Rc::ptr_eq(&self.excursion, &other.excursion) || self.excursion == other.excursion)
            && self.index == other.index
            && self.cycle == other.cycle
            && self.state == other.state
            && self.pending == other.pending
    }
}

impl<X> PlaybackEngine<X> {
    pub fn new(excursion: impl Into<Rc<Excursion<X>>>) -> Self {
        let excursion = excursion.into();
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

    pub fn excursion(&self) -> &Rc<Excursion<X>> {
        &self.excursion
    }
    pub fn state(&self) -> PlaybackState {
        self.state
    }
    /// Index of the current step; equals [`Self::total`] once finished.
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
    /// The started attempt, if any.
    pub fn pending(&self) -> Option<PlaybackToken> {
        self.pending
    }
    /// Cursor presentation configured by the root excursion.
    pub fn cursor(&self) -> Option<&CursorConfig> {
        self.excursion.cursor.as_ref()
    }

    pub fn current(&self) -> Option<&Step<X>> {
        if matches!(self.state, PlaybackState::Finished | PlaybackState::Stopped) {
            None
        } else {
            self.excursion.steps.get(self.index)
        }
    }

    /// Start the current step and wait for its token. Returns `None` unless playing.
    pub fn start(&mut self) -> Option<PlaybackToken> {
        if self.state != PlaybackState::Playing || self.current().is_none() {
            return None;
        }
        let token = PlaybackToken(NEXT_TOKEN.fetch_add(1, Ordering::Relaxed));
        self.pending = Some(token);
        self.state = PlaybackState::Waiting;
        Some(token)
    }

    /// Start and execute one step synchronously. A failed step remains current
    /// so the host may retry.
    pub fn advance<H: PlaybackHost<X>>(&mut self, host: &mut H) -> Result<(), H::Error> {
        let Some(token) = self.start() else {
            return Ok(());
        };
        let excursion = self.excursion.clone();
        match host.execute(&excursion.steps[self.index], token) {
            Ok(PlaybackOutcome::Complete) => {
                self.complete_pending(token);
                Ok(())
            }
            Ok(PlaybackOutcome::Pending) => Ok(()),
            Err(error) => {
                self.fail_pending(token);
                Err(error)
            }
        }
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
        if self.state == PlaybackState::Waiting {
            self.state = PlaybackState::Playing;
        }
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

    fn excursion(repeat: bool, steps: Vec<Step>) -> Excursion {
        Excursion {
            version: 1,
            id: "demo".into(),
            title: "Demo".into(),
            repeat,
            cursor: None,
            steps,
        }
    }

    #[derive(Default)]
    struct Host {
        steps: Vec<Step>,
        fail: bool,
    }
    impl PlaybackHost for Host {
        type Error = &'static str;
        fn execute(
            &mut self,
            step: &Step,
            _: PlaybackToken,
        ) -> Result<PlaybackOutcome, Self::Error> {
            if self.fail {
                return Err("failed");
            }
            self.steps.push(step.clone());
            Ok(PlaybackOutcome::Complete)
        }
    }

    #[derive(Default)]
    struct PendingHost(Option<PlaybackToken>);
    impl PlaybackHost for PendingHost {
        type Error = ();
        fn execute(&mut self, _: &Step, token: PlaybackToken) -> Result<PlaybackOutcome, ()> {
            self.0 = Some(token);
            Ok(PlaybackOutcome::Pending)
        }
    }

    #[test]
    fn sequences_steps_and_keeps_failed_step_for_retry() {
        let mut engine =
            PlaybackEngine::new(excursion(false, vec![Step::say("Hi"), Step::say("Bye")]));
        let mut host = Host::default();
        engine.pause();
        assert!(engine.advance(&mut host).is_ok());
        assert_eq!(engine.index(), 0);
        engine.resume();
        host.fail = true;
        assert!(engine.advance(&mut host).is_err());
        assert_eq!(
            (engine.index(), engine.state()),
            (0, PlaybackState::Playing)
        );
        host.fail = false;
        assert!(engine.advance(&mut host).is_ok());
        assert!(engine.advance(&mut host).is_ok());
        assert_eq!(engine.state(), PlaybackState::Finished);
        assert_eq!(host.steps.len(), 2);
    }

    #[test]
    fn empty_and_stopped_excursions_do_not_execute() {
        assert_eq!(
            PlaybackEngine::new(excursion(false, vec![])).state(),
            PlaybackState::Finished
        );
        let mut engine = PlaybackEngine::new(excursion(false, vec![Step::say("Hi")]));
        engine.stop();
        engine.resume();
        let mut host = Host::default();
        assert!(engine.advance(&mut host).is_ok());
        assert!(engine.start().is_none());
        assert!(host.steps.is_empty());
        assert_eq!(engine.state(), PlaybackState::Stopped);
    }

    #[test]
    fn pending_steps_wait_retry_and_ignore_stale_completions() {
        let tour = Rc::new(excursion(false, vec![Step::say("Hi"), Step::say("Bye")]));
        let mut engine = PlaybackEngine::new(tour.clone());
        let mut host = PendingHost::default();
        assert!(engine.advance(&mut host).is_ok());
        let first = host.0.expect("pending token");
        assert_eq!(engine.state(), PlaybackState::Waiting);
        assert_eq!(engine.pending(), Some(first));
        assert!(engine.advance(&mut host).is_ok());
        assert_eq!(engine.index(), 0);
        assert!(engine.fail_pending(first));
        let second = engine.start().expect("retry token");
        assert!(!engine.complete_pending(first));
        engine.pause();
        assert!(engine.complete_pending(second));
        assert_eq!(engine.state(), PlaybackState::Paused);
        assert_eq!(engine.index(), 1);
        engine.resume();
        let third = engine.start().expect("next token");
        engine.stop();
        assert!(!engine.complete_pending(third));
        let mut restarted = PlaybackEngine::new(tour);
        let fourth = restarted.start().expect("new token");
        assert!(!restarted.complete_pending(third));
        assert!(restarted.complete_pending(fourth));
    }

    #[test]
    fn repeats_and_keeps_paused_state_across_the_loop() {
        let mut engine = PlaybackEngine::new(excursion(true, vec![Step::say("Hi")]));
        let mut host = Host::default();
        assert!(engine.advance(&mut host).is_ok());
        assert_eq!(
            (engine.index(), engine.cycle(), engine.state()),
            (0, 1, PlaybackState::Playing)
        );
        let token = engine.start().expect("token");
        engine.pause();
        assert!(engine.complete_pending(token));
        assert_eq!(
            (engine.index(), engine.cycle(), engine.state()),
            (0, 2, PlaybackState::Paused)
        );
        engine.resume();
        assert_eq!(engine.state(), PlaybackState::Playing);
    }
}
