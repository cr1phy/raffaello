use raffaello::{draw, run, View};

#[component(App)]
fn app() -> View {
    draw! {
        p { "Hello, world" }
        block [title="blak"] {
            p [color="red"] { "In da blak" }
        }
    }
}

fn main() {
    run!(App, Inline)
}