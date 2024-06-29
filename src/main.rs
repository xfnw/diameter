use std::collections::BTreeMap;
use std::env::args;
use std::io;

use diameter::{add_link, collect_servers, get_farthest};

macro_rules! get_args {
    (($($i:expr),*), $as:ty) => {
        ($({
            args()
                .nth($i)
                .expect(concat!(
                    "missing column ",
                    stringify!($i)
                ))
                .parse::<$as>()
                .expect(concat!(
                    "column ",
                    stringify!($i),
                    " is not valid"
                ))
        },)*)
    };
}

fn parse_input(
    mut reader: csv::Reader<impl io::Read>,
    columns: (usize, usize),
) -> (Vec<Vec<usize>>, Vec<String>) {
    let mut servers: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut namelookup: BTreeMap<String, usize> = BTreeMap::new();

    for result in reader.records() {
        let record = result.unwrap();
        let from = record[columns.0].to_string();
        let to = record[columns.1].to_string();

        add_link(from, to, &mut namelookup, &mut servers);
    }

    collect_servers(servers, namelookup)
}

fn main() {
    let columns = get_args!((1, 2), usize);

    let input = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .flexible(true)
        .from_reader(io::stdin());

    let (servers, servernames) = parse_input(input, columns);
    if servernames.is_empty() {
        return;
    }

    let (server_a, _) = get_farthest(0, &servers);
    let (server_b, diameter) = get_farthest(server_a, &servers);

    println!(
        "{} hops between {} and {}",
        diameter, servernames[server_a], servernames[server_b]
    );
}
