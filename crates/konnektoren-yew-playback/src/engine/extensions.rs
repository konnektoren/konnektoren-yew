use super::{PlaybackHost, PlaybackOutcome, PlaybackToken};
use crate::model::{ExtensionAction, PlaybackStep};

/// Executes application-specific actions. Pending actions use the same completion
/// tokens as built-in instructions.
pub trait PlaybackExtensions {
    type Error;

    fn execute_extension(
        &mut self,
        action: &ExtensionAction,
        token: PlaybackToken,
    ) -> Result<PlaybackOutcome, Self::Error>;
}

/// Routes built-in instructions to a host and namespaced actions to an extension.
pub struct PlaybackWithExtensions<H, E> {
    pub host: H,
    pub extensions: E,
}

impl<H, E> PlaybackHost for PlaybackWithExtensions<H, E>
where
    H: PlaybackHost,
    E: PlaybackExtensions<Error = H::Error>,
{
    type Error = H::Error;

    fn execute(
        &mut self,
        step: &PlaybackStep,
        token: PlaybackToken,
    ) -> Result<PlaybackOutcome, Self::Error> {
        match step {
            PlaybackStep::Extension(action) => self.extensions.execute_extension(action, token),
            _ => self.host.execute(step, token),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Excursion, PlaybackEngine, PlaybackState};

    #[derive(Default)]
    struct Host(usize);
    impl PlaybackHost for Host {
        type Error = ();
        fn execute(&mut self, _: &PlaybackStep, _: PlaybackToken) -> Result<PlaybackOutcome, ()> {
            self.0 += 1;
            Ok(PlaybackOutcome::Complete)
        }
    }
    #[derive(Default)]
    struct Extension(Option<PlaybackToken>);
    impl PlaybackExtensions for Extension {
        type Error = ();
        fn execute_extension(
            &mut self,
            action: &ExtensionAction,
            token: PlaybackToken,
        ) -> Result<PlaybackOutcome, ()> {
            assert_eq!(action.name, "music.play_notes");
            self.0 = Some(token);
            Ok(PlaybackOutcome::Pending)
        }
    }

    #[test]
    fn extension_completion_advances_the_same_engine() {
        let excursion: Excursion =
            serde_yaml::from_str(include_str!("../../examples/extension.yaml")).expect("example");
        let mut engine = PlaybackEngine::new(excursion);
        let mut host = PlaybackWithExtensions {
            host: Host::default(),
            extensions: Extension::default(),
        };
        assert!(engine.advance(&mut host).is_ok());
        assert_eq!(host.host.0, 1);
        assert!(engine.advance(&mut host).is_ok());
        assert_eq!(engine.state(), PlaybackState::Waiting);
        assert!(engine.complete_pending(host.extensions.0.expect("token")));
        assert_eq!(engine.state(), PlaybackState::Finished);
    }
}
