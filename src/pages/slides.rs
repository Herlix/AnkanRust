use log::info;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::KeyboardEvent;
use yew::services::keyboard::{KeyListenerHandle, KeyboardService};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::{agent::RouteRequest, prelude::*};

use crate::{
    slides_data::{Slide, SLIDES},
    switch::AppRoute,
};

#[derive(Clone, PartialEq, Properties)]
pub struct SlidesProps {
    pub slide: Slide<'static>,
}

pub struct SlidesModel {
    props: SlidesProps,
    number: usize,
    route_dispatcher: RouteAgentDispatcher,
    link: ComponentLink<Self>,
    keyboard_handle: KeyListenerHandle,
}

pub enum SlideMsg {
    Right,
    Left,
    NoOp,
}

impl Component for SlidesModel {
    type Message = SlideMsg;
    type Properties = SlidesProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let window: web_sys::Window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        let callback = link.callback(|x: KeyboardEvent| match x.key().as_str() {
            "ArrowLeft" => SlideMsg::Left,
            "ArrowRight" => SlideMsg::Right,
            _ => SlideMsg::NoOp,
        });

        let keyboard_handle = KeyboardService::register_key_press(&document, callback);
        let number = SLIDES
            .iter()
            .position(|x| x.slug == props.slide.slug)
            .unwrap_or(0);
        let mut result = SlidesModel {
            props,
            number,
            route_dispatcher: RouteAgentDispatcher::new(),
            link,
            keyboard_handle,
        };
        result.update_route();
        result
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            SlideMsg::Left => {
                if self.number > 0 {
                    self.number = self.number - 1;
                    self.props.slide = SLIDES.get(self.number).unwrap().clone();
                    self.update_route();
                }
                true
            }
            SlideMsg::Right => {
                if self.number != SLIDES.len() {
                    self.number = self.number + 1;
                    self.props.slide = SLIDES.get(self.number).unwrap().clone();
                    self.update_route();
                }
                true
            }
            SlideMsg::NoOp => false,
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
            </>
            // <div class="slides--wrapper">
            //     { self.view_data() }
            //     <div class="slides-navigation">
            //         <button disabled={ self.props.number == 0 } class="left" onclick=self.link.callback(|_| SlideMsg::Left) style={if self.props.number == 0 {"visibility:hidden;"} else {"visibility:visible;"}}>
            //             <i  class="arrow arrow-left"></i>
            //         </button>
            //         <button disabled={ self.props.number == MAX_COUNT } class="right" onclick=self.link.callback(|_| SlideMsg::Right) style={if self.props.number == MAX_COUNT {"visibility:hidden;"} else {"visibility:visible;"}}>
            //             <i class="arrow arrow-right"></i>
            //         </button>
            //     </div>
            // </div>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn rendered(&mut self, _first_render: bool) {}

    fn destroy(&mut self) {}
}

impl SlidesModel {
    fn update_route(&mut self) {
        let route = AppRoute::SlidesName(self.props.slide.slug.to_string());
        self.route_dispatcher
            .send(RouteRequest::ChangeRoute(route.into_route()));
    }
    fn colorize(&self, code: &str) -> String {
        let m = highlight(code);
        if let Some(res) = m.as_string() {
            info!("raw_html: {}", &res);
            res
        } else {
            "Hello non-js world!".to_string()
        }
    }
}

#[wasm_bindgen]
extern "C" {
    fn highlight(html: &str) -> JsValue;
}
