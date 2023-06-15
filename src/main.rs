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

macro_rules! make_ids {
    (($($name:expr),*), $lookup:expr, $nameslist:expr) => {
        ($({
            match $lookup.get(&$name) {
                Some(id) => *id,
                None => {
                    let newid = $nameslist.len();
                    $nameslist.push($name.clone());
                    $lookup.insert($name, newid);
                    newid
                }
            }
        },)*)
    };
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

        let (from, to) = make_ids!((from, to), namelookup, servernames);

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

    let (server_a, _) = get_farthest(0, &servers);
    let (server_b, diameter) = get_farthest(server_a, &servers);

    println!(
        "{} hops between {} and {}",
        diameter, servernames[server_a], servernames[server_b]
    );
}

#[test]
fn check_farthest() {
    let mut servers: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    servers.insert(0, vec![1, 3]);
    servers.insert(1, vec![0, 2]);
    servers.insert(2, vec![1]);
    servers.insert(3, vec![0, 4]);
    servers.insert(4, vec![3, 5]);
    servers.insert(5, vec![4]);
    let servers = servers;

    assert_eq!(get_farthest(0, &servers), (5, 3));
}

#[test]
#[should_panic(expected = "nonexistent server referenced")]
fn check_nonexist_server() {
    let mut servers: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    servers.insert(0, vec![1, 2, 3]);
    servers.insert(1, vec![0]);
    servers.insert(2, vec![0]);
    let servers = servers;

    get_farthest(0, &servers);
}
