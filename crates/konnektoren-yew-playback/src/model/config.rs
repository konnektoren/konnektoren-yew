/// Defaults for playback presentation. Unset options use inline rendering.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PlaybackConfig {
    pub overlay_viewport: Option<bool>,
    pub notification_fixed: Option<bool>,
}
