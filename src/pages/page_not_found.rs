use yew::prelude::*;

use crate::components::nav::Nav;

#[function_component(PageNotFoundPage)]
pub fn page_not_found_page() -> Html {
    html! {
        <>
            <Nav />
            <p>{ "Page not found." }</p>
        </>
    }
}
