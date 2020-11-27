use web_sys::Url;
use yew::{
    services::ConsoleService,
    virtual_dom::{Transformer, VComp},
};
use yew_router::{prelude::*, switch::Permissive};

#[derive(Debug, Switch, Clone)]
pub enum AppRoute {
    #[to = "/slides/{num}"]
    SlidesNumber(usize),
    #[to = "/slides/{name}"]
    SlidesName(String),
    #[to = "/page-not-found"]
    PageNotFound(Permissive<String>),
    #[to = "/!"]
    Home,
}

impl AppRoute {
    pub fn into_switch(self) -> UrlSwitch {
        UrlSwitch(self)
    }

    pub fn into_route(self) -> Route {
        Route::from(self.into_switch())
    }
}

/// Helper type which wraps the AppRoute enum but handles prefixes.
/// It makes it possible to host this sight in other locations such as '/foo/'
/// instead of simply '/'
#[derive(Debug, Clone)]
pub struct UrlSwitch(AppRoute);

impl UrlSwitch {
    fn base_url() -> Url {
        if let Ok(Some(href)) = yew::utils::document().base_uri() {
            Url::new(&href).unwrap()
        } else {
            Url::new("/").unwrap()
        }
    }
    fn base_path() -> String {
        let mut path = Self::base_url().pathname();
        if path.ends_with('/') {
            // pop the trailing slash because AppRoute already accounts for it
            path.pop();
        }
        path
    }
    pub fn route(self) -> AppRoute {
        self.0
    }
}

impl Transformer<AppRoute, UrlSwitch> for VComp {
    fn transform(from: AppRoute) -> UrlSwitch {
        from.into_switch()
    }
}

impl Switch for UrlSwitch {
    fn from_route_part<STATE>(part: String, state: Option<STATE>) -> (Option<Self>, Option<STATE>) {
        ConsoleService::log(format!("Switch: from_route_part {}", part).as_str());
        if let Some(part) = part.strip_prefix(&Self::base_path()) {
            let (route, state) = AppRoute::from_route_part(part.to_owned(), state);
            (route.map(Self), state)
        } else {
            (None, None)
        }
    }

    fn build_route_section<STATE>(self, route: &mut String) -> Option<STATE> {
        ConsoleService::log(format!("Adding to path {}", route).as_str());
        route.push_str(&Self::base_path());
        self.0.build_route_section(route)
    }
}

/// Wrapper type to let the UrlSwitch do pre-work
pub type AppRouter = Router<UrlSwitch>;
/// Wrapper type to let the UrlSwitch do pre-work
pub type AppAnchor = RouterAnchor<UrlSwitch>;
