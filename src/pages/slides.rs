use log::info;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::{KeyboardEvent, Node};
use yew::services::fetch::{FetchTask, Request, Response};
use yew::services::keyboard::{KeyListenerHandle, KeyboardService};
use yew::services::FetchService;
use yew::virtual_dom::VNode;
use yew::{
    format::Nothing, html, Bridge, Bridged, Component, ComponentLink, Html, Properties,
    ShouldRender,
};
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;

const MAX_COUNT: usize = 29;

#[derive(Clone, PartialEq, Properties)]
pub struct SlidesProps {
    pub number: usize,
}

pub struct SlidesModel {
    props: SlidesProps,
    router: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    handler: KeyListenerHandle,
    ft: Option<Result<FetchTask, anyhow::Error>>,
    fetching: bool,
    data: Option<String>,
}

pub enum SlideMsg {
    Right,
    Left,
    NoOp,
    FetchReady(Option<String>),
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

        let handler = KeyboardService::register_key_press(&document, callback);

        let mut res = SlidesModel {
            props,
            router: RouteAgent::bridge(link.callback(|_| SlideMsg::NoOp)),
            link,
            handler,
            data: None,
            ft: None,
            fetching: false,
        };
        res.ft = Some(res.fetch_slide(res.props.number));
        res
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            SlideMsg::FetchReady(res) => {
                self.fetching = false;
                match res {
                    Some(s) => {
                        self.data = Some(s);
                        self.router
                            .send(RouteRequest::ChangeRouteNoBroadcast(Route::from(format!(
                                "/slides/{}",
                                self.props.number
                            ))));
                    }
                    None => {}
                }
            }
            SlideMsg::Left => {
                if self.props.number > 0 {
                    self.props.number = self.props.number - 1;
                }
                self.fetching = true;
                self.ft = Some(self.fetch_slide(self.props.number));
            }
            SlideMsg::Right => {
                if self.props.number != MAX_COUNT {
                    self.props.number = self.props.number + 1;
                    self.fetching = true;
                    self.ft = Some(self.fetch_slide(self.props.number));
                }
            }
            SlideMsg::NoOp => return false,
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="slides--wrapper">
                { self.view_data() }
                <div class="slides-navigation">
                    <button disabled={ self.props.number == 0 } class="left" onclick=self.link.callback(|_| SlideMsg::Left) style={if self.props.number == 0 {"visibility:hidden;"} else {"visibility:visible;"}}>
                        <i  class="arrow arrow-left"></i>
                    </button>
                    <button disabled={ self.props.number == MAX_COUNT } class="right" onclick=self.link.callback(|_| SlideMsg::Right) style={if self.props.number == MAX_COUNT {"visibility:hidden;"} else {"visibility:visible;"}}>
                        <i class="arrow arrow-right"></i>
                    </button>
                </div>
            </div>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        self.colorize(
            r#"
        fn main() {
            let m = "Hello World!";
            println!("{}", m);
        }"#,
        );

        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }
}

impl SlidesModel {
    fn view_data(&self) -> Html {
        let v = if let Some(value) = &self.data {
            value
        } else {
            "<div><p>Data hasn't fetched yet.</p></div>"
        };

        let js_svg = {
            let div = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("div")
                .unwrap();
            div.set_inner_html(v);
            div
        };

        VNode::VRef(Node::from(js_svg))
    }

    fn fetch_slide(&mut self, slide: usize) -> Result<FetchTask, anyhow::Error> {
        let callback =
            self.link
                .callback(move |response: Response<Result<String, anyhow::Error>>| {
                    let (meta, data) = response.into_parts();
                    // info!("META: {:?}, {:?}", meta, data);
                    if meta.status.is_success() {
                        SlideMsg::FetchReady(Some(data.unwrap()))
                    } else {
                        SlideMsg::FetchReady(None)
                    }
                });
        let request = Request::get(format!("/api/slide/page_{}.html", slide))
            .body(Nothing)
            .unwrap();
        FetchService::fetch(request, callback)
    }

    fn colorize(&self, code: &str) -> String {
        info!("Raw: {}", code);
        let m = unsafe { highlight(code, "rust") };
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
    #[wasm_bindgen(js_namespace = Prism)]
    fn highlight(html: &str, lang: &str) -> JsValue;
}
