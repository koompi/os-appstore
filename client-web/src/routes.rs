use crate::pages::{
    Add, AppByName, Apps, Assets, Categories, Home, Login, Settings, SignUpPage, Updates,
};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Switch, Debug, Clone, PartialEq)]
pub enum AppRoute {
    #[to = "/signup"]
    SignupPage,
    #[to = "/login"]
    LoginPage,
    #[to = "/updates"]
    UpdatesPage,
    #[to = "/settings"]
    SettingsPage,

    #[to = "/categories"]
    CategoriesPage,
    #[to = "/assets"]
    AssetsPage,
    #[to = "/apps/{name}"]
    AppByNamePage(String),
    #[to = "/apps"]
    AppsPage,
    #[to = "/add"]
    AddPage,
    #[to = "/"]
    IndexPage,
}

impl Default for AppRoute {
    fn default() -> Self {
        AppRoute::IndexPage
    }
}

pub struct Link {
    pub name: &'static str,
    pub route: AppRoute,
}

pub type AppRouteAnchor = RouterAnchor<AppRoute>;

#[derive(Debug, Clone)]
pub struct AppRouter {
    props: (),
    link: ComponentLink<Self>,
    page: String,
}
#[derive(Debug, Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    page: String,
}

pub enum Page {
    SwitchPage,
}

impl Component for AppRouter {
    type Message = Page;
    type Properties = ();
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            page: String::new(),
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let render_func = Router::render(|switch: AppRoute| match switch {
            AppRoute::IndexPage => html! {
                <Home />
            },
            AppRoute::AppsPage => html! {
                <Apps />
            },
            AppRoute::AddPage => html! {
                <Add />
            },
            AppRoute::AssetsPage => html! {
                <Assets />
            },
            AppRoute::CategoriesPage => html! {
                <Categories />
            },

            AppRoute::SettingsPage => html! {
                <Settings />
            },
            AppRoute::LoginPage => html! {
                <Login />
            },
            AppRoute::SignupPage => html! {
                <SignUpPage />
            },
            AppRoute::UpdatesPage => html! {
                <Updates />
            },
            AppRoute::AppByNamePage(s) => html! {
                <AppByName />
            },
        });

        html! {
            <Router<AppRoute, ()>
                render = render_func
            />
        }
    }
}