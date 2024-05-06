use yew::prelude::*;

use crate::components::nav::Nav;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <>
            <Nav />
            <p>{ "Hello world!" }</p>
        </>
    }
}
