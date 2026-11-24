use crate::model::{Excursion, PlaybackStep};

/// Executes an instruction in the host application.
///
/// Hosts may deserialize `PlaybackStep::Extension` parameters into their own
/// action types and should return an error for unknown extensions.
pub trait PlaybackHost {
    type Error;

    fn execute(&mut self, step: &PlaybackStep) -> Result<(), Self::Error>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Finished,
    Stopped,
}

/// Sequences an excursion; the host performs each instruction.
#[derive(Clone, Debug, PartialEq)]
pub struct PlaybackEngine {
    excursion: Excursion,
    index: usize,
    state: PlaybackState,
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
            state,
        }
    }

    pub fn state(&self) -> PlaybackState {
        self.state
    }
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn total(&self) -> usize {
        self.excursion.steps.len()
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
            host.execute(step)?;
            self.index += 1;
            if self.index == self.total() {
                self.state = PlaybackState::Finished;
            }
        }
        Ok(())
    }

    pub fn pause(&mut self) {
        if self.state == PlaybackState::Playing {
            self.state = PlaybackState::Paused;
        }
    }

    pub fn resume(&mut self) {
        if self.state == PlaybackState::Paused {
            self.state = PlaybackState::Playing;
        }
    }

    pub fn stop(&mut self) {
        if self.state != PlaybackState::Finished {
            self.state = PlaybackState::Stopped;
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
        fn execute(&mut self, step: &PlaybackStep) -> Result<(), Self::Error> {
            if self.fail {
                return Err("failed");
            }
            self.steps.push(step.clone());
            Ok(())
        }
    }

    #[test]
    fn sequences_steps_and_keeps_failed_step_for_retry() {
        let excursion = Excursion {
            version: 1,
            id: "demo".into(),
            title: "Demo".into(),
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
}
