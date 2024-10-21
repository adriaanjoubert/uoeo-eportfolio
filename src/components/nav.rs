use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <nav class="layout-navbar shadow-none py-0">
            <div class="container">
                <div class="navbar navbar-expand-lg landing-navbar px-3 px-md-8">
                    <div class="navbar-brand app-brand demo d-flex py-0 me-4 me-xl-8">
                        <button class="navbar-toggler border-0 px-0 me-4" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                            <i class="tf-icons bx bx-menu bx-lg align-middle text-heading fw-medium"></i>
                        </button>
                        <Link<Route> classes={classes!("app-brand-link")} to={Route::Home}>
                            <span class="app-brand-text demo menu-text fw-bold ms-2 ps-1">{ "Home" }</span>
                        </Link<Route>>
                    </div>

                    <div class="collapse navbar-collapse landing-nav-menu" id="navbarSupportedContent">
                        <button class="navbar-toggler border-0 text-heading position-absolute end-0 top-0 scaleX-n1-rtl p-2" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                            <i class="tf-icons bx bx-x bx-lg"></i>
                        </button>
                        <ul class="navbar-nav me-auto">
                            <li class="nav-item">
                                <Link<Route> classes={classes!("nav-link", "fw-medium")} to={Route::LaunchingIntoComputerScience}>{ "Launching Into Computer Science" }</Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> classes={classes!("nav-link", "fw-medium")} to={Route::SecureSoftwareDevelopment}>{ "Secure Software Development" }</Link<Route>>
                            </li>
                            <li class="nav-item">
                                <Link<Route> classes={classes!("nav-link", "fw-medium")} to={Route::ObjectOrientedProgramming}>{ "Object-oriented Programming" }</Link<Route>>
                            </li>
                        </ul>
                    </div>
                </div>
            </div>
        </nav>
    }
}
