use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    home::HomePage, launching_into_computer_science::LaunchingIntoComputerSciencePage,
    object_oriented_programming::ObjectOrientedProgrammingPage, page_not_found::PageNotFoundPage,
    secure_software_development::SecureSoftwareDevelopmentPage,
};

#[derive(Clone, Debug, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/launching-into-computer-science/")]
    LaunchingIntoComputerScience,
    #[at("/object-oriented-programming/")]
    ObjectOrientedProgramming,
    #[not_found]
    #[at("/404")]
    PageNotFound,
    #[at("/secure-software-development/")]
    SecureSoftwareDevelopment,
}

fn switch(routes: Route) -> Html {
    match routes.clone() {
        Route::Home => {
            html! { <HomePage /> }
        }
        Route::LaunchingIntoComputerScience => {
            html! { <LaunchingIntoComputerSciencePage /> }
        }
        Route::ObjectOrientedProgramming => {
            html! { <ObjectOrientedProgrammingPage /> }
        }
        Route::PageNotFound => {
            html! { <PageNotFoundPage /> }
        }
        Route::SecureSoftwareDevelopment => {
            html! { <SecureSoftwareDevelopmentPage /> }
        }
    }
}

#[function_component(Router)]
pub fn router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
