use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum SwipeDirection {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Properties, PartialEq)]
pub struct SwipeProps {
    pub children: Children,
    #[prop_or_default]
    pub on_swipe: Callback<SwipeDirection>,
    #[prop_or_default]
    pub left_icon: Option<String>,
    #[prop_or_default]
    pub right_icon: Option<String>,
    #[prop_or_default]
    pub up_icon: Option<String>,
    #[prop_or_default]
    pub down_icon: Option<String>,
    #[prop_or_default]
    pub show_hints: bool,
}

pub struct SwipeComponent {
    drag_start: Option<(i32, i32)>,
    current_position: (i32, i32),
    node_ref: NodeRef,
    #[cfg(feature = "csr")]
    _listeners: Vec<gloo::events::EventListener>,
}

pub enum SwipeMsg {
    DragStart(i32, i32),
    DragMove(i32, i32),
    DragEnd,
    ClickZone(SwipeDirection),
}

impl Component for SwipeComponent {
    type Message = SwipeMsg;
    type Properties = SwipeProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            drag_start: None,
            current_position: (0, 0),
            node_ref: NodeRef::default(),
            #[cfg(feature = "csr")]
            _listeners: Vec::new(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = format!(
            "transform: translate({}px, {}px)",
            self.current_position.0, self.current_position.1
        );

        let onmousedown = ctx.link().callback(|e: MouseEvent| {
            e.prevent_default();
            SwipeMsg::DragStart(e.client_x(), e.client_y())
        });

        let ontouchstart = ctx.link().callback(|e: TouchEvent| {
            #[cfg(feature = "csr")]
            {
                use wasm_bindgen::JsCast;
                use web_sys::TouchEvent;
                if let Ok(event) = e.dyn_into::<TouchEvent>() {
                    if let Some(touch) = event.touches().get(0) {
                        return SwipeMsg::DragStart(touch.client_x(), touch.client_y());
                    }
                }
            }
            SwipeMsg::DragEnd
        });

        let on_click_left = ctx
            .link()
            .callback(|_| SwipeMsg::ClickZone(SwipeDirection::Left));
        let on_click_right = ctx
            .link()
            .callback(|_| SwipeMsg::ClickZone(SwipeDirection::Right));
        let on_click_up = ctx
            .link()
            .callback(|_| SwipeMsg::ClickZone(SwipeDirection::Up));
        let on_click_down = ctx
            .link()
            .callback(|_| SwipeMsg::ClickZone(SwipeDirection::Down));

        html! {
            <div class="swipe">
                if ctx.props().show_hints {
                    <div class="swipe__hints">
                        if let Some(left_icon) = &ctx.props().left_icon {
                            <div class="swipe__hint swipe__hint--left" onclick={on_click_left}>
                                <i class={left_icon.clone()} />
                            </div>
                        }
                        if let Some(right_icon) = &ctx.props().right_icon {
                            <div class="swipe__hint swipe__hint--right" onclick={on_click_right}>
                                <i class={right_icon.clone()} />
                            </div>
                        }
                        if let Some(up_icon) = &ctx.props().up_icon {
                            <div class="swipe__hint swipe__hint--up" onclick={on_click_up}>
                                <i class={up_icon.clone()} />
                            </div>
                        }
                        if let Some(down_icon) = &ctx.props().down_icon {
                            <div class="swipe__hint swipe__hint--down" onclick={on_click_down}>
                                <i class={down_icon.clone()} />
                            </div>
                        }
                    </div>
                }
                <div
                    ref={self.node_ref.clone()}
                    class="swipe__content"
                    style={style}
                    {onmousedown}
                    {ontouchstart}
                >
                    { for ctx.props().children.iter() }
                </div>
            </div>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SwipeMsg::DragStart(x, y) => {
                self.drag_start = Some((x, y));
                true
            }
            SwipeMsg::DragMove(x, y) => {
                if let Some((start_x, start_y)) = self.drag_start {
                    self.current_position = (x - start_x, y - start_y);
                    true
                } else {
                    false
                }
            }
            SwipeMsg::DragEnd => {
                if let Some((start_x, start_y)) = self.drag_start {
                    let (end_x, end_y) = self.current_position;
                    let dx = end_x - start_x;
                    let dy = end_y - start_y;

                    // Determine swipe direction based on the larger movement
                    if dx.abs() > dy.abs() && dx.abs() > 50 {
                        if dx > 0 {
                            ctx.props().on_swipe.emit(SwipeDirection::Right);
                        } else {
                            ctx.props().on_swipe.emit(SwipeDirection::Left);
                        }
                    } else if dy.abs() > dx.abs() && dy.abs() > 50 {
                        if dy > 0 {
                            ctx.props().on_swipe.emit(SwipeDirection::Down);
                        } else {
                            ctx.props().on_swipe.emit(SwipeDirection::Up);
                        }
                    }
                }
                self.drag_start = None;
                self.current_position = (0, 0);
                true
            }
            SwipeMsg::ClickZone(direction) => {
                ctx.props().on_swipe.emit(direction);
                true
            }
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        #[cfg(feature = "csr")]
        {
            use gloo::events::EventListener;
            use wasm_bindgen::JsCast;
            use web_sys::{Document, Element, MouseEvent, TouchEvent};

            if first_render {
                if let Some(element) = self.node_ref.cast::<Element>() {
                    let document = web_sys::window().unwrap().document().unwrap();

                    // Mouse events
                    let mousemove = {
                        let link = ctx.link().clone();
                        EventListener::new(&document, "mousemove", move |event| {
                            let event = event.dyn_ref::<MouseEvent>().unwrap();
                            link.send_message(SwipeMsg::DragMove(
                                event.client_x(),
                                event.client_y(),
                            ));
                        })
                    };

                    let mouseup = {
                        let link = ctx.link().clone();
                        EventListener::new(&document, "mouseup", move |_| {
                            link.send_message(SwipeMsg::DragEnd);
                        })
                    };

                    // Touch events
                    let touchmove = {
                        let link = ctx.link().clone();
                        EventListener::new(&element, "touchmove", move |event| {
                            let event = event.dyn_ref::<TouchEvent>().unwrap();
                            if let Some(touch) = event.touches().get(0) {
                                link.send_message(SwipeMsg::DragMove(
                                    touch.client_x(),
                                    touch.client_y(),
                                ));
                            }
                        })
                    };

                    let touchend = {
                        let link = ctx.link().clone();
                        EventListener::new(&element, "touchend", move |_| {
                            link.send_message(SwipeMsg::DragEnd);
                        })
                    };

                    self._listeners = vec![mousemove, mouseup, touchmove, touchend];
                }
            }
        }
    }
}

#[cfg(feature = "yew-preview")]
mod preview {
    use super::*;
    use yew_preview::prelude::*;

