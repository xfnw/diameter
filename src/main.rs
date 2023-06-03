use std::collections::BTreeMap;
use std::env::args;

macro_rules! get_arg {
    ($i:expr, $as:ty) => {
        args()
            .nth($i)
            .expect("not enough args")
            .parse::<$as>()
            .expect("invalid column number")
    };
}

fn main() {
    let columns = (get_arg!(1, u16), get_arg!(2, u16));
    println!("{} and {}!", columns.0, columns.1);
}
