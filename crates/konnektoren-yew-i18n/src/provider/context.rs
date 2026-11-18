use crate::{I18nConfig, SelectedLanguage};
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct I18nContext {
    pub config: UseStateHandle<I18nConfig>,
    pub selected_language: SelectedLanguage,
}

#[hook]
pub fn use_i18n() -> UseStateHandle<I18nConfig> {
    use_context::<I18nContext>()
        .expect("No I18n context provided")
        .config
}

#[hook]
pub fn use_selected_language() -> SelectedLanguage {
    use_context::<I18nContext>()
        .expect("No I18n context provided")
        .selected_language
}