    #[function_component(PreviewContent)]
    fn preview_content() -> Html {
        html! {
            <div style="width: 200px; height: 200px; background: #f0f0f0; display: flex; align-items: center; justify-content: center;">
                {"Swipe me!"}
            </div>
        }
    }

    yew_preview::create_preview!(
        SwipeComponent,
        SwipeProps {
            children: Children::new(vec![html! { <PreviewContent /> }]),
            on_swipe: Callback::from(|_| ()),
            left_icon: Some("fas fa-times".to_string()),
            right_icon: Some("fas fa-heart".to_string()),
            up_icon: None,
            down_icon: None,
            show_hints: true
        },
        (
            "With All Directions",
            SwipeProps {
                children: Children::new(vec![html! { <PreviewContent /> }]),
                on_swipe: Callback::from(|_| ()),
                left_icon: Some("fas fa-times".to_string()),
                right_icon: Some("fas fa-heart".to_string()),
                up_icon: Some("fas fa-arrow-up".to_string()),
                down_icon: Some("fas fa-arrow-down".to_string()),
                show_hints: true
            }
        ),
        (
            "Without Hints",
            SwipeProps {
                children: Children::new(vec![html! { <PreviewContent /> }]),
                on_swipe: Callback::from(|_| ()),
                left_icon: Some("fas fa-times".to_string()),
                right_icon: Some("fas fa-heart".to_string()),
                up_icon: None,
                down_icon: None,
                show_hints: false
            }
        )
    );
}
