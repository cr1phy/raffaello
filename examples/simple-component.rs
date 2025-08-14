use raffaello::{Draw, draw};

#[component(App)]
fn app() -> Draw {
    draw! {
        p { "Hello, world" }
        block [title="blak"] {
            p [color="red"] { "In da blak" }
        }
    }
}

fn main() {
    run!(App)
}
