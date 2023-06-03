use std::collections::BTreeMap;
use std::env::args;
use std::io;

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

fn get_farthest(from: &String) -> (&String, u32) {
    (from, 926)
}

fn main() {
    let columns = get_args!((1, 2), usize);

    let mut input = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_reader(io::stdin());

    let mut servers: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for result in input.records() {
        let record = result.unwrap();
        let from = record[columns.0].to_string();
        let to = record[columns.1].to_string();

        match servers.get_mut(&from) {
            Some(connections) => {
                connections.push(to.clone());
            }
            None => {
                servers.insert(from.clone(), vec![to.clone()]);
            }
        }

        match servers.get_mut(&to) {
            Some(connections) => {
                connections.push(from);
            }
            None => {
                servers.insert(to, vec![from]);
            }
        }
    }

    let servers = servers;

    println!("{:?}", servers);

    let (some_server, _) = servers.iter().next().expect("no servers found");
    let (server_a, _) = get_farthest(some_server);
    let (server_b, diameter) = get_farthest(server_a);
    println!("{} hops between {} and {}", diameter, server_a, server_b);
}
