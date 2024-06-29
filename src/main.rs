use std::env::args;
use std::io;

use diameter::SpanningTree;

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

fn parse_input(mut reader: csv::Reader<impl io::Read>, columns: (usize, usize)) -> SpanningTree {
    let mut graph = SpanningTree::default();

    for result in reader.records() {
        let record = result.unwrap();
        let from = &record[columns.0];
        let to = &record[columns.1];

        graph.add_link(from, to);
    }

    graph
}

fn main() {
    let columns = get_args!((1, 2), usize);

    let input = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .flexible(true)
        .from_reader(io::stdin());

    let graph = parse_input(input, columns);

    let (diameter, server_a, server_b) = graph.diameter().expect("no input?");

    println!("{} hops between {} and {}", diameter, server_a, server_b);
}
