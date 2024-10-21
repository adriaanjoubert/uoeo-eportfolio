use yew::prelude::*;

use crate::components::nav::Nav;

#[function_component(PageNotFoundPage)]
pub fn page_not_found_page() -> Html {
    html! {
        <>
            <Nav />
            <section class="section-py first-section-pt">
                <div class="container">
                    <div class="row g-6">
                        <div class="col-lg-8 offset-lg-2">
                            <p>{ "Page not found." }</p>
                        </div>
                    </div>
                </div>
            </section>
        </>
    }
}
