use raffaello::{draw, run};

fn main() {
    let c = draw! {
        p [color="dedde1" edf=12] { "Hello, world!" }
    };
    println!("{c:?}");

    run!(c, Alternative)
}
