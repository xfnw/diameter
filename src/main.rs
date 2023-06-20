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

fn get_farthest(from: usize, servers: &[Vec<usize>]) -> (usize, usize) {
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

        let connections = servers
            .get(*current)
            .expect("nonexistent server referenced");

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
    (($($name:expr),*), $lookup:expr) => {
        ($({
            match $lookup.get(&$name) {
                Some(id) => *id,
                None => {
                    let newid = $lookup.len();
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
) -> (Vec<Vec<usize>>, Vec<String>) {
    let mut servers: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    let mut namelookup: BTreeMap<String, usize> = BTreeMap::new();

    for result in reader.records() {
        let record = result.unwrap();
        let from = record[columns.0].to_string();
        let to = record[columns.1].to_string();

        let (from, to) = make_ids!((from, to), namelookup);

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

    let servers: Vec<Vec<usize>> = servers.into_values().collect();

    let mut servernames: Vec<String> = vec!["".to_string(); namelookup.len()];
    for (name, id) in namelookup.into_iter() {
        servernames[id] = name;
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

#[test]
fn check_farthest() {
    let servers: Vec<Vec<usize>> = vec![
        vec![1, 3],
        vec![0, 2],
        vec![1],
        vec![0, 4],
        vec![3, 5],
        vec![4],
    ];

    assert_eq!(get_farthest(0, &servers), (5, 3));
}

#[test]
#[should_panic(expected = "nonexistent server referenced")]
fn check_nonexist_server() {
    let servers: Vec<Vec<usize>> = vec![vec![1, 2, 3], vec![0], vec![0]];

    get_farthest(0, &servers);
}
