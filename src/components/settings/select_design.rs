use crate::model::{Design, DesignMode};
use crate::providers::{use_design, use_design_mode};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SelectDesignProps {
    /// Fixed designs offered besides `Auto`. Defaults to Desktop + Mobile.
    #[prop_or_default]
    pub designs: Option<Vec<Design>>,
}

#[function_component(SelectDesign)]
pub fn select_design(props: &SelectDesignProps) -> Html {
    // Body class syncing and persistence are owned by DesignProvider; this
    // component only cycles the configured mode.
    let design = use_design();
    let mode = use_design_mode();

    let available_modes: Vec<DesignMode> = std::iter::once(DesignMode::Auto)
        .chain(
            props
                .designs
                .clone()
                .unwrap_or_else(|| vec![Design::Desktop, Design::Mobile])
                .into_iter()
                .map(DesignMode::Fixed),
        )
        .collect();

    let get_next_mode = move |current_mode: DesignMode| -> DesignMode {
        let current_index = available_modes
            .iter()
            .position(|m| m == &current_mode)
            .unwrap_or(0);

        let next_index = (current_index + 1) % available_modes.len();
        available_modes[next_index].clone()
    };

    let toggle_mode = {
        let mode = mode.clone();
        Callback::from(move |_| {
            let new_mode = get_next_mode((*mode).clone());
            mode.set(new_mode);
        })
    };

    let get_design_icon = |design: &Design| -> &str {
        match design {
            Design::Desktop => "fa-desktop",
            Design::Mobile => "fa-mobile-alt",
            Design::Other(_) => "fa-puzzle-piece",
        }
    };

    let get_mode_icon = |mode: &DesignMode, design: &Design| -> &str {
        match mode {
            DesignMode::Auto => get_design_icon(design),
            DesignMode::Fixed(design) => get_design_icon(design),
        }
    };

    let get_design_name = |design: &Design| -> String {
        match design {
            Design::Desktop => "Desktop Design".to_string(),
            Design::Mobile => "Mobile Design".to_string(),
            Design::Other(name) => {
                let name = name.strip_prefix("design-").unwrap_or(name);
                let mut chars = name.chars();
                let label = match chars.next() {
                    Some(first) => format!("{}{}", first.to_uppercase(), chars.as_str()),
                    None => "Custom".to_string(),
                };
                format!("{label} Design")
            }
        }
    };

    let get_mode_name = |mode: &DesignMode, design: &Design| -> String {
        match mode {
            DesignMode::Auto => format!("Auto ({})", get_design_name(design)),
            DesignMode::Fixed(design) => get_design_name(design),
        }
    };

    html! {
        <div class="select-design">
            <button class="select-design__button" onclick={toggle_mode}>
                <i class={classes!("select-design__icon", "fas", get_mode_icon(&mode, &design))}></i>
                <span class="select-design__label">{get_mode_name(&mode, &design)}</span>
            </button>
        </div>
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    yew_preview::create_preview!(
        SelectDesign,
        SelectDesignProps {
            designs: Some(vec![
                Design::Desktop,
                Design::Mobile,
                Design::Other("design-custom".to_string())
            ])
        },
        (
            "Desktop / Mobile",
            SelectDesignProps {
                designs: Some(vec![Design::Desktop, Design::Mobile])
            }
        )
    );
}
