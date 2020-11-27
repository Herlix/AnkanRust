use crate::{
    pages::home::HomeModel, pages::not_found::PageNotFound, pages::slides::SlidesModel,
    switch::UrlSwitch,
};
use crate::{
    slides_data::SLIDES,
    switch::{AppAnchor, AppRoute, AppRouter},
};
use yew::{prelude::*, services::ConsoleService};
use yew_router::{prelude::*, switch::Permissive};

pub enum Msg {
    ToggleNavbar,
}

pub struct AppModel {
    link: ComponentLink<Self>,
    navbar_active: bool,
}

impl Component for AppModel {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        AppModel {
            link,
            navbar_active: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleNavbar => {
                self.navbar_active = !self.navbar_active;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.get_nav() }
                { self.get_main() }
                { self.get_footer() }
            </>
        }
    }
}

impl AppModel {
    fn get_nav(&self) -> Html {
        let Self {
            ref link,
            navbar_active,
            ..
        } = *self;

        let active_class = if navbar_active { "is-active" } else { "" };

        let pages: Vec<Html> = SLIDES
            .iter()
            .map(|x| {
                html! {
                    <a class="navbar-item">
                    <AppAnchor classes="navbar-item" route=AppRoute::SlidesName(x.slug.to_string())>
                        { x.title }
                    </AppAnchor>
                </a>
                }
            })
            .collect();

        html! {
            <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
                <div class="navbar-brand">
                <AppAnchor classes="navbar-item is-size-3" route=AppRoute::Home>
                    {"RustForLife"}
                </AppAnchor>

                    <a role="button"
                        class=("navbar-burger", "burger", active_class)
                        aria-label="menu" aria-expanded="false"
                        onclick=link.callback(|_| Msg::ToggleNavbar)
                    >
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                        <span aria-hidden="true"></span>
                    </a>
                </div>
                <div class=("navbar-menu", active_class)>
                    <div class="navbar-start">
                        <AppAnchor classes="navbar-item" route=AppRoute::Home>
                            { "Home" }
                        </AppAnchor>

                        <div class="navbar-item has-dropdown is-hoverable">
                            <a class="navbar-link">
                                { "Slides" }
                            </a>
                            <div class="navbar-dropdown">
                                { for pages }
                            </div>
                        </div>
                    </div>
                </div>
            </nav>
        }
    }

    fn get_main(&self) -> Html {
        html! {
            <AppRouter
                render=AppRouter::render(Self::switch)
                redirect=AppRouter::redirect(|route: Route| {
                    AppRoute::PageNotFound(Permissive(Some(route.route))).into_switch()
                })
            />
        }
    }

    fn get_footer(&self) -> Html {
        html! {
            <footer class="footer">
                <div class="content has-text-centered">
                    { "Powered by " }
                    <a href="https://yew.rs">{ "Yew" }</a>
                    { " using " }
                    <a href="https://bulma.io">{ "Bulma" }</a>
                    { " and images from " }
                    <a href="https://unsplash.com">{ "Unsplash" }</a>
                </div>
            </footer>
        }
    }

    fn switch(switch: UrlSwitch) -> Html {
        match switch.route() {
            AppRoute::Home => {
                ConsoleService::info("Switch to home".into());
                html! { <HomeModel/> }
            }
            AppRoute::SlidesNumber(n) => {
                ConsoleService::info("SlidesNumber");
                if let Some(v) = SLIDES.get(n) {
                    html! { <SlidesModel slide=v /> }
                } else {
                    html! { <PageNotFound route=Some(format!("/slides/{}", n)) /> }
                }
            }
            AppRoute::SlidesName(n) => {
                ConsoleService::info("SlidesName");
                if let Some(v) = SLIDES.iter().find(|x| x.slug.to_string() == n) {
                    html! { <SlidesModel slide=v /> }
                } else {
                    html! { <PageNotFound route=Some(format!("/slides/{}", n)) /> }
                }
            }
            AppRoute::PageNotFound(Permissive(route)) => {
                ConsoleService::info("NotFound");
                html! { <PageNotFound route=route /> }
            }
        }
    }
}
