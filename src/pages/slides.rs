use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::KeyboardEvent;
use yew::services::{
    keyboard::{KeyListenerHandle, KeyboardService},
    ConsoleService,
};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::{agent::RouteRequest, prelude::*};

use crate::{slides_data::Slide, slides_data::SLIDES, switch::AppRoute};

#[derive(Clone, PartialEq)]
pub enum SlideId {
    Str(String),
    Num(usize),
}
#[derive(Clone, PartialEq, Properties)]
pub struct SlidesProps {
    pub slide: SlideId,
}

pub struct SlidesModel {
    slide: &'static Slide<'static>,
    number: usize,
    route_dispatcher: RouteAgentDispatcher,
    _link: ComponentLink<Self>,
    //_keyboard_handle: KeyListenerHandle,
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
        ConsoleService::log("Got into slide");

        // let window: web_sys::Window = web_sys::window().unwrap();
        // let document = window.document().unwrap();

        // let callback = link.callback(|x: KeyboardEvent| match x.key().as_str() {
        //     "ArrowLeft" => SlideMsg::Left,
        //     "ArrowRight" => SlideMsg::Right,
        //     _ => SlideMsg::NoOp,
        // });

        // let keyboard_handle = KeyboardService::register_key_press(&document, callback);

        let number = match &props.slide {
            SlideId::Str(s) => SLIDES.iter().position(|x| x.slug == s).unwrap_or(0),
            SlideId::Num(n) => {
                if n >= &SLIDES.len() {
                    0
                } else {
                    n.clone()
                }
            }
        };

        let mut result = SlidesModel {
            number,
            slide: SLIDES.get(number).unwrap(),
            route_dispatcher: RouteAgentDispatcher::new(),
            _link: link,
            // _keyboard_handle: keyboard_handle,
        };
        // result.update_route();
        result
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            SlideMsg::Left => {
                if self.number > 0 {
                    self.number -= 1;
                    self.slide = SLIDES.get(self.number).unwrap();
                    self.update_route();
                }
                true
            }
            SlideMsg::Right => {
                if self.number != SLIDES.len() {
                    self.number += 1;
                    self.slide = SLIDES.get(self.number).unwrap();
                    self.update_route();
                }
                true
            }
            SlideMsg::NoOp => false,
        }
    }

    fn view(&self) -> Html {
        let images: Vec<Html> = self
            .slide
            .images
            .iter()
            .map(|x| {
                html! {
                    <img src={x.path} alt={x.alt} class="ferris-main" />
                }
            })
            .collect();
        html! {
            <>
                <p>{ self.slide.title }</p>
                <p>{ self.slide.text.unwrap_or("Nothing to see here") }</p>
                {for images}

                {self.colorize("println!(\"Hello world\"!);")}
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
        true
    }
}

impl SlidesModel {
    fn update_route(&mut self) {
        let route = AppRoute::SlidesName(self.slide.slug.to_string());
        self.route_dispatcher
            .send(RouteRequest::ChangeRoute(route.into_route()));
    }
    fn colorize(&self, code: &str) -> String {
        let m = highlight(code);
        if let Some(res) = m.as_string() {
            ConsoleService::info(format!("raw_html: {}", &res).as_str());
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
