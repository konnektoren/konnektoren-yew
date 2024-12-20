use crate::model::Design;
use crate::providers::use_design;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SelectDesignProps {
    #[prop_or_default]
    pub designs: Option<Vec<Design>>,
}

#[function_component(SelectDesign)]
pub fn select_design(props: &SelectDesignProps) -> Html {
    let design = use_design();

    let update_design_class = |new_design: &Design| {
        if let Some(body) = gloo::utils::document().body() {
            let current_classes: Vec<String> = body
                .class_name()
                .split_whitespace()
                .filter(|class| !class.starts_with("design-"))
                .map(String::from)
                .collect();

            let mut classes = current_classes;
            classes.push(new_design.as_class().to_string());

            body.set_class_name(&classes.join(" "));
        }
    };

    {
        let design = design.clone();
        use_effect(move || {
            update_design_class(&*design);
            || ()
        });
    }

    let available_designs = props
        .designs
        .clone()
        .unwrap_or_else(|| vec![Design::Desktop, Design::Mobile]);

    let get_next_design = move |current_design: Design| -> Design {
        let current_index = available_designs
            .iter()
            .position(|d| d == &current_design)
            .unwrap_or(0);

        let next_index = (current_index + 1) % available_designs.len();
        available_designs[next_index].clone()
    };

    let toggle_design = {
        let design = design.clone();
        Callback::from(move |_| {
            let new_design = get_next_design((*design).clone());
            design.set(new_design);
        })
    };

    let get_design_icon = |design: &Design| -> &str {
        match design {
            Design::Desktop => "fa-desktop",
            Design::Mobile => "fa-mobile-alt",
            Design::Other(_) => "fa-puzzle-piece",
        }
    };

    let get_design_name = |design: &Design| -> String {
        match design {
            Design::Desktop => "Desktop Design".to_string(),
            Design::Mobile => "Mobile Design".to_string(),
            Design::Other(name) => format!(
                "{} Design",
                name.strip_prefix("design-")
                    .unwrap_or(name)
                    .chars()
                    .next()
                    .unwrap()
                    .to_uppercase()
                    .chain(name.strip_prefix("design-").unwrap_or(name).chars().skip(1))
                    .collect::<String>()
            ),
        }
    };

    html! {
        <div class="select-design">
            <button onclick={toggle_design}>
                <i class={classes!("fas", get_design_icon(&design))}></i>
                <span>{get_design_name(&design)}</span>
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
