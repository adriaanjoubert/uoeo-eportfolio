use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <ul>
            <li>
                <Link<Route> to={Route::Home}>
                    { "Home" }
                </Link<Route>>
            </li>
            <li>
                <Link<Route> to={Route::LaunchingIntoComputerScience}>
                    { "Launching Into Computer Science" }
                </Link<Route>>
            </li>
            <li>
                <Link<Route> to={Route::SecureSoftwareDevelopment}>
                    { "Secure Software Development" }
                </Link<Route>>
            </li>
            <li>
                <Link<Route> to={Route::ObjectOrientedProgramming}>
                    { "Object-oriented Programming" }
                </Link<Route>>
            </li>
        </ul>
    }
}
