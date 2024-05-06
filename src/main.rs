mod components;
mod pages;
mod router;
use router::Router;

fn main() {
    yew::Renderer::<Router>::new().render();
}
