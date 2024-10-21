use yew::prelude::*;

use crate::components::nav::Nav;

#[function_component(HomePage)]
pub fn home_page() -> Html {
    html! {
        <>
            <Nav />
            <section class="section-py first-section-pt">
                <div class="container">
                    <div class="row g-6">
                        <div class="col-lg-8 offset-lg-2">
                            <p>{ "Hello world!" }</p>
                        </div>
                    </div>
                </div>
            </section>
        </>
    }
}
