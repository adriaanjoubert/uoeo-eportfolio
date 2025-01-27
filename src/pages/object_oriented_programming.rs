use yew::prelude::*;

use crate::components::nav::Nav;

#[function_component(ObjectOrientedProgrammingPage)]
pub fn object_oriented_programming_page() -> Html {
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
                                    { "Appraise and critically evaluate object-oriented programming compared to other programming paradigms." }
                                </li>
                                <li>
                                    { "Design and implement programs that demonstrate appropriate use of object-oriented design principles." }
                                </li>
                                <li>
                                    { "Select and use appropriate data structures for a given problem." }
                                </li>
                                <li>
                                    { "Propose object-oriented solutions using an appropriate modelling language, such as UML." }
                                </li>
                            </ul>
                            <h1>{ "Reflections" }</h1>
                            <h2>{ "Introduction" }</h2>
                            <p>
                                { "The reflections are written according to the four-stage model (University of Essex Online, N.D.) and are categorized by unit. " }
                            </p>
                            <h2>{ "Unit 1" }</h2>
                            <p>
                                { "In this unit I studied the concept of object-oriented programming in the context of Python. I read Padhy et al. (2017) and prioritised the reusability properties they identified and posted my conclusion in the discussion forum. I expected this unit to be straight-forward since I work with Python every day. Nevertheless this unit served to refresh my memory about important concepts like encapsulation and polymorphism which I regularly apply without thinking about it." }
                            </p>
                            <h3>{ "Artefacts" }</h3>
                            <ul>
                                <li>{ "Initial post: https://www.my-course.co.uk/mod/forum/discuss.php?d=269370#p508672" }</li>
                            </ul>
                            <h2>{ "Unit 2" }</h2>
                            <p>
                                { "I studied functional requirements and state machine diagrams. I responded to two peers' posts from unit 1 in the discussion forum. I reflected on how I have worked with project managers for the past five years and how their role as an interface between various stakeholders of the software product is a critical function in the business." }
                            </p>
                            <h3>{ "Artefacts" }</h3>
                            <ul>
                                <li>{ "Peer response: https://www.my-course.co.uk/mod/forum/discuss.php?d=261146#p508691" }</li>
                                <li>{ "Peer response: https://www.my-course.co.uk/mod/forum/discuss.php?d=261035#p508708" }</li>
                            </ul>
                            <h2>{ "Unit 3" }</h2>
                            <p>
                                { "I studied Unified Modelling Language (UML). I posted a summary of my peers' forum contributions from units 1 and 2 in the discussion forum. In the previous module (Secure Software Development) I also studied UML and found it surprisingly challenging. I rarely use UML at work. Mostly I encounter small subsets of it in open source software documentation." }
                            </p>
                            <h3>{ "Artefacts" }</h3>
                            <ul>
                                <li>{ "Summary post: https://www.my-course.co.uk/mod/forum/discuss.php?d=269389#p508744" }</li>
                            </ul>
                            <h2>{ "Unit 4" }</h2>
                            <p>
                                { "I continued studying UML and in particular, class diagrams." }
                            </p>
                            <h2>{ "Unit 5" }</h2>
                            <p>
                                { "I studied inheritance." }
                            </p>
                            <h2>{ "Unit 6" }</h2>
                            <p>
                                { "I studied interfaces. I reflected on how I use interfaces in Python at work by exposing a public interface through attributes and methods, and hiding private attributes and methods by prepending underscores to their names, and how this is pure convention and does not prevent client code from accessing these private attributes and methods. In comparison, languages like Java and Rust allow you to explicitly specify whether an attribute or method is public or private, and client code can only access the public interface." }
                            </p>
                            <h2>{ "Unit 7" }</h2>
                            <p>
                                { "I studied debugging, linting, data structures and search techniques. I finalized my system design document. I created the UML documents using Visual Paradigm. I reflected on how Visual Paradigm has a bit of a learning curve to it, even though I have already used it in the previous module (Secure Software Development)." }
                            </p>
                            <h2>{ "Unit 8" }</h2>
                            <p>
                                { "I studied sets and set operations and continued learning about search techniques, in particular, linear data search. I realized that I was already familiar with the new term symmetric difference; I had previously known it as disjoint union." }
                            </p>
                            <h2>{ "Unit 9" }</h2>
                            <p>
                                { "I studied packaging, PEP257 and software testing." }
                            </p>
                            <h2>{ "Unit 10" }</h2>
                            <p>
                                { "I studied unit testing, linting and documentation. I reflected on the fact that I have been using a test-driven development approach at work for several years and how this approach has become second-nature in most of the software projects that I work on." }
                            </p>
                            <h2>{ "Unit 11" }</h2>
                            <p>
                                { "I studied pointers and environmental considerations of software design. I had previously encountered pointers in The C Programming Language by Kernighan & Ritchie (1988), a book I really enjoyed working through. I realised that environmental considerations are often neglected when designing software, although there often are economic incentives that encourage the writing of efficient software, which is also better for the environment. This unit reminded me of a relatively new language that I have been learning called Rust, which allows developers to write performant and memory-safe code. I have used Rust to write small web applications which I normally would have written in Python. Although the learning curve is much steeper and the Rust ecosystem is much less \"batteries included\", it is a pleasurable language to write in and a programme that compiles successfully provides much satisfaction. I finalized my programming assignment and documented my code with comments and a README. I included unit and integration tests and included the flake8 linter to ensure the code style adheres to PEP8 (Alchin, 2010)." }
                            </p>
                            <h3>{ "Unit 12" }</h3>
                            <p>
                                { "I studied the factory design pattern. Again, I realized that I actually already use the factory pattern a lot already in library code without knowing it, and I could probably use more of it in my own code. I finalized my ePortfolio." }
                            </p>
                            <h3>{ "Artefacts" }</h3>
                            <ul>
                                <li>{ "ePortfolio: https://github.com/adriaanjoubert/uoeo-eportfolio/tree/aj-250127-object-oriented-programming" }</li>
                            </ul>
                            <h2>{ "References" }</h2>
                            <ul>
                                <li>{ "Alchin, M. (2010) 'PEP 8 Style Guide for Python', in: Alchin, M. Pro Python. United States: Apress L. P." }</li>
                                <li>{ "Padhy, N., Satapathy, S. & Singh, R.P. (2017) ‘State-of-the-Art Object-Oriented Metrics and Its Reusability: A Decade Review’, in: Smart Computing and Informatics. Singapore: Springer Singapore Pte. Limited (Smart Innovation, Systems and Technologies). 431–441. Available from: https://doi.org/10.1007/978-981-10-5544-7_42." }</li>
                                <li>{ "Padhy, N., Satapathy, S. & Singh, R.P. (2017) ‘State-of-the-Art Object-Oriented Metrics and Its Reusability: A Decade Review’, in: Smart Computing and Informatics. Singapore: Springer Singapore Pte. Limited (Smart Innovation, Systems and Technologies). 431–441. Available from: https://doi.org/10.1007/978-981-10-5544-7_42." }</li>
                                <li>{ "University of Essex Online (N.D.) A short guide to Reflective Writing." }</li>
                            </ul>
                        </div>
                    </div>
                </div>
            </section>
        </>
    }
}
