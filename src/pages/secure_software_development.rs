use yew::prelude::*;

use crate::components::nav::Nav;

#[function_component(SecureSoftwareDevelopmentPage)]
pub fn secure_software_development_page() -> Html {
    html! {
        <>
            <Nav />
            <h1>{ "Learning outcomes" }</h1>
            <ul>
                <li>
                    { "Identify and manage security risks as part of a software development project." }
                </li>
                <li>
                    { "Critically analyse development problems and determine appropriate methodologies, tools and techniques (including program design and development) to solve them." }
                </li>
                <li>
                    { "Design and develop/adapt computer programs and to produce a solution that meets the design brief and critically evaluate solutions that are produced." }
                </li>
                <li>
                    { "Systematically develop and implement the skills required to be effective member of a development team in a virtual professional environment, adopting real-life perspectives on team roles and organisation."}
                </li>
            </ul>
        </>
    }
}
