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

// annoyingly, from needs a lifetime too since it is in
// the default return value
fn get_farthest<'a>(
    from: &'a String,
    servers: &'a BTreeMap<String, Vec<String>>,
) -> (&'a String, usize) {
    let mut longest: (&String, usize) = (from, 0);
    let mut path: Vec<(&String, usize)> = vec![(from, 0)];
    let mut visited: BTreeMap<&String, ()> = BTreeMap::new();

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
            match visited.get(next) {
                Some(_) => {
                    continue;
                }
                None => {
                    path.push((next, 0));
                }
            }
        }
    }

    longest
}

fn parse_input<R>(
    mut reader: csv::Reader<R>,
    columns: (usize, usize),
    mut servers: BTreeMap<String, Vec<String>>,
) -> BTreeMap<String, Vec<String>>
where
    R: std::io::Read,
{
    for result in reader.records() {
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

    servers
}

fn main() {
    let columns = get_args!((1, 2), usize);

    let input = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_reader(io::stdin());

    let servers = parse_input(input, columns, BTreeMap::new());

    println!("{:?}", servers);

    let (some_server, _) = servers.iter().next().expect("no servers found");
    let (server_a, _) = get_farthest(some_server, &servers);
    let (server_b, diameter) = get_farthest(server_a, &servers);

    println!("{} hops between {} and {}", diameter, server_a, server_b);
}
