/// Colour of a pill, fact or figure; maps to daisyUI's semantic colours.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tone {
    #[default]
    Neutral,
    Good,
    Warn,
    Bad,
    Info,
}

impl Tone {
    /// `good`, `warn`, …; `None` for neutral.
    pub fn name(self) -> Option<&'static str> {
        match self {
            Self::Neutral => None,
            Self::Good => Some("good"),
            Self::Warn => Some("warn"),
            Self::Bad => Some("bad"),
            Self::Info => Some("info"),
        }
    }

    /// The BEM modifier class for `block`, e.g. `pill--good`; `None` for neutral.
    pub fn modifier(self, block: &str) -> Option<String> {
        self.name().map(|tone| format!("{block}--{tone}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modifiers() {
        assert_eq!(Tone::Good.modifier("pill").as_deref(), Some("pill--good"));
        assert_eq!(Tone::Neutral.modifier("pill"), None);
    }
}
