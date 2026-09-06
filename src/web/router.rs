use yew::prelude::*;
use yew_router::prelude::*;

use crate::web::pages::{AppPage, GeneratePage, HomePage};

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/generate")]
    Generate,
    #[at("/app")]
    App,
    #[at("/")]
    Home,
}

fn switch(route: Route) -> Html {
    match route {
        Route::Generate => html! { <GeneratePage /> },
        Route::App => html! { <AppPage /> },
        Route::Home => html! { <HomePage /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
