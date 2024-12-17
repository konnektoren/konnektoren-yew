use super::SCALE;
use crate::components::ModelCoordinate;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct SvgPathProps {
    pub(crate) poly: Vec<ModelCoordinate>,
}

#[function_component(SvgPath)]
pub fn svg_path(props: &SvgPathProps) -> Html {
    if props.poly.is_empty() {
        return html! { <path /> };
    }
    let start_point = props.poly[0].to_svg(SCALE);
    let mut d = format!("M{} {}", start_point.0, start_point.1);

    for point in &props.poly[1..] {
        let svg_point = point.to_svg(SCALE);
        d.push_str(&format!(" L{} {}", svg_point.0, svg_point.1));
    }

    html! {
        <path
            d={d}
            fill="none"
            stroke="black"
            stroke-width="2"
        />
    }
}
