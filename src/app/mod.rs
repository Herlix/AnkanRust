mod pages;

use crate::app::pages::home::HomeModel;
use crate::app::pages::slides::SlidesModel;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Switch, Clone)]
pub enum AppRoutes {
    #[to = "/slides/{num}"]
    Slides(usize),
    #[to = "/"]
    Home,
}

pub struct AppModel {}

impl Component for AppModel {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        AppModel {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="flex-wrapper">
                <div class="menu-wrapper">
                    { self.get_menu() }
                </div>
                <div class="main-wrapper">
                     { self.get_router() }
                </div>
            </div>
        }
    }
}

impl AppModel {
    fn get_menu(&self) -> Html {
        html! {
            <nav>
                <RouterButton<AppRoutes> route = AppRoutes::Home > { "Home" } </RouterButton<AppRoutes>>
                <RouterButton<AppRoutes> route = AppRoutes::Slides(0) > { "Slide" } </RouterButton<AppRoutes>>
            </nav>
        }
    }

    fn get_router(&self) -> Html {
        html! {
            <Router<AppRoutes, ()>
                render = Router::render(|switch: AppRoutes| match switch {
                    AppRoutes::Slides(num) => html! {<SlidesModel number = num /> },
                    AppRoutes::Home => html! { <HomeModel /> }
                })
            />
        }
    }
}
