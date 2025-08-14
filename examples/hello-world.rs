use raffaello::draw;

fn main() {
    let c = draw! {
        p { "Hello, world!" }
    };
    println!("{c:?}");

    // run!(c)
}
