// macros4.rs
//
// Execute `rustlings hint macros4` or use the `hint` watch subcommand for a
// hint.


#[rustfmt::skip]
macro_rules! my_macro {
    ($x:expr) => {
        println!("Look at this other macro: {}", $x);
    };

    () => {
        println!("Check out my macro!");
    }
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
