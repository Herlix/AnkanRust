use yew::prelude::*;

pub struct HomeModel {}
pub enum HomeMsg {}

impl Component for HomeModel {
    type Message = HomeMsg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        HomeModel {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
             <div class="home-wrapper">
                <div class="home-content">
                    <img src="/images/me.png" alt="Portrait" class="main-image"/>
                    <h1>{"Alexander Herlin"}</h1>
                </div>
            </div>
        }
    }
}
