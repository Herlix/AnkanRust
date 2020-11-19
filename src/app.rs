use crate::pages::home::HomeModel;
use crate::pages::slides::SlidesModel;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Switch, Clone)]
enum AppRoute {
    #[to = "/slides/{n}"]
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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="wrapper">
                <div class="wrapper__menu">
                    <nav>
                        <RouterButton<AppRoute> route = AppRoute::Home > { "Home" } </RouterButton<AppRoute>>
                        <RouterButton<AppRoute> route = AppRoute::Slides(0) > { "Slide" } </RouterButton<AppRoute>>
                    </nav>
                </div>
                <div class="wrapper__content">
                     { self.get_router() }
                </div>
            </div>
        }
    }
}

impl AppModel {
    fn get_router(&self) -> Html {
        html! {
            <Router<AppRoute>
                render = Router::render(|switch: AppRoute| match switch {
                    AppRoute::Slides(num) => html! {<SlidesModel number = num /> },
                    _ => html! { <HomeModel /> },
                })
                redirect = Router::redirect(|_: Route| {
                    AppRoute::Home
                })
            />
        }
    }
}
