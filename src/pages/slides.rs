use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::KeyboardEvent;
use yew::{
    format::Nothing,
    services::{
        fetch::{FetchTask, Request, Response},
        keyboard::{KeyListenerHandle, KeyboardService},
        ConsoleService, FetchService,
    },
};
use yew::{html, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::{agent::RouteRequest, prelude::*};

use crate::{slides_data::Slide, slides_data::SLIDES, switch::AppRoute};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SlidesProps {
    pub id: SlideId,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SlideId {
    Str(String),
    Num(usize),
}

pub enum Direction {
    Back,
    Forward,
}

pub enum Move {
    ById(SlideId),
    ByDirection(Direction),
}

pub enum Msg {
    FetchMarkdown(Move),
    ReceiveResponse(Result<String, anyhow::Error>),
    NoOp,
}

/// Have a look at:
/// https://github.com/yewstack/yew/blob/66d506e13329a06f7dce0b55a9427972b8aad3ff/docs/concepts/services/fetch.md
#[derive(Debug)]
pub struct SlidesModel {
    _keyboard_handle: Option<KeyListenerHandle>,
    props: SlidesProps,
    route_dispatcher: RouteAgentDispatcher,
    link: ComponentLink<Self>,
    fetch_task: Option<FetchTask>,
    fetch_error: Option<String>,
    markdown: Option<String>,
    slide: &'static Slide<'static>,
    number: usize,
}

impl Component for SlidesModel {
    type Message = Msg;
    type Properties = SlidesProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        ConsoleService::log("New");
        let callback = link.callback(|event: KeyboardEvent| match event.key().as_str() {
            "ArrowLeft" => Msg::FetchMarkdown(Move::ByDirection(Direction::Back)),
            "ArrowRight" => Msg::FetchMarkdown(Move::ByDirection(Direction::Forward)),
            _ => Msg::NoOp,
        });

        let keyboard_handle = KeyboardService::register_key_down(&yew::utils::document(), callback);

        let (slide, number) = Self::get_slide(&props.id);
        let mut res = SlidesModel {
            fetch_task: None,
            fetch_error: None,
            markdown: None,
            props,
            number,
            slide,
            route_dispatcher: RouteAgentDispatcher::new(),
            link,
            _keyboard_handle: Some(keyboard_handle),
        };
        res.fetch();
        res.update_route();
        res
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        ConsoleService::log("Update");
        use Msg::*;
        match msg {
            FetchMarkdown(mv) => {
                let (slide, num) = match mv {
                    Move::ByDirection(dir) => self.get_next(dir),
                    Move::ById(id) => Self::get_slide(&id),
                };
                self.number = num;
                self.slide = slide;

                self.fetch();
                // we want to redraw so that the page displays a 'fetching...' message to the user
                // so return 'true'
                true
            }
            ReceiveResponse(response) => {
                match response {
                    Ok(md) => {
                        self.markdown = Some(md);
                    }
                    Err(error) => self.fetch_error = Some(error.to_string()),
                }
                self.fetch_task = None;
                true
            }
            NoOp => false,
        }
    }

    fn view(&self) -> Html {
        html! {
            <>
                <p>{ self.slide.title }</p>
                {self.view_md()}
                { self.view_fetching() }
                { self.view_error() }
                {self.colorize("println!(\"Hello world\"!);")}
            </>


            // <div class="slides--wrapper">
            //     { self.view_data() }
            //     <div class="slides-navigation">
            //         <button disabled={ self.props.number == 0 } class="left" onclick=self.link.callback(|_| Msg::Left) style={if self.props.number == 0 {"visibility:hidden;"} else {"visibility:visible;"}}>
            //             <i  class="arrow arrow-left"></i>
            //         </button>
            //         <button disabled={ self.props.number == MAX_COUNT } class="right" onclick=self.link.callback(|_| Msg::Right) style={if self.props.number == MAX_COUNT {"visibility:hidden;"} else {"visibility:visible;"}}>
            //             <i class="arrow arrow-right"></i>
            //         </button>
            //     </div>
            // </div>
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props && self.fetch_task.is_none() {
            let (slide, number) = Self::get_slide(&props.id);
            self.props = props;
            self.slide = slide;
            self.number = number;
            self.fetch();
            self.update_route();
            true
        } else {
            false
        }
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
            res
        } else {
            "Hello non-js world!".to_string()
        }
    }

    fn get_next(&self, dir: Direction) -> (&'static Slide<'static>, usize) {
        use Direction::*;
        match dir {
            Back => {
                if self.number > 0 {
                    let num = self.number - 1;
                    (SLIDES.get(num).unwrap(), num)
                } else {
                    (self.slide, self.number)
                }
            }
            Forward => {
                if self.number < SLIDES.len() - 1 {
                    let num = self.number + 1;
                    (SLIDES.get(num).unwrap(), num)
                } else {
                    (self.slide, self.number)
                }
            }
        }
    }

    fn get_slide(id: &SlideId) -> (&'static Slide<'static>, usize) {
        let number = match id {
            SlideId::Str(s) => SLIDES.iter().position(|x| x.slug == s).unwrap_or(0),
            SlideId::Num(n) => {
                if n >= &SLIDES.len() {
                    0
                } else {
                    n.clone()
                }
            }
        };
        (SLIDES.get(number).unwrap(), number)
    }

    fn fetch(&mut self) {
        // 1. build the request
        let request = Request::get(self.slide.url)
            .body(Nothing)
            .expect("Could not build request.");
        // 2. construct a callback
        let callback = self
            .link
            .callback(|response: Response<Result<String, anyhow::Error>>| {
                Msg::ReceiveResponse(response.into_body())
            });
        // 3. pass the request and callback to the fetch service
        let task = FetchService::fetch(request, callback).expect("failed to start request");
        // 4. store the task so it isn't canceled immediately
        self.fetch_task = Some(task);
    }

    fn view_md(&self) -> Html {
        let res = self
            .markdown
            .clone()
            .unwrap_or("Data not loaded...".to_string());
        html! {<p> { res } </p>}
    }

    fn view_fetching(&self) -> Html {
        if self.fetch_task.is_some() {
            html! { <p>{ "Fetching data..." }</p> }
        } else {
            html! { <p></p> }
        }
    }

    fn view_error(&self) -> Html {
        if let Some(ref error) = self.fetch_error {
            html! { <p>{ error.clone() }</p> }
        } else {
            html! {}
        }
    }
}

#[wasm_bindgen]
extern "C" {
    fn highlight(html: &str) -> JsValue;
}
