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

fn get_farthest(from: usize, servers: &BTreeMap<usize, Vec<usize>>) -> (usize, usize) {
    let mut longest: (&usize, usize) = (&from, 0);
    let mut path: Vec<(&usize, usize)> = vec![(&from, 0)];
    let mut visited: BTreeMap<&usize, ()> = BTreeMap::new();

    while let Some((current, i)) = path.pop() {
        // since we popped, no need to subtract 1 from length to
        // get number of hops
        let length = path.len();
        if length > longest.1 {
            longest = (current, length);
        }

        visited.insert(current, ());

        let connections = servers.get(current).expect("nonexistent server referenced");

        if i < connections.len() {
            path.push((current, i + 1));

            let next = &connections[i];
            if !visited.contains_key(next) {
                path.push((next, 0));
            }
        }
    }

    (*longest.0, longest.1)
}

fn parse_input(
    mut reader: csv::Reader<impl io::Read>,
    columns: (usize, usize),
) -> (BTreeMap<usize, Vec<usize>>, Vec<String>) {
    let mut servers: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut namelookup: BTreeMap<String, usize> = BTreeMap::new();
    let mut servernames: Vec<String> = vec![];

    for result in reader.records() {
        let record = result.unwrap();
        let from = record[columns.0].to_string();
        let to = record[columns.1].to_string();

        let from = match namelookup.get(&from) {
            Some(id) => *id,
            None => {
                let newid = servernames.len();
                servernames.push(from.clone());
                namelookup.insert(from, newid);
                newid
            }
        };
        let to = match namelookup.get(&to) {
            Some(id) => *id,
            None => {
                let newid = servernames.len();
                servernames.push(to.clone());
                namelookup.insert(to, newid);
                newid
            }
        };

        match servers.get_mut(&from) {
            Some(connections) => {
                connections.push(to);
            }
            None => {
                servers.insert(from, vec![to]);
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

    (servers, servernames)
}

fn main() {
    let columns = get_args!((1, 2), usize);

    let input = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .flexible(true)
        .from_reader(io::stdin());

    let (servers, servernames) = parse_input(input, columns);

    let (&some_server, _) = servers.iter().next().expect("no servers found");
    let (server_a, _) = get_farthest(some_server, &servers);
    let (server_b, diameter) = get_farthest(server_a, &servers);

    println!(
        "{} hops between {} and {}",
        diameter, servernames[server_a], servernames[server_b]
    );
}
