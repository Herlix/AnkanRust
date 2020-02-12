mod data;

use data::PresentationData;
use log::info;
use stdweb::web::document;
use stdweb::web::event::{IKeyboardEvent, KeyUpEvent};
use yew::services::keyboard::{KeyListenerHandle, KeyboardService};
use yew::{html, Bridge, Bridged, Component, ComponentLink, Html, Properties, ShouldRender};
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;

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
    data: PresentationData,
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
        let callback = link.callback(|_| SlideMsg::NoOp);
        let router = RouteAgent::bridge(callback);
        info!("Smooth: {:?}", props.number);

        let handler = KeyboardService::register_key_up(
            &document().body().unwrap(),
            link.callback(|x: KeyUpEvent| match x.key().as_str() {
                "ArrowLeft" => SlideMsg::Left,
                "ArrowRight" => SlideMsg::Right,
                _ => SlideMsg::NoOp,
            }),
        );
        let data = PresentationData::new();
        SlidesModel {
            props,
            router,
            link,
            handler,
            data,
        }
    }

    fn mounted(&mut self) -> ShouldRender {
        info!("Handler: {:?}", self.handler);
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            SlideMsg::Left => {
                if self.props.number > 0 {
                    self.props.number = self.props.number - 1;
                }
            }
            SlideMsg::Right => {
                if self.props.number < self.data.len() - 1 {
                    self.props.number = self.props.number + 1;
                }
            }
            SlideMsg::NoOp => return false,
        }

        // Don't tell the router to alert its subscribers,
        // because the changes made here only affect the current component,
        // so mutation might as well be contained to the core component update loop
        // instead of being sent through the router.
        self.router
            .send(RouteRequest::ChangeRouteNoBroadcast(Route::from(format!(
                "/slides?num={}",
                self.props.number
            ))));
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="slides-wrapper">
                <div class="slides-navigation">
                    <button class="left" onclick=self.link.callback(|_| SlideMsg::Left)>
                        <i class="arrow arrow-left"></i>
                    </button>
                    <div class="center">
                   { self.props.number }
                    </div>
                    <button class="right" onclick=self.link.callback(|_| SlideMsg::Right)>
                        <i class="arrow arrow-right"></i>
                    </button>
                </div>
                <div class="slides-content">
                   { self.data.get_content(&self.props.number) }
                </div>
            </div>
        }
    }
}
