use yew::prelude::*;

use crate::components::nav::Nav;

#[function_component(LaunchingIntoComputerSciencePage)]
pub fn launching_into_computer_science_page() -> Html {
    html! {
        <>
            <Nav />
            <section class="section-py first-section-pt">
                <div class="container">
                    <div class="row g-6">
                        <div class="col-lg-8 offset-lg-2">
                            <h1>{ "Learning outcomes" }</h1>
                            <ul>
                                <li>
                                    { "Identify and explain the architecture, structure and functionality of basic components of computer system." }
                                </li>
                                <li>
                                    { "Demonstrate a critical understanding of core data structures and programming concepts, including algorithm computability." }
                                </li>
                                <li>
                                    { "Critically evaluate the functionality of different types of software, i.e., operating system, utility programs, languages and applications." }
                                </li>
                                <li>
                                    { "Critically appraise the emerging trends in the field, such as cloud computing, big data, cyber security, and the professional and ethical requirements for dealing with such contemporary computer-based technologies."}
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
            </section>
        </>
    }
}
