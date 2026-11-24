/// A fixed set of choices with stable values and translatable labels: a filter or a
/// sort order.
///
/// The [`Default`] choice is the one without filtering (for filters) or the standard
/// order (for sorts); "Clear filters" resets every filter to it.
pub trait HistoryOption: Copy + PartialEq + Default + 'static {
    /// Every choice, in the order shown.
    const ALL: &'static [Self];

    /// Stable value for forms and URLs; never translated.
    fn value(self) -> &'static str;

    /// Translation key shown to the player.
    fn label(self) -> &'static str;

    /// The choice with this [`value`](Self::value), or the default.
    fn parse(value: &str) -> Self {
        Self::ALL
            .iter()
            .copied()
            .find(|option| option.value() == value)
            .unwrap_or_default()
    }
}
