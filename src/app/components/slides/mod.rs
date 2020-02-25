
use log::info;
use stdweb::js;
use stdweb::web::document;
use stdweb::web::event::{IKeyboardEvent, KeyUpEvent};
use stdweb::web::Node;
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
    #[props(required)]
    pub number: usize,
}

pub struct SlidesModel {
    props: SlidesProps,
    router: Box<dyn Bridge<RouteAgent>>,
    link: ComponentLink<Self>,
    handler: KeyListenerHandle,
    ft: Option<FetchTask>,
    fetch_service: FetchService,
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
        let handler = KeyboardService::register_key_up(
            &document().body().unwrap(),
            link.callback(|x: KeyUpEvent| match x.key().as_str() {
                "ArrowLeft" => SlideMsg::Left,
                "ArrowRight" => SlideMsg::Right,
                _ => SlideMsg::NoOp,
            }),
        );
        SlidesModel {
            props,
            router: RouteAgent::bridge(link.callback(|_| SlideMsg::NoOp)),
            link,
            handler,
            data: None,
            ft: None,
            fetch_service: FetchService::new(),
            fetching: false,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        info!("Handler: {:?}", self.handler);
        self.ft = Some(self.fetch_slide(self.props.number));
        self.colorize(
            r#"
fn main() {
    let m = "Hello World!";
    println!("{}", m);
}"#,
        );
        false
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
                                "/slides?num={}",
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
}

impl SlidesModel {
    fn view_data(&self) -> Html {
        let v = if let Some(value) = &self.data {
            value
        } else {
            "<div><p>Data hasn't fetched yet.</p></div>"
        };
        let node = Node::from_html(v).expect("Could not generate Html object");
        VNode::VRef(node)
    }

    fn fetch_slide(&mut self, slide: usize) -> FetchTask {
        let callback =
            self.link
                .callback(move |response: Response<Result<String, failure::Error>>| {
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
        self.fetch_service.fetch(request, callback)
    }

    fn colorize(&self, code: &str) -> String {
        info!("Raw: {}", code);
        let raw_html = (js! {
            return Prism.highlight(@{code}, Prism.languages.rust);
        })
        .into_string();

        if let Some(res) = raw_html {
            info!("raw_html: {}", &res);
            res
        } else {
            "Hello non-js world!".to_string()
        }
    }
}
