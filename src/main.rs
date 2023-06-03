use csv;
use std::collections::BTreeMap;
use std::env::args;
use std::io;

macro_rules! get_arg {
    ($i:expr, $as:ty) => {
        args()
            .nth($i)
            .expect("missing column $i")
            .parse::<$as>()
            .expect("invalid column number")
    };
}

fn main() {
    let columns = (get_arg!(1, usize), get_arg!(2, usize));

    let mut input = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_reader(io::stdin());

    let mut servers: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for result in input.records() {
        let record = result.unwrap();
        let from = record[columns.0].to_string();
        let to = record[columns.1].to_string();

        match servers.get_mut(&to) {
            Some(connections) => {connections.push(from);}
            None => {servers.insert(to, vec![from]);}
        }
    }

    let servers = servers;

    println!("{:?}", servers);
}
