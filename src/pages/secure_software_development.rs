use yew::prelude::*;

use crate::components::nav::Nav;

#[function_component(SecureSoftwareDevelopmentPage)]
pub fn secure_software_development_page() -> Html {
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
                                    { "Identify and manage security risks as part of a software development project." }
                                </li>
                                <li>
                                    { "Critically analyse development problems and determine appropriate methodologies, tools and techniques (including program design and development) to solve them." }
                                </li>
                                <li>
                                    { "Design and develop/adapt computer programs and to produce a solution that meets the design brief and critically evaluate solutions that are produced." }
                                </li>
                                <li>
                                    { "Systematically develop and implement the skills required to be effective member of a development team in a virtual professional environment, adopting real-life perspectives on team roles and organisation." }
                                </li>
                            </ul>
                            <h1>{ "Reflections" }</h1>
                            <h2>{ "Introduction" }</h2>
                            <p>
                                { "The reflections are written according to the four-stage model (University of Essex Online, N.D.) and are categorized by unit. " }
                            </p>
                            <h2>{ "Unit 1" }</h2>
                            <p>
                                { "In this unit I studied software development methodologies such as waterfall and agile. I understood that waterfall can slow down software development projects and that agile is preferred today. I learnt about Unified Modelling Language (UML) and secure software development standards such as OWASP. I created a UML diagram and posted it in the discussion forum. I learned how to use a new piece of software, namely Visual Paradigm, to create UML diagrams. I watched the seminar introducing the module and the team project." }
                            </p>
                            <h3>{ "Artefacts" }</h3>
                            <ul>
                                <li>{ "Initial post: https://www.my-course.co.uk/mod/forum/discuss.php?d=245616#p450277" }</li>
                            </ul>
                            <h2>{ "Unit 2" }</h2>
                            <p>
                                { "I studied the agile software development lifecycle (SDLC). I responded to two peers' posts from unit 1 in the discussion forum. I watched the seminar where the team development project design document and the SDLC were discussed. I attended a team meeting where we decided to create an online retailer application for the development team project. We discussed the required functionalities of the application and agreed on the next steps. I thought it was a good idea to hold our first meeting early in the module in order to understand the scope of the project requirements and to agree on regular future meetings to ensure we will meet the submission deadline." }
                            </p>
                            <h3>{ "Artefacts" }</h3>
                            <ul>
                                <li>{ "Peer response: https://www.my-course.co.uk/mod/forum/discuss.php?d=245583#p473222" }</li>
                                <li>{ "Peer response: https://www.my-course.co.uk/mod/forum/discuss.php?d=245446#p476152" }</li>
                            </ul>
                            <h2>{ "Unit 3" }</h2>
                            <p>
                                { "I studied programming language concepts using Python as an example. I posted a summary of my peers' forum contributions from units 1 and 2 in the discussion forum. A team discussion was held where we discussed questions relating to secure programming languages. I attended another team meeting where we discussed the requirements of the development team project and agreed on next steps. I felt that the teamwork component of the module makes studying online feel more presencial, even though we are all in different parts of the world. Looking at the participant list it is clear that not everyone participated in the forum (Collaborative Discussion 1: UML flowchart, N.D.; Enrolled users, N.D.), making it slightly challenging to hold a genuine discussion." }
                            </p>
                            <h3>{ "Artefacts" }</h3>
                            <ul>
                                <li>{ "Summary post: https://www.my-course.co.uk/mod/forum/discuss.php?d=256843#p477857" }</li>
                            </ul>
                            <h2>{ "Unit 4" }</h2>
                            <p>
                                { "I studied regular expressions and recursion and attended a team meeting where we divided responsibilities for the team development project design document and agreed on the scope of each section and next steps. I watched the seminar about programming language concepts. I find regular expressions and recursion to be some of the more challenging aspects of software development." }
                            </p>
                            <h2>{ "Unit 5" }</h2>
                            <p>
                                { "I studied software testing and software testing frameworks. I attended a team meeting where we worked together and completed some of the sections of the design document and agreed on next steps. I felt reassured that the design document would be ready by the submission deadline." }
                            </p>
                            <h2>{ "Unit 6" }</h2>
                            <p>
                                { "I studied Python linters and watched the seminar about using linters in Python testing. I attended a team meeting where we worked together and finalized the design document and prepared it for submission. Our team member submitted the development team project design document and I submitted a peer review form. Our team's discipline of meeting regularly paid off and our document was ready in time for the submission deadline without any last-minute scrambling." }
                            </p>
                            <h2>{ "Unit 7" }</h2>
                            <p>
                                { "I studied operating systems and did the activity about exploring the Python shell. I find the coding aspects of this module rather easy, given that I work with Python every day. However, I keep reminding myself that there are people from other walks of life taking this module who don't necessarily have a lot of programming experience, and for them such exercises may be very useful." }
                            </p>
                            <h2>{ "Unit 8" }</h2>
                            <p>
                                { "I studied cryptography and watched the related seminar. I read a security analysis report of TrueCrypt by Junestam & Guigo (2014) and created a forum post discussing the veracity of the statement \"Using TrueCrypt is not secure as it may contain unfixed security issues\"." }
                            </p>
                            <h3>{ "Artefacts" }</h3>
                            <ul>
                                <li>{ "Initial post: https://www.my-course.co.uk/mod/forum/discuss.php?d=259562#p483627" }</li>
                            </ul>
                            <h2>{ "Unit 9" }</h2>
                            <p>
                                { "I studied CRUD using Python. I responded to two of my peers' posts from unit 8 in the discussion forum." }
                            </p>
                            <h3>{ "Artefacts" }</h3>
                            <ul>
                                <li>{ "Peer response: https://www.my-course.co.uk/mod/forum/discuss.php?d=256426#p476683" }</li>
                                <li>{ "Peer response: https://www.my-course.co.uk/mod/forum/discuss.php?d=259583#p483716" }</li>
                            </ul>
                            <h2>{ "Unit 10" }</h2>
                            <p>
                                { "I studied system architectures including distributed systems and microservices.  I posted a summary of my peers' forum contributions from units 8 and 9 in the discussion forum. I watched the seminar about application programming interfaces (APIs)." }
                            </p>
                            <h3>{ "Artefacts" }</h3>
                            <ul>
                                <li>{ "Summary post: https://www.my-course.co.uk/mod/forum/discuss.php?d=259635#p483805" }</li>
                            </ul>
                            <h2>{ "Unit 11" }</h2>
                            <p>
                                { "I studied future trends in software development. I read the newsgroup debate between Linus Torvalds and Andrew Tanenbaum about microservices vs. monoliths in the context of operating systems. I created a forum post arguing against the statement \"Torvalds has been proven wrong and it only took nearly thirty years. Microservices and microkernels are the future.\" I responded to two of my peers' posts and created a post summarizing my peers' contributions to the discussion forum. I attended a team meeting where we discussed this statement and we all agreed that the statement is false, citing Linux as an example of a monolithic piece of software that is successful and on which many people collaborate. I submitted my development project along with a report document documenting the execution and output of the application. My implementation deviated from the design in some respect, e.g. we planned to use hashlib for encryption but I found a different library called argon2 which has an API I preferred over hashlib and so I used that instead. I also used the unittest testing framework instead of Pytest as planned because I am personally more familiar with unittest and found it easier to implement tests using unittest." }
                            </p>
                            <h3>{ "Artefacts" }</h3>
                            <ul>
                                <li>{ "Initial post: https://www.my-course.co.uk/mod/forum/discuss.php?d=259401#p483249" }</li>
                                <li>{ "Peer response: https://www.my-course.co.uk/mod/forum/discuss.php?d=259283#p483739" }</li>
                                <li>{ "Peer response: https://www.my-course.co.uk/mod/forum/discuss.php?d=259453#p483787" }</li>
                                <li>{ "Summary post: https://www.my-course.co.uk/mod/forum/discuss.php?d=259630#p483795" }</li>
                                <li>{ "Development project: https://github.com/adriaanjoubert/uoeo-ssd-project" }</li>
                            </ul>
                            <h3>{ "Unit 12" }</h3>
                            <p>
                                { "I continued studying the future trends in software development. I watched the seminar about microservices and microkernels. I collated my reflections and finalized my ePortfolio." }
                            </p>
                            <h3>{ "Artefacts" }</h3>
                            <ul>
                                <li>{ "ePortfolio: https://github.com/adriaanjoubert/uoeo-eportfolio/tree/aj-241024-secure-software-development" }</li>
                            </ul>
                            <h2>{ "References" }</h2>
                            <ul>
                                <li>{ "Collaborative Discussion 1: UML flowchart (N.D.) University of Essex Online." }</li>
                                <li>{ "Enrolled users (N.D.) University of Essex Online. Available from: https://www.my-course.co.uk/user/index.php?id=12059." }</li>
                                <li>{ "University of Essex Online (N.D.) A short guide to Reflective Writing." }</li>
                            </ul>
                        </div>
                    </div>
                </div>
            </section>
        </>
    }
}
